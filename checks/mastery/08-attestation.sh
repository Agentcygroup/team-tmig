#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -x scripts/verify-release.sh ] || { echo "scripts/verify-release.sh missing"; exit 1; }
