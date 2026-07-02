#!/bin/bash
# Forwarding stub: hooks.json's session_end trigger needs a script it can
# find at a fixed location under .claude/hooks/. The real, portable
# implementation lives in .claude/skills/add-memory/scripts/, so that skill
# folder alone can be copied into other projects.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/../skills/add-memory/scripts/add-memory.sh" "${1:-}"
