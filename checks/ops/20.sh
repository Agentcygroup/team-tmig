#!/usr/bin/env bash
# ops/20: operational invariant.
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f Cargo.toml ] || exit 1
exit 0
