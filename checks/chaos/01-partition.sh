#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# placeholder invariant: chaos tests land here; asserts the crate exposes a resilience module
[ -d src/resilience ] || { echo "src/resilience missing"; exit 1; }
