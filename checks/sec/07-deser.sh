#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# Blocking: serde_json deserialization from untrusted sources.
# Allow if annotated with "// bounded" on the same line.
hits=$(grep -rnE 'serde_json::(from_str|from_slice|from_reader)' src --include='*.rs' \
  | grep -v '// bounded' || true)
[ -z "$hits" ] || { echo "$hits"; exit 1; }
