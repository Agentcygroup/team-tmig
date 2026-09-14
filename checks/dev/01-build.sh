#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
cargo build --release --all-targets
