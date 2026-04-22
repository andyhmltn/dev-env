# Mac OS Dev Setup

## Quick Start

```bash
git clone <repo-url> ~/dev/dev-env
cd ~/dev/dev-env
./setup.sh
```

## What It Does

`setup.sh` runs, in order:

1. `homebrew/install.sh` (installs any missing formulae and casks)
2. `neovim/setup.sh` (symlinks nvim config)
3. `tmux/setup.sh` (symlinks tmux config)
4. `fish/setup.sh` (symlinks fish config)
5. `claude/setup.sh` (symlinks Claude Code config)

## Structure

```
dev-env/
├── setup.sh                      # Main setup script
├── homebrew/
│   ├── install.sh                # Installs missing formulae and casks
│   └── sync.sh                   # Syncs installed packages back to install.sh
├── neovim/
│   ├── setup.sh                  # Symlinks nvim config
│   ├── init.lua
│   ├── lua/
│   └── snippets/
├── tmux/
│   ├── setup.sh                  # Symlinks tmux config
│   ├── .tmux.conf
│   └── tmux-cd.sh
├── fish/
│   ├── setup.sh                  # Symlinks fish config
│   ├── config.fish
│   ├── fish_plugins
│   ├── fish_variables
│   ├── functions/
│   └── themes/
├── ghostty/
│   ├── setup.sh                  # Symlinks ghostty config (run manually)
│   └── config
├── zsh/
│   ├── setup.sh
│   └── .zprofile
├── claude/
│   ├── setup.sh                  # Symlinks Claude Code config
│   ├── CLAUDE.md
│   ├── .claude.json
│   ├── commands/
│   └── skills/
├── keyboard/
│   ├── README.md                 # Corne firmware + keymap docs
│   ├── build.yaml                # ZMK build matrix
│   ├── config/                   # corne.keymap, corne.conf, west.yml
│   ├── draw.sh                   # Regenerates keymap.svg
│   ├── keymap-drawer.config.yaml
│   ├── keymap.svg
│   └── old/                      # Archived Voyager/QMK source
└── docs/                         # Per-tool documentation
```

See [docs/](./docs/index.md) for per-tool guides and [keyboard/README.md](./keyboard/README.md) for the Corne firmware.

## Homebrew Packages

**Formulae:** fish, fisher, fzf, go, lazygit, neovim, nvm, pnpm, ripgrep, tmux, zoxide, biome, duckdb, fd, fnm, gh, lua-language-server, tailwindcss-language-server, tree, uv, xh

**Casks:** claude-code

Source of truth: [homebrew/install.sh](./homebrew/install.sh).

## Post-Install

- `fisher update` to install fish plugins
- `nvm install <version>` to install Node.js
- `chsh -s /opt/homebrew/bin/fish` to set fish as default shell

## Manual Setup

- Ghostty: run `./ghostty/setup.sh` to symlink the config
- [Aerospace](https://github.com/nikitabobko/AeroSpace) window manager
- [Homerow](https://www.homerow.app) (bind search to cmd+shift+/)
