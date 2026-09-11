import json
import pathlib
import tempfile
import unittest

from sync_transport import Git, State, SyncError


class GitTransportTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.urls = {s: str(pathlib.Path(self.directory.name) / (s + ".git")) for s in ("od", "gh")}
        self.git = Git(self.urls, {"od": "fake", "gh": "fake"})
        for path in self.urls.values():
            self.git.run("init", "--bare", path)
        tree = self.git.run("mktree", data="")[0]
        self.root = self.git.run("commit-tree", tree, "-m", "Initial commit")[0]
        for side in self.urls:
            self.git.push(side, self.root, "refs/heads/main")

    def tearDown(self):
        self.git.temp.cleanup()
        self.directory.cleanup()

    def child(self, parent, text):
        blob = self.git.run("hash-object", "-w", "--stdin", data=text)[0]
        tree = self.git.run("mktree", data=f"100644 blob {blob}\tREADME.md\n")[0]
        return self.git.run("commit-tree", tree, "-p", parent, "-m", text)[0]

    def test_target_converges_fast_forward_in_either_direction(self):
        gh_commit = self.child(self.root, "GitHub merge")
        self.git.push("gh", gh_commit, "refs/heads/main")
        self.git.converge("main")
        self.assertEqual(self.git.head("od", "refs/heads/main"), gh_commit)
        od_commit = self.child(gh_commit, "OneDev merge")
        self.git.push("od", od_commit, "refs/heads/main")
        self.git.converge("main")
        self.assertEqual(self.git.head("gh", "refs/heads/main"), od_commit)

    def test_divergence_preserves_both_heads(self):
        heads = {s: self.child(self.root, s) for s in self.urls}
        for side, commit in heads.items():
            self.git.push(side, commit, "refs/heads/main")
        with self.assertRaisesRegex(SyncError, "diverged"):
            self.git.converge("main")
        for side, commit in heads.items():
            self.assertEqual(self.git.head(side, "refs/heads/main"), commit)

    def test_state_is_durable_and_excludes_code(self):
        state = State(self.git, True)
        state.acquire()
        state.data["pairs"].append({"od": 1, "gh": 2})
        state.save()
        state.release()
        loaded = State(self.git, False)
        self.assertEqual(loaded.data["pairs"], [{"od": 1, "gh": 2}])
        self.assertIsNone(loaded.data["lock"])
        self.assertEqual(self.git.run("ls-tree", "--name-only", loaded.sha)[0], "state.json")

    def test_two_workers_cannot_acquire_same_state(self):
        first, second = State(self.git, True), State(self.git, True)
        first.acquire()
        with self.assertRaises(SyncError):
            second.acquire()
        first.release()

    def test_live_lease_blocks_another_worker(self):
        first = State(self.git, True)
        first.acquire()
        with self.assertRaisesRegex(SyncError, "lease"):
            State(self.git, True).acquire()
        first.release()

    def test_read_only_never_creates_state_ref(self):
        state = State(self.git, False)
        state.acquire()
        state.save()
        state.release()
        self.assertIsNone(self.git.head("od", State.REF))

    def test_github_pr_head_is_copied_to_reserved_branch(self):
        head = self.child(self.root, "Fork contribution")
        self.git.push("gh", head, "refs/pull/1/head")
        branch = self.git.copy_pr_head("gh", 1)
        self.assertEqual(branch, "sync/github/pr-1")
        self.assertEqual(self.git.head("od", "refs/heads/" + branch), head)

    def test_changed_pipeline_is_never_imported(self):
        blob = self.git.run("hash-object", "-w", "--stdin", data="untrusted pipeline")[0]
        tree = self.git.run("mktree", data=f"100644 blob {blob}\t.onedev-buildspec.yml\n")[0]
        head = self.git.run("commit-tree", tree, "-p", self.root, "-m", "Pipeline change")[0]
        self.git.push("gh", head, "refs/pull/2/head")
        with self.assertRaisesRegex(SyncError, "CI definitions"):
            self.git.copy_pr_head("gh", 2)
        self.assertIsNone(self.git.head("od", "refs/heads/sync/github/pr-2"))


if __name__ == "__main__":
    unittest.main()
