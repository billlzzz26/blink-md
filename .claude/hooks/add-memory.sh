#!/bin/bash
# add-memory.sh - Append a work summary to today's session log and
# .claude/MEMORY.md's Work Log. Thin wrapper; the actual file editing
# (section/heading handling, numbered lists, dedup) lives in add-memory.py
# since that logic is far more reliable in Python than sed/awk.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "$SCRIPT_DIR/add-memory.py" "${1:-}"
