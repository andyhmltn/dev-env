---
description: Start work on a Linear issue - load context, create worktree, move to In Progress
---

Start work on Linear issue `$ARGUMENTS`. This is a mutating workflow with confirms at each step.

Steps:

1. Run `/issue $ARGUMENTS` first to load and display full context.

2. Suggest a branch name:
   - Format: `<initials>/<id-lower>-<slugified-title-truncated-50>`. Example: `ah/eng-432-fix-translation-fallback`.
   - Derive `<initials>` from git config user.name (lowercase, first letters).
   - Slugify the issue title: lowercase, replace non-alphanum with `-`, collapse repeats, trim.
   - Show the proposed name and ask: `Use branch name '<name>'? (y/edit/n)`. Accept overrides.
   - If `n`, stop.

3. Create a worktree using the existing `superpowers:using-git-worktrees` skill. Invoke that skill with the chosen branch name.

4. After worktree is created, move the Linear issue to `In Progress`:
   - `mcp__claude_ai_Linear__get_issue_status` to find the correct state ID for the issue's team.
   - Confirm: `Move <ID> to In Progress? (y/n)`.
   - On yes, `mcp__claude_ai_Linear__save_issue` with the new state.

5. Print a summary:

```
Started <ID>

Worktree: <path>
Branch:   <name>
Linear:   <url>

cd <path>
```

Rules:
- If any step fails, stop and report. Do not silently skip.
- Do not create a draft PR up-front; that happens at `/ship-issue` time.
- If the issue is already `In Progress`, skip step 4 and note it.
- If the worktree skill is unavailable, fall back to `git worktree add ../<branch> -b <branch>` and confirm before running.
