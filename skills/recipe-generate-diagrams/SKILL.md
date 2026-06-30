---
name: recipe-generate-diagrams
description: "Read code/docs and generate Mermaid architecture diagrams."
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
        - mmd-mcp
        - jules-mcp
---

# Generate Architecture Diagrams

> **PREREQUISITE:** Load skills: `md-mcp`, `mmd-mcp`, `jules-mcp`

Read code/docs and generate Mermaid architecture diagrams.
## Steps

1. Extract structure from markdown
2. Generate Mermaid diagram
3. Use Jules for AI suggestions

