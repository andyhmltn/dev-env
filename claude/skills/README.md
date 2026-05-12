# Claude Skills

Skills are reusable capabilities Claude auto-invokes when the situation matches. Unlike commands (which the user types), skills fire based on patterns Claude detects in the conversation.

Each skill lives in its own subdirectory with a `SKILL.md` file. The `os` TUI symlinks every subdirectory into `~/.claude/skills/`.

## Layout

```
claude/skills/
├── merge-worktree/    # Merge current worktree branch into main
├── genny-checks/      # Run lint + tsc + prettier in the genny monorepo
└── genny-install/     # Run pnpm install in genny with required auth tokens
```

## Skills

### merge-worktree

Fires when the user wants to merge their current worktree's branch into main. Verifies clean state, merges with no-ff, pushes, offers to clean up the worktree. Invoked by the `/merge-worktree` command (see `claude/commands/`).

### genny-checks

Fires when the user asks Claude to run checks, lint, typecheck, or prettier in `~/dev/genny`. Runs the canonical `pnpm run check:lint && pnpm run check:tsc && pnpm run check:prettier` from the monorepo root, parses output, and surfaces the first failure with file paths.

Enforces the rules in the user's CLAUDE.md: no `any` types, no eslint-disable comments, no code comments.

### genny-install

Fires when the user asks Claude to install or update dependencies in `~/dev/genny`. Exports the required `GITHUB_TOKEN` and `TIPTAP_PRO_TOKEN` before running `pnpm install`, since the registry calls fail without them.

## Conventions

Frontmatter must include both `name` and `description`. The description is the most important field — it's how Claude decides when to auto-invoke the skill. Make the description specific about *when* the skill should fire, not just *what* it does.

Skill bodies follow this rough shape:

1. **When to fire** — explicit triggers, in plain language
2. **Workflow** — numbered steps, each small and verifiable
3. **Rules** — invariants the skill must respect (often pulled from the user's CLAUDE.md)

Skills should never silently mutate state. Always confirm before destructive operations.

## See also

- [docs/claude-cockpit.md](../../docs/claude-cockpit.md) — user-facing reference for the cockpit work
- Anthropic's [Agent Skills documentation](https://docs.claude.com/en/docs/agents-and-tools/agent-skills)
