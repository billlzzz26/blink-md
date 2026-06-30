#!/bin/bash
# bootstep-memory.sh - Create MEMORY.md from codebase when missing

set -euo pipefail

cd "${1:-$(pwd)}"
OUT=".claude/MEMORY.md"
[ -f "$OUT" ] && exit 0

cat > "$OUT" << EOF
# Project Memory

**Last Updated:** $(date +%Y-%m-%d)

## Project Overview

$(basename "$PWD")

## Architecture

$(find src -name '*.rs' -type f 2>/dev/null | head -10 | sed 's/^/- /' || echo "- No Rust source found")

## Key Components

- See src/ structure

## Build & CI

- make ci

## Work Log

### $(date +%Y-%m-%d)
- Bootstepped from codebase
EOF

echo "Created $OUT"