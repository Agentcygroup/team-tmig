#!/usr/bin/env bash
set -euo pipefail
ROOT="${SWARM_ROOT:-$HOME/team-tmig}"
LAYER="${1:?usage: swarm-check.sh <layer> <file>}"
FILE="${2:?usage: swarm-check.sh <layer> <file>}"
PASS=0; FAIL=0
for c in "$ROOT/checks/$LAYER"/*.sh; do
  [ -f "$c" ] || continue
  n="$(basename "$c" .sh)"
  if "$c" "$FILE" >/dev/null 2>&1; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); echo "  ✗ $n"; fi
done
echo "  $LAYER: $PASS passed, $FAIL failed"
