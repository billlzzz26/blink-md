---
name: add-memory
description: "Append work summary to MEMORY.md without overwriting. Distinction: MEMORY.md for project facts, memory/ for session logs."
metadata:
  version: 1.0.0
---

# Add Memory

Append work summary to `.claude/skills/add-memory/MEMORY.md` without overwriting.

## Memory Types

- **MEMORY.md** - Project facts (stable, persistent): architecture, components, work log
- **memory/<session-memory>.md** - Session logs (per-session, may change): meeting notes, temporary findings

## Usage

1. Read current MEMORY.md to understand structure
2. Summarize work in 1-2 concise sentences
3. Append dated entry under `## Work Log` section
4. Do NOT overwrite existing content

## Work Log Entry Format

```markdown
### YYYY-MM-DD
- <work summary>
```

## Rules

- Merge new info with existing structure
- Keep entries factual and concise
- Preserve historical continuity
- One entry per work session/task