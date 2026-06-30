---
name: recipe-data-migration
description: "Migrate data from one platform to another."
metadata:
  version: 0.4.1
  openclaw:
    category: "recipe"
    domain: "utility"
    requires:
      bins:
        - blink-md
      skills:
        - lark-mcp
        - notion-mcp
        - md-mcp
---

# Data Migration

> **PREREQUISITE:** Load skills: `lark-mcp`, `notion-mcp`, `md-mcp`

Migrate data from one platform to another.
## Steps

1. Export source data from Lark
2. Convert to IR format
3. Validate IR structure
4. Import to target platform

