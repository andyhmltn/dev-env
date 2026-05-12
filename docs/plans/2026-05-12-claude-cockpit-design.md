# Claude Cockpit — design

Date: 2026-05-12
Status: accepted, implementation underway

## Goal

Collapse tool switching (Linear, GitHub, Slack, Datadog, Notion) into a small set of Claude commands that compose MCP servers into briefings and workflows. Codify recurring patterns as Claude skills so they fire automatically.

## Surfaces

Two surfaces, deliberate split:

- **Commands** (`claude/commands/*.md`) — verbs the user types. Short markdown files with optional YAML frontmatter and a prompt body that references MCP tools by name.
- **Skills** (`claude/skills/<name>/SKILL.md`) — reusable capabilities Claude auto-invokes when the situation matches. Frontmatter has `name` and `description`; body is a workflow.

Both directories are symlinked into `~/.claude/` by the `os` TUI (see `src/items.rs:204-243`). Adding a file requires no wiring; just re-run `./os` after creating it.

## Shared design principles

1. **Briefings, not feeds.** Aggregator commands produce 10–30 lines of curated text, never a dump. The point is to make a decision in one glance.
2. **Idempotent and re-runnable.** No commands that send or mutate without a confirm step.
3. **MCP-native.** Lean on `mcp__claude_ai_Linear__*`, `mcp__datadog__*`, `mcp__claude_ai_Slack__*`, etc. No re-implementing what MCPs already do.
4. **Composable.** `/today` is just `/linear-inbox` + `/gh-inbox` + `/slack-inbox` + `/dd-alerts` concatenated. Each sub-piece is independently useful.
5. **Codified in this repo.** Everything in `claude/commands/` and `claude/skills/`, symlinked by the TUI.

## Phase 1 — Daily situational awareness

Splits the existing daily-briefing routine (life + news + calendar, pushed to Slack each morning) from a new `/today` terminal command (engineering live state, pulled on demand).

### Routine (push, async, Slack DM)

Owns: calendar, email follow-ups, world/tech news, plus a new `Engineering overnight` section covering Linear / GH / Datadog deltas since end of last working day. Delivered by an existing claude.ai scheduled routine; updated prompt lives at `docs/daily-briefing-routine.md`.

### `/today` (pull, interactive, terminal)

Owns: live engineering state. Calls four sub-commands in parallel:

| Sub-command | MCP source | Output |
|---|---|---|
| `/linear-inbox` | `mcp__claude_ai_Linear__list_issues` filtered to assignee=me, state in (todo, in progress, in review) | ID, title, state, age, blockers |
| `/gh-inbox` | `gh pr list --author @me` + `gh pr list --search 'review-requested:@me'` + CI status | yours (CI + review state), reviews requested |
| `/slack-inbox` | `mcp__claude_ai_Slack__slack_search_public_and_private` for @me since last invocation | grouped by channel |
| `/dd-alerts` | `mcp__datadog__monitors` (triggered only) + `mcp__datadog__incidents` (open) | active alerts only |

Format: markdown headers, scannable in 5 seconds. Stale items (>30 days no movement) get a trailing `## Stale` section. Failing sub-command renders `## X — unavailable` instead of breaking the whole briefing.

## Phase 2 — Linear-aware issue workflow

Three commands that turn a Linear issue ID into a full working loop.

- **`/issue <ID>`** — context loader. `get_issue` + `list_comments` + linked PRs + local branches matching the ID. Pure read.
- **`/start-issue <ID>`** — loads context, suggests branch name from issue title, creates worktree via the existing `using-git-worktrees` skill, moves Linear to In Progress, prints worktree path + branch + URL.
- **`/ship-issue`** — detects current branch/worktree, verifies clean tree + green CI, generates PR title/body from commits + Linear description (leverages existing `review-gh` fish function), `gh pr create` with confirm, attaches PR back to Linear via `create_attachment`, moves to In Review.

Every mutating step is gated by a one-line confirm. Branch-name convention configurable in `claude/config/issue-workflow.toml` if needed later — start with hard-coded defaults.

## Phase 3 — Datadog-driven debugging

Two commands.

- **`/investigate <query>`** — symptom-to-hypothesis. Classifies input (trace ID / error string / service name), picks the right MCP entry point, aggregates past hour (override with `--since 6h`), cross-references monitors and incidents, produces a 30-line dossier ranking 2-3 hypotheses by evidence. Includes deep-links to Datadog.
- **`/incident <ID>`** — full-context dossier for an active incident: timeline, linked monitors, log sample, recent deploys, Slack thread (via `slack_read_thread` if external reference set).

Both are fire-on-demand, not auto-triggered.

## Phase 4 — Genny-tailored skills

Two skills that fire automatically when the user works inside `~/dev/genny`:

- **`genny-checks`** — auto-invokes when the user asks Claude to lint/typecheck/prettify. Runs the canonical `pnpm run check:lint && pnpm run check:tsc && pnpm run check:prettier`. Parses output, surfaces the first failure with file paths.
- **`genny-install`** — auto-invokes when the user runs `pnpm install` in genny. Exports `GITHUB_TOKEN=$(gh auth token)` and `TIPTAP_PRO_TOKEN=123` before running.

## Roadmap stubs (not in scope today)

- `/dd-trace <id>` deep dive (subset of `/investigate`)
- `/pr-review <pr#>` — pull a peer's PR, render diff + context, draft review comments
- `/notion-capture` — capture session decisions to a Notion page
- Genny skill expansion: monorepo navigation helpers, translation flow (already partially served by `deepl-translate` skill)

## File map

```
docs/plans/2026-05-12-claude-cockpit-design.md   (this file)
docs/claude-cockpit.md                            (user-facing reference)
docs/daily-briefing-routine.md                    (the routine prompt to paste)

claude/commands/README.md
claude/commands/today.md
claude/commands/linear-inbox.md
claude/commands/gh-inbox.md
claude/commands/slack-inbox.md
claude/commands/dd-alerts.md
claude/commands/issue.md
claude/commands/start-issue.md
claude/commands/ship-issue.md
claude/commands/investigate.md
claude/commands/incident.md

claude/skills/README.md
claude/skills/genny-checks/SKILL.md
claude/skills/genny-install/SKILL.md
```

## Install / activation

After files land in this repo:

1. Run `./os` to symlink new commands and skills into `~/.claude/`.
2. Restart Claude Code so it picks up the new command/skill manifest.
3. Try `/today` to verify MCP plumbing.
4. Paste the prompt from `docs/daily-briefing-routine.md` into the existing claude.ai routine.
