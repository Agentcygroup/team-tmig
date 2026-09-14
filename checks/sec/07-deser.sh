#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
# forbid bincode/ron/serde_json deserialize from untrusted paths without a size limit
hits=$(grep -rnE 'from_reader\(|from_slice\(' src --include='*.rs' | grep -v '// bounded' || true)
[ -z "$hits" ] && exit 0
echo "review unbounded deser:"; echo "$hits"; exit 1
