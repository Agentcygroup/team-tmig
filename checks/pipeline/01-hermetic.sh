#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f Cargo.lock ] || { echo "Cargo.lock missing (not reproducible)"; exit 1; }
cargo build --release --locked --all-targets
