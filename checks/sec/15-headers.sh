#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
req=(X-Frame-Options Content-Security-Policy Strict-Transport-Security)
found=0; fail=0
while IFS= read -r -d '' f; do
  if grep -Eiq 'header\(|set_header|add_header|response\.headers|HeaderValue' "$f"; then
    found=1
    for h in "${req[@]}"; do grep -Eiq "$h" "$f" || { echo "MISSING: $h in $f"; fail=1; }; done
  fi
done < <(find src docs configs .github -type f -print0 2>/dev/null)
[ "$found" -eq 0 ] && { echo "no HTTP surfaces — pass"; exit 0; }
exit "$fail"
