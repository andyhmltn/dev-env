---
description: Show currently triggered Datadog monitors and open incidents
---

Show Datadog alerts that are currently firing or open.

Use:
- `mcp__datadog__monitors` filtered to `state: alert` (triggered only)
- `mcp__datadog__incidents` filtered to `status: active`

Run in parallel. Render:

```
## Datadog
### Monitors
- <severity> · <monitor-name> · <service or scope> · firing <age>

### Incidents
- <severity> · <title> · commander @<commander> · opened <age>
```

Where:
- `<severity>` is `P1`/`P2`/etc. for incidents, `critical`/`warn` for monitors
- `<age>` humanised: `12m`, `2h`
- `<service or scope>` taken from the monitor tags

Rules:
- If both empty, output `## Datadog — all clear`
- If only one of the two has items, omit the empty subsection header
- Cap each subsection at 10 items
- Sort by severity descending, then age descending

Include deep-link Datadog URLs only when there is exactly one item in a subsection (otherwise it gets noisy). Use `mcp__datadog__monitors` output's URL field if present.

If the MCP call fails, output `## Datadog — unavailable (<short error>)` and stop.

Never fabricate alerts.
