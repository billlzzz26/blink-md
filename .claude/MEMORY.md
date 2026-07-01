# blink-md Project Memory

**Last Updated:** 2026-07-01

## Project Overview

**blink-md** (v0.4.2) is a high-performance document sync and conversion engine in Rust.
- Universal Intermediate Representation (IR) for lossless format conversion
- Platforms: Notion (primary), Markdown/GFM, Lark/Feishu, Google Docs, HTML/PDF/Docx, Sheets/Excel
- Two binaries: `blink-md` (CLI/TUI) and `blink-md-mcp` (feature `mcp`)
- Single crate with workspace.dependencies pattern
- CLI output: table by default, `--format json` for scripts; errors print `error: <msg>: <cause>` with `-v/--verbose` for full chain (`src/cli/output.rs`)
- Adapter architecture overhaul proposed in `docs/ARCHITECTURE.md` (Reader/Writer + Source/Sink split, filters, Capabilities, ChangeSet write path, frontmatter-based remote-ID addressing); migration plan M1-M5, not yet started

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
- **src/api/** - Platform adapters (Notion, Markdown frontmatter, webhooks)
- **src/mcp/** - Unified MCP server (feature `mcp`)
- **src/tui/** - Terminal UI with theme system
- **tooling/jules** - Jules/Hermes agent tooling (outside build)

## Current Focus (v0.4.2)

Markdown + YAML Frontmatter <-> Notion Database: complete (Phases A-E - detection,
property mapping, converter, sync glue, page export). Next up: M1 of the
`docs/ARCHITECTURE.md` migration plan (freeze/version the IR contract), pending
user go-ahead. Deferred Notion work tracked in issues #39 (OAuth), #40 (webhook
worker), #41 (remaining API surface). v0.4.2 tag/crates.io publish also pending
user go-ahead (PR #43 merged to main; tagging triggers an irreversible crates.io
publish).

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

### 2026-07-01

*Details: [.claude/memory/session-2026-07-01.md](.claude/memory/session-2026-07-01.md)*

- Fixed add-memory.sh hook: PROJECT_ROOT path resolution was one level too shallow (wrote to phantom .claude/.claude/memory/) and the sed multi-line Work Log append was malformed (silently never ran); replaced with a portable awk insert
- Released v0.4.2: bumped Cargo.toml, cut CHANGELOG Unreleased into 0.4.2 section, synced README/TODO.md/docs/PLAN.md off stale 0.3.1 references; PR #43 merged to main; v0.4.2 git tag + crates.io publish still pending explicit user go-ahead
- Added paths-ignore to ci.yml/coverage.yml/cross-platform.yml/audit.yml so doc-only changes skip CI (tag pushes, daily audit schedule unaffected)
- Drafted and revised docs/ARCHITECTURE.md: Reader/Writer + Source/Sink + filters + Capabilities + ChangeSet design; platform survey (Notion, Lark/Feishu, GitHub Markdown, Obsidian, AppFlowy, Anytype, Craft) led to dropping Google Docs as write-path template (favor Lark batch_update) and adding frontmatter-based remote-ID addressing; PR #43
- Opened issues #39 (Notion OAuth), #40 (webhook worker), #41 (remaining Notion API surface) for deferred Notion work
- Built CLI UX overhaul: src/cli/output.rs table/JSON renderer, standard error output (error: prefix, exit codes, -v/--verbose), fixed offline commands (convert/diff/upgrade) wrongly requiring NOTION_TOKEN
- Merged PR #37 (webhook signature verification) and PR #38 (get_block, search_all pagination, fixed silent-truncation bug -> PaginationLimitExceeded)

#### feat

1. rewrite add-memory.sh/.py into type-sectioned GFM format with day-linked session logs, add AGENTS.md file-placement/documentation rules (23:38)

### 2026-06-30
- refactor(.claude): consolidate memory system hooks and scripts
- feat(.claude): add memory system with hooks and scripts
- chore: add project hooks.json for session end memory
- docs: add .claude/MEMORY.md and add-memory skill
- git add commit push: committed Cargo.lock regeneration and skills/ directory (19 files, 6136 insertions)
- Created .claude/MEMORY.md with project summary and add-memory skill
- Configured hooks.json and post-commit for auto-summarizing work
