# blink-md Project Memory

**Last Updated:** 2026-06-30

## Project Overview

**blink-md** (v0.3.1/v0.4.1) is a high-performance document sync and conversion engine in Rust.
- Universal Intermediate Representation (IR) for lossless format conversion
- Platforms: Notion (primary), Markdown/GFM, Lark/Feishu, Google Docs, HTML/PDF/Docx, Sheets/Excel
- Two binaries: `blink-md` (CLI/TUI) and `blink-md-mcp` (feature `mcp`)
- Single crate with workspace.dependencies pattern

## Architecture

```
[ Source Platform ]       [ Universal IR ]       [ Target Platform ]
      (Notion)    <----->  (The Core)    <----->     (Markdown)
      (Lark)               /    |    \               (HTML/PDF)
      (GDocs)             /     |     \              (Docx)
                         v      v      v
                [ Local Files / CLI / TUI ]
                [ MCP Server / AI Agents ]
```

## Key Components

- **src/ir/** - Universal IR types (document, block, inline, style, table, metadata)
- **src/api/** - Platform adapters (Notion, Markdown frontmatter)
- **src/mcp/** - Unified MCP server (feature `mcp`)
- **src/tui/** - Terminal UI with theme system
- **tooling/jules** - Jules/Hermes agent tooling (outside build)

## Current Focus (v0.4.1)

Active work: Markdown + YAML Frontmatter ↔ Notion Database
- Phase A (detection): Complete - `src/api/markdown_frontmatter.rs`
- Phase B (property mapping): In progress
- Phase C (converter): Pending
- Phase D (sync glue): Pending
- Phase E (export): Pending

## Build & CI

- Quality gate: `make ci` (fmt, lint, test, check, package-check)
- Package gate: `python scripts/check-package-hygiene.py`
- Android builds use rustls-based self_update (no OpenSSL)
- Cargo.lock should be committed after dependency changes

## Exclusions (DO NOT COMMIT)

- `.gemini/`, `.qwen/`, `.learnings/`, `.cavekit/`
- `secrets/`, `*.key`, `*.pem`, `*.secret`
- `docs/mcp/conductor/`
- `src/mcp/*/target/`
- Agent-local skills (unless intentionally part of project)

## Work Log

### 2026-06-30
- refactor(.claude): consolidate memory system hooks and scripts
- feat(.claude): add memory system with hooks and scripts
- chore: add project hooks.json for session end memory
- docs: add .claude/MEMORY.md and add-memory skill
- git add commit push: committed Cargo.lock regeneration and skills/ directory (19 files, 6136 insertions)
- Created .claude/MEMORY.md with project summary and add-memory skill
- Configured hooks.json and post-commit for auto-summarizing work