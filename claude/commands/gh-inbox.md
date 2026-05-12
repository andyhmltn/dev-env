---
description: List my open PRs plus PRs awaiting my review, with CI and review state
---

List GitHub PRs that need my attention. Two groups: PRs I authored, and PRs where my review is requested.

Use the `gh` CLI via Bash (no MCP needed):

```
gh pr list --author @me --state open --json number,title,headRefName,statusCheckRollup,reviewDecision,updatedAt,url --limit 20
gh pr list --search "review-requested:@me state:open" --json number,title,author,statusCheckRollup,updatedAt,url --limit 20
```

Run both in parallel. Render two sections:

```
## GitHub (yours)
- #<number> <ci> · <review-state> · '<title>' · <age>

## GitHub (review requested of you)
- #<number> @<author> · <ci> · '<title>' · <age>
```

Where:
- `<ci>` is one of `CI green`, `CI red`, `CI running`, `CI ?` derived from `statusCheckRollup` (look at the conclusions of the latest check runs)
- `<review-state>` for yours: `approved`, `changes requested`, `awaiting review`, derived from `reviewDecision` (null = awaiting)
- `<age>` humanised from `updatedAt`: `2h`, `3d`, `2w`

If a group is empty, omit the section. If both empty, output `## GitHub — clean`.

If `gh` fails (auth, network), output `## GitHub — unavailable (<short error>)` and stop.

Do not list draft PRs unless they have CI red or review activity. Skip dependabot PRs unless CI red.
