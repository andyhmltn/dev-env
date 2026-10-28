#!/usr/bin/env zsh

COUNT=$(pgrep -x claude 2>/dev/null | wc -l | tr -d ' ')

if [ "$COUNT" -gt 0 ]; then
    sketchybar --set "$NAME" \
        drawing=on \
        label="$COUNT" \
        icon.color=0xffa6e3a1
else
    sketchybar --set "$NAME" \
        drawing=on \
        label="idle" \
        icon.color=0xff6c7086
fi
