#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
bad=0
while IFS= read -r f; do
  python3 -c "import json,sys;json.load(open(sys.argv[1]))" "$f" 2>/dev/null || { echo "BAD $f"; bad=1; }
done < <(find declarations tests checks -name '*.json' 2>/dev/null)
exit "$bad"
