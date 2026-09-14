#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -d data ] || { echo "no data/ dir"; exit 0; }
find data -name '*.json' -print0 | xargs -0 -n1 -I{} sh -c 'python3 -m json.tool "{}" >/dev/null'
