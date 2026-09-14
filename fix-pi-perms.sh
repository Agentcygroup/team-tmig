#!/usr/bin/env bash
# fix-pi-perms.sh — fix EACCES on ~/.pi/agent/sessions/*.jsonl
set -euo pipefail

PI_DIR="${HOME}/.pi"
TARGET_FILE="/Users/metadusa/.pi/agent/sessions/--Users-metadusa-.pi-agent-skills--/2026-09-12T06-28-21-930Z_01a0944d-7f2a-72c8-aede-13d115f228e9.jsonl"
CURRENT_USER="$(whoami)"
CURRENT_GROUP="$(id -gn)"

echo "==> User:  ${CURRENT_USER}:${CURRENT_GROUP}"
echo "==> Root:  ${PI_DIR}"
echo

if [[ ! -d "${PI_DIR}" ]]; then
  echo "!! ${PI_DIR} does not exist — nothing to fix."
  exit 0
fi

echo "==> Before:"
ls -leO@ "${PI_DIR}" 2>/dev/null || true
ls -leO@"${PI_DIR}"/agent 2>/dev/null || true
ls -leO@"${PI_DIR}"/agent/sessions 2>/dev/null || true
[[ -e "${TARGET_FILE}" ]] && ls -leO@"${TARGET_FILE}" || echo "   (target file not present)"
echo

SUDO=""
if [[ ! -w "${PI_DIR}" || "$(stat -f '%Su' "${PI_DIR}")" != "${CURRENT_USER}" ]]; then
  echo "==> Need sudo to fix ownership (root-owned files detected)."
  SUDO="sudo"
fi

echo "==> Clearing immutable/append-only flags (if any)..."
${SUDO} chflags -R nouchg,noschg,nosappnd "${PI_DIR}" 2>/dev/null || true

echo "==> Fixing ownership -> ${CURRENT_USER}:${CURRENT_GROUP}..."
${SUDO} chown -R "${CURRENT_USER}:${CURRENT_GROUP}" "${PI_DIR}"

echo "==> Fixing permissions (u+rwX, dirs 755, files 644)..."
${SUDO} find "${PI_DIR}" -type d -exec chmod u+rwx,go+rx {} +
${SUDO} find "${PI_DIR}" -type f -exec chmod u+rw,go+r {} +

echo "==> Removing ACLs that may block access..."
${SUDO} chmod -R -N "${PI_DIR}" 2>/dev/null || true

echo "==> Removing the offending session file (agent will recreate it)..."
${SUDO} rm -f "${TARGET_FILE}" 2>/dev/null || true

echo
echo "==> After:"
ls -leO@ "${PI_DIR}"
ls -leO@"${PI_DIR}"/agent
ls -leO@"${PI_DIR}"/agent/sessions
echo
echo "==> Done. Re-run your agent now."
