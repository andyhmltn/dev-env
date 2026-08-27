# Ghostty Terminal Configuration

Ghostty is a fast, GPU-accelerated terminal emulator.

## Settings

| Setting | Value |
|---------|-------|
| Shell | Fish (`/opt/homebrew/bin/fish`) |
| Shell integration | Fish |
| Font | Menlo Bold |
| Font size | 18 |
| Theme | Tokyo Night Moon |

## Installation

Ghostty is not installed via Homebrew. Download from [ghostty.org](https://ghostty.org).

## Configuration Location

The config file is at `ghostty/config` in this repo.

Setup script symlinks it to `~/.config/ghostty/config`.

## Theme

Uses [Tokyo Night Moon](https://github.com/folke/tokyonight.nvim), a clean dark theme.

## Font

Menlo Bold at 18pt provides good readability. Menlo is a built-in macOS font.

For icons and special characters, you may want to install a Nerd Font like Iosevka Nerd Font (configured in Neovim's GUI settings).
