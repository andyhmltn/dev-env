# os -- Interactive Setup TUI

`./os` is a terminal UI for managing the dev environment. It shows sync status for each tool and lets you run setup scripts, flash keyboard firmware, and browse keymap layers.

## Usage

```bash
cd ~/dev/dev-env
./os
```

The wrapper script builds the Rust binary (release mode, quiet) then executes it.

## Navigation

Vim-style keybindings throughout.

| Key | Action |
|-----|--------|
| `j` / `k` | Move cursor up/down |
| `g` / `G` | Jump to top/bottom |
| `Enter` | Select item (run setup script, open view) |
| `Esc` | Go back / clear search / quit |
| `:q` | Quit |
| `Ctrl+C` | Force quit |

## Search

Press `/` to search menu items by label. Matching text is highlighted in yellow.

| Key | Action |
|-----|--------|
| `/` | Open search bar |
| _(type)_ | Incremental search -- cursor jumps to first match |
| `Enter` | Confirm search (keeps highlights, enables `n`/`N`) |
| `Esc` | Cancel search (clears query) |
| `Backspace` | Delete character (exits search if empty) |
| `n` | Jump to next match |
| `N` | Jump to previous match |

After confirming a search with `Enter`, press `Esc` once to clear highlights, again to quit.

When the git update banner is visible, `n` dismisses it instead of jumping to the next match.

## Menu Items

### Sync Items

These show live sync status (synced, not synced, partial, checking):

- **Homebrew** -- install missing formulae and casks
- **Neovim** -- symlink nvim config
- **Tmux** -- symlink tmux config
- **Fish** -- symlink fish config
- **Claude** -- symlink Claude Code config
- **Ghostty** -- symlink ghostty config
- **Aerospace** -- symlink aerospace config

### Action Items

- **Corne Flash** -- flash keyboard firmware
- **Keyboard Layout** -- browse keymap layers (`Tab` to cycle, press keys to highlight)
- **Homebrew Sync** -- find untracked packages and add them to `install.sh`

## Git Integration

On startup, the TUI checks if the local repo is behind the remote. If updates are available, a banner appears with `y` to pull and `n` to dismiss.

## Architecture

Built with [ratatui](https://ratatui.rs) and [crossterm](https://docs.rs/crossterm). Source lives in `src/`:

| File | Purpose |
|------|---------|
| `main.rs` | Terminal init, event loop |
| `app.rs` | State machine, key dispatch |
| `ui.rs` | All rendering |
| `keys.rs` | Key modes (normal, command, search, text input) |
| `items.rs` | Menu items, status checking |
| `keymap.rs` | ZMK keymap parsing and layout rendering |
| `homebrew.rs` | Homebrew package management |
| `git.rs` | Remote status and pull |
| `runner.rs` | Script execution |
| `banner.rs` | ASCII art |
