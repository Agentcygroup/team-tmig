#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
pass=0; fail=0
c1() { cargo build --release --quiet; }
c2() { n=$(ls target/release 2>/dev/null | grep -v '^\.' | grep -v '\.d$' | wc -l | tr -d ' '); [ "$n" -ge 3 ]; }
c3() { n=$(find src -name mod.rs | wc -l | tr -d ' '); [ "$n" -ge 30 ]; }
c4() { grep -q sha2 Cargo.toml; }
c5() { [ -d .audit ]; }
c6() { cargo test --release --quiet --lib; }
c7() { grep -q 'Provider::Ollama' src/router/targets.rs; }
c8() { [ -f src/claims/verify.rs ]; }
c9() { cargo doc --release --no-deps --quiet; }
for i in $(seq 1 9); do
    if "c$i" >/dev/null 2>&1; then pass=$((pass+1)); printf 'C%d PASS\n' "$i"
    else fail=$((fail+1)); printf 'C%d FAIL\n' "$i"; fi
done
total=$((pass+fail))
pct=$(echo "scale=4; $pass * 100 / $total" | bc 2>/dev/null || echo "0")
echo
echo "clauses: $pass/$total = ${pct}%"
[ "$fail" -eq 0 ] && echo CONFORMANT || echo NON-CONFORMANT
