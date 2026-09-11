# OneDev and GitHub collaboration sync

`sync_collaboration.py` links `rust/ferriswatch` on OneDev with
`Ozy-Viking/ferriswatch` on GitHub. It uses only Python's standard library and Git.
The crate itself does not depend on this tooling.

## Operation

The OneDev `Sync collaboration` job runs the trusted code on `main` every five
minutes. It requires job secrets `ONEDEV_TOKEN` and `GITHUB_TOKEN`. The OneDev
identity needs permission to manage issues and PRs and write code in this project.
The GitHub token needs Contents, Issues, and Pull requests read/write access to
this repository. Restrict these secrets to trusted builds on `main` in OneDev.

New issues and open PRs are copied in either direction. Titles, descriptions, and
open/closed state can be edited on either platform. Concurrent changes to the
same field stop that field's sync and fail the job with a conflict message. Set
the field to the same value on both sides to resolve the conflict.

Discussion comments are copied with author attribution. Edits to an original
comment update its copy, unless somebody also edited the copy. To reply, add a
new comment on either platform. Deleted comments and objects are not deleted on
the other platform. Issue numbers and user identities remain platform-specific.
Markdown and attachment links are preserved verbatim; attachment files, labels,
assignees, milestones, review approvals, and inline code reviews are not copied.

OneDev's `Open`, `In Progress`, `Blocked`, and `In Review` states map to GitHub
`open`. `Closed` maps to `closed`. Reopening from GitHub sets OneDev to `Open`.
Existing intermediate OneDev states survive unrelated updates.

Confidential OneDev issues are excluded. If an already copied issue becomes
confidential, further synchronization stops and reports that its existing public
copy needs manual attention. Making an issue confidential cannot retract text
already published to GitHub.

## Pull requests

Each mirrored PR gets a reserved source branch, `sync/github/pr-N` on OneDev or
`sync/onedev/pr-N` on GitHub. Only the original PR controls that branch. Rebased
source commits update the reserved branch using a Git lease. Do not develop on
these generated branches. New/open PRs are supported; historical closed PRs are
not recreated.

The target branch must exist on both platforms. Before syncing a PR, the bridge
fast-forwards whichever target branch is behind. Diverged targets require manual
reconciliation. A merge must be performed by a person on one platform. The bridge
can transfer the resulting commits and close the counterpart, but never calls
either platform's merge or approval API. The counterpart may be displayed as
closed/discarded rather than merged.

The bridge uses a bare Git repository and does not check out PR code. It refuses
automatic transfer if `.onedev-buildspec.yml` or `.github/workflows` differs from
trusted OneDev `main`, since creating a branch can trigger CI. Review and transfer
those PRs manually. Keep future trusted pipeline definitions suitable for imported
branches, and keep secret-bearing jobs limited to trusted `main` builds.

## State and recovery

The OneDev-only `ferriswatch-sync-state` branch stores object mappings, attribution
markers, content hashes, and a worker lease in `state.json`. It contains no build
spec or credentials. Do not delete, mirror, or rewrite this branch. The bridge
uses compare-and-swap Git pushes and a twenty-minute lease to prevent concurrent
writers; jobs stop after ten minutes. After a killed worker, the next run waits
for its lease to expire.

Create intent is saved before posting an issue, PR, or comment. If a response is
lost, the next run recovers the created object using its attribution marker and
the authenticated account's author ID. It never blindly repeats an ambiguous
create. If no copy is found, inspect both platforms and the pending mapping.
Only after verifying that no object was created, reset an object's `attempted`
flag to `false`, or remove a pending comment mapping, in the state branch while
the job is stopped. Preserve all other mappings. Do not remove sync attribution
from generated descriptions or comments.

Cross-platform APIs do not offer a shared transaction. The bridge rechecks a
destination field immediately before writing it, but an edit in the final
read/write window can still race. The platform's edit history is the recovery
source for that case.

## Verification

```sh
python -m unittest discover -s tools -p 'test_sync*.py'
python tools/sync_collaboration.py          # Read-only API and state preflight
python tools/sync_collaboration.py --apply  # Apply changes with a state lease
```

Credentials come from the environment. Never put them in arguments or files in
this repository. HTTP errors report status and endpoint, without response bodies.

API references: [OneDev REST API](https://docs.onedev.io/restful-api),
[GitHub issues](https://docs.github.com/en/rest/issues/issues),
[GitHub PRs](https://docs.github.com/en/rest/pulls/pulls).
