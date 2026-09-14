#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f README.md ] || { echo "README.md missing"; exit 1; }
[ -f SECURITY.md ] || { echo "SECURITY.md missing"; exit 1; }
[ -f CHANGELOG.md ] || { echo "CHANGELOG.md missing"; exit 1; }
[ -f LICENSE ] || { echo "LICENSE missing"; exit 1; }
