# Keyboard

ZMK firmware for a Corne 42-key split keyboard running on Nice Nano v2 controllers with Nice View displays.

## File Layout

```
config/                      # Must live at repo root (ZMK build requirement)
  corne.keymap               # Keymap source (Device Tree format)
  corne.conf                 # ZMK build flags
  west.yml                   # Zephyr/ZMK manifest
  zephyr/module.yml          # ZMK extra module config
  boards/shields/             # Custom shield definitions (nice_view_custom)

keyboard/
  build.yaml                 # Build matrix (left half, right half, settings_reset)
  keymap.svg                 # Generated visual keymap
  keymap-drawer.config.yaml  # SVG renderer macro labels
  draw.sh                    # Regenerates keymap.svg via keymap-drawer
  corne-flash/               # Rust TUI for flashing firmware
  old/                       # Archived ZSA Voyager QMK source
```

## Keymap

The keymap has 8 layers:

| Layer | Name | Purpose |
|-------|------|---------|
| 0 | BASE | QWERTY with homerow mods, hold-tap layer keys |
| 1 | FN | Vim macros, tmux control, clipboard, arrows, diagnostics |
| 2 | SYM | Symbols, brackets, operators, fat arrow, double slash |
| 3 | NUM | Number pad (right hand), activated by holding F on base |
| 4 | SCRN | Screenshot and window management shortcuts |
| 5 | MOVE | Move windows to Aerospace workspaces 1-9 |
| 6 | BT | Bluetooth and Aerospace workspace switching/moving |
| 7 | RS | RuneScape -- F-keys and simplified input |

### Behaviors

- **Tap-dance** (`td_colon`): single tap = `:`, double tap = `1`
- **Hold-tap** (`lt_enter`): hold = layer, tap = key. Hold-preferred flavor for thumb keys
- **Hold-tap** (`lt_homerow`): balanced flavor, right-hand trigger positions only. Used for `F` key (hold = layer 3)
- **Caps word**: left pinky bottom row. Auto-capitalizes until a non-alpha key

### Combos

- Top-left outer two keys (`0 + 5`): bootloader (left half)
- Top-right outer two keys (`6 + 11`): bootloader (right half)

### Macros

| Macro | Output | Purpose |
|-------|--------|---------|
| `m_esc_w_cr` | `Esc :w Enter` | Vim save |
| `m_q_bang_cr` | `:q! Enter` | Vim force quit |
| `m_sp_otd` | `Space -otd` | Custom command |
| `m_sp_gg` | `Space gg` | Lazygit (Neovim leader) |
| `m_sp_ca` | `Space ca` | Code action (Neovim leader) |
| `m_sp_dash` | `Space -` | Leader dash prefix |
| `m_tmux_z` | `Ctrl-b z` | Tmux zoom pane |
| `m_tmux_c` | `Ctrl-b c` | Tmux new window |
| `m_tmux_x` | `Ctrl-b x` | Tmux kill pane |
| `m_tmux_copy` | `Ctrl-b [` | Tmux copy mode |
| `m_tmux_l` | `Ctrl-b l` | Tmux last window |
| `m_tmux_split_h` | `Ctrl-b "` | Tmux horizontal split |
| `m_tmux_split_v` | `Ctrl-b %` | Tmux vertical split |
| `m_fat_arrow` | `=>` | JS/TS fat arrow |
| `m_slash_slash` | `//` | Double slash |
| `m_dotdot_slash` | `../` | Parent directory |
| `m_jump_prev_diag` | `[g` | Neovim previous diagnostic |
| `m_jump_next_diag` | `]g` | Neovim next diagnostic |
| `m_tick_A/B/C` | `` `A `` / `` `B `` / `` `C `` | Vim marks |
| `m_sp_gtl` / `m_sp_btl` | `Space -gtl` / `Space -btl` | Custom commands |
| `m_55_star` | `55*` | Calculator shortcut |
| `m_col_plus_1_col` | `:+1:` | Emoji shortcut |

## Build Configuration

`config/corne.conf`:
```
CONFIG_ZMK_STUDIO=y           # Live layer viewer/editor via zmk.studio
CONFIG_ZMK_STUDIO_LOCKING=n   # No unlock binding required
CONFIG_ZMK_DISPLAY=y          # Enable Nice View displays
CONFIG_NICE_VIEW_WIDGET_STATUS=y
```

`keyboard/build.yaml` -- three targets:
1. `nice_nano_v2` + `corne_left nice_view_adapter nice_view_custom`
2. `nice_nano_v2` + `corne_right nice_view_adapter nice_view_custom`
3. `nice_nano_v2` + `settings_reset`

## Regenerating the Layout SVG

```bash
./keyboard/draw.sh
```

Requires `uvx` (from uv). Uses [keymap-drawer](https://github.com/caksoylar/keymap-drawer) to parse `config/corne.keymap` and render `keyboard/keymap.svg`. Macro labels are mapped in `keymap-drawer.config.yaml`. The pre-commit hook auto-regenerates this when the keymap changes.

## ZMK Studio

Connect to the keyboard over USB or Bluetooth to view/edit layers live:

1. Flash latest firmware
2. Open [zmk.studio](https://zmk.studio) in a Chromium browser
3. Click Connect and pick the Corne

No unlock binding needed (`CONFIG_ZMK_STUDIO_LOCKING=n`).

## Corne Flash Utility

`keyboard/corne-flash/` is a standalone Rust TUI for flashing firmware. Launched from the os TUI via the "Corne Flash" menu item.

### How It Works

1. Fetches the latest successful GitHub Actions build via `gh run list`
2. Downloads firmware artifacts via `gh run download` to a temp directory
3. Finds left/right `.uf2` files in the artifacts
4. Waits for left Nice Nano to appear as a USB volume (NICENANO mount)
5. Copies the `.uf2` file to flash
6. Waits for right Nice Nano, repeats

### Usage

```bash
cd keyboard/corne-flash
sudo cargo run --release              # Fetch from GitHub
sudo cargo run --release -- --local /path/to/firmware  # Use local files
```

Requires root for raw device access. Requires `gh` CLI authenticated.

### Controls

- `s` -- skip current half
- `r` -- retry on error
- `Esc` -- quit
- `j/k` -- scroll log

### Architecture

| File | Purpose |
|------|---------|
| `main.rs` | Terminal setup, event loop, `--local` arg parsing |
| `app.rs` | State machine (Fetching -> Downloading -> WaitLeft -> FlashLeft -> WaitRight -> FlashRight -> Done) |
| `github.rs` | `gh` CLI wrapper for fetching runs and downloading artifacts |
| `flasher.rs` | USB volume detection and `.uf2` file copy |
| `keys.rs` | Key handler (normal mode, command mode) |
| `ui.rs` | Ratatui rendering |

## CI/CD

See [GitHub Actions](./github-actions.md) for the firmware build workflow.

## Custom Nice View Art

See [nice-view-art.md](./nice-view-art.md) for the plan to display custom pixel art on the right half's Nice View display.
