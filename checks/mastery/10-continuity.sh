#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
for f in AGENTS.md CONTRIBUTING.md docs/RUNBOOK.md; do
  [ -f "$f" ] || { echo "missing $f"; exit 1; }
done
