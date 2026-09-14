#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# ensure no credential files are tracked
hits=$(git ls-files | grep -E '(^|/)(\.env|\.netrc|credentials|id_rsa|id_ed25519)$' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
