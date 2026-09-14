#!/usr/bin/env bash
# Emit mastery-map.json and append a Declaration to the ledger.
set -euo pipefail

ROOT="${HOME}/team-tmig"
LEDGER="${HOME}/.tmig/hitl.jsonl"
MAP="${ROOT}/target/provenance/mastery-map.json"
DECL_DIR="${ROOT}/declarations"

python3 "${ROOT}/bin/mastery-map.py"

HASH=$(shasum -a 256 "$MAP" | awk '{print $1}')

mkdir -p "$DECL_DIR"
cat > "${DECL_DIR}/mastery-map.json" <<JSON
{
  "claim_id": "mastery-map",
  "subject": "specialization over experience, derived from the hitl ledger",
  "evidence_path": "target/provenance/mastery-map.json",
  "evidence_hash": "$HASH",
  "ts": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
JSON

echo "declaration: ${DECL_DIR}/mastery-map.json -> $HASH"

# Append an Annotated event pointing at the map, if the ledger exists.
if [ -f "$LEDGER" ]; then
  TS=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  printf '{"type":"Annotated","ts":"%s","target_event_hash":"%s","declaration":{"claim_id":"mastery-map","evidence_hash":"%s"}}\n' \
    "$TS" "$HASH" "$HASH" >> "$LEDGER"
  echo "ledger: appended Annotated mastery-map event"
fi
