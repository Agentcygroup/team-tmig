#!/usr/bin/env bash
cd "$1/.." 2>/dev/null || exit 0
cargo build --release >/dev/null 2>&1
