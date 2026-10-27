#!/usr/bin/env zsh

CPU=$(top -l 2 -n 0 -s 1 | awk '/CPU usage/ {gsub("%",""); print $3 + $5}' | tail -1)
CPU_INT=${CPU%.*}

sketchybar --set "$NAME" label="${CPU_INT}%"
