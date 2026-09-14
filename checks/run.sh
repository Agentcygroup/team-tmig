#!/usr/bin/env bash
# Unified runner for everything under checks/.
set -euo pipefail
cd "$(dirname "$0")/.."
groups=("$@")
[ "${#groups[@]}" -eq 0 ] && groups=(dev sec red ops hackathon it pentest org infra pipeline compliance data sre ml docs perf chaos)
fail=0
for g in "${groups[@]}"; do
  [ -d "checks/$g" ] || continue
  for f in checks/"$g"/*.sh; do
    [ -x "$f" ] || chmod +x "$f"
    printf '\n[%s] %s\n' "$g" "$(basename "$f")"
    if ! "$f"; then fail=1; fi
  done
done
exit $fail
