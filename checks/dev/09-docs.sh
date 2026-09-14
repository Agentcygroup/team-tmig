#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
