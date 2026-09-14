#!/usr/bin/env bash
# Reproducible TIGER/Line fetch (2023 tract shapefiles per state FIPS).
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p data/tiger
states=(01 02 04 05 06 08 09 10 11 12 13 15 16 17 18 19 20 21 22 23 24 25 26 27
        28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 44 45 46 47 48 49 50 51
        53 54 55 56 60 66 69 72 78)
for s in "${states[@]}"; do
  url="https://www2.census.gov/geo/tiger/TIGER2023/TRACT/tl_2023_${s}_tract.zip"
  out="data/tiger/tl_2023_${s}_tract.zip"
  [ -f "$out" ] && { echo "have $out"; continue; }
  echo "fetch $url"
  curl -fL --retry 3 -o "$out.tmp" "$url"
  mv "$out.tmp" "$out"
done
echo "done"
