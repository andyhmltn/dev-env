# Mac OS Dev Setup

## Quick Start

```bash
# Clone the repo
git clone <repo-url> ~/dev/dev-env
cd ~/dev/dev-env

# Run setup (installs missing packages and symlinks configs)
./setup.sh
```

## What It Does

The setup script will:
1. Check for missing homebrew packages and install them
2. Symlink neovim, tmux, and fish configs to your home directory

## Structure

```
dev-env/
├── setup.sh              # Main setup script
├── homebrew/
│   └── install.sh        # Homebrew package installer
├── neovim/
│   ├── setup.sh          # Symlinks nvim config
│   ├── init.lua
│   ├── lua/
│   └── snippets/
├── tmux/
│   ├── setup.sh          # Symlinks tmux config
│   └── .tmux.conf
├── fish/
│   ├── setup.sh          # Symlinks fish config
│   ├── config.fish
│   ├── functions/
│   ├── completions/
│   └── conf.d/
└── backups/              # Auto-generated backups
```

## Homebrew Packages

**Formulae:**
- fish, fisher (shell + plugin manager)
- neovim (editor)
- tmux (terminal multiplexer)
- fzf, ripgrep, zoxide (search tools)
- lazygit (git UI)
- go, nvm, pnpm (languages/package managers)

**Casks:**
- claude-code

## Post-Install

- Run `fisher update` to install fish plugins
- Run `nvm install <version>` to install Node.js
- Set fish as default shell: `chsh -s /opt/homebrew/bin/fish`

## Manual Setup (not automated)

- Ghostty terminal: https://github.com/catppuccin/ghostty
- Aerospace (window manager)
- Homerow (set keyboard shortcut cmd+shift+/ for search)
