#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

rm -f ~/.tmux.conf
ln -sf "$SCRIPT_DIR/.tmux.conf" ~/.tmux.conf

echo "Tmux config symlinked"
