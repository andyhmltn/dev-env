# Claude Commands

Custom slash commands for Claude Code. Each `.md` file is a verb you can type in a Claude session.

The `os` TUI symlinks every `.md` file in this directory into `~/.claude/commands/`. Add a file, re-run `./os`, restart Claude Code, and the command is available.

## Layout

```
claude/commands/
├── today.md              # Daily live engineering briefing (aggregator)
│   ├── linear-inbox.md   #   Sub: Linear assigned to me
│   ├── gh-inbox.md       #   Sub: GitHub PRs (yours + review requested)
│   ├── slack-inbox.md    #   Sub: Slack DMs + @mentions
│   └── dd-alerts.md      #   Sub: Datadog firing monitors and open incidents
│
├── issue.md              # Linear: load full context for an issue ID
├── start-issue.md        # Linear: kick off work (worktree + In Progress)
├── ship-issue.md         # Linear: open PR + attach + move to In Review
│
├── investigate.md        # Datadog: symptom-to-hypothesis dossier
├── incident.md           # Datadog: full context brief for an active incident
│
└── merge-worktree.md     # Existing: merge current worktree to main
```

## Command groups

### Daily situational awareness

| Command | What it does |
|---|---|
| `/today` | Renders the four sub-inboxes plus a "suggested focus" line. Use first thing each day and whenever you need to know the current state of play. |
| `/linear-inbox` | Just Linear. Useful on its own. |
| `/gh-inbox` | Just GitHub PRs. Useful on its own. |
| `/slack-inbox` | Just Slack pings. Useful on its own. |
| `/dd-alerts` | Just Datadog. Useful on its own. |

### Linear workflow

| Command | What it does |
|---|---|
| `/issue <ID>` | Pure read. Loads description, comments, linked PRs, local branches matching the ID. |
| `/start-issue <ID>` | Load context, suggest branch name, create worktree, move issue to In Progress. Confirms before mutating. |
| `/ship-issue` | Detect current branch, verify clean tree + green CI, generate PR, attach to Linear, move to In Review. Confirms before each mutation. |

### Datadog debugging

| Command | What it does |
|---|---|
| `/investigate <query>` | Pass an error string, trace ID, or service name. Optional `--since 6h`. Produces a dossier with ranked hypotheses and Datadog deep-links. |
| `/incident <ID>` | Assembles a one-page brief for an active incident: timeline, monitors, deploys, logs, linked Slack. |

### Existing

| Command | What it does |
|---|---|
| `/merge-worktree` | Merges the current worktree branch into main and offers to clean up. See `claude/skills/merge-worktree/` for the workflow. |

## Conventions

- Frontmatter has a single `description:` line that surfaces in Claude's command picker.
- Prompt body is markdown, written as instructions to Claude (second person).
- Commands that hit external services (Linear, GH, Slack, Datadog) should always gracefully render `## <section> — unavailable` if the call fails, never fabricate data.
- Commands that mutate state require a confirm step.
- Commands should target output that fits in one screen (~50 lines).

## See also

- [docs/claude-cockpit.md](../../docs/claude-cockpit.md) — user-facing reference
- [docs/plans/2026-05-12-claude-cockpit-design.md](../../docs/plans/2026-05-12-claude-cockpit-design.md) — design rationale
- [docs/daily-briefing-routine.md](../../docs/daily-briefing-routine.md) — companion claude.ai routine prompt
