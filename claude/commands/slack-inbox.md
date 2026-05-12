---
description: Show Slack DMs and @mentions since my last invocation (or last 18 hours)
---

Show Slack messages that need my attention: DMs and @mentions.

Window: since my last invocation if known, otherwise the last 18 hours.

Use `mcp__claude_ai_Slack__slack_search_public_and_private` with a query that matches DMs to me plus `@me` mentions. Filter to messages where I have not already responded in the thread.

Render grouped by channel/DM:

```
## Slack
### #<channel-name>
- @<author>: <one-line excerpt> · <age>

### DM from @<author>
- <one-line excerpt> · <age>
```

Rules:
- Truncate excerpts to ~80 chars
- `<age>` humanised: `2h`, `1d`
- If I have already replied in the thread (look for my user ID after the message), skip it
- Skip bot messages unless they are deploy notifications addressed to me
- Cap output at 10 items total. If more exist, append `... and N more`.

If there is nothing, output `## Slack — nothing new`.

If the MCP call fails, output `## Slack — unavailable (<short error>)` and stop.

Never invent message content. If the search returns nothing, say so.
