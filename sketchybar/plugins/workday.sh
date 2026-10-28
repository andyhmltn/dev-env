#!/usr/bin/env zsh

WORKDAY_END_HOUR=18
WORKDAY_END_MIN=0

DOW=$(date +%u)
if [ "$DOW" -ge 6 ]; then
    sketchybar --set "$NAME" drawing=off
    exit 0
fi

NOW_MIN=$(( $(date +%H) * 60 + $(date +%M) ))
END_MIN=$(( WORKDAY_END_HOUR * 60 + WORKDAY_END_MIN ))
DIFF=$(( END_MIN - NOW_MIN ))

if [ "$DIFF" -le 0 ]; then
    OVER=$(( -DIFF ))
    H=$(( OVER / 60 ))
    M=$(( OVER % 60 ))
    if [ "$H" -gt 0 ]; then
        LABEL="+${H}h${M}m"
    else
        LABEL="+${M}m"
    fi
    COLOR=0xfff38ba8
else
    H=$(( DIFF / 60 ))
    M=$(( DIFF % 60 ))
    if [ "$H" -gt 0 ]; then
        LABEL="${H}h${M}m"
    else
        LABEL="${M}m"
    fi
    if [ "$DIFF" -le 30 ]; then
        COLOR=0xfffab387
    else
        COLOR=0xffe4e4ef
    fi
fi

sketchybar --set "$NAME" drawing=on label="$LABEL" label.color="$COLOR"
