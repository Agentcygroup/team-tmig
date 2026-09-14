#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -d src/level8_metrology ] || { echo "level8_metrology missing"; exit 1; }
