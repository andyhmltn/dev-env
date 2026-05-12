# Claude Cockpit

A set of Claude Code commands and skills that pull Linear, GitHub, Slack, and Datadog into one terminal surface, plus genny-specific skills that fire when working in the monorepo.

## Why

Daily friction is context switching between tabs (Linear, Slack, GitHub, Datadog) and remembering the right env tokens / commands for genny. The cockpit closes both gaps.

## What you get

### Daily situational awareness

| Surface | Mode | Owns |
|---|---|---|
| Existing claude.ai routine | Push, Slack DM, async | Calendar, email follow-ups, world/tech news, plus a new "Engineering overnight" section (Linear / GH / DD deltas since end of last working day) |
| `/today` | Pull, terminal, sync | Live Linear assignments, live PR state, Slack pings since last invocation, live Datadog alerts |

The routine is the morning push. `/today` is the during-the-day pull. They share MCP queries; format is the same shape so muscle memory transfers.

### Linear workflow

| Command | Use when |
|---|---|
| `/issue <ID>` | Want full Linear context (description + comments + linked PRs + local branches) loaded into Claude before deciding anything. Pure read. |
| `/start-issue <ID>` | Ready to start work. Loads context, suggests a branch name, creates a worktree, moves the issue to In Progress. |
| `/ship-issue` | Work is done. Verifies clean tree + green CI, drafts a PR title and body from your commits + Linear, opens the PR, attaches it back to Linear, moves to In Review. |

`/merge-worktree` (already present) handles the final merge step after the PR is approved.

### Datadog debugging

| Command | Use when |
|---|---|
| `/investigate <query>` | Something is wrong. Pass an error string, trace ID, or service name. Optional `--since 6h`. You get a dossier with the affected scope, recent deploys, metric deltas, and 2-3 ranked hypotheses, each with Datadog deep-links. |
| `/incident <ID>` | Just got paged. Assembles a single-page brief with the timeline, linked monitors, recent deploys, error sample, and Slack thread summary. |

### Genny skills (auto-invoked)

| Skill | Fires when |
|---|---|
| `genny-checks` | You ask Claude to lint, typecheck, prettier-check, or pre-commit-verify in genny. Runs the canonical `pnpm run check:lint && pnpm run check:tsc && pnpm run check:prettier` from the monorepo root and surfaces failures with file paths. |
| `genny-install` | You ask Claude to install or update dependencies in genny. Exports `GITHUB_TOKEN` and `TIPTAP_PRO_TOKEN` before running `pnpm install`. |

## Installation

The cockpit lives entirely in this repo. The `os` TUI symlinks `claude/commands/*.md` into `~/.claude/commands/` and `claude/skills/<name>/` into `~/.claude/skills/<name>/`. After pulling new files:

```bash
./os
```

Then restart Claude Code so it picks up the new manifest.

## Verifying it works

After install, try:

```
/today
```

You should see four sections (Linear, GitHub, Slack, Datadog) plus a "Suggested focus" line. If any section says `unavailable`, the relevant MCP server probably needs to be reconnected from claude.ai or the local CLI auth needs refreshing (`gh auth status`).

## Updating the morning routine

The routine lives on claude.ai, not in this repo (it runs as a scheduled remote agent). To update it, copy the prompt from [docs/daily-briefing-routine.md](./daily-briefing-routine.md) and paste it into the existing routine.

## Extending

When you find yourself doing the same MCP dance more than twice, codify it.

- Aggregator command (multiple MCPs, briefing format) → look at `/today` and its sub-commands for the pattern
- Single MCP wrapper (briefing format) → look at `/linear-inbox` or `/dd-alerts`
- Mutating workflow (with confirms) → look at `/start-issue` and `/ship-issue`
- Tooling that should fire automatically → make it a skill, not a command. See `claude/skills/genny-checks/` for the shape.

## References

- [Design document](./plans/2026-05-12-claude-cockpit-design.md) — architecture and rationale
- [claude/commands/README.md](../claude/commands/README.md) — per-command details
- [claude/skills/README.md](../claude/skills/README.md) — per-skill details
- [daily-briefing-routine.md](./daily-briefing-routine.md) — the morning routine prompt
