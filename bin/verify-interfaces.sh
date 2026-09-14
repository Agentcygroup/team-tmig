#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
cargo doc --release --no-deps --quiet
echo "interfaces: target/doc/tmig/index.html"
