# GitHub Actions

## Build Corne Firmware

Workflow: `.github/workflows/build-corne.yml`

Builds ZMK firmware for both halves of the Corne keyboard.

### Triggers

- **Push to main**: when files change in `config/**`, `keyboard/build.yaml`, or `.github/workflows/build-corne.yml`
- **Manual**: `workflow_dispatch` (run from GitHub UI)

### What It Does

Uses the official ZMK reusable workflow (`zmkfirmware/zmk/.github/workflows/build-user-config.yml@v0.3.0`). The build matrix at `keyboard/build.yaml` defines three targets:

| Board | Shield | Output |
|-------|--------|--------|
| nice_nano_v2 | corne_left nice_view_adapter nice_view_custom | Left half `.uf2` |
| nice_nano_v2 | corne_right nice_view_adapter nice_view_custom | Right half `.uf2` |
| nice_nano_v2 | settings_reset | Settings reset `.uf2` |

### Artifacts

Firmware `.uf2` files are uploaded as GitHub Actions artifacts. Download them from the Actions tab or use the corne-flash utility which fetches them automatically via `gh run download`.

### Permissions

The workflow has `contents: read` permission only.

### Flashing

Two options:

1. **corne-flash TUI**: select "Corne Flash" from the os TUI menu, or run `sudo cargo run --release` in `keyboard/corne-flash/`. Auto-fetches the latest successful build and walks through flashing each half.
2. **Manual**: download artifacts from the Actions tab, double-tap the reset button on each Nice Nano to mount it as `NICENANO`, drag the `.uf2` file onto the mount.
