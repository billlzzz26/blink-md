# blink-md TODO.md — v0.2.0 Universal Document IR

## Overview
**Version**: 0.2.0
**Target**: Universal Document IR — platform-agnostic Intermediate Representation for document conversion
**Architecture**: Source Platform → Universal IR (platform-agnostic IR) → Target Platform (Relational DB / UI)

---

## Phase 1: Universal IR & Database Foundation (Week 1)

### 1.1 Universal IR Types (`src/ir/`)
- [x] `src/ir/mod.rs` — `UniversalDocument`, `Platform` enum, re-exports
- [x] `src/ir/blocks.rs` — `UniversalBlock` enum (all block variants)
- [x] `src/ir/inline.rs` — `InlineElement` enum
- [x] `src/ir/style.rs` — `StyleSheet`, `StyleRef`, `Style`
- [x] `src/ir/table.rs` — `TableRow`, `TableCell`
- [x] `src/ir/metadata.rs` — `DocumentMetadata`

### 1.2 Universal Database Store (`db/` & `src/models/db.rs`)
- [x] `db/schema.sql` — PostgreSQL schema with `documents`, `blocks` (LexoRank), `styles`, `users`
- [x] `src/models/db.rs` — Relational models mapping 1:1 to SQL schema
- [x] `src/models/db.rs` — Implement `from_ir` for lossless DB ingestion
- [x] JSONB & GIN Index implementation for high-fidelity content search

### 1.3 Dead Code Cleanup & Stabilization
- [x] Remove obsolete `universal_parser.rs`, `universal_mapper.rs`, `docx.rs`, `html.rs`, `pdf.rs`
- [x] Clean up `src/api/mod.rs` and `src/models/mod.rs`
- [x] Update `Cargo.toml` dependencies to latest stable versions (reqwest 0.13, tokio 1.52, etc.)
- [x] Verify project compilation with `cargo check`

### 1.4 Critical API Fixes (Pre-v0.2.0) — [URGENT]
- [x] [CRITICAL] Implement `query_database` standard endpoint in `src/api/databases.rs`
- [x] [CRITICAL] Add Pagination support (Auto-iterator) for `search`, `get_block_children`, and `query_database`
- [x] [MAJOR] Implement `Retry-After` header inspection in `NotionClient` for smart backoff
- [x] [MAJOR] Refactor `src/converter/notion.rs` to support recursive child block fetching (Lossless Notion -> IR)
- [x] [MINOR] De-duplicate `CreatePageRequest` and other shared models between API and Converter

---

## Phase 2: UX & TUI Refinement (Week 1-2) — [HIGH PRIORITY]

### 2.1 TUI Theme & Token Integration
- [ ] Create `src/cli/theme.rs` mapping `DESIGN.md` tokens to Ratatui styles
- [ ] [UX-CRITICAL] Replace hardcoded colors in `src/cli/tui.rs` with `Theme` tokens
- [ ] [UX-CRITICAL] Add Footer block to TUI with keyboard hints (`[q]uit [tab]switch [j/k]move`)
- [ ] Add visual hierarchy to Detail view (Keys in Accent Blue, Values in Primary)

### 2.2 TUI System Status & Error Handling
- [ ] [UX-CRITICAL] Implement loading indicators (spinner/text) during async API calls
- [ ] [UX-CRITICAL] Add status/error message area in TUI to show API failures
- [ ] Implement help overlay (`?` key) in TUI

### 2.3 IR-based Page Preview
- [ ] Implement `MarkdownToPlatform` for IR to provide high-fidelity markdown previews in TUI
- [ ] Add "Preview" mode to TUI page detail

---

## Phase 3: Platform Converters (Week 2-3)

### 3.1 Notion ↔ IR Converter (`src/converter/notion.rs`)
- [ ] `NotionFromPlatform` — Notion API models → Universal IR
  - [ ] Page → UniversalDocument (metadata + blocks)
  - [ ] Block → UniversalBlock (all 25+ block types)
  - [ ] RichText → InlineElement (Text, Mention, Equation)
- [ ] `NotionToPlatform` — Universal IR → Notion API models
- [ ] Roundtrip tests: Notion → IR → Notion

### 3.2 Markdown ↔ IR Converter (`src/converter/markdown.rs`)
- [ ] `MarkdownFromPlatform` — Markdown → Universal IR (using `pulldown-cmark` 0.13)
- [ ] `MarkdownToPlatform` — Universal IR → Markdown
- [ ] Parse/Emit Notion-flavored extensions (`<callout>`, `<columns>`)

### 3.3 Roundtrip Test Infrastructure (`tests/ir_roundtrip.rs`)
- [ ] Test framework: `roundtrip_test<F, T>()`
- [ ] Cross-platform tests: Notion → IR → Markdown → IR → Notion (lossless)

---

## Phase 4: Extended Converters (Week 3-4)

### 4.1 GitHub Markdown (GFM) ↔ IR
- [ ] Extend Markdown converter for GFM specifics (Tables, Alerts, Footnotes)
### 4.2 HTML ↔ IR
- [ ] Semantic HTML5 parser/emitter for IR using `scraper` 0.27
### 4.3 Google Docs ↔ IR
- [ ] Structural mapping using `google-docs1` 7.0
### 4.4 Docx/PDF Export
- [ ] IR → Docx (via `docx-rs` 0.4)
- [ ] IR → PDF (via `lopdf` 0.40)

---

## Phase 5: Notion API v0.2.0 Features (Week 4)

- [ ] File Uploads (3-step process) — `src/api/files.rs`
- [ ] Page Markdown Endpoints (GET/PATCH markdown) — `src/api/pages.rs`
- [ ] Data Source CRUD — `src/api/databases.rs`
- [ ] Webhook Event handling & Signature verification — `src/api/webhooks.rs`
- [ ] Auto-pagination helper in `src/client.rs`

---

## Phase 6: MCP & Integration (Week 5)

- [ ] Full `pmcp` 2.9 server implementation in `src/cli/mcp.rs`
- [ ] MCP Tools for all core Notion capabilities
- [ ] Unified `blink-md convert` command using rewritten IR-platform converters
- [ ] Sync command enhancement with IR-based local ↔ Notion sync

---

## Code Quality Gates (Every PR)

- [x] `cargo fmt --check`
- [x] `cargo clippy --all-targets --all-features -D warnings`
- [x] `cargo test --all-targets`
- [x] `oh-my-product verify` pass
- [ ] Roundtrip tests pass (Semantic Equality)

---

*Updated: 2026-06-04 based on Schema, Design, and UX Critique reports.*
