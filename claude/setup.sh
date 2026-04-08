#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p ~/.claude

rm -f ~/.claude/CLAUDE.md
rm -f ~/.claude.json

ln -sf "$SCRIPT_DIR/CLAUDE.md" ~/.claude/CLAUDE.md
ln -sf "$SCRIPT_DIR/.claude.json" ~/.claude.json

mkdir -p ~/.claude/skills
mkdir -p ~/.claude/commands

for skill in "$SCRIPT_DIR/skills/"*/; do
    skill_name=$(basename "$skill")
    rm -f ~/.claude/skills/"$skill_name"
    ln -sf "$skill" ~/.claude/skills/"$skill_name"
done

for cmd in "$SCRIPT_DIR/commands/"*.md; do
    [ -e "$cmd" ] || continue
    cmd_name=$(basename "$cmd")
    rm -f ~/.claude/commands/"$cmd_name"
    ln -sf "$cmd" ~/.claude/commands/"$cmd_name"
done

echo "Claude config symlinked"
