#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f Dockerfile ] || { echo "Dockerfile missing"; exit 1; }
command -v docker >/dev/null || { echo "skip: docker not installed"; exit 0; }
docker build -q . >/dev/null
