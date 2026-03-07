# Claude Configuration
Never write code comments. Ever

Before answering, compare my current branch to main to get an idea of my changes

DO NOT take shortcuts, use any type, fake implementation to get things working
YOU MUST ensure all changes are typesafe
YOU MUST ensure all prettier / tsc checks pass before a change is finished. When in genny you can do:

```
pnpm run check:lint && pnpm run check:tsc && pnpm run check:prettier
```

Do not use emdashes, ever

Never use eslint-disable-line or eslint-disable-next-line. Fix the underlying issue instead (e.g. use refs to break dependency cycles in useEffect).

Exception: Side effects are unavoidable for DB/API interactions. Isolate them and name clearly—createNewUser obviously has side effects, but calculateTotal should be pure.

Always use Zread MCP when I need library/API documentation, code generation, setup or configuration steps without me having to explicitly ask.

# Prefer Direct Code

Write the simplest code that solves today's problem. Avoid abstractions, cleverness, or "might be helpful later" patterns until real needs emerge. Direct code is easier to build, understand, and debug. Generalize later when actual patterns reveal themselves.

# Prefer Composition Over Inheritance

Favor "has-a" over "is-a" relationships. Build objects/functions from smaller reusable parts rather than class hierarchies. Inheritance locks you into early abstractions that are costly to change. Composition stays flexible with fewer assumptions.

# Prefer Pure Functions and Immutable Data

Favor functions that produce the same output given the same inputs. Avoid mutating data structures. Pure functions are isolated and repeatable; immutable data is set once. Both reduce cognitive load and places for bugs to hide.

# CLI Tools

Prefer these installed CLI tools over built-in equivalents when using Bash:

- `rg` (ripgrep) instead of grep -- respects .gitignore, faster. Use `rg -l` to list matching files, `rg -c` for counts, `rg --json` for structured output
- `fd` instead of find -- simpler syntax, respects .gitignore. Use `fd -e tsx` to find by extension, `fd -t f pattern` for files
- `duckdb` for analyzing CSV/JSON/Parquet data with SQL instead of chaining awk/sed/jq
- `xh` instead of curl -- cleaner output, separates headers/status/body. Use `xh GET url` or `xh POST url key=value`
- `semgrep` for static analysis and pattern matching across codebases. Use `semgrep --config auto .` or write custom rules
- `just` as task runner -- check for a Justfile before writing ad-hoc shell commands
- `watchexec` for watching file changes -- use `watchexec -e ts,tsx -- command` instead of polling loops
- `delta` for readable diffs -- use `git diff | delta --no-gitconfig --line-numbers` for structured diff output

When scoping searches, prefer `rg -l pattern` or `fd pattern` first, then Read specific files. This reduces token output compared to reading entire directories.

# Permissions

### Allow
- `Bash(find:*)`

### Deny
- None configured
