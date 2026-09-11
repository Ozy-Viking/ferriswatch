import copy
import unittest

from sync_collaboration import Bridge, Platforms, choose, digest, other, strip_footer
from sync_transport import SyncError


class MemoryState:
    def __init__(self):
        self.data = {"pairs": []}

    def save(self):
        pass

    def check_deadline(self):
        pass


class MemoryGit:
    def __init__(self):
        self.branches = []

    def converge(self, branch):
        self.branches.append(branch)

    def copy_pr_head(self, origin, number):
        return f"sync/{origin}/{number}"


class MemoryPlatforms(Platforms):
    def __init__(self):
        self.rows = {kind: {side: {} for side in ("od", "gh")} for kind in ("issues", "pulls")}
        self.discussions = {"od": {}, "gh": {}}
        self.writes = []
        self.fail_create_after_write = False

    def seed(self, side, kind="issues", **changes):
        key = len(self.rows[kind][side]) + 1
        row = {"id": key, "number": key, "title": "Example", "description": "Text",
               "body": "Text", "state": "Open" if side == "od" else "open",
               "status": "OPEN", "html_url": f"https://github.com/example/{key}",
               "bot": False, "confidential": False, "targetBranch": "main", "base": {"ref": "main"}}
        row.update(changes)
        self.rows[kind][side][key] = row
        return row

    def inventory(self, kind, side):
        return copy.deepcopy(self.rows[kind][side])

    def get(self, kind, side, key):
        return copy.deepcopy(self.rows[kind][side][key])

    def author(self, side, row):
        return "author"

    def is_bot(self, side, row):
        return row.get("bot", False)

    def create(self, pair, source, branch=None):
        origin = pair["origin"]
        destination = other(origin)
        body = self.body(origin, source) + pair["footers"][destination]
        row = self.seed(destination, pair["kind"], title=source["title"],
                        body=body, description=body, bot=True)
        self.writes.append(("create", destination))
        if self.fail_create_after_write:
            self.fail_create_after_write = False
            raise SyncError("Simulated timeout after server accepted create")
        return self.key(destination, row)

    def update(self, pair, side, field, value):
        row = self.rows[pair["kind"]][side][pair[side]]
        if field == "body":
            row["body" if side == "gh" else "description"] = value + pair["footers"][side]
        elif field == "state":
            row["state"] = value if side == "gh" else value.title()
            row["status"] = "OPEN" if value == "open" else "DISCARDED"
        else:
            row[field] = value
        self.writes.append(("update", side, field))

    def comments(self, pair, side):
        return copy.deepcopy(list(self.discussions[side].values()))

    def write_comment(self, pair, side, body, key=None):
        key = key or len(self.discussions[side]) + 1
        self.discussions[side][key] = {"id": key, "body": body, "content": body, "bot": True}
        self.writes.append(("comment", side))
        return key


class SyncTests(unittest.TestCase):
    def setUp(self):
        self.p, self.git, self.state = MemoryPlatforms(), MemoryGit(), MemoryState()
        self.bridge = Bridge(self.p, self.git, self.state, True)
        self.bridge.report = lambda message: None

    def run_sync(self, apply=True):
        bridge = Bridge(self.p, self.git, self.state, apply)
        bridge.report = lambda message: None
        return bridge.run()

    def test_issues_created_both_directions_without_feedback(self):
        self.p.seed("od", title="OneDev issue")
        self.p.seed("gh", title="GitHub issue")
        self.assertTrue(self.run_sync())
        self.assertEqual(len(self.state.data["pairs"]), 2)
        self.assertEqual(len(self.p.rows["issues"]["od"]), 2)
        self.assertEqual(len(self.p.rows["issues"]["gh"]), 2)
        writes = list(self.p.writes)
        self.assertTrue(self.run_sync())
        self.assertEqual(writes, self.p.writes)

    def test_destination_edits_flow_back_to_original(self):
        self.p.seed("od")
        self.run_sync()
        self.p.rows["issues"]["gh"][1]["title"] = "Edited on GitHub"
        self.assertTrue(self.run_sync())
        self.assertEqual(self.p.rows["issues"]["od"][1]["title"], "Edited on GitHub")

    def test_conflict_does_not_advance_baseline_or_overwrite(self):
        self.p.seed("od")
        self.run_sync()
        baseline = copy.deepcopy(self.state.data["pairs"][0]["baseline"]["title"])
        self.p.rows["issues"]["gh"][1]["title"] = "GitHub edit"
        self.p.rows["issues"]["od"][1]["title"] = "OneDev edit"
        self.assertFalse(self.run_sync())
        self.assertEqual(baseline, self.state.data["pairs"][0]["baseline"]["title"])
        self.assertEqual(self.p.rows["issues"]["gh"][1]["title"], "GitHub edit")
        self.p.rows["issues"]["od"][1]["title"] = "GitHub edit"
        self.assertTrue(self.run_sync())

    def test_create_timeout_recovers_without_duplicate(self):
        self.p.seed("gh")
        self.p.fail_create_after_write = True
        self.assertFalse(self.run_sync())
        self.assertTrue(self.run_sync())
        self.assertEqual(len(self.p.rows["issues"]["od"]), 1)
        self.assertEqual(len(self.state.data["pairs"]), 1)

    def test_missing_pending_create_is_not_blindly_retried(self):
        source = self.p.seed("gh")
        pair = self.bridge.new_pair("issues", "gh", source)
        pair["attempted"] = True
        self.state.data["pairs"].append(pair)
        self.assertFalse(self.run_sync())
        self.assertEqual(self.p.rows["issues"]["od"], {})

    def test_comment_copy_does_not_loop_and_edits_propagate(self):
        self.p.seed("od")
        self.p.discussions["od"][1] = {"id": 1, "content": "Hello", "bot": False}
        self.run_sync()
        count = len(self.p.writes)
        self.assertTrue(self.run_sync())
        self.assertEqual(len(self.p.writes), count)
        self.p.discussions["od"][1]["content"] = "Updated"
        self.run_sync()
        self.assertTrue(self.p.discussions["gh"][1]["body"].startswith("Updated"))

    def test_confidential_issue_never_exports(self):
        self.p.seed("od", confidential=True)
        self.assertTrue(self.run_sync())
        self.assertEqual(self.p.writes, [])

    def test_issue_made_confidential_stops_all_further_sync(self):
        self.p.seed("od")
        self.run_sync()
        self.p.rows["issues"]["od"][1]["confidential"] = True
        self.p.rows["issues"]["od"][1]["description"] = "Now private"
        self.p.discussions["od"][1] = {"id": 1, "content": "Private"}
        writes = list(self.p.writes)
        self.assertFalse(self.run_sync())
        self.assertEqual(self.p.writes, writes)

    def test_read_only_does_not_create_objects_or_state(self):
        self.p.seed("od")
        self.p.seed("gh", kind="pulls")
        self.assertTrue(self.run_sync(apply=False))
        self.assertEqual(self.p.writes, [])
        self.assertEqual(self.state.data["pairs"], [])
        self.assertEqual(self.git.branches, [])

    def test_closure_and_reopening_both_ways(self):
        self.p.seed("od", state="Closed")
        self.run_sync()
        self.assertEqual(self.p.rows["issues"]["gh"][1]["state"], "closed")
        self.p.rows["issues"]["gh"][1]["state"] = "open"
        self.run_sync()
        self.assertEqual(self.p.rows["issues"]["od"][1]["state"], "Open")

    def test_intermediate_onedev_state_survives_unrelated_edits(self):
        self.p.seed("od", state="In Progress")
        self.run_sync()
        self.p.rows["issues"]["gh"][1]["title"] = "Updated"
        self.run_sync()
        self.assertEqual(self.p.rows["issues"]["od"][1]["state"], "In Progress")

    def test_pr_creation_and_closure_do_not_merge(self):
        self.p.seed("gh", kind="pulls")
        self.assertTrue(self.run_sync())
        self.p.rows["pulls"]["gh"][1]["state"] = "closed"
        self.p.rows["pulls"]["gh"][1]["merged_at"] = "2026-09-11"
        self.assertTrue(self.run_sync())
        self.assertEqual(self.p.rows["pulls"]["od"][1]["status"], "DISCARDED")

    def test_attribution_removal_is_visible(self):
        with self.assertRaises(SyncError):
            strip_footer("edited", "\nfooter")

    def test_equal_concurrent_edits_converge(self):
        baseline = {side: digest("old") for side in ("od", "gh")}
        self.assertEqual(choose("same", "same", baseline), "equal")


if __name__ == "__main__":
    unittest.main()
