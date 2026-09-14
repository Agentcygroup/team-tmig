#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/.."
[ $# -eq 1 ] || { echo "usage: $0 <tag>"; exit 2; }
git tag -v "$1"
sha=$(git tag -l --format='%(contents)' "$1" | awk -F': ' '/receipt-sha256/ {print $2}')
[ -n "$sha" ] && echo "tag claims receipt sha256: $sha" || echo "no receipt sha in tag message"
if command -v gh >/dev/null 2>&1; then
  gh attestation verify team-tmig-*.tar.gz --owner Agentcygroup 2>/dev/null || \
    echo "(no local artifacts to verify — download from release first)"
fi
