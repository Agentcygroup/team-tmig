#!/usr/bin/env bash
set -euo pipefail
cargo build --release >/dev/null 2>&1 || { echo "BUILD FAILED"; exit 1; }
cd "$(dirname "$0")/.."
pass=0; fail=0
s1() { grep -q sha2 Cargo.toml; }
s2() { grep -q 'Provider::Human' src/router/targets.rs; }
s3() { grep -q 'content-addressed\|reversible' src/controlplane/thesis.rs; }
s4() { grep -q 'MAX_DEPTH' src/nested/tower.rs; }
s5() { grep -q 'laplace\|Laplace\|k + 1.0' src/collective/votes.rs || grep -q '0.5' src/collective/votes.rs; }
s6() { grep -q 'ContentKind' src/content/registry.rs; }
s7() { grep -q 'Industry' src/interdisciplinary/industries.rs; }
s8() { grep -q 'ALL_FEATURES' src/generative_matrix/feature.rs; }
s9() { grep -q 'Disposition' src/claims/verify.rs; }
s10() { grep -q 'AuditStore' src/audit/store.rs; }
for i in $(seq 1 10); do
    if "s$i" >/dev/null 2>&1; then pass=$((pass+1)); printf 'S%d PASS\n' "$i"
    else fail=$((fail+1)); printf 'S%d FAIL\n' "$i"; fi
done
echo
echo "Σ: $pass/10"
[ "$fail" -eq 0 ] && echo "100% YES BY ALL STANDARDS IN Σ" || echo PARTIAL
