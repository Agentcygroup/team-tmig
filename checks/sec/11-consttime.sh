#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# forbid comparing secrets with == (should be subtle::ConstantTimeEq)
hits=$(grep -rnE 'secret|hmac|mac\b' src --include='*.rs' | grep -E '==|!=' | grep -v '// ' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
