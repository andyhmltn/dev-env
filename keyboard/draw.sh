#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
KEYMAP="$REPO_ROOT/config/corne.keymap"
CONFIG="$REPO_ROOT/keyboard/keymap-drawer.config.yaml"
OUT="$REPO_ROOT/keyboard/keymap.svg"

uvx --from keymap-drawer keymap -c "$CONFIG" parse -z "$KEYMAP" -c 5 \
  | uvx --from keymap-drawer keymap -c "$CONFIG" draw - > "$OUT"

echo "Wrote $OUT"
