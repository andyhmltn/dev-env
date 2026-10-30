#!/usr/bin/env bash
set -euo pipefail

index="$HOME/vimwiki/index.wiki"
target=""
if [ -f "$index" ]; then
    target=$(sed -n 's/.*\[\[\([^]|]*\).*/\1/p' "$index" | head -n1 || true)
fi
if [ -z "$target" ]; then
    target="index"
fi

file="$HOME/vimwiki/${target}.wiki"
[ -f "$file" ] || file="$index"

exec nvim "$file"
