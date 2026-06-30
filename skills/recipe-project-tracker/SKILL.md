---
name: recipe-project-tracker
description: "Sync project data between Lark and Notion."
metadata:
  version: 0.4.1
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins:
        - blink-md
      skills:
        - lark-mcp
        - notion-mcp
---

# Project Tracking

> **PREREQUISITE:** Load skills: `lark-mcp`, `notion-mcp`

Sync project data between Lark and Notion.
## Steps

1. Read Lark data
2. Compare with Notion
3. Update Notion
4. Schedule daily sync

