---
name: add-memory
description: "Append work summary to .claude/MEMORY.md without overwriting. MEMORY.md for project facts, .claude/memory/ for session logs."
---

# Add Memory

Append work summary to `.claude/MEMORY.md` without overwriting.

## Memory Types

- **MEMORY.md** - Project facts (stable, persistent): architecture, components, work log
- **memory/** - Session logs (per-session, may change)

## Usage

`.claude/hooks/add-memory.sh "work summary"`

## Work Log Format

```markdown
### YYYY-MM-DD
- <work summary>
```

## Rules

- Do NOT overwrite existing content
- Merge with existing structure
- Keep entries factual and concise