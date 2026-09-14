#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
command -v cargo-audit >/dev/null || { echo "skip: cargo-audit not installed"; exit 0; }
cargo audit
