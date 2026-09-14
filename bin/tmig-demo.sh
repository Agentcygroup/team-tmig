#!/usr/bin/env bash
set -euo pipefail
BIN="$HOME/team-tmig/target/release/tmig"; ROOT="$HOME/team-tmig/src"
rm -f "$HOME/.tmig/ledger.jsonl"
echo "=== init ==="; $BIN init
echo; echo "=== set-intent: construction ==="
H1=$($BIN set-intent --construction --pattern 'unsafe ' --root "$ROOT" | awk '{print $3}'); echo "intent: $H1"; $BIN execute "$H1" --kind construction --pattern 'unsafe ' --root "$ROOT"
echo; echo "=== set-intent: coverage ==="
H2=$($BIN set-intent --coverage --invariant 'unwrap\(\)' --root "$ROOT" | awk '{print $3}'); echo "intent: $H2"; $BIN execute "$H2" --kind coverage --invariant 'unwrap\(\)' --root "$ROOT"
echo; echo "=== set-intent: fan ==="
H3=$($BIN set-intent --fan --command 'echo hello' | awk '{print $3}'); echo "intent: $H3"; $BIN execute "$H3" --kind fan --command 'echo hello' --nodes 10
echo; echo "=== audit ==="; $BIN audit
echo; echo "=== red-team ==="; $BIN red-team
echo; echo "=== ledger ==="; $BIN ledger --last 10
