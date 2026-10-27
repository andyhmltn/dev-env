#!/usr/bin/env zsh

STATE_FILE="/tmp/sketchybar_pomodoro"
WORK_SEC=$((25 * 60))

if [ "$SENDER" = "mouse.clicked" ]; then
    if [ -f "$STATE_FILE" ]; then
        rm -f "$STATE_FILE"
    else
        date +%s > "$STATE_FILE"
    fi
fi

if [ ! -f "$STATE_FILE" ]; then
    sketchybar --set "$NAME" label="25:00" icon.color=0xff6c7086
    exit 0
fi

START=$(cat "$STATE_FILE")
NOW=$(date +%s)
ELAPSED=$(( NOW - START ))
REMAIN=$(( WORK_SEC - ELAPSED ))

if [ "$REMAIN" -le 0 ]; then
    osascript -e 'display notification "Pomodoro complete" with title "Sketchybar"' 2>/dev/null
    rm -f "$STATE_FILE"
    sketchybar --set "$NAME" label="done!" icon.color=0xffa6e3a1
    exit 0
fi

MIN=$(( REMAIN / 60 ))
SEC=$(( REMAIN % 60 ))
printf -v DISPLAY "%02d:%02d" "$MIN" "$SEC"

sketchybar --set "$NAME" label="$DISPLAY" icon.color=0xfff38ba8
