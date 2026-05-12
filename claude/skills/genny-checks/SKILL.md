---
name: genny-checks
description: Run lint + tsc + prettier checks in the genny monorepo using the canonical command. Use whenever the user asks Claude to lint, typecheck, format-check, or run pre-commit checks in ~/dev/genny.
---

# Genny Checks

## When to fire

Auto-invoke whenever the user asks Claude to run checks, lint, typecheck, prettier, or pre-commit verification in a directory under `~/dev/genny` (or any subpackage like `~/dev/genny/frontend`).

Triggers include: "run the checks", "is this typesafe", "format check", "lint this", "fix prettier", "ready to commit".

## Workflow

### Step 1: Locate the monorepo root

Walk up from the current directory until you find a `pnpm-workspace.yaml`. That is the genny root. Run the checks from there - they are workspace-aware.

If the current directory is not under a pnpm workspace, do not use this skill.

### Step 2: Run the canonical command

```bash
pnpm run check:lint && pnpm run check:tsc && pnpm run check:prettier
```

Run from the monorepo root. The `&&` is intentional - stop on first failure so the user sees the highest-priority issue first.

### Step 3: Parse output and surface failures

For each failing step, render:

```
## <step> failed

<first 20 lines of relevant output, file paths preserved>

Affected files:
- <path>:<line>
- ...
```

Drop verbose pnpm prefixes (`> @scope/pkg ...`) and turbo output lines that are not actionable.

### Step 4: Propose next action

- For lint failures: suggest `pnpm run check:lint --fix` if the failures look auto-fixable.
- For tsc failures: do not propose --fix; offer to read the failing file and propose a typesafe fix.
- For prettier failures: suggest `pnpm run fix:prettier` (if defined) or `pnpm exec prettier --write <files>`.

Never apply fixes without confirming with the user first.

## Rules

- All three checks must pass before declaring done. Per the user's CLAUDE.md: "YOU MUST ensure all prettier / tsc checks pass before a change is finished".
- Do not use `any` to suppress tsc errors. Do not use `eslint-disable` comments to suppress lint errors. Fix the underlying issue.
- If prettier itself misbehaves (stale daemon), suggest `killall prettierd` per the user's auto-memory note.
- Do not write code comments. Per the user's CLAUDE.md: "Never write code comments. Ever".
