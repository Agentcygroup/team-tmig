#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# Flag serde_json deserialization from untrusted sources.
# A site is bounded if the call line OR either of the two lines above it
# carries "// bounded".
hits=""
while IFS=: read -r f ln _rest; do
  [ -z "${f:-}" ] && continue
  lo=$(( ln > 2 ? ln - 2 : 1 ))
  ctx=$(sed -n "${lo},${ln}p" "$f")
  if ! printf '%s' "$ctx" | grep -q '// bounded'; then
    hits="${hits}${f}:${ln}"$'\n'
  fi
done < <(grep -rnE 'serde_json::(from_str|from_slice|from_reader)' src --include='*.rs' || true)

if [ -n "$hits" ]; then
  printf '%s' "$hits"
  exit 1
fi
