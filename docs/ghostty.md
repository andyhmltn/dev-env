# Ghostty Terminal Configuration

Ghostty is a fast, GPU-accelerated terminal emulator.

## Settings

| Setting | Value |
|---------|-------|
| Shell | Fish (`/opt/homebrew/bin/fish`) |
| Shell integration | Fish |
| Font | Menlo Bold |
| Font size | 18 |
| Background opacity | 0.9 (90%) |
| Theme | Catppuccin Mocha |

## Installation

Ghostty is not installed via Homebrew. Download from [ghostty.org](https://ghostty.org).

## Configuration Location

The config file is at `ghostty/config` in this repo.

Setup script symlinks it to `~/.config/ghostty/config`.

## Theme

Uses [Catppuccin Mocha](https://github.com/catppuccin/ghostty), a warm dark theme that matches the Neovim color scheme.

To install the theme manually:
1. Visit [catppuccin/ghostty](https://github.com/catppuccin/ghostty)
2. Follow installation instructions

## Transparency

Background opacity is set to 0.9, providing slight transparency. This works well with the transparent Catppuccin theme in Neovim.

## Font

Menlo Bold at 18pt provides good readability. Menlo is a built-in macOS font.

For icons and special characters, you may want to install a Nerd Font like Iosevka Nerd Font (configured in Neovim's GUI settings).
