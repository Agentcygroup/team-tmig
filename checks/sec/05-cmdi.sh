#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=$(grep -rnE 'Command::new\("(sh|bash)"\)' src --include='*.rs' | grep -v '// allowed:' || true)
[ -z "$hits" ] || { echo "$hits"; exit 1; }
