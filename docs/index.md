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

## Documentation

- [Installation Guide](./installation.md) - Full setup instructions
- [os TUI](./os-tui.md) - Interactive setup tool (`./os`)
- [Fish Shell](./fish.md) - Shell configuration and aliases
- [Neovim](./neovim.md) - Editor setup, plugins, and keybindings
- [Tmux](./tmux.md) - Terminal multiplexer configuration
- [Ghostty](./ghostty.md) - Terminal emulator settings
- [nice!view Custom Art](./nice-view-art.md) - Corne display customization

## Structure

```
dev-env/
├── setup.sh              # Main orchestration script
├── os                    # TUI launcher (builds + runs Rust binary)
├── src/                  # Rust source for the os TUI (ratatui)
├── Cargo.toml
├── homebrew/
│   ├── install.sh        # Homebrew package installer
│   └── sync.sh           # Syncs installed packages back to install.sh
├── fish/
│   ├── setup.sh          # Symlinks fish config
│   ├── config.fish       # Shell configuration
│   ├── fish_plugins      # Plugin list
│   ├── fish_variables    # Shell variables
│   └── functions/        # Custom fish functions
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
│   ├── .tmux.conf        # Tmux configuration
│   └── tmux-cd.sh        # Tmux directory helper
├── ghostty/
│   ├── setup.sh          # Symlinks ghostty config (manual)
│   └── config            # Terminal settings
├── aerospace/
│   └── aerospace.toml    # Window manager config
├── claude/
│   ├── setup.sh          # Symlinks Claude Code config
│   ├── CLAUDE.md         # Claude Code settings
│   ├── commands/         # Custom slash commands
│   └── skills/           # Superpowers skills
├── zsh/
│   ├── setup.sh
│   └── .zprofile         # Zsh fallback config
├── config/               # ZMK Corne keyboard config
├── keyboard/             # Corne firmware, keymap, flash utility
└── hooks/
    └── pre-commit        # Regenerates keymap SVG + TUI screenshot
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
| fnm | Node.js version manager |
| pnpm | Fast npm alternative |
| biome | Linter/formatter |
| duckdb | Analytical SQL engine |
| fd | Fast file finder |
| gh | GitHub CLI |
| uv | Python package manager |
| xh | HTTP client |

Source of truth: [homebrew/install.sh](../homebrew/install.sh).

## Post-Install Steps

After running `setup.sh`:

1. Install Fish plugins: `fisher update`
2. Install Node.js: `fnm install 20`
3. Set Fish as default shell: `chsh -s /opt/homebrew/bin/fish`
4. Install Tmux plugins: Press `prefix + I` in tmux

## Manual Setup

These tools are not automated by `setup.sh`:
- Ghostty: run `./ghostty/setup.sh` to symlink the config
- [Aerospace](https://github.com/nikitabobko/AeroSpace) window manager
- [Homerow](https://www.homerow.app) keyboard navigation (cmd+shift+/)
