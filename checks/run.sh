#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
groups=("$@")
[ "${#groups[@]}" -eq 0 ] && groups=(dev sec red ops hackathon it pentest org infra pipeline compliance data sre ml docs perf chaos compose)
fail=0
for g in "${groups[@]}"; do
  [ -d "checks/$g" ] || continue
  for f in checks/"$g"/*.sh; do
    [ -e "$f" ] || continue
    [ -x "$f" ] || chmod +x "$f"
    printf '\n[%s] %s\n' "$g" "$(basename "$f")"
    if ! "$f"; then
      case "$g" in
        dev|sec) fail=1 ;;
        *) echo "ADVISORY ($g): $(basename "$f") failed" ;;
      esac
    fi
  done
done
exit $fail
