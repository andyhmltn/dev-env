# Rust TUI Architecture

The `os` TUI is a ratatui-based terminal application that manages the dev environment. Binary name: `os`. Crate name: `dev-env-os`.

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| ratatui | 0.29 | TUI rendering framework |
| crossterm | 0.28 | Terminal control (raw mode, events, alternate screen) |
| sysinfo | 0.33 | CPU, memory, disk, network metrics |
| serde + serde_json | 1 | JSON parsing (GitHub API, system data) |
| anyhow | 1 | Error handling |

## Entry Point

`./os` is a bash script that runs `cargo build --release` then executes `./target/release/os`.

`src/main.rs` sets up the terminal (raw mode, alternate screen, keyboard enhancement flags), creates the `App`, and runs the event loop. Polls for key events every 100ms, dispatches to `app.handle_key()`, calls `app.tick()` for async work, and renders via `ui::draw()`.

Special case: when the user selects "Corne Flash", the TUI restores the terminal, spawns `cargo run --release` in `keyboard/corne-flash/`, then re-initializes the terminal when it returns.

## Module Overview

| Module | Lines | Responsibility |
|--------|-------|----------------|
| `app.rs` | 1232 | State machine, key dispatch, background thread coordination |
| `ui.rs` | 1192 | All rendering (menu, status bars, dashboard, keyboard layout, brew sync) |
| `keymap.rs` | 939 | ZMK keymap parser -- reads Device Tree format, builds layer/key/macro data |
| `system.rs` | 718 | System metrics collection (CPU, memory, disk, battery, network, WiFi, Docker, ports, tmux, Bluetooth) |
| `items.rs` | 480 | Menu item definitions, symlink target paths, sync status checking |
| `keys.rs` | 433 | Key handler with modes (Normal, Command, Search, TextInput) |
| `homebrew.rs` | 300 | Homebrew package sync -- parses install.sh, detects untracked packages |
| `main.rs` | 113 | Terminal setup/teardown, event loop, repo root detection |
| `runner.rs` | 76 | Spawns shell scripts, streams stdout/stderr back via channels |
| `git.rs` | 68 | Checks remote behind count, pulls updates |
| `banner.rs` | 6 | ASCII art constant |

## State Machine

`AppState` controls what the TUI shows:

```
Main           -- Menu with sync status items
Running(n)     -- Executing a script, showing output
HomebrewSync   -- Interactive package sync (Loading -> Prompting -> Done)
KeyboardLayout -- Layer browser for the Corne keymap
Dashboard      -- System metrics display
Error(msg)     -- Error overlay
```

## Threading Model

Background work runs on dedicated threads communicating via `mpsc` channels:

- **Metrics thread** (`system.rs`): collects CPU/memory/network every second, sends `MetricsMsg` to the app
- **Dashboard thread**: heavier metrics (Docker, ports, Bluetooth, tmux sessions) on a slower interval
- **Git check thread**: queries `git fetch` + `git rev-list` to detect upstream changes
- **Git pull thread**: runs `git pull` when the user confirms
- **Status receivers**: one per menu item, checks if symlinks are in sync
- **Runner thread**: executes shell scripts (`homebrew/install.sh`, etc.), streams output lines
- **Brew sync thread**: runs `homebrew/sync.sh` logic to find untracked packages

The `App::tick()` method drains all channels each frame (non-blocking `try_recv`).

## Key Handling

`KeyHandler` maintains a mode and translates raw `KeyEvent` into `Action` variants:

- **Normal**: `j/k` navigate, `Enter` selects, `q/Esc` quits, `/` enters search, `:` enters command
- **Command**: vim-style (`:q`, `:pull`)
- **Search**: incremental filter, `Enter` confirms, `Esc` cancels
- **TextInput**: used during Homebrew sync comment input

## Menu Items

Defined in `items.rs`. Each `MenuItem` has:
- `id` (ItemId enum): Homebrew, Neovim, Tmux, Fish, Claude, Ghostty, Aerospace, NFlow, CorneFlash, KeyboardLayout, HomebrewSync, Dashboard
- `kind`: SyncItem (checks symlink), ActionItem (runs something)
- `symlink_target`: where the config should be symlinked to
- `sync_status`: Checking, Synced, NotSynced, NotInstalled

## Keymap Parser

`keymap.rs` parses ZMK's Device Tree Source format:
1. Extracts macro definitions (name -> key sequence)
2. Extracts keymap layers with display names
3. Maps key codes to human-readable labels
4. Used by the KeyboardLayout view to render an interactive layer browser

## Rendering

`ui.rs` handles all drawing with ratatui widgets:
- Main view: two-column layout (menu + status/preview panel)
- Git banner at top (behind/pulling/up-to-date)
- Keyboard layout: renders 42-key grid with layer switching
- Dashboard: multi-panel system metrics with CPU history sparkline
- Homebrew sync: package-by-package prompt flow

## Adding a New Menu Item

1. Add a variant to `ItemId` in `items.rs`
2. Add the `MenuItem` to the items list with symlink source/target
3. If it needs an action, add the handler in `app.rs` `handle_key` / `handle_action`
4. If it needs UI, add rendering in `ui.rs`

## Adding System Metrics

1. Add the collection function in `system.rs`
2. Add the field to `DashboardMetrics` or `SystemMetrics`
3. Send it via `MetricsMsg`
4. Render it in the dashboard section of `ui.rs`
