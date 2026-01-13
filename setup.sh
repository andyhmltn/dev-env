#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Setting up dev environment..."

# Check for missing homebrew packages and install if needed
"$SCRIPT_DIR/homebrew/install.sh"

# Symlink config files
"$SCRIPT_DIR/neovim/setup.sh"
"$SCRIPT_DIR/tmux/setup.sh"
"$SCRIPT_DIR/fish/setup.sh"
"$SCRIPT_DIR/claude/setup.sh"

echo "Dev environment setup complete!"
