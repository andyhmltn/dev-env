# Git Hooks

The repo includes a pre-commit hook at `hooks/pre-commit`. The os TUI symlinks it to `.git/hooks/pre-commit`.

## Pre-commit

Runs automatically before every `git commit`. Two jobs:

### 1. Keymap SVG Regeneration

Triggered when staged changes include `config/corne.keymap` or `keyboard/keymap-drawer.config.yaml`.

Runs `keyboard/draw.sh` to regenerate `keyboard/keymap.svg` via [keymap-drawer](https://github.com/caksoylar/keymap-drawer), then stages the updated SVG.

### 2. TUI Screenshot Regeneration

Triggered when `vhs` and `ffmpeg` are both installed.

1. Builds the os TUI binary (`cargo build --release`)
2. Records a terminal session via VHS (`docs/screenshot.tape`)
3. Extracts the last frame as a PNG (`docs/os-screenshot.png`)
4. Cleans up the intermediate GIF
5. Stages the updated screenshot

The screenshot tape (`docs/screenshot.tape`) runs `./target/release/os` and captures the output. This keeps the README screenshot in sync with the actual TUI.

## Setup

The os TUI handles symlinking, but you can set it up manually:

```bash
ln -sf ../../hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```
