#!/usr/bin/env bash
# Full local gate before any push. Mirrors every CI job.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
step() { printf '\n\033[1;34m== %s\033[0m\n' "$1"; }
run()  { if "$@"; then :; else echo "FAIL: $*"; fail=1; fi; }
step "fmt";    run cargo fmt --all -- --check
step "clippy"; run cargo clippy --all-targets --all-features -- -D warnings
step "test";   run cargo test --release --all-targets
step "checks"; [ -x checks/run.sh ] && run checks/run.sh dev sec red || true
step "receipt"; run cargo xt receipt
step "audit";  command -v cargo-audit >/dev/null && run cargo audit || echo "skip cargo-audit"
step "deny";   command -v cargo-deny  >/dev/null && run cargo deny check || echo "skip cargo-deny"
if [ "$fail" -eq 0 ]; then printf '\n\033[1;32mpreflight: OK\033[0m\n'; else printf '\n\033[1;31mpreflight: NOT OK\033[0m\n'; exit 1; fi
