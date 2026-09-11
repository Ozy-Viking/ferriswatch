"""HTTP and isolated Git operations for the OneDev/GitHub bridge."""

import base64
import json
import os
import subprocess
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
import uuid


class SyncError(RuntimeError):
    pass


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None


class API:
    def __init__(self, base, token):
        self.base = base.rstrip("/")
        if not self.base.startswith("https://"):
            raise SyncError("API URLs must use HTTPS")
        self.token = token
        self.opener = urllib.request.build_opener(NoRedirect)

    def call(self, method, path, data=None, **query):
        url = self.base + "/" + path.lstrip("/")
        if query:
            url += "?" + urllib.parse.urlencode(query)
        headers = {"Authorization": "Bearer " + self.token,
                   "Accept": "application/json", "Content-Type": "application/json",
                   "User-Agent": "ferriswatch-sync"}
        req = urllib.request.Request(url, method=method, headers=headers,
                                     data=None if data is None else json.dumps(data).encode())
        try:
            with self.opener.open(req, timeout=45) as response:
                body = response.read()
                return json.loads(body) if body else None
        except urllib.error.HTTPError as error:
            # Response bodies can contain submitted private text or credentials.
            raise SyncError(f"{method} {path}: HTTP {error.code}") from None
        except (urllib.error.URLError, TimeoutError):
            # Never automatically retry a POST whose outcome is unknown.
            raise SyncError(f"{method} {path}: connection failed; outcome may be unknown") from None

    def pages(self, path, github=False, **query):
        result = []
        for page in range(1, 10001):
            pagination = {"page": page, "per_page": 100} if github else {
                "offset": (page - 1) * 100, "count": 100}
            batch = self.call("GET", path, **query, **pagination)
            if not isinstance(batch, list):
                raise SyncError(f"Unexpected page response from {path}")
            result.extend(batch)
            if len(batch) < 100:
                return result
        raise SyncError("Pagination limit exceeded; refusing an incomplete inventory")


class Git:
    """Never checks out or executes code from either remote."""

    def __init__(self, urls, tokens):
        self.temp = tempfile.TemporaryDirectory(prefix="ferriswatch-sync-")
        self.urls = urls
        self.env = dict(os.environ)
        for key in list(self.env):
            if key.startswith("GIT_"):
                del self.env[key]
        self.env.update(GIT_TERMINAL_PROMPT="0", GIT_CONFIG_NOSYSTEM="1",
                        GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_COUNT="4")
        for i, side in enumerate(("od", "gh")):
            self.env[f"GIT_CONFIG_KEY_{i}"] = f"http.{urls[side]}.extraHeader"
            encoded = base64.b64encode(("sync:" + tokens[side]).encode()).decode()
            self.env[f"GIT_CONFIG_VALUE_{i}"] = "Authorization: Basic " + encoded
        self.env.update(GIT_CONFIG_KEY_2="http.followRedirects", GIT_CONFIG_VALUE_2="false",
                        GIT_CONFIG_KEY_3="core.hooksPath", GIT_CONFIG_VALUE_3=os.devnull,
                        GIT_AUTHOR_NAME="Ferriswatch sync", GIT_COMMITTER_NAME="Ferriswatch sync",
                        GIT_AUTHOR_EMAIL="sync@ferriswatch.invalid",
                        GIT_COMMITTER_EMAIL="sync@ferriswatch.invalid")
        self.run("init", "--bare", ".")

    def run(self, *args, data=None, allowed=(0,)):
        proc = subprocess.run(["git", *args], input=data, text=True, capture_output=True,
                              cwd=self.temp.name, env=self.env, timeout=90)
        if proc.returncode not in allowed:
            raise SyncError(f"Git {args[0]} failed with exit code {proc.returncode}")
        return proc.stdout.strip(), proc.returncode

    def head(self, side, ref):
        out, _ = self.run("ls-remote", self.urls[side], ref)
        matches = [line.split()[0] for line in out.splitlines() if line.split()[1] == ref]
        return matches[0] if matches else None

    def fetch(self, side, ref):
        local = "refs/fetched/" + uuid.uuid4().hex
        self.run("fetch", "--no-tags", self.urls[side], f"+{ref}:{local}")
        return self.run("rev-parse", local)[0]

    def push(self, side, sha, ref, expected=None):
        args = ["push"]
        if expected is not None:
            args += [f"--force-with-lease={ref}:{expected}"]
        self.run(*args, self.urls[side], f"{sha}:{ref}")

    def converge(self, branch):
        """Advance a target branch only when one side is a strict ancestor."""
        self.run("check-ref-format", "refs/heads/" + branch)
        ref = "refs/heads/" + branch
        heads = {s: self.head(s, ref) for s in ("od", "gh")}
        if not all(heads.values()):
            raise SyncError(f"Target branch {branch} must exist on both platforms")
        if heads["od"] == heads["gh"]:
            return
        for side in heads:
            heads[side] = self.fetch(side, ref)
        for older, newer in (("od", "gh"), ("gh", "od")):
            _, code = self.run("merge-base", "--is-ancestor", heads[older], heads[newer],
                               allowed=(0, 1))
            if code == 0:
                self.push(older, heads[newer], ref)
                return
        raise SyncError(f"Target branch {branch} diverged; reconcile it before syncing PRs")

    def copy_pr_head(self, origin, number):
        source_ref = f"refs/{'pulls' if origin == 'od' else 'pull'}/{number}/head"
        sha = self.fetch(origin, source_ref)
        trusted = self.fetch("od", "refs/heads/main")
        pipeline_paths = (".onedev-buildspec.yml", ".github/workflows")
        # Creating a branch may trigger the destination's CI. Never import changed
        # pipeline definitions automatically, including pipelines from public forks.
        if self.run("ls-tree", sha, "--", *pipeline_paths)[0] != self.run(
                "ls-tree", trusted, "--", *pipeline_paths)[0]:
            raise SyncError("PR changes CI definitions; review and transfer it manually")
        destination = "gh" if origin == "od" else "od"
        branch = f"sync/{'onedev' if origin == 'od' else 'github'}/pr-{number}"
        ref = "refs/heads/" + branch
        previous = self.head(destination, ref)
        if previous != sha:
            self.push(destination, sha, ref, expected=previous or "")
        return branch


class State:
    """A private OneDev state ref, with compare-and-swap updates and a lease."""

    REF = "refs/heads/ferriswatch-sync-state"

    def __init__(self, git, apply):
        self.git, self.apply = git, apply
        self.owner = uuid.uuid4().hex
        self.start = time.monotonic()
        self.sha = git.head("od", self.REF)
        self.data = {"version": 1, "pairs": [], "lock": None}
        if self.sha:
            self.sha = git.fetch("od", self.REF)
            self.data = json.loads(git.run("show", self.sha + ":state.json")[0])
        if self.data.get("version") != 1:
            raise SyncError("Unsupported sync state version")
        self.locked = False

    def acquire(self):
        if not self.apply:
            return
        lock = self.data.get("lock")
        if lock and lock["expires"] > time.time():
            raise SyncError("Another sync owns the state lease; retry after it finishes")
        self.data["lock"] = {"owner": self.owner, "expires": time.time() + 1200}
        self.save()
        self.locked = True

    def check_deadline(self):
        if time.monotonic() - self.start > 480:
            raise SyncError("Sync reached its eight-minute deadline; next run can continue")

    def save(self):
        if not self.apply:
            return
        self.check_deadline()
        blob = self.git.run("hash-object", "-w", "--stdin",
                            data=json.dumps(self.data, sort_keys=True, indent=2) + "\n")[0]
        tree = self.git.run("mktree", data=f"100644 blob {blob}\tstate.json\n")[0]
        args = ["commit-tree", tree]
        if self.sha:
            args += ["-p", self.sha]
        sha = self.git.run(*args, "-m", "chore: persist collaboration sync state")[0]
        self.git.push("od", sha, self.REF, expected=self.sha or "")
        self.sha = sha

    def release(self):
        if self.locked:
            self.data["lock"] = None
            # A timed-out run leaves an expiring lease rather than writing stale state.
            self.save()
            self.locked = False
