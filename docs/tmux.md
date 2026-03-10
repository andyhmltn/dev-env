# Tmux Configuration

Terminal multiplexer configuration with vim-style navigation and session persistence.

## Prefix Key

The default prefix is `Ctrl-b`.

## Pane Navigation

Seamless navigation between tmux panes and Neovim splits using the same keys:

| Keybinding | Action |
|------------|--------|
| `Ctrl-h` | Move to left pane |
| `Ctrl-j` | Move to pane below |
| `Ctrl-k` | Move to pane above |
| `Ctrl-l` | Move to right pane |
| `Ctrl-\` | Move to last pane |

These work identically whether you're in tmux or Neovim, provided by vim-tmux-navigator.

## Pane Management

| Keybinding | Action |
|------------|--------|
| `x` | Kill current pane (no confirmation) |
| Mouse | Click to select pane, drag to resize |

## Copy Mode

Vi-style copy mode:

| Keybinding | Action |
|------------|--------|
| `prefix + [` | Enter copy mode |
| `v` | Begin selection |
| `y` | Copy selection |
| Pane navigation keys | Navigate in copy mode |

Mouse selection does not auto-copy (unbind MouseDragEnd1Pane).

## Session Persistence

Sessions are automatically saved and restored using:

- **tmux-resurrect** - Saves sessions manually
- **tmux-continuum** - Auto-restores sessions on tmux start

Settings:
- Pane contents are captured
- Auto-restore is enabled
- Panes don't remain after process exit

## Plugins

Managed by TPM (Tmux Plugin Manager):

| Plugin | Purpose |
|--------|---------|
| tpm | Plugin manager |
| catppuccin/tmux | Catppuccin theme |
| nordtheme/tmux | Nord theme |
| tmux-resurrect | Session save/restore |
| tmux-continuum | Auto session restore |

### Installing Plugins

After cloning TPM to `~/.tmux/plugins/tpm`:

```bash
# Inside tmux
prefix + I  # Install plugins
```

## Shell

Tmux uses Fish shell:
```
default-shell "/opt/homebrew/bin/fish"
default-command "exec /opt/homebrew/bin/fish"
```

## Theme

The status bar uses a transparent background with Nord theme:
- Status bar: transparent
- Pane borders: transparent
- Message bar: transparent

## Performance

| Setting | Value |
|---------|-------|
| escape-time | 0 (no delay on escape) |
| mode-keys | vi |
| mouse | on |

## Fish Integration

Fish auto-starts tmux on shell launch, creating or attaching to a session named "main":

```fish
exec tmux new-session -A -s main
```

This means:
- Opening a terminal automatically enters tmux
- Closing the terminal preserves your session
- Re-opening attaches to the existing session

## Common Workflows

### Create new window
```
prefix + c
```

### Split panes
```
prefix + %   # Vertical split
prefix + "   # Horizontal split
```

### Navigate windows
```
prefix + n   # Next window
prefix + p   # Previous window
prefix + 0-9 # Jump to window number
```

### Detach and reattach
```
prefix + d   # Detach
tmux a       # Reattach
```
