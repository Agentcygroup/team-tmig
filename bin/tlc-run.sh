#!/usr/bin/env bash
# Run TLC on spec/murmuration.tla if tla2tools.jar is present.
set -euo pipefail

ROOT="${HOME}/team-tmig"
JAR="${ROOT}/spec/tla2tools.jar"
SPEC="${ROOT}/spec/murmuration.tla"
CFG="${ROOT}/spec/murmuration.cfg"

if [ ! -f "$JAR" ]; then
  echo "tla2tools.jar not present."
  echo "Download: https://github.com/tlaplus/tlaplus/releases/latest/download/tla2tools.jar"
  echo "Place at: $JAR"
  echo "Then re-run: bash bin/tlc-run.sh"
  exit 2
fi

java -cp "$JAR" tlc2.TLC -config "$CFG" "$SPEC"
