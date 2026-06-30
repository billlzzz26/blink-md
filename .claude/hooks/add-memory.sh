#!/bin/bash
# add-memory.sh — Append work summary to memory files (no overwrite)
# Usage: Called by Hermes hooks or manually: ./add-memory.sh "summary text"

set -euo pipefail

SUMMARY="${1:-}"
TIMESTAMP=$(date +%Y-%m-%d)
SESSION_ID=$(date +%H%M%S)

# Change to project root (where .claude/ exists)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd 2>/dev/null || pwd)"
cd "$PROJECT_ROOT" 2>/dev/null || true

# Ensure memory directories exist
mkdir -p ".claude/memory"

# Create session-memory template if missing
SESSION_FILE=".claude/memory/session-${SESSION_ID}.md"
if [ ! -f "$SESSION_FILE" ]; then
    cat > "$SESSION_FILE" << 'EOF'
# Session Memory Template

## Tasks Completed
- 

## Decisions Made
- 

## Files Changed
- 

## Next Steps
- 
EOF
fi

# Append summary to session memory
if [ -n "$SUMMARY" ]; then
    # Add to session file
    sed -i "/^## Tasks Completed/a\\- $SUMMARY $(date +%H:%M)" "$SESSION_FILE" 2>/dev/null || true
    
    # Update project MEMORY.md Work Log (if exists)
    if [ -f ".claude/MEMORY.md" ]; then
        if grep -q "### $TIMESTAMP" ".claude/MEMORY.md"; then
            sed -i "/^### $TIMESTAMP$/a\\- $SUMMARY" ".claude/MEMORY.md" 2>/dev/null || true
        else
            sed -i "/^## Work Log/,\$s/.*/&\n\n### $TIMESTAMP\n- $SUMMARY/" ".claude/MEMORY.md" 2>/dev/null || true
        fi
    fi
fi

echo "Memory recorded in .claude/memory/session-${SESSION_ID}.md"