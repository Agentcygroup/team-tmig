#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=$(grep -rnE 'innerHTML|dangerouslySetInnerHTML|write!\(.*<script' src --include='*.rs' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
