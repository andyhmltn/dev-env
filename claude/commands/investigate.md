---
description: Datadog-driven symptom-to-hypothesis investigation
---

Investigate the issue described by `$ARGUMENTS`. Produce a dossier that ranks 2-3 plausible hypotheses by evidence.

Input parsing:

- If `$ARGUMENTS` contains `--since <duration>`, parse the duration (e.g. `6h`, `30m`) and use it as the lookback window. Default: 1 hour.
- Strip the `--since ...` flag from the remaining query.
- Classify the remaining query:
  - Pure hex/UUID-ish → treat as trace ID
  - Contains spaces and natural language → treat as error message / symptom
  - Single token matching a service naming pattern → treat as service name

Steps (run in parallel where possible):

1. Pull primary data based on classification:
   - **Trace ID:** `mcp__datadog__traces` for the trace, plus correlated logs.
   - **Error string:** `mcp__datadog__logs` with the string as query, grouped by service and error message.
   - **Service:** `mcp__datadog__logs` filtered to that service, grouped by status (error/warn).

2. Cross-reference:
   - `mcp__datadog__events` filtered to `tags:deploy` in the window plus 30 min before → recent deploys.
   - `mcp__datadog__monitors` filtered to alerting on the affected service(s).
   - `mcp__datadog__incidents` open, possibly related.

3. Pull metric deltas for the affected service(s):
   - p95 latency current vs. same window 24h ago
   - error rate current vs. same window 24h ago

4. Render the dossier:

```
# Investigation · <query>

## Symptom
<1-2 sentences summarising what is going wrong>

## Scope
- Service: <name(s)>
- Window: <window>
- Affected count: <N events / requests / users if available>

## Recent changes
- <deploy event 1 with time and link>
- <deploy event 2>

## Signals
- p95 latency: <current> (was <baseline>, <delta>)
- error rate: <current> (was <baseline>, <delta>)
- monitors firing: <list with deep-links>
- incidents open: <list>

## Hypotheses
1. **<hypothesis>** · evidence: <bullets> · suggested next step: <action with deep-link>
2. ...
```

Rules:
- Cap dossier at ~40 lines. Drop subsections that have no signal.
- Each hypothesis should be falsifiable - state what would confirm or deny it.
- Always include Datadog deep-links for the artefacts cited.
- If MCP calls fail, render whatever sections did succeed and note `<section> — unavailable` for the rest.
- After the dossier, offer: `Want me to file a Linear bug with this dossier as the description?`.
- Do not propose code fixes from the dossier alone. Hypotheses point at next-step queries or owners, not patches.
