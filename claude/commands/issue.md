---
description: Load full Linear issue context (description, comments, linked PRs, local branches) into the session
---

Load full context for Linear issue `$ARGUMENTS`. Pure read - never mutate state.

Steps:

1. Parse `$ARGUMENTS` as a Linear issue identifier (e.g. `ENG-432`). If it does not look like one, ask for the ID and stop.

2. In parallel:
   - `mcp__claude_ai_Linear__get_issue` for the issue body, state, assignee, priority, project, milestone, labels.
   - `mcp__claude_ai_Linear__list_comments` for the last 10 comments.
   - `mcp__claude_ai_Linear__list_diffs` filtered to this issue, to find linked PRs.
   - `git branch --all --list '*<id>*'` to find local/remote branches matching the ID (case-insensitive).
   - `gh pr list --search '<id> in:title,body'` to find any PRs that reference the ID but are not formally linked.

3. Render:

```
# Linear <ID> · <title>

**Status:** <state> · **Assignee:** <name> · **Priority:** <p> · **Project:** <project> · **Milestone:** <ms>

## Description
<rendered markdown, preserve formatting>

## Recent comments
- @<author> <age>: <one-line summary>
  ...

## Linked PRs
- #<num> '<title>' · <ci> · <review-state>
  ...

## Local references
- branch: <name>
- pr (unlinked): #<num> '<title>'
```

Rules:
- If a section has no items, omit it.
- Comment summaries: keep to one line each; full body available if user asks.
- For PR CI/review state, follow the same convention as `/gh-inbox`.
- Output is read-only context priming. Do not propose changes unless asked.

After rendering, state explicitly:

> Context loaded. What do you want to do with this issue?
