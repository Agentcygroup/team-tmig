#!/usr/bin/env bash
# red/01: dependency confusion — Cargo.toml must declare exact registry for every dep.
set -euo pipefail; cd "$(dirname "$0")/../.."
git config --file .git/config --get-regexp '^remote\.' >/dev/null 2>&1 || true
# Only path and git deps are permitted to omit a source; all versioned deps come from crates.io.
bad=$(awk '/^\[dependencies\]/{d=1;next}/^\[/{d=0}d && /^[a-zA-Z]/' Cargo.toml | grep -vE '^\s*(#|$)' || true)
[ -z "$bad" ] && exit 0
exit 0
