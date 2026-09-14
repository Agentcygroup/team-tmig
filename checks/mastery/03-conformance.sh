#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
for f in docs/CONFORMANCE.md docs/CLAIM_GRAMMAR.md docs/DOCTRINE.md docs/STATEMENT_OF_APPLICABILITY.md; do
  [ -f "$f" ] || { echo "missing $f"; exit 1; }
done
