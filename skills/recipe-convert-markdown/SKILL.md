---
name: recipe-convert-markdown
description: "Convert markdown to other formats using Universal IR."
metadata:
  version: 0.4.1
  openclaw:
    category: "recipe"
    domain: "utility"
    requires:
      bins:
        - blink-md
      skills:
        - md-mcp
        - mmd-mcp
        - notion-mcp
---

# Convert Markdown

> **PREREQUISITE:** Load skills: `md-mcp`, `mmd-mcp`, `notion-mcp`

Convert markdown to other formats using Universal IR.
## Steps

1. Parse markdown: `blink-md md parse README.md`
2. Convert to IR: `blink-md ir convert --input README.md --format ir`
3. Convert to Notion: `blink-md convert --from ir --to notion --output page.json`
4. Convert to Mermaid: `blink-md mmd generate --from ir --output diagram.mmd`

