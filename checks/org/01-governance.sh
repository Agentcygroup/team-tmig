#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
for f in CODEOWNERS .github/CODEOWNERS SECURITY.md CONTRIBUTING.md CHANGELOG.md LICENSE; do
  [ -f "$f" ] || [ -f ".github/$f" ] || { echo "missing $f"; exit 1; }
done
