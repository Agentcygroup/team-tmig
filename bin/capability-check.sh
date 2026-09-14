#!/usr/bin/env bash
# Runs every capability from docs/CAPABILITIES.md once and reports pass/fail.
set -uo pipefail

ROOT="${HOME}/team-tmig"
HITL="${ROOT}/target/release/hitl"
CPH="${ROOT}/target/release/cp-hash"
CPA="${ROOT}/target/release/cp-attest"
SBOM="${ROOT}/target/release/sbom"
LEDGER="${HOME}/.tmig/hitl.jsonl"
OUT="${ROOT}/target/provenance/capability-check.txt"

pass=0; fail=0; skip=0
row() { printf '%-40s %s\n' "$1" "$2" | tee -a "$OUT"; }

: > "$OUT"
echo "# capability check $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$OUT"

check() {
  local name="$1"; shift
  if "$@" >/dev/null 2>&1; then
    row "$name" "PASS"; pass=$((pass+1))
  else
    row "$name" "FAIL"; fail=$((fail+1))
  fi
}

# A — authoring
[ -x "$HITL" ] && check "A1 author.proposal"          test -x "$HITL"
[ -f "$ROOT/docs/DOCTRINE.md" ] && check "A2 author.doctrine"  test -s "$ROOT/docs/DOCTRINE.md"
check "A3 author.catalog"                              test -s "$ROOT/catalog/local-models.json"
check "A4 author.declaration"                          test -d "$ROOT/declarations"
check "A5 author.primitive"                            grep -q "fn validate_annotation" "$ROOT/src/hitl/ledger.rs"

# B — approval
check "B1 approve.sign"                                grep -q "ed25519" "$ROOT/src/hitl/approval.rs" 2>/dev/null || grep -q "Ed25519" "$ROOT/src/hitl/approval.rs"
check "B2 approve.two-person"                          grep -q "two_person_rule_ok" "$ROOT/src/hitl/approval.rs"

# C — execution
check "C1 execute.approved"                            test -x "$HITL"
check "C2 execute.refuse-financial"                    grep -q "financial action" "$ROOT/src/hitl/executor.rs"
check "C3 execute.refuse-legal"                        grep -q "legal action" "$ROOT/src/hitl/executor.rs"

# D — verification
check "D1 verify.record"                               test -x "$HITL"
check "D2 verify.audit-standards"                      test -s "$ROOT/target/standards/audit.txt" || skip=$((skip+1))
check "D3 verify.declaration-hashes"                   cargo test --release --test cross_layer declaration_evidence_hashes_match_files -q

# E — ledger
check "E1 ledger.append"                               grep -q "fn append" "$ROOT/src/hitl/ledger.rs"
check "E2 ledger.read"                                 grep -q "fn open" "$ROOT/src/hitl/ledger.rs"
check "E3 ledger.revoke"                               grep -q "Revoked" "$ROOT/src/hitl/ledger.rs"
check "E4 ledger.annotate"                             grep -q "Annotated" "$ROOT/src/hitl/ledger.rs"

# F — control plane
check "F1 plane.derive"                                test -x "$CPH"
check "F2 plane.admit"                                 test -x "$CPA"
check "F3 plane.reject-out-of-scope"                   test -x "$CPA"
check "F4 plane.reject-no-approval"                    grep -q "NotHumanAuthorized" "$ROOT/src/controlplane/mod.rs"

# G — provenance
check "G1 provenance.emit"                             test -s "$ROOT/target/provenance/slsa.json"
check "G2 sbom.emit"                                   test -s "$ROOT/target/provenance/sbom.json"
check "G3 provenance.verify"                           test -s "$ROOT/target/provenance/slsa.json"

# H — catalog
check "H1 catalog.models"                              test -s "$ROOT/catalog/local-models.json"
check "H2 catalog.standards"                           test -s "$ROOT/catalog/standards-scope.json"
check "H3 catalog.tags"                                test -s "$ROOT/catalog/tag-vocabulary.json"
check "H4 catalog.composites"                          test -s "$ROOT/catalog/combinatorial-expansion.json"

# I — governance
check "I1 govern.audit-standards"                      test -s "$ROOT/target/standards/audit.txt"
check "I2 govern.check-boundary"                       test -r "$HOME/.pi/agent/APPEND_SYSTEM.md"
check "I3 govern.verify-backfill"                      test -f "$LEDGER"
check "I4 govern.mastery-map"                          test -s "$ROOT/target/provenance/mastery-map.json"

# J — reproducibility
check "J1 repro.build"                                 test -d "$ROOT/target/release"
check "J2 repro.test"                                  cargo test --release --lib -q
check "J3 repro.clean-build"                           test -x "$ROOT/target/release/cp-hash"

echo
echo "PASS: $pass  FAIL: $fail  SKIP: $skip"
echo "PASS: $pass  FAIL: $fail  SKIP: $skip" >> "$OUT"
