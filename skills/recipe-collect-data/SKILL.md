---
name: recipe-collect-data
description: "Collect data from Lark spreadsheet and convert to Notion database."
metadata:
  version: 0.4.1
  openclaw:
    category: "recipe"
    domain: "research"
    requires:
      bins:
        - blink-md
      skills:
        - lark-mcp
        - notion-mcp
---

# Collect Data

> **PREREQUISITE:** Load skills: `lark-mcp`, `notion-mcp`

Collect data from Lark spreadsheet and convert to Notion database.
## Steps

1. Export Lark sheet: `blink-md lark export --sheet-id SHEET_ID --format csv`
2. Convert to UniversalBlock: `blink-md ir import --source csv --target ir`
3. Create Notion database: `blink-md notion create-database --from ir --name DATA`
4. Sync regularly via crontab

