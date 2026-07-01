# blink-md TODO.md — v0.3.1

## Overview
- Version: 0.4.1
- Current branch: `feature/google-workspace-oauth`
- Quality gate: `make ci`
- Package gate: `python scripts/check-package-hygiene.py`
- CI: `.github/workflows/rust-ci.yml` and `.github/workflows/cross-platform.yml`

## Definition of done for every change
- Code, tests, docs, CI/package gates, and release notes are updated together.
- New behavior has tests that would fail without the change.
- `make ci` passes locally before merge.
- `cargo package --list --allow-dirty` does not include local agent data, secrets, or internal conductor docs.
- README, TODO.md, CHANGELOG.md, and docs/PLAN.md stay version/status-synced.

---

## Completed — v0.4.1
### MCP server (unified)
- [x] Single `blink-md-mcp` binary (feature `mcp`) bundling all tools.
- [x] In-crate `src/mcp/core.rs` shared pmcp utilities (was `mcp-core`).
- [x] Markdown, Notion/IR, Lark Sheets, and Mermaid tool groups.
- [x] Live Notion tools (search/get_page/create_page/get_block_children/trash) shared with `blink-md mcp-serve`.
- [x] Jules/Hermes bridge relocated to `tooling/jules` (outside the build).

## Completed — v0.3.1
### Release and platform
- [x] Cross-platform release workflow for Linux, macOS Intel, macOS ARM, and Windows.
- [x] Thai TUI hardening for grapheme clusters and visual width.
- [x] Global install scripts and CLI upgrade command.
- [x] Enhanced CLI help and usage examples.
- [x] Workspace package versions synchronized to 0.3.1.

### Universal IR and converters
- [x] Universal IR document/block/inline/style/table/metadata types.
- [x] Notion ↔ IR conversion for pages, blocks, rich text, mentions, properties, and platform extensions.
- [x] Markdown/GFM ↔ IR conversion with roundtrip tests.
- [x] Lark/Feishu Sheets and CSV adapters through Universal IR.

### TUI
- [x] Theme system with JSON themes and syntect highlighting.
- [x] Theme token usage in TUI rendering.
- [x] Loading/error status text in footer.
- [x] Keyboard help popup.

### CI and package hygiene
- [x] Android cross build uses rustls-based self_update path instead of native-tls/OpenSSL.
- [x] MSRV step fails loudly instead of hiding failures.
- [x] CI includes package hygiene check.
- [x] Release crates publish step fails loudly.
- [x] `.qwen/`, `.gemini/`, `.learnings/`, and conductor docs are excluded from package.

---

## Active — next work
### 0. Markdown + YAML Frontmatter ↔ Notion Database (merged)
- [x] **Phases A–E complete**: detection, property mapping, converter, sync glue, and page export all landed. See merged PRs #27, #28, #30.

### 1. Google Workspace OAuth + API Adapter
- [ ] Create `src/oauth.rs` - token provider trait + caching (adapted from google-workspace-cli)
- [ ] Create `src/services.rs` - service registry mapping aliases to API names/versions
- [ ] Add `google` feature to Cargo.toml
- [ ] Create `src/api/google/mod.rs` - common Google API utilities
- [ ] Create `src/api/google/docs.rs` - Google Docs read/write
- [ ] Create `src/api/google/sheets.rs` - Spreadsheet API
- [ ] Create `src/api/google/keep.rs` - Google Keep notes
- [ ] Create `src/api/google/chat.rs` - Chat spaces/messages
- [ ] Create `src/api/google/calendar.rs` - Calendar events
- [ ] Create `src/api/google/tasks.rs` - Task lists
- [ ] Extend IR with Google property types for lossless conversion
- [ ] Build GoogleDocConverter for Docs ↔ Universal IR

### 2. Platform adapters behind Universal IR
- [x] **GFM tables**: `MarkdownConverter` round-trips pipe tables (parse → IR `Table`, render IR → aligned pipe table). `block_ir_to_notion` writes Notion `Table`/`TableRow` blocks (so `sync` pushes tables) and `NotionFromPlatform` regroups flattened API rows back into one IR table (so `export-page` renders a single table). Implemented in [`src/api/markdown.rs`](src/api/markdown.rs), [`src/converter/markdown.rs`](src/converter/markdown.rs), and [`src/converter/notion.rs`](src/converter/notion.rs).
- [ ] GitHub Markdown/GFM extensions: footnotes, alerts, issue/PR refs, mentions, commit refs. _(Cell alignment is lost on the md→Notion→IR path since the Notion table model has no per-column alignment.)_
- [ ] HTML adapter: semantic tags, styles, images, links, and platform extensions.
- [ ] Full Lark/Feishu API adapter: 48 block types, mentions, topics, files, Bitable references.
- [ ] Google Docs adapter: paragraphs, tables, TOC, section breaks, inline objects, suggestions.
- [ ] PDF export: layout, fonts, pagination, headers/footers, TOC.
- [ ] Docx adapter: paragraphs, runs, tables, images, styles, numbering, headers/footers.
- [ ] Sheets/Excel adapter: merged cells, formulas, validation, conditional formatting.

### 3. Notion API surface
- [ ] Page markdown endpoints: GET/PATCH markdown.
- [ ] Data source CRUD: create, update, delete, list, query with pagination.
- [ ] Webhooks: event types, payload parsing, signature verification.
- [ ] Search enhancements: sort, filter, page_size, start_cursor.
- [ ] Block operations: update all block types, delete, get, append with position.
- [ ] File upload polish: multipart, external URL, base64, retry/error handling.

### 4. TUI preview/edit workflows
- [ ] Preview page as Markdown through IR.
- [ ] Edit page in `$EDITOR`, convert back, and push to Notion.
- [ ] Conflict resolution: local wins, remote wins, merge.
- [ ] Live search results and better status/help surfaces.

### 5. Integration tests
- [ ] Enable or intentionally document ignored wiremock tests.
- [ ] Add API tests for pages, databases, blocks, files, webhooks, search, and errors.
- [ ] Add converter roundtrips for Notion, Markdown, HTML, Google Docs, Lark, Docx, PDF, and Sheets.

### 6. Documentation
- [ ] Keep README user-facing and current after each release.
- [ ] Keep CHANGELOG.md updated with Unreleased entries before merge.
- [ ] Keep docs/PLAN.md aligned with TODO.md.
- [ ] Archive or remove docs that duplicate TODO.md or describe obsolete work.

---

## Release checklist
- [ ] Update Cargo.toml version only when the user explicitly approves a version bump.
- [ ] Add Unreleased entries to CHANGELOG.md before release.
- [ ] Update README roadmap and feature list.
- [ ] Update TODO.md and docs/PLAN.md.
- [ ] Run `make ci`.
- [ ] Run `python scripts/check-package-hygiene.py`.
- [ ] Verify GitHub Actions CI and Cross-Platform Build are green.
- [ ] Verify release artifacts are produced for all target platforms.

*Updated: 2026-06-30 | Frontmatter Phases A–E complete (sync glue + page export landed).*
