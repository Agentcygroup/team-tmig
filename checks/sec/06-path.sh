#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# catch raw join of an input that looks like a path variable into PathBuf
hits=$(grep -rnE 'PathBuf::from\(.*input|Path::new\(.*input' src --include='*.rs' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
