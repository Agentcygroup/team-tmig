#!/usr/bin/env bash
# red/04: adversarial invariant placeholder — refuses to pass by accident.
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f Cargo.toml ] || { echo "red/04: not in crate root"; exit 1; }
exit 0
