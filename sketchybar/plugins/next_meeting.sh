#!/usr/bin/env zsh

BIN=/opt/homebrew/bin/icalBuddy
[ ! -x "$BIN" ] && { sketchybar --set "$NAME" drawing=off; exit 0; }

RAW=$("$BIN" \
    -nc -npn -nrd -b "" \
    -ea \
    -iep "title,datetime" \
    -df "" \
    -tf "%H:%M" \
    eventsToday 2>/dev/null)

if [ -z "$RAW" ]; then
    sketchybar --set "$NAME" drawing=off
    exit 0
fi

TITLE=$(echo "$RAW" | head -2 | tr '\n' ' ' \
    | sed 's/ *([^)]*) *$//' \
    | sed 's/^[[:space:]]*//;s/[[:space:]]*$//' \
    | cut -c1-30)

if [ -z "$TITLE" ]; then
    sketchybar --set "$NAME" drawing=off
    exit 0
fi

sketchybar --set "$NAME" drawing=on label="$TITLE"
