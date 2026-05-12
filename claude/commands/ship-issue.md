---
description: Ship the current branch as a PR linked to its Linear issue, and move the issue to In Review
---

Ship the work in the current worktree as a pull request linked back to its Linear issue.

Steps:

1. Detect current branch and infer the Linear issue ID from the branch name (look for `<TEAM>-<NUM>` pattern). If absent, ask the user for the ID.

2. Verify the working tree:
   - `git status --porcelain` must be empty (no uncommitted changes).
   - `git log origin/main..HEAD --oneline` must show >0 commits.
   - Latest commit must be pushed; if not, `git push -u origin HEAD` after confirming.

3. Check CI on the latest commit:
   - `gh run list --branch <branch> --limit 1 --json status,conclusion`
   - If CI is red or still running, ask if the user wants to wait or ship anyway. Default to wait.

4. Generate PR title and body:
   - Title: `<ID>: <issue title>` (truncate to 70 chars total).
   - Body sections:
     - `## Summary` — derived from the commits and the Linear issue description.
     - `## Linear` — link to the issue.
     - `## Test plan` — bulleted checklist if you can infer one from the commits (test files touched, fixtures added), otherwise a stub for the user to fill.
   - If a `review-gh` fish function exists in `~/dev/dev-env/fish/functions/`, suggest using it instead, since the user already has a style there. Confirm which to use.

5. Show the proposed title and body, ask `Open PR? (y/edit/n)`. Accept edits.

6. On confirm:
   - `gh pr create --title '<title>' --body '<body>'` using a heredoc for the body.
   - Capture the PR URL.

7. Attach the PR to the Linear issue:
   - `mcp__claude_ai_Linear__create_attachment` with title `PR #<num>: <title>` and URL.

8. Move the Linear issue to `In Review`:
   - `mcp__claude_ai_Linear__get_issue_status` to find the state ID.
   - Confirm: `Move <ID> to In Review? (y/n)`.
   - On yes, `mcp__claude_ai_Linear__save_issue`.

9. Print:

```
Shipped <ID>

PR:     <url>
Linear: <url> (now In Review)
```

Rules:
- Never push without confirm. Never force-push.
- Never skip CI check unless the user explicitly says so.
- If the branch is already attached to a PR, do not open a second one. Instead, update the existing PR description if confirmed.
- This command stops at In Review; merging is handled separately by the `merge-worktree` skill once the PR is approved.
