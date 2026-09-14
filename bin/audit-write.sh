#!/bin/sh
set -eu
path="$1"; reason="${2:-unspecified}"
full="$PWD/$path"
audit="$HOME/team-tmig/.audit"
mkdir -p "$audit/objects" "$audit/log"
sha() { (sha256sum "$1" 2>/dev/null || shasum -a 256 "$1") | awk '{print $1}'; }
if [ -f "$full" ]; then before=$(sha "$full"); else before="none"; fi
tmp="${full}.partial"
cat > "$tmp"
[ -s "$tmp" ] || { echo "refused"; rm -f "$tmp"; exit 1; }
after=$(sha "$tmp")
cp "$tmp" "$audit/objects/$after" 2>/dev/null || true
mv "$tmp" "$full"
ts=$(date -u +%Y-%m-%dT%H:%M:%SZ)
printf '{"ts":"%s","path":"%s","before":"%s","after":"%s","actor":"%s","reason":"%s"}\n' \
    "$ts" "$path" "$before" "$after" "$(whoami)" "$reason" >> "$audit/log/writes.jsonl"
echo "[audit] $path $before -> $after"
