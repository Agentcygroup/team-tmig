#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f target/provenance/green-receipt.txt ] || { echo "no green receipt — run: cargo xt receipt"; exit 1; }
claimed=$(awk -F': ' '/^commit:/{print $2}' target/provenance/green-receipt.txt)
commit=$(git rev-parse HEAD)
[ "$claimed" = "$commit" ] || { echo "receipt commit mismatch: $claimed vs $commit"; exit 1; }
sha=$(shasum -a 256 target/provenance/green-receipt.txt | awk '{print $1}')
echo "receipt $sha @ $commit"
