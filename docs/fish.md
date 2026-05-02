# Fish Shell Configuration

Fish is the primary shell for this environment. It auto-starts tmux on login and provides aliases for common development tasks.

## Features

- Auto-starts tmux in a session named "main"
- Zoxide integration for smart directory navigation
- FZF-powered project selection
- Node.js version management via fnm
- pnpm package manager support

## Aliases

| Alias | Command | Description |
|-------|---------|-------------|
| `n` | `nvim` | Open Neovim |
| `zp` | `nvim ~/.config/fish/config.fish` | Edit and reload fish config |
| `gg` | `lazygit` | Open Lazygit |
| `t` | `tmux` | Run tmux |
| `q` | `tmux kill-pane` | Kill current tmux pane |
| `p` | `pnpm` | Run pnpm |
| `cd` | `z` (zoxide) | Smart directory navigation |
| `scripts` | `cat package.json \| jq .scripts` | Show npm scripts |
| `aws-login` | `aws sso login --sso-session my-sso` | AWS SSO login |
| `cf` | `sudo $HOME/.cargo/bin/corne-flash` | Flash Corne keyboard |
| `os` | `$HOME/dev/dev-env/os` | Launch dev-env TUI |
| `build-raw` | `xcodebuild -scheme ...` | Xcode release build |

## Functions

### `c` - Change to dev directory

Interactive project selector using fzf. Searches `~/dev` for directories.

```fish
c  # Opens fzf, select a project, cd into it
```

### `nd` - Open project in Neovim

Same as `c` but opens the selected directory in Neovim instead of changing to it.

```fish
nd  # Opens fzf, select a project, opens in nvim
```

## Abbreviations

| Abbreviation | Expansion |
|--------------|-----------|
| `:q` | `tmux kill-pane` |
| `:q!` | `tmux kill-pane` |

These allow vim-style quitting of tmux panes.

## Plugins

Managed via Fisher. Listed in `fish_plugins`:

- `jorgebucaran/fisher` - Plugin manager
- `patrickf1/fzf.fish` - FZF integration
- `joshmedeski/fish-lf-icons` - File icons for lf

Install plugins:
```bash
fisher update
```

## Environment Variables

| Variable | Value |
|----------|-------|
| `PNPM_HOME` | `/Users/andy/Library/pnpm` |

## Tmux Auto-Start

When Fish starts interactively:
1. Checks if already inside tmux
2. If not, creates or attaches to a session named "main"

This ensures you always work within tmux for session persistence.

## Configuration File

The main configuration is in `fish/config.fish`. After editing, it auto-sources on save when using the `zp` alias.

The setup script symlinks this to `~/.config/fish/config.fish`.
