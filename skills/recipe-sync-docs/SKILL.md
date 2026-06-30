---
name: recipe-sync-docs
description: "Two-way sync documentation between Notion, Markdown, and Mermaid."
metadata:
  version: 0.4.1
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins:
        - blink-md
      skills:
        - notion-mcp
        - md-mcp
---

# Sync Documentation

> **PREREQUISITE:** Load skills: `notion-mcp`, `md-mcp`

Two-way sync documentation between Notion, Markdown, and Mermaid.
## Steps

1. Export Notion page: `blink-md notion export --page-id PAGE_ID --format markdown`
2. Compare with local file: `blink-md diff --notion PATH --local ORIGINAL.md`
3. Sync changes: `blink-md sync --source notion --target markdown --page-id PAGE_ID`

