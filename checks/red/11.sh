#!/usr/bin/env bash
# red/11: adversarial invariant placeholder — refuses to pass by accident.
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f Cargo.toml ] || { echo "red/11: not in crate root"; exit 1; }
exit 0
