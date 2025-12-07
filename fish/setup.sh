#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create fish config directory if it doesn't exist
mkdir -p ~/.config/fish

# Remove existing config.fish and fish_plugins
rm -f ~/.config/fish/config.fish
rm -f ~/.config/fish/fish_plugins
rm -rf ~/.config/fish/functions
rm -rf ~/.config/fish/completions
rm -rf ~/.config/fish/conf.d
rm -rf ~/.config/fish/themes

# Create symlinks
ln -sf "$SCRIPT_DIR/config.fish" ~/.config/fish/config.fish
ln -sf "$SCRIPT_DIR/fish_plugins" ~/.config/fish/fish_plugins
ln -sf "$SCRIPT_DIR/functions" ~/.config/fish/functions
ln -sf "$SCRIPT_DIR/completions" ~/.config/fish/completions
ln -sf "$SCRIPT_DIR/conf.d" ~/.config/fish/conf.d
[ -d "$SCRIPT_DIR/themes" ] && ln -sf "$SCRIPT_DIR/themes" ~/.config/fish/themes

echo "Fish config symlinked"
