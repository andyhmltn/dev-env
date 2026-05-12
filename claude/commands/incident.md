---
description: Assemble full context for an active Datadog incident
---

Build a single-page context brief for incident `$ARGUMENTS`.

Steps (run in parallel):

1. `mcp__datadog__incidents` to fetch the incident: status, severity, commander, customer impact, timeline, linked Slack channel.

2. For each linked monitor on the incident: `mcp__datadog__monitors` to fetch current state and recent transitions.

3. Pull a sample of logs from the affected service(s) in the 30 minutes around incident start:
   - `mcp__datadog__logs` filtered to service(s) and `status:error`
   - Cap at 20 lines, deduplicated by message template

4. Recent deploys in the 2 hours preceding incident start:
   - `mcp__datadog__events` filtered to `tags:deploy` and the affected service(s)

5. If the incident has a linked Slack channel, pull the most recent 30 messages with `mcp__claude_ai_Slack__slack_read_channel`. Summarise into a 5-line timeline.

Render:

```
# Incident <ID> · <severity> · <title>

**Status:** <status> · **Commander:** @<commander> · **Opened:** <age>
**Customer impact:** <yes/no, summary if yes>

## Timeline (from incident)
<bullets>

## Slack timeline (last 30 messages)
<bullets, condensed>

## Linked monitors
- <name> · <state> · last transition <age>

## Recent deploys
- <service> · <sha or version> · deployed <age> · by @<author>

## Error sample
```
<log lines, plain>
```

## Open questions
<bullets - what is unknown that would help resolve>
```

Rules:
- Total output target ~80 lines. Truncate aggressively.
- If incident has no Slack channel, omit that section.
- Never speculate about cause. This brief is for situational awareness, not diagnosis.
- Include Datadog deep-link for the incident page.
- If incident is already resolved, render anyway but prepend `**Incident resolved <age>** - this is a post-hoc brief.`.
