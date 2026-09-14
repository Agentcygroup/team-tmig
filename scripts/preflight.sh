#!/usr/bin/env bash
# Full local gate. Non-fatal on missing optional tools; skips intentionally-red checks.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
step() { printf '\n\033[1;34m== %s\033[0m\n' "$1"; }
run()  { if "$@"; then :; else echo "FAIL: $*"; fail=1; fi; }
opt()  { command -v "$1" >/dev/null 2>&1; }

step "fmt";        run cargo fmt --all -- --check
step "clippy";     run cargo clippy --all-targets --all-features -- -D warnings
step "test";       run cargo test --release --all-targets
step "docs";       run cargo doc --no-deps --document-private-items
step "checks";     [ -x checks/run.sh ] && run checks/run.sh dev sec || true
step "receipt";    run cargo xt receipt
opt cargo-audit    && { step "audit";    run cargo audit; }                                       || echo "skip cargo-audit"
opt cargo-deny     && { step "deny";     run cargo deny check advisories bans licenses sources; } || echo "skip cargo-deny"
opt cargo-machete  && { step "machete";  run cargo machete; }                                     || echo "skip cargo-machete"
opt cargo-outdated && { step "outdated"; run cargo outdated --format json > /dev/null; }          || echo "skip cargo-outdated"
if [ "$fail" -eq 0 ]; then printf '\n\033[1;32mpreflight: OK\033[0m\n'; else printf '\n\033[1;31mpreflight: NOT OK\033[0m\n'; exit 1; fi
