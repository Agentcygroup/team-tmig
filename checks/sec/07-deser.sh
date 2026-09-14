#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=""
while IFS=: read -r f ln _rest; do
  [ -z "${f:-}" ] && continue
  lo=$(( ln > 2 ? ln - 2 : 1 ))
  sed -n "${lo},${ln}p" "$f" | grep -q '// bounded' || hits="${hits}${f}:${ln}"$'\n'
done < <(grep -rnE 'serde_json::(from_str|from_slice|from_reader)' src --include='*.rs' || true)
[ -z "$hits" ] || { printf '%s' "$hits"; exit 1; }
