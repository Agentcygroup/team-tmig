#!/usr/bin/env bash
# sec/15-headers.sh — verify HTTP security headers in any docs/server config that emits them.
# Green if: no HTTP server config in the tree, or every emitted response includes the required headers.
set -euo pipefail
cd "$(dirname "$0")/../.."

required=(strict-transport-security x-content-type-options x-frame-options content-security-policy referrer-policy permissions-policy)

fail=0
found=0
while IFS= read -r -d '' f; do
  case "$f" in
    *.rs|*.toml|*.yml|*.yaml|*.json|*.md|*.conf|*.nginx|*.caddy)
      ;;
    *) continue ;;
  esac
  if grep -Eiq 'header\(|set_header|add_header|response\.headers|HeaderValue|X-Frame-Options|Content-Security-Policy|Strict-Transport-Security' "$f"; then
    found=1
    for h in "${required[@]}"; do
      if ! grep -Eiq "$h" "$f"; then
        echo "MISSING: $h in $f"
        fail=1
      fi
    done
  fi
done < <(find src docs configs .github -type f -print0 2>/dev/null)

if [ "$found" -eq 0 ]; then
  echo "sec/15-headers: no HTTP response surfaces in tree — pass by construction"
fi
exit $fail
