#!/usr/bin/env zsh

PAGE_SIZE=$(vm_stat | awk '/page size of/ {print $8}')
STATS=$(vm_stat)

FREE=$(echo "$STATS" | awk '/Pages free/ {gsub("\\.",""); print $3}')
ACTIVE=$(echo "$STATS" | awk '/Pages active/ {gsub("\\.",""); print $3}')
INACTIVE=$(echo "$STATS" | awk '/Pages inactive/ {gsub("\\.",""); print $3}')
SPECULATIVE=$(echo "$STATS" | awk '/Pages speculative/ {gsub("\\.",""); print $3}')
WIRED=$(echo "$STATS" | awk '/Pages wired down/ {gsub("\\.",""); print $4}')
COMPRESSED=$(echo "$STATS" | awk '/Pages occupied by compressor/ {gsub("\\.",""); print $5}')

TOTAL_PAGES=$((FREE + ACTIVE + INACTIVE + SPECULATIVE + WIRED + COMPRESSED))
USED_PAGES=$((ACTIVE + WIRED + COMPRESSED))

PERCENT=$(( USED_PAGES * 100 / TOTAL_PAGES ))

sketchybar --set "$NAME" label="${PERCENT}%"
