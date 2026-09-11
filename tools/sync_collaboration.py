#!/usr/bin/env python3
"""Bidirectional OneDev/GitHub issues and PR discussions. Read-only by default."""

import argparse
import hashlib
import json
import os
import sys
import uuid
from datetime import datetime, timezone

from sync_transport import API, Git, State, SyncError

ONEDEV = "https://onedev.hankin.io"
PROJECT = "rust/ferriswatch"
PROJECT_ID = 24
GITHUB = "Ozy-Viking/ferriswatch"
SIDES = ("od", "gh")


def other(side):
    return "gh" if side == "od" else "od"


def digest(value):
    return hashlib.sha256(value.encode()).hexdigest()


def choose(left, right, baseline):
    """Three-way comparison. An unresolved conflict never advances the baseline."""
    if left == right:
        return "equal"
    changed = [digest(left) != baseline["od"], digest(right) != baseline["gh"]]
    return {(True, False): "od", (False, True): "gh", (False, False): "unchanged"}.get(
        tuple(changed), "conflict")


def strip_footer(body, footer):
    body = body or ""
    if footer and not body.endswith(footer):
        raise SyncError("Sync attribution was removed or edited; restore it before continuing")
    return body[:-len(footer)] if footer else body


class Platforms:
    def __init__(self, od, gh):
        self.api = {"od": od, "gh": gh}
        self.users = {"od": od.call("GET", "users/me"), "gh": gh.call("GET", "user")}
        self.repo = "repos/" + GITHUB

    def inventory(self, kind, side):
        if side == "gh":
            rows = self.api[side].pages(f"{self.repo}/{kind}", github=True, state="all")
            if kind == "issues":
                rows = [row for row in rows if "pull_request" not in row]
        else:
            field = "Project" if kind == "issues" else "Target Project"
            rows = self.api[side].pages(kind, query=f'"{field}" is "{PROJECT}"')
        return {self.key(side, row): row for row in rows}

    @staticmethod
    def key(side, row):
        return row["number"] if side == "gh" else row["id"]

    def get(self, kind, side, key):
        prefix = self.repo + "/" if side == "gh" else ""
        return self.api[side].call("GET", f"{prefix}{kind}/{key}")

    @staticmethod
    def url(kind, side, row):
        if side == "gh":
            return row["html_url"]
        return f"{ONEDEV}/{PROJECT}/~{kind}/{row['number']}"

    def author(self, side, row):
        if side == "gh":
            return row["user"]["login"]
        user_id = row.get("submitterId", row.get("userId"))
        if user_id is None:
            raise SyncError("OneDev author ID missing")
        return self.api[side].call("GET", f"users/{user_id}")["name"]

    def is_bot(self, side, row):
        actor = row.get("user", {}).get("id") if side == "gh" else row.get(
            "submitterId", row.get("userId"))
        return actor == self.users[side]["id"]

    @staticmethod
    def body(side, row):
        return (row.get("body") if side == "gh" else row.get("description")) or ""

    @staticmethod
    def status(kind, side, row):
        if side == "gh":
            return row["state"]
        if kind == "issues":
            return "closed" if row["state"] == "Closed" else "open"
        return "open" if row["status"] == "OPEN" else "closed"

    @staticmethod
    def merged(side, row):
        return bool(row.get("merged_at")) if side == "gh" else row["status"] == "MERGED"

    @staticmethod
    def target(side, row):
        return row["base"]["ref"] if side == "gh" else row["targetBranch"]

    def values(self, pair, side, row):
        return {"title": row["title"], "body": strip_footer(self.body(side, row),
                pair["footers"][side]), "state": self.status(pair["kind"], side, row)}

    def update(self, pair, side, field, value):
        kind, key = pair["kind"], pair[side]
        if field == "body":
            value += pair["footers"][side]
        if side == "gh":
            self.api[side].call("PATCH", f"{self.repo}/{kind}/{key}", {field: value})
        else:
            root = f"{kind}/{key}"
            if field == "state":
                if kind == "issues":
                    self.api[side].call("POST", root + "/state-transitions", {
                        "state": "Closed" if value == "closed" else "Open", "fields": {}})
                else:
                    self.api[side].call("POST", root + (
                        "/discard" if value == "closed" else "/reopen"),
                        "State synchronized from GitHub. See the linked pull request.")
            else:
                self.api[side].call("POST", root + "/" + (
                    "description" if field == "body" else field), value)

    def create(self, pair, source, branch=None):
        origin, destination, kind = pair["origin"], other(pair["origin"]), pair["kind"]
        title = source["title"]
        body = self.body(origin, source) + pair["footers"][destination]
        if destination == "gh":
            data = {"title": title, "body": body}
            if kind == "pulls":
                data.update(head=branch, base=self.target(origin, source))
            result = self.api[destination].call("POST", f"{self.repo}/{kind}", data)
            return result["number"]
        data = {"title": title, "description": body}
        if kind == "issues":
            data.update(projectId=PROJECT_ID, confidential=False,
                        fields={"Type": "Task", "Priority": "Normal"})
        else:
            data.update(targetProjectId=PROJECT_ID, sourceProjectId=PROJECT_ID,
                        targetBranch=self.target(origin, source), sourceBranch=branch,
                        mergeStrategy="CREATE_MERGE_COMMIT", reviewerIds=[], assigneeIds=[])
        return self.api[destination].call("POST", kind, data)

    def comments(self, pair, side):
        key, kind = pair[side], pair["kind"]
        if side == "gh":
            return self.api[side].pages(f"{self.repo}/issues/{key}/comments", github=True)
        return self.api[side].call("GET", f"{kind}/{key}/comments")

    @staticmethod
    def comment_body(side, row):
        return row.get("body" if side == "gh" else "content") or ""

    def write_comment(self, pair, side, body, key=None):
        if side == "gh":
            path = (f"{self.repo}/issues/comments/{key}" if key else
                    f"{self.repo}/issues/{pair[side]}/comments")
            result = self.api[side].call("PATCH" if key else "POST", path, {"body": body})
            return result["id"]
        kind = "issue" if pair["kind"] == "issues" else "pull-request"
        path = kind + "-comments"
        if key:
            self.api[side].call("POST", f"{path}/{key}", body)
            return key
        return self.api[side].call("POST", path, {
            "issueId" if kind == "issue" else "requestId": pair[side],
            "userId": self.users[side]["id"], "content": body,
            "date": datetime.now(timezone.utc).isoformat()})


class Bridge:
    def __init__(self, platforms, git, state, apply):
        self.p, self.git, self.state, self.apply = platforms, git, state, apply
        self.errors = []

    def report(self, message):
        print(message, flush=True)

    def problem(self, message):
        self.errors.append(message)
        self.report("Needs attention: " + message)

    def new_pair(self, kind, origin, source):
        destination = other(origin)
        marker = "<!-- ferriswatch-sync:" + uuid.uuid4().hex + " -->"
        link = self.p.url(kind, origin, source)
        author = self.p.author(origin, source)
        # JSON quoting keeps attribution unambiguous, even for unusual login names.
        footer = f"\n\n---\nSynced from {link}, opened by {json.dumps(author)}.\n{marker}"
        return {"kind": kind, "origin": origin,
                origin: self.p.key(origin, source), destination: None,
                "footers": {origin: "", destination: footer}, "marker": marker,
                "baseline": {}, "comments": [], "attempted": False}

    def complete_pair(self, pair, inventory):
        origin, destination, kind = pair["origin"], other(pair["origin"]), pair["kind"]
        source = self.p.get(kind, origin, pair[origin])
        if source.get("confidential"):
            raise SyncError("Confidential OneDev issue is excluded")
        matches = [row for row in inventory[destination].values()
                   if self.p.body(destination, row).endswith(pair["footers"][destination])
                   and self.p.is_bot(destination, row)]
        if len(matches) > 1:
            raise SyncError("Multiple copies match the pending operation")
        if matches:
            pair[destination] = self.p.key(destination, matches[0])
        elif pair["attempted"]:
            raise SyncError("Earlier create has an unknown outcome; inspect before resetting attempted")
        else:
            branch = None
            if kind == "pulls":
                if self.p.status(kind, origin, source) != "open":
                    raise SyncError("Cannot create a historical PR after its branch was closed")
                self.git.converge(self.p.target(origin, source))
                branch = self.git.copy_pr_head(origin, source["number"])
            pair["attempted"] = True
            self.state.save()  # Persist intent before a potentially ambiguous POST.
            pair[destination] = self.p.create(pair, source, branch)
        # If creation succeeded but state persistence failed, recover from attribution.
        destination_row = self.p.get(kind, destination, pair[destination])
        destination_values = self.p.values(pair, destination, destination_row)
        for field in ("title", "body", "state"):
            # New objects begin open; let the original's closed state propagate next.
            base = destination_values[field]
            pair["baseline"][field] = {s: digest(base) for s in SIDES}
        self.state.save()
        inventory[destination][pair[destination]] = destination_row
        self.report(f"Linked {kind}: OneDev {pair['od']} ↔ GitHub {pair['gh']}")

    def sync_fields(self, pair):
        kind = pair["kind"]
        rows = {s: self.p.get(kind, s, pair[s]) for s in SIDES}
        if rows["od"].get("confidential"):
            raise SyncError("Issue is now confidential; public copy needs manual attention")
        if kind == "pulls":
            targets = {self.p.target(s, rows[s]) for s in SIDES}
            if len(targets) != 1:
                raise SyncError("PR target branches differ; reconcile them on the platforms")
            if self.apply:
                self.git.converge(next(iter(targets)))
                origin = pair["origin"]
                if self.p.status(kind, origin, rows[origin]) == "open":
                    self.git.copy_pr_head(origin, rows[origin]["number"])
                rows = {s: self.p.get(kind, s, pair[s]) for s in SIDES}
        values = {s: self.p.values(pair, s, rows[s]) for s in SIDES}
        for field in ("title", "body", "state"):
            result = choose(values["od"][field], values["gh"][field], pair["baseline"][field])
            if result == "conflict":
                self.problem(f"{kind} {pair['od']}/{pair['gh']}: concurrent {field} edits")
                continue
            if result in SIDES:
                destination = other(result)
                if field == "state" and kind == "pulls" and self.p.merged(destination, rows[destination]):
                    self.problem(f"PR {pair['od']}/{pair['gh']}: merged PR cannot be reopened")
                    continue
                self.report(f"{kind} {pair['od']}/{pair['gh']}: {field} {result} → {destination}")
                if not self.apply:
                    continue
                self.state.check_deadline()
                # Re-read immediately before writing. API writes lack cross-platform transactions.
                current = self.p.get(kind, destination, pair[destination])
                if self.p.values(pair, destination, current)[field] != values[destination][field]:
                    raise SyncError("Destination changed during sync; retry next run")
                self.p.update(pair, destination, field, values[result][field])
                values[destination][field] = values[result][field]
            if result != "unchanged" and self.apply:
                baseline = {s: digest(values[s][field]) for s in SIDES}
                if baseline != pair["baseline"][field]:
                    pair["baseline"][field] = baseline
                    self.state.save()

    def sync_comments(self, pair):
        rows = {s: {r["id"]: r for r in self.p.comments(pair, s)} for s in SIDES}
        for mapping in pair["comments"]:
            origin, destination = mapping["origin"], other(mapping["origin"])
            source = rows[origin].get(mapping[origin])
            if source is None:
                # Never delete discussion history on the other platform.
                continue
            body = self.p.comment_body(origin, source)
            if mapping[destination] is None:
                matches = [r for r in rows[destination].values()
                           if self.p.comment_body(destination, r).endswith(mapping["footer"])
                           and self.p.is_bot(destination, r)]
                if len(matches) != 1:
                    self.problem("Pending comment has an unknown outcome; inspect before retrying")
                    continue
                mapping[destination] = matches[0]["id"]
                self.state.save()
            copy = rows[destination].get(mapping[destination])
            if copy is None:
                continue  # Do not recreate deliberately removed copies.
            if digest(body) == mapping["baseline"]:
                continue
            copied = strip_footer(self.p.comment_body(destination, copy), mapping["footer"])
            if digest(copied) != mapping["baseline"]:
                self.problem("Both original and mirrored comment were edited; reconcile manually")
                continue
            self.report(f"Comment edit {origin} → {destination}")
            if self.apply:
                self.state.check_deadline()
                self.p.write_comment(pair, destination, body + mapping["footer"], mapping[destination])
                mapping["baseline"] = digest(body)
                self.state.save()
        known = {s: {m[s] for m in pair["comments"] if m[s] is not None} for s in SIDES}
        for origin in SIDES:
            destination = other(origin)
            for key, row in rows[origin].items():
                if key in known[origin]:
                    continue
                self.report(f"New comment {origin} → {destination}")
                if not self.apply:
                    continue
                self.state.check_deadline()
                author = self.p.author(origin, row)
                marker = "<!-- ferriswatch-comment:" + uuid.uuid4().hex + " -->"
                footer = f"\n\n---\nComment by {json.dumps(author)} on {origin}, ID {key}.\n{marker}"
                body = self.p.comment_body(origin, row)
                mapping = {"origin": origin, origin: key, destination: None,
                           "footer": footer, "baseline": digest(body)}
                pair["comments"].append(mapping)
                self.state.save()
                mapping[destination] = self.p.write_comment(pair, destination, body + footer)
                self.state.save()
                known[destination].add(mapping[destination])

    def run(self):
        for kind in ("issues", "pulls"):
            inventory = {s: self.p.inventory(kind, s) for s in SIDES}
            pairs = [p for p in self.state.data["pairs"] if p["kind"] == kind]
            for pair in pairs:
                if any(pair[s] is None for s in SIDES):
                    if self.apply:
                        try:
                            self.complete_pair(pair, inventory)
                        except SyncError as error:
                            self.problem(str(error))
            known = {s: {p[s] for p in pairs if p[s] is not None} for s in SIDES}
            # Snapshot inventory before creating copies so they cannot loop back.
            for origin in SIDES:
                for key, source in list(inventory[origin].items()):
                    if key in known[origin] or source.get("confidential"):
                        continue
                    if kind == "pulls" and self.p.status(kind, origin, source) != "open":
                        continue  # Only new/open PRs can be recreated faithfully.
                    self.report(f"New {kind} from {origin}, ID {key}")
                    if not self.apply:
                        continue
                    pair = self.new_pair(kind, origin, source)
                    self.state.data["pairs"].append(pair)
                    pairs.append(pair)
                    self.state.save()
                    try:
                        self.complete_pair(pair, inventory)
                        known[other(origin)].add(pair[other(origin)])
                    except SyncError as error:
                        self.problem(str(error))
            for pair in pairs:
                if any(pair[s] is None for s in SIDES):
                    continue
                try:
                    self.sync_fields(pair)
                    self.sync_comments(pair)
                except SyncError as error:
                    self.problem(f"{kind} {pair['od']}/{pair['gh']}: {error}")
        return not self.errors


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="Write changes to both platforms")
    args = parser.parse_args()
    state = None
    try:
        tokens = {"od": os.environ["ONEDEV_TOKEN"], "gh": os.environ["GITHUB_TOKEN"]}
        platforms = Platforms(API(ONEDEV + "/~api", tokens["od"]),
                              API("https://api.github.com", tokens["gh"]))
        git = Git({"od": ONEDEV + "/" + PROJECT + ".git",
                   "gh": "https://github.com/" + GITHUB + ".git"}, tokens)
        state = State(git, args.apply)
        state.acquire()
        result = Bridge(platforms, git, state, args.apply).run()
        print("Sync completed" if args.apply else "Read-only preflight completed")
        return 0 if result else 1
    except (SyncError, KeyError) as error:
        print(f"Sync stopped: {error}", file=sys.stderr)
        return 1
    finally:
        if state is not None:
            state.release()


if __name__ == "__main__":
    sys.exit(main())
