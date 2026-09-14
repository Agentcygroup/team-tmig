#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# no SQL in this crate — pass unless SQL string interpolation appears
hits=$(grep -rnE '(format!\(.*SELECT|format!\(.*INSERT|format!\(.*UPDATE)' src --include='*.rs' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
