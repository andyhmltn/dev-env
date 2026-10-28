#!/usr/bin/env zsh

BIN=/opt/homebrew/bin/icalBuddy
[ ! -x "$BIN" ] && { sketchybar --set "$NAME" drawing=off; exit 0; }

RAW=$("$BIN" \
    -nc -npn -nrd -b "" \
    -ea \
    -iep "title,datetime" \
    -df "%Y-%m-%d" \
    -tf "%H:%M" \
    eventsToday+1 2>/dev/null)

if [ -z "$RAW" ]; then
    sketchybar --set "$NAME" drawing=off
    exit 0
fi

NOW_EPOCH=$(date +%s)
TODAY=$(date +%Y-%m-%d)

TITLE=""
DATE=""
TIME=""

while IFS= read -r LINE; do
    STRIPPED=$(printf '%s' "$LINE" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
    [ -z "$STRIPPED" ] && continue

    if printf '%s' "$STRIPPED" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2} at [0-9]{2}:[0-9]{2}'; then
        D=$(printf '%s' "$STRIPPED" | grep -oE '^[0-9]{4}-[0-9]{2}-[0-9]{2}')
        T=$(printf '%s' "$STRIPPED" | grep -oE '[0-9]{2}:[0-9]{2}' | head -1)
        EV_EPOCH=$(date -j -f "%Y-%m-%d %H:%M" "$D $T" +%s 2>/dev/null)
        if [ -n "$EV_EPOCH" ] && [ "$EV_EPOCH" -ge "$NOW_EPOCH" ]; then
            DATE="$D"
            TIME="$T"
            break
        fi
        TITLE=""
    else
        TITLE="$STRIPPED"
    fi
done <<< "$RAW"

if [ -z "$TITLE" ] || [ -z "$TIME" ]; then
    sketchybar --set "$NAME" drawing=off
    exit 0
fi

if [ "$DATE" = "$TODAY" ]; then
    PREFIX="$TIME"
else
    DAY_LABEL=$(date -j -f "%Y-%m-%d" "$DATE" "+%a" 2>/dev/null)
    PREFIX="$DAY_LABEL $TIME"
fi

SHORT=$(printf '%s' "$TITLE" | cut -c1-28)
[ "${#TITLE}" -gt 28 ] && SHORT="${SHORT}…"

sketchybar --set "$NAME" drawing=on label="$PREFIX $SHORT"
