---
description: Briefing of live engineering state - Linear, GitHub, Slack, Datadog
---

Produce a compact briefing of my current engineering state. Run the four sub-commands below in parallel, then concatenate their outputs under a single header.

Sub-commands (each is its own `claude/commands/*.md` file):

1. `/linear-inbox`
2. `/gh-inbox`
3. `/slack-inbox`
4. `/dd-alerts`

Header:

```
# Today · <Weekday DD Mon YYYY> · <HH:MM> London
```

Then the four sections in that order, each rendered by its sub-command.

Rules:
- If a sub-command renders `unavailable`, keep it in the output. Don't suppress.
- Total output should fit comfortably in one screen (target ~50 lines max).
- After the four sections, append a one-line `## Suggested focus` that picks the single highest-priority item across all four sources, based on this priority order:
  1. Datadog P1 incident
  2. PR with CI red authored by me
  3. PR awaiting my review opened >24h ago
  4. Linear issue in `review` state assigned to me with no movement in 3+ days
  5. Slack DM unreplied >4h
- If nothing matches the priority list, say `## Suggested focus — wide open, your call`.

Do not ask clarifying questions. Just produce the briefing.
