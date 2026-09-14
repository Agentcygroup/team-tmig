#!/usr/bin/env bash
cd "$1/.." 2>/dev/null || exit 0
cargo test --release --lib >/dev/null 2>&1
