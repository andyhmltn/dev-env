# NFlow

Alternative tiling window manager with profile-based auto-layout. Config lives at `nFlow/config.toml`, symlinked by the os TUI.

## Keybindings

| Key | Action |
|-----|--------|
| `alt-{n}` | Switch to space N |
| `alt-shift-{n}` | Move window to space N |
| `alt-t` | Move window to a new space |
| `alt-h/j/k/l` | Focus left/down/up/right |
| `alt-shift-h/j/k/l` | Move window left/down/up/right |
| `alt-v` | Split vertical |
| `alt-z` | Toggle zoom |
| `cmd-alt-shift-r` | Launcher split down |
| `cmd-alt-shift-t` | Launcher split right |

## Gaps

- Outer: 50px
- Inner: 50px

Overridden to 0 on the laptop profile.

## Ignored Apps

Raycast, Spotlight, Alfred, 1Password -- these float above tiling.

## Profiles

NFlow auto-selects a profile based on screen width.

### Laptop (width <= 3439px)

| Space | Apps |
|-------|------|
| 1 | Ghostty |
| 2 | Zen |
| 3 | Slack |

Gaps: 0 inner, 0 outer.

### Ultrawide (width >= 3440px)

| Space | Apps |
|-------|------|
| 1 | RuneLite, Ghostty |
| 3 | Spotify |
| 4 | Linear |
| 5 | Figma |

Default gaps (50px).

## Launcher

`nFlow/launcher.toml` configures the app launcher. Currently minimal -- just sets the GitHub user (`andyhmltn`).

## Relationship to Aerospace

Both NFlow and Aerospace manage window tiling with identical `alt-hjkl` navigation. The profile-based auto-layout (assigning apps to spaces based on screen size) is NFlow-specific. Aerospace uses persistent named workspaces (1-9, A-Z) without auto-assignment. Only one should be active at a time.
