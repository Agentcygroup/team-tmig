#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -d catalog ] || exit 0
python3 - <<'PY'
import json, pathlib
for p in pathlib.Path("catalog").glob("*.json"):
    json.loads(p.read_text())
PY
