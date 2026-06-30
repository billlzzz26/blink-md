---
name: recipe-blog-workflow
description: "Convert Notion posts to markdown for blog publishing."
metadata:
  version: 0.4.1
  openclaw:
    category: "recipe"
    domain: "content"
    requires:
      bins:
        - blink-md
      skills:
        - notion-mcp
        - md-mcp
---

# Blog Publishing

> **PREREQUISITE:** Load skills: `notion-mcp`, `md-mcp`

Convert Notion posts to markdown for blog publishing.
## Steps

1. Export Notion post
2. Clean up for blog
3. Optimize frontmatter
4. Publish

