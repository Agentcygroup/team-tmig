#!/usr/bin/env bash
# red/02: supply chain — every lock entry must come from crates.io or a pinned git rev.
set -euo pipefail; cd "$(dirname "$0")/../.."
bad=$(grep -E '^source = ' Cargo.lock | grep -v 'crates.io-index' | grep -vE 'git\+.*#[0-9a-f]{40}' || true)
[ -z "$bad" ] && exit 0
echo "$bad"; exit 1
