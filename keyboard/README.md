# Keyboard

ZMK firmware for a Corne (42-key split) running on `nice_nano_v2` controllers with `nice_view` displays.

The authoritative keymap lives at [`config/corne.keymap`](../config/corne.keymap). Firmware is built by the [Build Corne firmware](../.github/workflows/build-corne.yml) GitHub Action on pushes to `config/**` or `build.yaml`.

## Layout

![keymap](keymap.svg)

## Regenerating the layout image

```bash
./keyboard/draw.sh
```

Requires `uvx` (from [uv](https://github.com/astral-sh/uv)). Uses [keymap-drawer](https://github.com/caksoylar/keymap-drawer) to parse `config/corne.keymap` and render `keyboard/keymap.svg`.

## Folders

- `old/` — archived ZSA Voyager QMK source and firmware binary.
- `new/` — an upstream clone of [miryoku_zmk](https://github.com/manna-harbour/miryoku_zmk) kept for reference. Not used by the build.
