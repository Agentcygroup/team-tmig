#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
groups=("$@")
[ "${#groups[@]}" -eq 0 ] && groups=(dev sec)
fail=0
for g in "${groups[@]}"; do
  [ -d "checks/$g" ] || continue
  for f in checks/"$g"/*.sh; do
    [ -e "$f" ] || continue
    [ -x "$f" ] || chmod +x "$f"
    if ! "$f"; then
      case "$g" in
        dev|sec) echo "FAIL: $f"; fail=1 ;;
        *) echo "ADVISORY ($g): $f" ;;
      esac
    fi
  done
done
exit $fail
