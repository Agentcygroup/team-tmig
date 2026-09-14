#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
python3 - <<'PY'
import pathlib, re
p = pathlib.Path("Cargo.toml")
s = p.read_text()
need = '[package.metadata.cargo-udeps.ignore]\nnormal = ["geo", "thiserror"]\n'
if "cargo-udeps.ignore" not in s:
    s = s.rstrip() + "\n\n" + need
    p.write_text(s)
    print("added cargo-udeps.ignore")
else:
    print("cargo-udeps.ignore present")
PY
