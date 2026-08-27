# Homebrew

Package management for the dev environment. Source of truth: `homebrew/install.sh`.

## Scripts

### install.sh

Declarative package list. Running it installs only missing packages -- safe to run repeatedly.

```bash
./homebrew/install.sh
```

The os TUI runs this when you select "Homebrew Sync" from the menu.

Auto-installs Homebrew itself if not present.

### sync.sh

Detects packages installed on the system but not tracked in `install.sh`. Walks through each untracked package interactively:

```bash
./homebrew/sync.sh
```

Options per package:
- `y` -- add to install.sh
- `n` -- skip
- `c` -- add with a comment (e.g. `postgresql@16  # for local dev`)
- `q` -- quit

Uses `brew leaves` (formulae) and `brew list --cask` (casks) to compare against the arrays in `install.sh`.

## Formulae

| Package | Purpose |
|---------|---------|
| fish | Primary shell |
| fisher | Fish plugin manager |
| fzf | Fuzzy finder (files, history, directories) |
| go | Go programming language |
| lazygit | Terminal git UI |
| neovim | Text editor |
| nvm | Node version manager (legacy) |
| pnpm | Fast npm alternative |
| ripgrep | Fast text search (`rg`) |
| tmux | Terminal multiplexer |
| zoxide | Smart cd replacement (`z`) |
| biome | Linter/formatter |
| duckdb | Analytical SQL engine |
| fd | Fast file finder |
| fnm | Fast Node manager (primary) |
| gh | GitHub CLI |
| lua-language-server | LSP for Neovim Lua config |
| tailwindcss-language-server | Tailwind CSS LSP |
| tree | Directory tree viewer |
| uv | Python package/project manager |
| xh | HTTP client (curl alternative) |
| postgresql@16 | PostgreSQL database |
| prettierd | Prettier daemon for fast formatting |
| wget | HTTP downloads |
| xcodegen | Xcode project generator |
| ffmpeg | Video/audio processing |
| git-filter-repo | Git history rewriting |
| lftp | FTP client |
| libpq | PostgreSQL client library |
| poppler | PDF rendering library |

## Casks

| Package | Purpose |
|---------|---------|
| claude-code | Claude Code CLI |
| aerospace | Tiling window manager |

## TUI Integration

The os TUI (`src/homebrew.rs`) checks sync status on startup:
1. Parses `install.sh` for declared packages
2. Runs `brew leaves` and `brew list --cask` for installed packages
3. Shows sync/out-of-sync status in the menu
4. The "Homebrew Sync" action runs the interactive sync flow
