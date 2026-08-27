# Corne Flash -- CLI Flashing Tool

A Rust TUI application for flashing both halves of a Corne split keyboard with ZMK firmware built by GitHub Actions.

## Context

- Keyboard: Corne split with nice!nano v2 controllers and nice_view displays
- Firmware: ZMK, built via GitHub Actions workflow in `andyhmltn/dev-env`
- Flashing: nice!nano enters bootloader via ZMK combo (keys 0+5 left, 6+11 right), mounts as `NICENANO` USB volume, firmware is a `.uf2` file copied to that volume
- Platform: macOS (Darwin)

## Architecture

Single binary `corne-flash` with three modules:

### `github` module
- Shells out to `gh` CLI (already authenticated) to:
  - Fetch the latest successful run of the `build-corne.yml` workflow
  - Download the firmware artifact zip
  - Extract left and right `.uf2` files to a temp directory
- Parses run metadata: commit SHA, date, workflow status

### `flasher` module
- Polls `/Volumes/` for a `NICENANO` mount (100ms interval)
- Copies the correct `.uf2` file to the mounted volume using `std::fs::copy`
- Waits for the volume to unmount (indicates flash complete, nice!nano reboots automatically)

### `ui` module
- Ratatui + crossterm backend
- Drives the state machine, renders each phase, handles keyboard input
- Full vim keybindings: `:q`/`Esc` to quit, `j`/`k` to scroll logs, `s` to skip a half

## State Machine

```
Fetching -> Downloading -> WaitLeftHalf -> FlashingLeft -> WaitRightHalf -> FlashingRight -> Done
```

Any state can transition to `Error` on failure. `Error` state shows the error message and offers retry (`r`) or quit (`:q`/`Esc`).

## UI Layout

### Header
- App name "Corne Flash" left-aligned
- Current step indicator right-aligned (e.g., "Step 3/6")

### Main Content (per state)
- **Fetching**: Spinner + "Fetching latest firmware build..." then workflow run info (commit, date)
- **Downloading**: Progress indicator + artifact details
- **WaitLeftHalf / WaitRightHalf**: Large centered prompt with the combo keys to press, pulsing indicator showing active watch
- **FlashingLeft / FlashingRight**: File path, target volume, progress bar
- **Done**: Summary of both halves flashed with commit SHA and timestamps
- **Error**: Error message with retry/quit options

### Footer
- Contextual keybinding hints
- Always: `:q quit  Esc quit`
- Wait states: `s skip`
- Error state: `r retry`

### Colors
- Default terminal background
- Cyan: headers, highlights
- Green: success
- Yellow: waiting/in-progress
- Red: errors

## Vim Keybindings

| Key | Action |
|-----|--------|
| `:q` | Quit |
| `Esc` | Quit |
| `j` | Scroll down (log output) |
| `k` | Scroll up (log output) |
| `s` | Skip current half (wait states) |
| `r` | Retry (error state) |
| `Enter` | Confirm/proceed |
| `g` | Scroll to top |
| `G` | Scroll to bottom |

The `:` key enters command mode (vim-style). Only `:q` is supported as a command.

## Dependencies

- `ratatui` -- TUI framework
- `crossterm` -- terminal backend
- `serde` / `serde_json` -- parsing `gh` CLI JSON output
- `zip` -- extracting firmware artifact
- `tempfile` -- temp directory for extracted firmware
- `anyhow` -- error handling

## Build & Install

```
cd keyboard/corne-flash
cargo build --release
cp target/release/corne-flash ~/.local/bin/
```

## File Structure

```
keyboard/corne-flash/
  Cargo.toml
  src/
    main.rs        -- entry point, terminal setup/teardown
    app.rs         -- state machine, main event loop
    ui.rs          -- ratatui rendering for each state
    github.rs      -- gh CLI interaction, artifact download
    flasher.rs     -- volume detection, firmware copy
    keys.rs        -- vim keybinding handling, command mode
```
