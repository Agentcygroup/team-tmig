#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
command -v cargo-cyclonedx >/dev/null || { echo "skip: cargo-cyclonedx not installed"; exit 0; }
cargo cyclonedx --format json --output-file target/provenance/sbom.cdx.json
