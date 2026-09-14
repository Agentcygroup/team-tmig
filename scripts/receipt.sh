#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/provenance
sha=$(git rev-parse HEAD)
{
  echo "green baseline"
  echo "commit: $sha"
  echo "date:   $(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > target/provenance/green-receipt.txt
echo "target/provenance/green-receipt.txt"
