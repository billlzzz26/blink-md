---
name: recipe-code-documentation
description: "Generate and sync code documentation."
metadata:
  version: 0.4.1
  openclaw:
    category: "recipe"
    domain: "development"
    requires:
      bins:
        - blink-md
      skills:
        - md-mcp
        - notion-mcp
        - jules-mcp
---

# Code Documentation

> **PREREQUISITE:** Load skills: `md-mcp`, `notion-mcp`, `jules-mcp`

Generate and sync code documentation.
## Steps

1. Extract code comments
2. Generate markdown docs
3. Create Notion page
4. Use Jules to improve docs

