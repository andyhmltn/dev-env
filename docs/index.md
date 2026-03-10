# Dev Environment Documentation

A macOS development environment configuration with Fish shell, Neovim, Tmux, and Ghostty terminal.

## Quick Start

```bash
git clone <repo-url> ~/dev/dev-env
cd ~/dev/dev-env
./setup.sh
```

The setup script will:
1. Install missing Homebrew packages
2. Symlink all configurations to your home directory
3. Create backups of existing configs

## Documentation

- [Installation Guide](./installation.md) - Full setup instructions
- [Fish Shell](./fish.md) - Shell configuration and aliases
- [Neovim](./neovim.md) - Editor setup, plugins, and keybindings
- [Tmux](./tmux.md) - Terminal multiplexer configuration
- [Ghostty](./ghostty.md) - Terminal emulator settings

## Structure

```
dev-env/
├── setup.sh              # Main orchestration script
├── homebrew/
│   └── install.sh        # Homebrew package installer
├── fish/
│   ├── setup.sh          # Symlinks fish config
│   ├── config.fish       # Shell configuration
│   ├── fish_plugins      # Plugin list
│   └── fish_variables    # Shell variables
├── neovim/
│   ├── setup.sh          # Symlinks nvim config
│   ├── init.lua          # Entry point
│   ├── lua/
│   │   ├── settings.lua  # Editor settings
│   │   ├── plugins/      # Plugin configurations
│   │   └── macros/       # Keyboard shortcuts
│   └── snippets/         # Code snippets
├── tmux/
│   ├── setup.sh          # Symlinks tmux config
│   └── .tmux.conf        # Tmux configuration
├── ghostty/
│   ├── setup.sh          # Symlinks ghostty config
│   └── config            # Terminal settings
├── zsh/
│   └── .zprofile         # Zsh fallback config
└── claude/
    └── CLAUDE.md         # Claude Code settings
```

## Installed Tools

| Tool | Purpose |
|------|---------|
| fish | Primary shell |
| fisher | Fish plugin manager |
| neovim | Text editor |
| tmux | Terminal multiplexer |
| fzf | Fuzzy finder |
| ripgrep | Fast text search |
| zoxide | Smart cd replacement |
| lazygit | Terminal git UI |
| go | Go programming language |
| nvm | Node.js version manager |
| pnpm | Fast npm alternative |

## Post-Install Steps

After running `setup.sh`:

1. Install Fish plugins: `fisher update`
2. Install Node.js: `nvm install 20`
3. Set Fish as default shell: `chsh -s /opt/homebrew/bin/fish`
4. Install Tmux plugins: Press `prefix + I` in tmux

## Manual Setup

These tools are not automated:
- [Ghostty terminal](https://ghostty.org)
- [Aerospace](https://github.com/nikitabobko/AeroSpace) window manager
- [Homerow](https://www.homerow.app) keyboard navigation (cmd+shift+/)
