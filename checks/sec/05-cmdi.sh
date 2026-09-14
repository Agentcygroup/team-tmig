#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# forbid shell-string command construction with interpolated user data
hits=$(grep -rnE 'Command::new\("sh"\)|Command::new\("bash"\)|sh -c|bash -c' src --include='*.rs' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
