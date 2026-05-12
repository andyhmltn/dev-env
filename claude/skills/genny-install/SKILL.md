---
name: genny-install
description: Run pnpm install in the genny monorepo with the required auth tokens exported. Use whenever the user asks Claude to install dependencies, add a package, or update lockfile in ~/dev/genny.
---

# Genny Install

## When to fire

Auto-invoke whenever the user asks Claude to:

- Install or update dependencies in `~/dev/genny` or any subpackage
- Add a package: "add @foo/bar to frontend"
- Update the lockfile
- Run `pnpm install` for any reason in genny

## The problem this solves

The genny monorepo requires two auth tokens to be present in the environment before `pnpm install` will succeed:

- `GITHUB_TOKEN` — for private GitHub Packages
- `TIPTAP_PRO_TOKEN` — for the Tiptap Pro package registry

If either is missing, install fails with a confusing 401 from the registry.

## Workflow

### Step 1: Locate the monorepo root

Walk up from the current directory until you find `pnpm-workspace.yaml`. Run install from there unless the user specified a subpackage.

### Step 2: Export the tokens, then install

Always run install as a single shell invocation with the tokens exported in the same line. Tokens do not persist between Bash tool calls.

```bash
export GITHUB_TOKEN=$(gh auth token); export TIPTAP_PRO_TOKEN=123; pnpm install
```

For adding a package:

```bash
export GITHUB_TOKEN=$(gh auth token); export TIPTAP_PRO_TOKEN=123; pnpm --filter <pkg> add <dep>
```

### Step 3: Sanity check

After install completes, check that `node_modules` exists and contains a top-level entry for the new or updated package. If the install succeeded but the package is missing, something is wrong - report and stop.

## Rules

- Never run `pnpm install` in genny without the token exports - it will fail.
- Never commit the actual `TIPTAP_PRO_TOKEN` value to the repo. The placeholder `123` here comes from the user's CLAUDE.md - the real token must be present in the environment some other way (e.g. shell rc file).
- Never run install with `--no-frozen-lockfile` unless the user explicitly asks. Lockfile drift is a separate issue.
- If `gh auth token` returns nothing, stop and tell the user they need to `gh auth login`.
