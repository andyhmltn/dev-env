# Daily Briefing Routine

This is the prompt for the scheduled claude.ai routine that DMs Andy a daily briefing each morning. It is not used by Claude Code locally - copy the prompt below into the routine on claude.ai.

## Why a separate document

The routine runs in the cloud as a scheduled agent, not in this repo's local Claude Code setup. Keeping the prompt in version control lets us evolve it deliberately (instead of editing it in a web textbox and losing the diff).

The routine is the companion to the [Claude Cockpit](./claude-cockpit.md) `/today` command:

- Routine = morning push to Slack: calendar, news, follow-ups, **plus overnight engineering deltas**.
- `/today` = on-demand terminal pull: live engineering state.

## How to update

Open the routine in claude.ai, replace the prompt with the version below, save.

## Prompt

```
Build a daily briefing for Andy and DM it to his Slack.

Steps:

1. Determine today's date and the previous weekday in Europe/London (Fri if today is Mon, otherwise yesterday).

2. Fetch today's calendar via the Microsoft 365 connector using outlook_calendar_search with query "*", afterDateTime set to today 00:00 UTC, beforeDateTime set to tomorrow 00:00 UTC, limit 50. Convert all times to London local (BST or GMT as appropriate). List cancelled events as struck-through at the bottom of the calendar section; flag tentative ones.

3. Fetch the previous-weekday's emails via outlook_email_search with query "followup OR follow-up OR action OR approval OR pending OR overdue", afterDateTime previous-weekday 00:00 UTC, beforeDateTime previous-weekday 23:59 UTC. Extract follow-up items: approvals pending, Linear/Rippling notifications, partnership threads, unread monthly updates. Ignore promotional mail.

4. For each meeting today, generate 1-3 short prep bullets drawn from the meeting subject, attendees, and any email thread from the previous 7 days referencing the same people or topic.

5. Fetch engineering overnight state. Window: from 17:00 London the previous weekday to now.
   a. Linear (mcp__claude_ai_Linear__list_issues, assignee=me, updated in window): any issues that moved state (e.g. to In Review), any newly assigned issues, any new comments from someone else on my issues.
   b. GitHub: PRs I authored where CI changed state in the window (run `gh pr list --author @me --json number,title,statusCheckRollup,updatedAt --limit 20` and filter by updatedAt). PRs where my review was newly requested in the window (`gh pr list --search "review-requested:@me sort:updated-desc" --limit 10` and filter by updatedAt).
   c. Datadog (mcp__datadog__monitors, mcp__datadog__incidents): any monitor that triggered in the window, any incident opened in the window. Active state only - resolved alerts skipped unless they were P1+.

6. Run two WebSearch queries:
   - "world news headlines <today's date>"
   - "top tech AI news <today's date>"
   Summarise each into 4-5 bullets.

7. Compose the briefing in the exact structure below.

8. Send it as a Slack DM to user U08RD32164Q via the Slack connector's slack_send_message tool. Use plain markdown (no HTML entities like &amp;, no blockquote lines, no emoji shortcodes - literal emojis are fine). If Slack rejects the message with invalid_blocks, strip remaining emojis and retry.

Structure:

*Daily Briefing — <Weekday DD Mon YYYY>*

*Today's calendar* (London time)
<bulleted list: HH:MM-HH:MM — Subject (location or attendees)>

*Prep needed*
<bulleted list tied to today's meetings>

*Follow-ups from <previous weekday date>*
<bulleted list of open threads, approvals, unread items>

*Engineering overnight*
<bulleted list. Subgroup by Linear / GitHub / Datadog only if any subgroup has more than one item. If all three are empty, say "nothing moved overnight">

*World news*
<4-5 bullets>

*Tech news*
<4-5 bullets>

Rules:

- If the calendar has no events, say "No meetings today" rather than skipping the section.
- Keep total length under 3500 characters.
- Do not ask any clarifying questions. Run end-to-end and send.
- Do not include sources or footers. Andy wants a clean briefing.
- For the engineering section: if a sub-source fails (e.g. Datadog rate-limited), include the other sub-sources and add "Datadog: unavailable" as the last line. Do not skip the whole section.
```

## Diff from the previous version

The new section is `*Engineering overnight*` (step 5 in the prompt). It adds three sub-sources:

1. **Linear** deltas - state moves on my issues, new assignments, new comments by others.
2. **GitHub** deltas - my PRs whose CI changed in the window, new review requests.
3. **Datadog** alerts/incidents that opened in the window.

Everything else (calendar, email follow-ups, news) is unchanged from the previous routine.

The character cap was bumped from 3000 to 3500 to fit the new section. If overall length becomes a problem in practice, trim news from 4-5 bullets to 3.
