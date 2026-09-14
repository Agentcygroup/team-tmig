#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
command -v cargo-llvm-cov >/dev/null || { echo "skip: cargo-llvm-cov not installed"; exit 0; }
cargo llvm-cov --all-targets --lcov --output-path lcov.info
