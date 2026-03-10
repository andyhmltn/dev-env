# Installation Guide

Complete setup instructions for a fresh macOS installation.

## Prerequisites

- macOS (tested on Apple Silicon)
- Command Line Tools: `xcode-select --install`
- Homebrew: `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`

## Quick Install

```bash
git clone git@github.com:andyhmltn/dev-env.git ~/dev/dev-env
cd ~/dev/dev-env
./setup.sh
```

## What the Setup Script Does

1. **Installs Homebrew packages** (`homebrew/install.sh`)
   - Formulae: fish, fisher, neovim, tmux, fzf, ripgrep, zoxide, lazygit, go, nvm, pnpm
   - Casks: claude-code

2. **Symlinks configurations** (each tool's `setup.sh`)
   - `~/.config/nvim` -> `neovim/`
   - `~/.config/fish/config.fish` -> `fish/config.fish`
   - `~/.tmux.conf` -> `tmux/.tmux.conf`
   - `~/.config/ghostty/config` -> `ghostty/config`
   - `~/.claude/CLAUDE.md` -> `claude/CLAUDE.md`

3. **Creates backups** of existing configs in `backups/`

## Post-Install Steps

### 1. Set Fish as Default Shell

```bash
echo /opt/homebrew/bin/fish | sudo tee -a /etc/shells
chsh -s /opt/homebrew/bin/fish
```

### 2. Install Fish Plugins

```bash
fisher update
```

### 3. Install Node.js

```bash
nvm install 20
```

### 4. Install Tmux Plugin Manager

```bash
git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm
```

Then in tmux, press `prefix + I` to install plugins.

### 5. Install Tmux Theme

```bash
git clone https://github.com/arcticicestudio/nord-tmux.git ~/.tmux/themes/nord
mkdir -p ~/.tmux-themepack
# Download basic.tmuxtheme from jimeh/tmux-themepack
```

### 6. Install Neovim Plugins

Open Neovim and Lazy.nvim will auto-install plugins:

```bash
nvim
```

### 7. Install LSP Servers

Inside Neovim:
```vim
:Mason
```

Install: gopls, lua_ls, prettierd

## Manual Application Setup

### Ghostty Terminal

1. Download from [ghostty.org](https://ghostty.org)
2. The config is already symlinked by setup.sh

### Aerospace Window Manager

1. Install: `brew install --cask aerospace`
2. Configure according to your preferences

### Homerow

1. Download from [homerow.app](https://www.homerow.app)
2. Set keyboard shortcut to `Cmd+Shift+/`

## Troubleshooting

### Fish not finding commands

Ensure Homebrew is in your path. The config sources:
```fish
eval (/opt/homebrew/bin/brew shellenv)
```

### Tmux doesn't auto-start

Check that Fish is your login shell and config.fish is properly symlinked:
```bash
ls -la ~/.config/fish/config.fish
```

### Neovim plugins not loading

Run `:Lazy sync` inside Neovim.

### LSP not working

1. Check Mason installed the server: `:Mason`
2. Check LSP is attached: `:LspInfo`

## Directory Structure After Install

```
~/.config/
├── nvim/           -> ~/dev/dev-env/neovim/
├── fish/
│   └── config.fish -> ~/dev/dev-env/fish/config.fish
└── ghostty/
    └── config      -> ~/dev/dev-env/ghostty/config

~/.tmux.conf        -> ~/dev/dev-env/tmux/.tmux.conf
~/.claude/
└── CLAUDE.md       -> ~/dev/dev-env/claude/CLAUDE.md
```

## Updating

Pull the latest changes and re-run setup:

```bash
cd ~/dev/dev-env
git pull
./setup.sh
```

The setup script is idempotent. It will skip already-installed packages and update symlinks.
