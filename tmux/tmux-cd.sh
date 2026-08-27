#!/bin/sh
dir=$(eval echo "$1")
tmux attach-session -t . -c "$dir"
