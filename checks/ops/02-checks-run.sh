#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
bad=""
for f in $(find checks -name '*.sh' | sort); do
  bash "$f" >/dev/null 2>&1 || bad="$bad $f"
done
[ -z "$bad" ] || { echo "failing:$bad"; exit 1; }
