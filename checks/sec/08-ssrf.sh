#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=$(grep -rnE 'reqwest::|hyper::Client|ureq::' src --include='*.rs' || true)
[ -z "$hits" ] && exit 0
echo "$hits"; exit 1
