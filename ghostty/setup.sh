#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p ~/.config/ghostty/
rm -f ~/.config/ghostty/config
ln -sf "$SCRIPT_DIR/config" ~/.config/ghostty/config

echo "Ghostty config symlinked"
