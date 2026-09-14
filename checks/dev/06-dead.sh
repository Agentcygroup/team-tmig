#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
command -v cargo-udeps >/dev/null 2>&1 || { echo "skip: cargo-udeps"; exit 0; }
cargo +nightly udeps --all-targets >/dev/null 2>&1
exit 0
