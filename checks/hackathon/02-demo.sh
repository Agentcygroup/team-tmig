#!/usr/bin/env bash
# 02-demo.sh: hackathon submission invariant.
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f README.md ] || exit 1
exit 0
