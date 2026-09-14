#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f benches/classify.rs ] || exit 0
cargo bench --no-run
