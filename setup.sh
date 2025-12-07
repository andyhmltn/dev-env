#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Setting up dev environment..."

"$SCRIPT_DIR/neovim/setup.sh"
"$SCRIPT_DIR/tmux/setup.sh"
"$SCRIPT_DIR/fish/setup.sh"

echo "Dev environment setup complete!"
