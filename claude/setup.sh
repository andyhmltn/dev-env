#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p ~/.claude

rm -f ~/.claude/CLAUDE.md

ln -sf "$SCRIPT_DIR/CLAUDE.md" ~/.claude/CLAUDE.md

echo "Claude config symlinked"
