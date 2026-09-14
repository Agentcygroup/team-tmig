#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=$(grep -rnE 'from_reader\(|from_slice\(' src --include='*.rs' | grep -v '// bounded' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
