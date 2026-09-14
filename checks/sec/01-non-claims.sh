#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f docs/NON_CLAIMS.md ] || { echo "docs/NON_CLAIMS.md missing"; exit 1; }
for n in no-mastery no-accreditation no-coherence-proof no-completeness; do
  grep -q "$n" docs/NON_CLAIMS.md || { echo "missing non-claim: $n"; exit 1; }
done
