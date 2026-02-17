# Claude Configuration
Never write code comments. Ever

Before answering, compare my current branch to main to get an idea of my changes

DO NOT take shortcuts, use any type, fake implementation to get things working
YOU MUST ensure all changes are typesafe
YOU MUST ensure all prettier / tsc checks pass before a change is finished. When in genny you can do:

```
pnpm run check:lint && pnpm run check:tsc && pnpm run check:prettier
```

Exception: Side effects are unavoidable for DB/API interactions. Isolate them and name clearly—createNewUser obviously has side effects, but calculateTotal should be pure.

Always use Zread MCP when I need library/API documentation, code generation, setup or configuration steps without me having to explicitly ask.

# Prefer Direct Code

Write the simplest code that solves today's problem. Avoid abstractions, cleverness, or "might be helpful later" patterns until real needs emerge. Direct code is easier to build, understand, and debug. Generalize later when actual patterns reveal themselves.

# Prefer Composition Over Inheritance

Favor "has-a" over "is-a" relationships. Build objects/functions from smaller reusable parts rather than class hierarchies. Inheritance locks you into early abstractions that are costly to change. Composition stays flexible with fewer assumptions.

# Prefer Pure Functions and Immutable Data

Favor functions that produce the same output given the same inputs. Avoid mutating data structures. Pure functions are isolated and repeatable; immutable data is set once. Both reduce cognitive load and places for bugs to hide.

# Long-Running Agent Workflow

For complex, multi-session projects use these skills:

## Skills Available

1. **initializer** - Run ONCE at project start
   - Sets up feature_list.json with all requirements
   - Creates init.sh startup script
   - Initializes claude-progress.txt log
   - Makes initial git commit

2. **coding-agent** - Run REPEATEDLY for development
   - Gets oriented by reading progress files and git log
   - Works on ONE feature at a time
   - Tests thoroughly before marking features complete
   - Commits work and updates progress log
   - Leaves codebase in clean, working state

3. **test-agent** - Run for thorough testing
   - Uses browser automation for end-to-end testing
   - Tests user flows as a real user would
   - Documents bugs with screenshots
   - Only marks features passing if truly working

## Usage Pattern

```bash
# First session - setup environment
/initializer
"Build a [description of your app]"

# Development sessions - repeat as needed
/coding-agent

# Testing sessions - verify features
/test-agent
```

## Key Files Created

- `feature_list.json` - All features with pass/fail status
- `claude-progress.txt` - Session-by-session progress log
- `init.sh` - Script to start dev environment
- `test-results.md` - Detailed test reports (created by test-agent)

## Benefits

- Work incrementally across many context windows
- Never lose track of progress or forget features
- Clean git history with recovery points
- Thorough testing catches bugs early
- Clear handoffs between sessions

# Permissions

### Allow
- `Bash(find:*)`

### Deny
- None configured
