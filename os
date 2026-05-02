#!/bin/bash
cd "$(dirname "$0")" && cargo build --release -q 2>&1 && exec ./target/release/os "$@"
