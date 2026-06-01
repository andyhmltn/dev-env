# Aerospace

Tiling window manager for macOS. Config lives at `aerospace/aerospace.toml`, symlinked to `~/.aerospace.toml` by the os TUI.

## Keybindings

All bindings use `alt` as the modifier (Option key). Ghostty is configured with `macos-option-as-alt = true` so these work inside the terminal.

### Navigation

| Key | Action |
|-----|--------|
| `alt-h/j/k/l` | Focus left/down/up/right |
| `alt-shift-h/j/k/l` | Move window left/down/up/right |
| `alt-1` through `alt-9` | Switch to workspace 1-9 |
| `alt-a` through `alt-z` | Switch to workspace A-Z |
| `alt-shift-1` through `alt-shift-9` | Move window to workspace 1-9 |
| `alt-shift-a` through `alt-shift-z` | Move window to workspace A-Z |
| `alt-tab` | Toggle last workspace |
| `alt-shift-tab` | Move workspace to next monitor |
| `ctrl-alt-shift-t` | Move window to first empty workspace and follow |

### Layout

| Key | Action |
|-----|--------|
| `alt-/` | Toggle tiles horizontal/vertical |
| `alt-,` | Toggle accordion horizontal/vertical |
| `alt-minus` | Shrink window (-50) |
| `alt-equal` | Grow window (+50) |

### Service Mode

Enter with `alt-shift-;`, exit with `esc`.

| Key | Action |
|-----|--------|
| `r` | Reset layout (flatten workspace tree) |
| `f` | Toggle floating/tiling |
| `backspace` | Close all windows except current |
| `alt-shift-h/j/k/l` | Join with adjacent container |

## Settings

- **Layout**: tiles (default), orientation auto-detected from monitor aspect ratio
- **Accordion padding**: 30px
- **Gaps**: all set to 0
- **Mouse**: follows focus on monitor change
- **Persistent workspaces**: 1-9 and A-Z (31 total, always visible in status bar)
- **Keyboard**: QWERTY preset

## Relationship to Norflow

Norflow (`norflow/config.toml`) is an alternative WM with profile-based auto-layout (laptop vs ultrawide). Both share the same `alt-hjkl` navigation pattern. Only one should be active at a time. Aerospace is installed via Homebrew cask; Norflow is configured separately.
