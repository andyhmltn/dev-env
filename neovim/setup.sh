#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Remove existing files/symlinks
rm -rf ~/.config/nvim/init.lua
rm -rf ~/.config/nvim/lua
rm -rf ~/.config/nvim/snippets

# Create nvim config directory if it doesn't exist
mkdir -p ~/.config/nvim

# Create symlinks
ln -sf "$SCRIPT_DIR/init.lua" ~/.config/nvim/init.lua
ln -sf "$SCRIPT_DIR/lua" ~/.config/nvim/lua
ln -sf "$SCRIPT_DIR/snippets" ~/.config/nvim/snippets

echo "Neovim config symlinked"
