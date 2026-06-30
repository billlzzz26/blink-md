---
name: recipe-export-archive
description: "Export all Notion docs for offline archive."
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
        - mmd-mcp
---

# Export Archive

> **PREREQUISITE:** Load skills: `notion-mcp`, `md-mcp`, `mmd-mcp`

Export all Notion docs for offline archive.
## Steps

1. List all pages in Notion workspace
2. Export each page
3. Convert to markdown
4. Generate mermaid for diagrams

