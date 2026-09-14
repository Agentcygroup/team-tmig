#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
cargo test --release --test boundaries
