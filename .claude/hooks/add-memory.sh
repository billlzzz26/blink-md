#!/bin/bash
# add-memory.sh - Append work summary without overwriting

set -euo pipefail

SUMMARY="${1:-}"
TIMESTAMP=$(date +%Y-%m-%d)
SESSION_ID=$(date +%H%M%S)

# Find project root (where .claude/ lives)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# Create memory dirs if needed
mkdir -p ".claude/memory"

# Create session template if missing
SESSION_FILE=".claude/memory/session-${SESSION_ID}.md"
[ ! -f "$SESSION_FILE" ] && cat > "$SESSION_FILE" << 'EOF'
# Session Memory

## Tasks Completed

## Decisions Made

## Files Changed

## Next Steps
EOF

# Append to session file
if [ -n "$SUMMARY" ]; then
    printf -- "- %s %s\n" "$SUMMARY" "$(date +%H:%M)" >> "$SESSION_FILE"
    
    # Append to .claude/MEMORY.md Work Log (avoid duplicates)
    MEMORY_FILE=".claude/MEMORY.md"
    if [ -f "$MEMORY_FILE" ]; then
        # Check if already exists
        if ! grep -qxF "- $SUMMARY" "$MEMORY_FILE" 2>/dev/null; then
            if grep -q "### $TIMESTAMP" "$MEMORY_FILE"; then
                sed -i "/^### $TIMESTAMP$/a\\- $SUMMARY" "$MEMORY_FILE"
            else
                sed -i "/^## Work Log/a\\

### $TIMESTAMP
- $SUMMARY" "$MEMORY_FILE"
            fi
        fi
    fi
fi

echo "Memory: $SESSION_FILE"