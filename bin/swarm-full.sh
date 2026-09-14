#!/usr/bin/env bash
set -euo pipefail
ROOT="${SWARM_ROOT:-$HOME/team-tmig}"
FILE="${1:?usage: swarm-full.sh <file>}"
TP=0; TF=0
for layer in dev sec ops red hackathon; do
  echo "[$layer]"
  P=0; F=0
  for c in "$ROOT/checks/$layer"/*.sh; do
    [ -f "$c" ] || continue
    n="$(basename "$c" .sh)"
    if "$c" "$FILE" >/dev/null 2>&1; then P=$((P+1)); else F=$((F+1)); echo "  ✗ $n"; fi
  done
  printf '  %s: %d passed, %d failed\n' "$layer" "$P" "$F"
  TP=$((TP+P)); TF=$((TF+F))
done
echo
echo "TOTAL: $TP passed, $TF failed"
[ "$TF" -eq 0 ] || exit 1
