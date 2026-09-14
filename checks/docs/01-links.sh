#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# relative markdown links must resolve
python3 - <<'PY'
import re, pathlib, sys
bad = []
for md in pathlib.Path(".").rglob("*.md"):
    if any(part in {"target",".git",".audit"} for part in md.parts): continue
    for m in re.finditer(r'\]\(([^)]+)\)', md.read_text()):
        href = m.group(1)
        if href.startswith(("http://","https://","#","mailto:")): continue
        target = (md.parent / href.split("#",1)[0]).resolve()
        if not target.exists(): bad.append(f"{md}: {href}")
if bad:
    print("\n".join(bad)); sys.exit(1)
PY
