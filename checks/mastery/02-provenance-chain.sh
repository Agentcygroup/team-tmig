#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f .audit/log/writes.jsonl ] || { echo "no audit ledger"; exit 1; }
tail -1 .audit/log/writes.jsonl | python3 -c 'import json,sys; json.loads(sys.stdin.read())'
