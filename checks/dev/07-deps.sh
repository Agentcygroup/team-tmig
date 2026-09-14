#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
command -v cargo-machete >/dev/null || { echo "skip: cargo-machete not installed"; exit 0; }
cargo machete
