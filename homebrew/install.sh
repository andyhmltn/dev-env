#!/bin/bash

# Dev environment homebrew packages

set -e

# Formulae
FORMULAE=(
    fish        # Fish shell
    fisher      # Fish plugin manager
    fzf         # Fuzzy finder
    go          # Go programming language
    lazygit     # Terminal UI for git
    neovim      # Text editor
    nvm         # Node version manager
    pnpm        # Fast npm alternative
    ripgrep     # Fast search (rg)
    tmux        # Terminal multiplexer
    zoxide      # Smarter cd command
)

# Casks
CASKS=(
    claude-code # Claude Code CLI
)

# Check for missing packages
missing_formulae=()
missing_casks=()

for formula in "${FORMULAE[@]}"; do
    if ! brew list "$formula" &> /dev/null; then
        missing_formulae+=("$formula")
    fi
done

for cask in "${CASKS[@]}"; do
    if ! brew list --cask "$cask" &> /dev/null; then
        missing_casks+=("$cask")
    fi
done

# Exit early if nothing to install
if [[ ${#missing_formulae[@]} -eq 0 && ${#missing_casks[@]} -eq 0 ]]; then
    echo "All homebrew packages already installed"
    exit 0
fi

# Check if Homebrew is installed
if ! command -v brew &> /dev/null; then
    echo "Homebrew not found. Installing..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi

# Install missing formulae
if [[ ${#missing_formulae[@]} -gt 0 ]]; then
    echo "Installing missing formulae: ${missing_formulae[*]}"
    for formula in "${missing_formulae[@]}"; do
        echo "  Installing $formula..."
        brew install "$formula"
    done
fi

# Install missing casks
if [[ ${#missing_casks[@]} -gt 0 ]]; then
    echo "Installing missing casks: ${missing_casks[*]}"
    for cask in "${missing_casks[@]}"; do
        echo "  Installing $cask..."
        brew install --cask "$cask"
    done
fi

echo "Homebrew packages installed!"
