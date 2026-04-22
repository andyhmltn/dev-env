# Keyboard

ZMK firmware for a Corne (42-key split) running on `nice_nano_v2` controllers with `nice_view` displays.

The authoritative keymap lives at [`config/corne.keymap`](../config/corne.keymap). Firmware is built by the [Build Corne firmware](../.github/workflows/build-corne.yml) GitHub Action on pushes to `config/**` or `build.yaml`.

## Layout

![keymap](keymap.svg)

## Regenerating the layout image

```bash
./keyboard/draw.sh
```

Requires `uvx` (from [uv](https://github.com/astral-sh/uv)). Uses [keymap-drawer](https://github.com/caksoylar/keymap-drawer) to parse `config/corne.keymap` and render `keyboard/keymap.svg`. Macro labels are mapped in `keymap-drawer.config.yaml`.

## Live layer viewer (ZMK Studio)

The firmware is built with `CONFIG_ZMK_STUDIO=y` (see `config/corne.conf`), which lets you connect to the keyboard over USB or Bluetooth and view the active layer in real time, or edit bindings without reflashing.

1. Flash the latest firmware (GH Action → artifacts → drag each half onto the `NICENANO` mount).
2. Open [zmk.studio](https://zmk.studio) in a Chromium browser (Chrome, Edge, Arc).
3. Click **Connect** → pick the Corne.

`CONFIG_ZMK_STUDIO_LOCKING=n` is set so Studio connects without an unlock binding. If you later want the keyboard locked until you press a dedicated key, flip that to `y` and bind `&studio_unlock` somewhere.

## Folders

- `old/` — archived ZSA Voyager QMK source and firmware binary.
