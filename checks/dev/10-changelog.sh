#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f CHANGELOG.md ] || { echo "CHANGELOG.md missing"; exit 1; }
grep -q '^## \[Unreleased\]' CHANGELOG.md || { echo "CHANGELOG.md missing [Unreleased]"; exit 1; }
