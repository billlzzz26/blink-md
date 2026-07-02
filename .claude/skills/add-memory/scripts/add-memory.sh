#!/bin/bash
# add-memory.sh - Append a work summary to today's session log and
# .claude/MEMORY.md's Work Log. Thin wrapper; the actual file editing
# (section/heading handling, numbered lists, dedup, templating) lives in
# add-memory.py since that logic is far more reliable in Python than
# sed/awk.
#
# This script and add-memory.py are the portable implementation of this
# skill: copy the whole .claude/skills/add-memory/ folder into another
# project and it works as-is, with no path assumptions beyond its own
# location relative to this file.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "$SCRIPT_DIR/add-memory.py" "${1:-}"
