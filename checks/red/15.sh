#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
if [ ! -f declarations/ai-glossary.gloss ]; then
  echo "red/15: no GLOSS file; pass by construction"; exit 0
fi
cargo test --release --lib gloss:: -- --nocapture
