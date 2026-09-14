#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=$(grep -rn 'unsafe' src --include='*.rs' | grep -v 'src/ffi.rs' | grep -v '^[^:]*:[0-9]*:\s*//' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
