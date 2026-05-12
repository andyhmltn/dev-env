---
description: List Linear issues currently assigned to me with state and age
---

List the Linear issues currently assigned to me, filtered to states that need my attention.

Use `mcp__claude_ai_Linear__list_issues` with:
- `assignee`: me
- `state`: any of `todo`, `in progress`, `in review`
- `limit`: 25
- order by updated descending

For each issue, render one line:

```
- <ID> <state> · '<title>' · <age>
```

Where:
- `<state>` is a short tag: `[todo]`, `[doing]`, `[review]`
- `<age>` is humanised: `2h`, `3d`, `2w`. Compute from `updatedAt`.

Group output:

```
## Linear (assigned to me)
<lines for items updated in the last 14 days>

## Stale (no movement 14d+)
<lines for older items>
```

If there are zero items in either group, omit that section header. If there are zero items overall, output `## Linear — nothing assigned`.

If the MCP call fails, output `## Linear — unavailable (<short error>)` and stop. Never fabricate issue data.
