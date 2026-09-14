#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=$(grep -rnE '\b(from_reader|from_slice|from_str)\b' src --include='*.rs' \
  | grep -vE '(extend_from_slice|copy_from_slice)' | grep -v '// bounded' || true)
[ -z "$hits" ] || { echo "$hits"; exit 1; }
