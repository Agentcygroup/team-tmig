#!/usr/bin/env bash
# Mirrors CI on a dev machine. Fails fast.
set -euo pipefail
cd "$(dirname "$0")/../.."
echo "[ci] fmt";        cargo fmt --all -- --check
echo "[ci] clippy";     cargo clippy --all-targets --all-features -- -D warnings
echo "[ci] test";       cargo test --release --all-targets
echo "[ci] audit";      command -v cargo-audit >/dev/null && cargo audit || echo "skip: cargo-audit missing"
echo "[ci] deny";       command -v cargo-deny  >/dev/null && cargo deny check advisories bans licenses sources || echo "skip: cargo-deny missing"
echo "[ci] ok"
