#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f declarations/mastery-map.json ] || { echo "mastery-map.json missing"; exit 1; }
python3 -c 'import json; json.load(open("declarations/mastery-map.json"))'
