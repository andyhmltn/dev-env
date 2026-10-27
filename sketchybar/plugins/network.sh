#!/usr/bin/env zsh

STATE_FILE="/tmp/sketchybar_network_state"
IFACE=$(route get default 2>/dev/null | awk '/interface:/ {print $2}')
[ -z "$IFACE" ] && { sketchybar --set "$NAME" label="offline"; exit 0; }

READ=$(netstat -ibn | awk -v i="$IFACE" '$1 == i && $4 ~ /Link/ {print $7; exit}')
WRITE=$(netstat -ibn | awk -v i="$IFACE" '$1 == i && $4 ~ /Link/ {print $10; exit}')
NOW=$(date +%s)

if [ -f "$STATE_FILE" ]; then
    read PREV_T PREV_R PREV_W < "$STATE_FILE"
    DT=$(( NOW - PREV_T ))
    [ "$DT" -lt 1 ] && DT=1
    DOWN_KB=$(( (READ  - PREV_R) / DT / 1024 ))
    UP_KB=$(( (WRITE - PREV_W) / DT / 1024 ))
else
    DOWN_KB=0
    UP_KB=0
fi

echo "$NOW $READ $WRITE" > "$STATE_FILE"

fmt() {
    local kb=$1
    if [ "$kb" -ge 1024 ]; then
        printf "%.1fM" "$(echo "$kb / 1024" | bc -l)"
    else
        printf "%dK" "$kb"
    fi
}

sketchybar --set "$NAME" label="↓$(fmt $DOWN_KB) ↑$(fmt $UP_KB)"
