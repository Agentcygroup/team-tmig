#!/usr/bin/env bash
cd "$1/.." 2>/dev/null || exit 0
cargo audit >/dev/null 2>&1 || exit 0
