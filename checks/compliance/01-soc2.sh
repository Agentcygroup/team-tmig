#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
for f in docs/ACCESS_CONTROL.md docs/CHANGE_MANAGEMENT.md docs/INCIDENT_RESPONSE.md docs/RISK_REGISTER.md; do
  [ -f "$f" ] || { echo "missing $f"; exit 1; }
done
