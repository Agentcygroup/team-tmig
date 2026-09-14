#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f docs/RUNBOOK.md ] || { echo "docs/RUNBOOK.md missing"; exit 1; }
