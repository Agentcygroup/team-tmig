#!/usr/bin/env bash
# Signed, receipt-backed tag. Usage: scripts/sign-and-tag.sh v0.1.1
set -euo pipefail
cd "$(dirname "$0")/.."
[ $# -eq 1 ] || { echo "usage: $0 <tag>"; exit 2; }
tag="$1"
scripts/preflight.sh
cargo xt receipt
sha=$(shasum -a 256 target/provenance/green-receipt.txt | awk '{print $1}')
git tag -s "$tag" -m "release $tag
receipt-sha256: $sha
commit: $(git rev-parse HEAD)"
git push origin main --follow-tags
echo "tagged $tag (receipt $sha)"
