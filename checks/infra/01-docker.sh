#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f Dockerfile ] || { echo "skip: no Dockerfile"; exit 0; }
grep -q '^FROM ' Dockerfile || { echo "Dockerfile missing FROM"; exit 1; }
