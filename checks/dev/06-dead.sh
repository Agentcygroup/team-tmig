#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
command -v cargo-udeps >/dev/null || { echo "skip: cargo-udeps not installed"; exit 0; }
cargo +nightly udeps --all-targets
