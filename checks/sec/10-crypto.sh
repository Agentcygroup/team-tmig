#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# forbid known-broken primitives
hits=$(grep -rnE '\b(md5|sha1|des|rc4)\b' src --include='*.rs' | grep -vi '// ' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
