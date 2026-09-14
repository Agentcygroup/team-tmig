#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# No dynamic typing escapes: forbid `dyn Any`, `transmute`, raw pointer arithmetic outside ffi.
if grep -rnE 'transmute|dyn Any|as \*const|as \*mut' src --include='*.rs' | grep -v 'src/ffi.rs'; then
  echo "type escape found outside src/ffi.rs"; exit 1
fi
