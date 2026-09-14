#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# grep for high-signal secret markers only — no false positives on placeholder tokens
hits=$(grep -rnE '(AKIA[0-9A-Z]{16}|-----BEGIN (RSA|EC|OPENSSH) PRIVATE KEY|sk_live_[A-Za-z0-9]{20,}|ghp_[A-Za-z0-9]{36})' . \
  --exclude-dir=target --exclude-dir=.git --exclude-dir=.audit --exclude='Cargo.lock' 2>/dev/null || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
