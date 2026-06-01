---
name: corne-build-local
description: Use when GitHub Actions is down, the user wants to build ZMK Corne firmware locally, or flash firmware without waiting for CI. Triggers on "build firmware", "flash locally", "compile keymap", "GitHub Actions is down".
---

# Corne Local Build & Flash

Build ZMK firmware locally with Docker and flash via `corne-flash --local`.

## Prerequisites

- Docker Desktop installed and running
- `corne-flash` binary installed at `~/.cargo/bin/corne-flash`
- Working directory: `~/dev/dev-env`

## Workflow

### Step 1: Pull the build image (skip if already pulled)

```bash
docker pull zmkfirmware/zmk-build-arm:stable
```

The `stable` tag matches what CI uses. Do NOT use `3.2` -- it has an SDK version mismatch with Zephyr 3.5.

### Step 2: Initialize west (first time only)

Skip if `zmk/` and `zephyr/` directories already exist in the repo root.

```bash
docker run --rm -v "$(pwd):/workspace" -w /workspace zmkfirmware/zmk-build-arm:stable bash -c "west init -l config && west update"
```

This takes a few minutes. The `config/zephyr/module.yml` file must exist for local builds -- it tells Zephyr that `config/` is a valid module with board definitions.

### Step 3: Build both halves

Run these sequentially (they share the west workspace):

```bash
docker run --rm -v "$(pwd):/workspace" -w /workspace zmkfirmware/zmk-build-arm:stable bash -c "west zephyr-export && west build -s zmk/app -b nice_nano_v2 -d build/left -- -DSHIELD='corne_left nice_view_adapter nice_view_custom' -DZMK_CONFIG='/workspace/config'"
```

```bash
docker run --rm -v "$(pwd):/workspace" -w /workspace zmkfirmware/zmk-build-arm:stable bash -c "west zephyr-export && west build -s zmk/app -b nice_nano_v2 -d build/right -- -DSHIELD='corne_right nice_view_adapter nice_view_custom' -DZMK_CONFIG='/workspace/config'"
```

`west zephyr-export` is required in each container run -- it registers Zephyr's CMake package in the container's home directory.

Output UF2 files land at:
- `build/left/zephyr/zmk.uf2`
- `build/right/zephyr/zmk.uf2`

### Step 4: Flash

```bash
sudo corne-flash --local build
```

This finds the UF2 files under `build/left/` and `build/right/`, then walks through the interactive flash UI (put each half into bootloader mode when prompted).

## Quick Reference

| Task | Command |
|------|---------|
| Pull image | `docker pull zmkfirmware/zmk-build-arm:stable` |
| Init west | `docker run --rm -v "$(pwd):/workspace" -w /workspace zmkfirmware/zmk-build-arm:stable bash -c "west init -l config && west update"` |
| Build left | See Step 3 (first command) |
| Build right | See Step 3 (second command) |
| Flash local | `sudo corne-flash --local build` |
| Flash from CI | `sudo corne-flash` (default, downloads from GitHub) |

## Common Mistakes

| Problem | Fix |
|---------|-----|
| `Could not find package "Zephyr"` | You forgot `west zephyr-export` -- it must run in each container invocation |
| `not a valid zephyr module` | `config/zephyr/module.yml` is missing -- it must exist with `board_root: ..` |
| SDK version mismatch (0.15.2 vs 0.16) | Use `zmkfirmware/zmk-build-arm:stable`, not `:3.2` |
| `Left firmware .uf2 not found` | Paths must contain `/left/` or `corne_left` in the directory structure |
| Rebuild after keymap change | Delete `build/left` and `build/right` first, or west will use cached build |

## Rules

- Always run from `~/dev/dev-env` (the repo root where `config/` lives)
- Do not commit `build/`, `zmk/`, `zephyr/`, `modules/`, `tools/`, or `bootloader/` directories -- they are build artifacts
- The `config/zephyr/module.yml` file IS committed -- it enables local builds without breaking CI
