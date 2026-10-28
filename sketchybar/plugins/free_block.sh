#!/usr/bin/env zsh

BIN=/opt/homebrew/bin/icalBuddy
[ ! -x "$BIN" ] && { sketchybar --set "$NAME" drawing=off; exit 0; }

WORKDAY_END_HOUR=18
MIN_GAP=30

DOW=$(date +%u)
if [ "$DOW" -ge 6 ]; then
    sketchybar --set "$NAME" drawing=off
    exit 0
fi

NOW_MIN=$(( $(date +%H) * 60 + $(date +%M) ))
END_MIN=$(( WORKDAY_END_HOUR * 60 ))

if [ "$NOW_MIN" -ge "$END_MIN" ]; then
    sketchybar --set "$NAME" drawing=off
    exit 0
fi

RAW=$("$BIN" \
    -nc -npn -nrd -b "" \
    -ea \
    -iep "title,datetime" \
    -df "" \
    -tf "%H:%M" \
    eventsToday 2>/dev/null)

INTERVALS=()
while IFS= read -r LINE; do
    STRIPPED=$(printf '%s' "$LINE" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
    [ -z "$STRIPPED" ] && continue
    if printf '%s' "$STRIPPED" | grep -qE "^[0-9]{2}:[0-9]{2} - [0-9]{2}:[0-9]{2}$"; then
        S=$(printf '%s' "$STRIPPED" | awk '{print $1}')
        E=$(printf '%s' "$STRIPPED" | awk '{print $3}')
        S_MIN=$(( ${S%%:*} * 60 + ${S##*:} ))
        E_MIN=$(( ${E%%:*} * 60 + ${E##*:} ))
        [ "$E_MIN" -le "$NOW_MIN" ] && continue
        [ "$S_MIN" -lt "$NOW_MIN" ] && S_MIN=$NOW_MIN
        INTERVALS+=("${S_MIN},${E_MIN}")
    fi
done <<< "$RAW"

IFS=$'\n' SORTED=($(printf '%s\n' "${INTERVALS[@]}" | sort -n -t, -k1))
unset IFS

CURSOR=$NOW_MIN
FREE_START=""
FREE_END=""

for ITV in "${SORTED[@]}"; do
    S=${ITV%,*}
    E=${ITV#*,}
    if (( S - CURSOR >= MIN_GAP )); then
        FREE_START=$CURSOR
        FREE_END=$S
        break
    fi
    (( E > CURSOR )) && CURSOR=$E
done

if [ -z "$FREE_START" ]; then
    if (( END_MIN - CURSOR >= MIN_GAP )); then
        FREE_START=$CURSOR
        FREE_END=$END_MIN
    else
        sketchybar --set "$NAME" drawing=off
        exit 0
    fi
fi

fmt() {
    printf '%02d:%02d' $(( $1 / 60 )) $(( $1 % 60 ))
}

sketchybar --set "$NAME" drawing=on label="$(fmt $FREE_START)-$(fmt $FREE_END)"
