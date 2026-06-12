# blink-md TODO.md — v0.3.0 High-Fidelity Universal IR

## Overview
**Version**: 0.3.0
**Target**: Universal Document IR — platform-agnostic Intermediate Representation for document conversion
**Architecture**: Source Platform → Universal IR (Lossless Store) → Target Platform (Relational DB / UI)

---

## Phase 0: Infrastructure & MCP (v0.3.0) — COMPLETE

### 0.1 Workspace Structure (`src/mcp/`)
- [x] `src/mcp/core/` — mcp-core v0.2.0 (shared pmcp utilities) <!-- id: 0.1.1 -->
  - [x] Re-exports: pmcp::Server, ToolHandler, McpError, McpResult <!-- id: 0.1.1.1 -->
  - [x] SchemaBuilder — tool input schema builder <!-- id: 0.1.1.2 -->
  - [x] run_cli_command() — CLI bridge helper <!-- id: 0.1.1.3 -->
  - [x] init_logging() — tracing setup <!-- id: 0.1.1.4 -->
- [x] `src/mcp/jules/` — jules-mcp-server v0.3.0 (Jules/Hermes bridge) <!-- id: 0.1.2 -->
  - [x] 8 tools: start_new_jules_task, list_jules_sessions, list_jules_repos <!-- id: 0.1.2.1 -->
  - [x] pull_jules_changes, hermes_query, hermes_list_skills <!-- id: 0.1.2.2 -->
  - [x] hermes_cron_status, check_dependencies <!-- id: 0.1.2.3 -->
- [x] `src/mcp/md/` — md-mcp-server v0.2.0 (Markdown operations) <!-- id: 0.1.3 -->
  - [x] parse_markdown — Parse Notion-flavored Markdown to blocks <!-- id: 0.1.3.1 -->
  - [x] to_markdown — Convert block to Notion-flavored Markdown <!-- id: 0.1.3.2 -->
- [x] `src/mcp/mmd/` — mmd-mcp-server v0.1.0 (uses mermaid-rs-renderer from crates.io) <!-- id: 0.1.4 -->
  - [x] 4 integration tests (TDD) <!-- id: 0.1.4.1 -->
  - [x] Tools: render_mermaid_svg, render_mermaid_png, list_diagram_types <!-- id: 0.1.4.2 -->

### 0.2 Shared Dependencies (`workspace.dependencies`)
- [x] serde, serde_json, tokio, anyhow, thiserror, pmcp <!-- id: 0.2.1 -->

### 0.3 Shared Lints (`workspace.lints`)
- [x] rust: unsafe_code = "forbid" <!-- id: 0.3.1 -->
- [x] clippy: all + pedantic = "warn" <!-- id: 0.3.2 -->

### 0.4 TUI Theme System (`src/cli/theme.rs`)
- [x] Theme struct with Notion colors <!-- id: 0.4.1 -->
- [x] 15 JSON themes in `src/cli/themes/` <!-- id: 0.4.2 -->
- [x] SyntaxHighlighter (syntect) — prepared for code blocks <!-- id: 0.4.3 -->

---

## Phase 1: Universal IR Foundation (Week 1-2)

### 1.1 Universal IR Types (`src/ir/`)
- [x] `src/ir/mod.rs` — `UniversalDocument`, `Platform` enum, re-exports <!-- id: 1.1.1 -->
- [x] `src/ir/blocks.rs` — `UniversalBlock` enum (all block variants) <!-- id: 1.1.2 -->
  - [x] Text & Structure: Paragraph, Heading, CodeBlock, Quote <!-- id: 1.1.2.1 -->
  - [x] Lists: BulletList, OrderedList, TaskList, ListItem, TaskItem <!-- id: 1.1.2.2 -->
  - [x] Media: Image, Video, File (with `MediaSource` enum: External/Uploaded/Base64) <!-- id: 1.1.2.3 -->
  - [x] Table: Table, TableRow, TableCell, TableRowType (header/body) <!-- id: 1.1.2.4 -->
  - [x] Platform extensions: Callout, Toggle, Columns, PageBreak, TableOfContents, Embed, Mention <!-- id: 1.1.2.5 -->
  - [x] `Raw { platform: Platform, data: serde_json::Value }` — preserve platform-specific data <!-- id: 1.1.2.6 -->
- [x] `src/ir/inline.rs` — `InlineElement` enum <!-- id: 1.1.3 -->
  - [x] `TextRun { content, style: TextStyle }` <!-- id: 1.1.3.1 -->
  - [x] `Mention { mention_type, target, label }` <!-- id: 1.1.3.2 -->
  - [x] `MentionType`: User, Page, Database, Date, LinkPreview, UserGroup, DateRange <!-- id: 1.1.3.3 -->
  - [x] `TextStyle`: bold, italic, strikethrough, underline, code, color, link <!-- id: 1.1.3.4 -->
- [x] `src/ir/style.rs` — `StyleSheet`, `StyleRef`, `Style` (named styles), `TextStyle`, `BlockStyle`, `CodeStyle`, `TableStyle` <!-- id: 1.1.4 -->
- [x] `src/ir/table.rs` — `TableRow`, `TableCell`, `TableRowType` (header/body), `TableStyle` <!-- id: 1.1.5 -->
- [x] `src/ir/metadata.rs` — `DocumentMetadata` (title, author, created_time, last_edited_time, properties: HashMap<String, PropertyValue>) <!-- id: 1.1.6 -->

### 1.2 Converter Traits (`src/converter/`)
- [x] `src/converter/trait.rs` — `FromPlatform`, `ToPlatform` traits + `ConverterError` <!-- id: 2.1 -->
- [x] `src/converter/mod.rs` — re-exports, registry pattern for converters <!-- id: 2.2 -->
- [x] `src/converter/registry.rs` — `ConverterRegistry` for dynamic platform discovery <!-- id: 2.2 -->

### 1.3 Notion ↔ IR Converter (`src/converter/notion.rs`)
- [x] `NotionFromPlatform` — Notion API models → Universal IR <!-- id: 3.1 -->
  - [x] Page → UniversalDocument (metadata + blocks) <!-- id: 3.1 -->
  - [x] Block → UniversalBlock (all 25+ block types) <!-- id: 3.1 -->
  - [x] RichText → InlineElement (Text, Mention, Equation) <!-- id: 3.1 -->
  - [x] Properties → DocumentMetadata.properties <!-- id: 3.1 -->
  - [x] Handle `in_trash`, `has_children`, parent references <!-- id: 3.1 -->
- [x] `NotionToPlatform` — Universal IR → Notion API models <!-- id: 3.2 -->
  - [x] UniversalDocument → CreatePageRequest / UpdatePageRequest <!-- id: 3.2 -->
  - [x] UniversalBlock → Block (Notion BlockType) <!-- id: 3.2 -->
  - [x] InlineElement → RichText <!-- id: 3.2 -->
  - [x] Preserve Notion-specific blocks (Callout, Toggle, Columns) losslessly <!-- id: 3.2 -->
- [x] Roundtrip tests: Notion → IR → Notion (serialization equality) <!-- id: 3.3 -->

### 1.4 Markdown (CommonMark + GFM) ↔ IR Converter (`src/converter/markdown.rs`)
- [ ] `MarkdownFromPlatform` — Markdown → Universal IR <!-- id: 4.1 -->
  - [ ] Use `pulldown-cmark` with all Options <!-- id: 4.1 -->
  - [ ] Parse: headings, paragraphs, code blocks, blockquotes, lists (bullet/ordered/task), tables <!-- id: 4.1 -->
  - [ ] Parse GFM: task lists, strikethrough, tables, autolinks <!-- id: 4.1 -->
  - [ ] Parse Notion-flavored extensions: `<callout>`, `<details>`, `<columns>`, `<mention-*>` <!-- id: 4.1 -->
- [ ] `MarkdownToPlatform` — Universal IR → Markdown <!-- id: 4.2 -->
  - [ ] Emit CommonMark + GFM <!-- id: 4.2 -->
  - [ ] Emit Notion-flavored tags for platform extensions <!-- id: 4.2 -->
  - [ ] Handle inline styles: bold, italic, strikethrough, code, underline, color, links <!-- id: 4.2 -->
- [ ] Roundtrip tests: Markdown → IR → Markdown (semantic equality) <!-- id: 4.3 -->

### 1.5 Roundtrip Test Infrastructure (`tests/ir_roundtrip.rs`)
- [ ] Test framework: `roundtrip_test<F, T>()` <!-- id: 5.1 -->
- [ ] Notion → IR → Notion test cases (sample pages, databases) <!-- id: 5.1 -->
- [ ] Markdown → IR → Markdown test cases (complex documents) <!-- id: 5.1 -->
- [ ] Cross-platform: Notion → IR → Markdown → IR → Notion (lossless) <!-- id: 5.1 -->

---

## Phase 2: Platform Converters (Week 2-4)

### 2.1 GitHub Markdown (GFM) ↔ IR (`src/converter/github_markdown.rs`)
- [ ] Extend Markdown converter for GFM specifics <!-- id: 6.1 -->
- [ ] Tables with alignment, task lists, footnotes, alerts (blockquote `>[!NOTE]`) <!-- id: 6.1 -->
- [ ] GitHub-specific: issue/PR references (`#123`), user mentions (`@user`), commit SHAs <!-- id: 6.1 -->
- [ ] Roundtrip tests with real GitHub README samples <!-- id: 6.1 -->

### 2.2 HTML ↔ IR (`src/converter/html.rs`)
- [ ] HTML → IR: parse with `tl` or `scraper` <!-- id: 6.2 -->
  - [ ] Semantic tags: h1-h6, p, ul/ol/li, table, blockquote, pre/code, img, a, hr <!-- id: 6.2 -->
  - [ ] Style attributes → TextStyle/BlockStyle <!-- id: 6.2 -->
  - [ ] Custom data attributes for platform extensions <!-- id: 6.2 -->
- [ ] IR → HTML: emit clean semantic HTML5 <!-- id: 6.2 -->
- [ ] CSS class mapping for styles <!-- id: 6.2 -->

### 2.3 Lark/Feishu ↔ IR (`src/converter/lark.rs`)
- [ ] Lark JSON format → IR (blocks: text, heading, list, image, table, divider, callout, etc.) <!-- id: 6.3 -->
- [ ] IR → Lark JSON (for Lark API create/update) <!-- id: 6.3 -->
- [ ] Handle Lark-specific: `@mention`, `#topic`, file tokens, bitable references <!-- id: 6.3 -->

### 2.4 Google Docs ↔ IR (`src/converter/google_docs.rs`)
- [ ] Use `google-docs1` crate <!-- id: 6.4 -->
- [ ] Docs API response → IR (structural elements: paragraph, table, tableOfContents, sectionBreak) <!-- id: 6.4 -->
- [ ] IR → Docs API requests (batchUpdate) <!-- id: 6.4 -->
- [ ] Handle: named styles, inline objects, positioned objects, suggestions <!-- id: 6.4 -->

### 2.5 PDF Export (`src/converter/pdf.rs`)
- [ ] IR → PDF using `genpdf` or `printpdf` <!-- id: 6.5 -->
- [ ] Layout: page size, margins, headers/footers, TOC generation <!-- id: 6.5 -->
- [ ] Fonts: embed Noto Sans / Noto Serif for Unicode <!-- id: 6.5 -->
- [ ] Tables: pagination, header repeat <!-- id: 6.5 -->

### 2.6 Docx ↔ IR (`src/converter/docx.rs`)
- [ ] Use `docx-rs` <!-- id: 6.6 -->
- [ ] Docx → IR: paragraphs, runs, tables, images, styles <!-- id: 6.6 -->
- [ ] IR → Docx: styles mapping, numbering, headers/footers <!-- id: 6.6 -->

### 2.7 Sheets/Excel ↔ IR (`src/converter/sheets.rs`)
- [ ] Use `calamine` (read) + `rust_xlsxwriter` (write) <!-- id: 6.7 -->
- [ ] IR Table blocks ↔ Sheet ranges <!-- id: 6.7 -->
- [ ] Handle: merged cells, formulas, data validation, conditional formatting <!-- id: 6.7 -->

---

## Phase 3: Notion API v0.2.0 Features (Parallel, Week 1-3)

### 3.1 File Uploads (3-step per Notion spec) — `src/api/files.rs`
- [ ] `create_file_upload(filename, content_type)` → `FileUpload { id, upload_url, expiry_time }` <!-- id: 7.1 -->
- [ ] `upload_file_bytes(upload_url, bytes)` — PUT to upload_url <!-- id: 7.1 -->
- [ ] `complete_file_upload(file_upload_id)` → `FileBlockContent` <!-- id: 7.1 -->
- [ ] Convenience: `upload_file_from_path(path)` — all 3 steps <!-- id: 7.1 -->
- [ ] Support: multipart (current), external URL, base64 <!-- id: 7.1 -->

### 3.2 Page Markdown Endpoints — `src/api/pages.rs`
- [ ] `get_page_markdown(page_id)` → `String` (GET /v1/pages/{id}/markdown) <!-- id: 7.2 -->
- [ ] `patch_page_markdown(page_id, markdown)` → `Page` (PATCH /v1/pages/{id}/markdown) <!-- id: 7.2 -->
- [ ] Update `CreatePageRequest` to support `markdown` field <!-- id: 7.2 -->

### 3.3 Data Source CRUD — `src/api/databases.rs`
- [ ] `create_data_source(parent, title, properties)` → `DataSource` (POST /v1/data_sources) <!-- id: 7.3 -->
- [ ] `update_data_source(id, title, properties)` → `DataSource` (PATCH /v1/data_sources/{id}) <!-- id: 7.3 -->
- [ ] `delete_data_source(id)` → `()` (DELETE /v1/data_sources/{id}) <!-- id: 7.3 -->
- [ ] `list_data_sources(parent_id?)` → `Vec<DataSource>` (GET /v1/data_sources with filter) <!-- id: 7.3 -->
- [ ] Distinguish `database_id` (for page parent) vs `data_source_id` (for queries) <!-- id: 7.3 -->

### 3.4 Webhook Events — `src/api/webhooks.rs`
- [ ] `WebhookEvent` enum: page.created, page.updated, page.deleted, block.created, block.updated, block.deleted, database.created, database.updated, comment.created, workspace.updated <!-- id: 7.4 -->
- [ ] `WebhookPayload` with `event_type`, `timestamp`, `data`, `attempt` <!-- id: 7.4 -->
- [ ] Signature verification helper: `verify_webhook_signature(secret, payload, signature)` <!-- id: 7.4 -->
- [ ] Update `Webhook` model with `events: Vec<WebhookEventType>` <!-- id: 7.4 -->

### 3.5 Pagination Helpers — `src/client.rs`
- [ ] `Paginated` trait with `next_page()`, `has_more()`, `start_cursor()` <!-- id: 7.5 -->
- [ ] Implement for: `query_data_source`, `search`, `list_users`, `list_comments`, `get_block_children`, `list_webhooks` <!-- id: 7.5 -->
- [ ] Auto-pagination: `collect_all_pages()` convenience method <!-- id: 7.5 -->

### 3.6 Search Enhancements — `src/api/search.rs`
- [ ] `sort` parameter: `SortDirection` (asc/desc), `SortTimestamp` (last_edited_time, created_time) <!-- id: 7.6 -->
- [ ] `filter` parameter: object type filter (page, database, data_source) <!-- id: 7.6 -->
- [ ] `page_size` and `start_cursor` support <!-- id: 7.6 -->

### 3.7 Block Operations — `src/api/blocks.rs`
- [ ] Verify `update_block` supports all block types <!-- id: 7.7 -->
- [ ] `delete_block` — confirm sets `in_trash=true` <!-- id: 7.7 -->
- [ ] `get_block(block_id)` — single block GET <!-- id: 7.7 -->
- [ ] Block position: `append_block_children` with `after`/`before` position <!-- id: 7.7 -->

---

## Phase 4: MCP Server (Week 3-4)

### 4.1 MCP Server Implementation — `src/cli/mcp.rs`
- [ ] Replace placeholder with full `pmcp` server <!-- id: 8.1 -->
- [ ] Load `NOTION_API_TOKEN` from env at startup <!-- id: 8.1 -->
- [ ] Health check endpoint <!-- id: 8.1 -->

### 4.2 MCP Tools (Notion Capabilities)
- [ ] `notion_search` | Search pages/databases | `client.search()` <!-- id: 8.2 -->
- [ ] `notion_get_page` | Get page by ID | `client.get_page()` <!-- id: 8.2 -->
- [ ] `notion_get_page_markdown` | Get page as markdown | `client.get_page_markdown()` <!-- id: 8.2 -->
- [ ] `notion_create_page` | Create page in database/page | `client.create_page()` <!-- id: 8.2 -->
- [ ] `notion_update_page` | Update page properties | `client.update_page()` <!-- id: 8.2 -->
- [ ] `notion_query_database` | Query data source | `client.query_data_source()` <!-- id: 8.2 -->
- [ ] `notion_list_users` | List workspace users | `client.list_users()` <!-- id: 8.2 -->
- [ ] `notion_get_block_children` | Get block children | `client.get_block_children()` <!-- id: 8.2 -->
- [ ] `notion_append_blocks` | Append blocks to page | `client.append_block_children()` <!-- id: 8.2 -->
- [ ] `notion_upload_file` | Upload file to Notion | `client.upload_file()` <!-- id: 8.2 -->
- [ ] `notion_create_webhook` | Create webhook | `client.create_webhook()` <!-- id: 8.2 -->

### 4.3 MCP Resources (Optional)
- [ ] `notion://page/{id}` — page content as markdown <!-- id: 8.3 -->
- [ ] `notion://database/{id}/schema` — database schema as JSON <!-- id: 8.3 -->
- [ ] `notion://user/me` — current user info <!-- id: 8.3 -->

### 4.4 MCP Testing
- [ ] Unit tests for each tool with wiremock <!-- id: 8.4 -->
- [ ] Integration test: run server, call tools via MCP client <!-- id: 8.4 -->
- [ ] Document MCP usage in README <!-- id: 8.4 -->

---

## Phase 5: Integration Tests with wiremock (Week 3-4)

### 5.1 Enable All Ignored Tests
- [ ] Remove `#[ignore = "requires wiremock"]` from all 9 tests in `tests/integration_tests.rs` <!-- id: 9.1 -->
- [ ] Run `cargo test --test integration_tests` — all pass <!-- id: 9.1 -->

### 5.2 New Test Coverage
- [ ] `pages_tests`: create_page, update_page, move_page, duplicate_page, get_page_property, get_page_markdown, patch_page_markdown <!-- id: 9.2 -->
- [ ] `databases_tests`: create_database, update_database, create_data_source, get_data_source, update_data_source, delete_data_source, query_data_source with pagination <!-- id: 9.2 -->
- [ ] `blocks_tests`: update_block, delete_block, get_block, append with position <!-- id: 9.2 -->
- [ ] `files_tests`: upload_file (multipart), create_file_upload, upload_file_bytes, complete_file_upload <!-- id: 9.2 -->
- [ ] `webhooks_tests`: create_webhook, delete_webhook, webhook payload parsing, signature verification <!-- id: 9.2 -->
- [ ] `search_tests`: search with sort, filter, pagination <!-- id: 9.2 -->
- [ ] `error_tests`: 400, 401, 403, 404, 429 (rate limit), 500 <!-- id: 9.2 -->
- [ ] `converter_tests`: Notion ↔ IR, Markdown ↔ IR, HTML ↔ IR roundtrips <!-- id: 9.2 -->

### 5.3 Test Infrastructure — `tests/common/`
- [ ] `mod.rs` — `mock_notion_server()`, `mock_page_response()`, `mock_database_response()`, `mock_block_children_response()`, `mock_error_response(status, code, message)` <!-- id: 9.3 -->
- [ ] `fixtures/` — JSON fixtures for all API responses <!-- id: 9.3 -->

---

## Phase 6: CLI Integration & Polish (Week 4-5)

### 6.1 Unified Convert Command — `src/cli/convert.rs`
- [ ] `--from <platform>` — source platform (notion, markdown, github, lark, gdoc, html, docx) <!-- id: 10.1 -->
- [ ] `--to <platform>` — target platform <!-- id: 10.1 -->
- [ ] `--input <spec>` — input spec (page_id=xxx, file=path, url=xxx, gdoc_id=xxx) <!-- id: 10.1 -->
- [ ] `--output <spec>` — output spec (file=path, page_id=xxx, database_id=xxx) <!-- id: 10.1 -->
- [ ] `--format <format>` — for markdown: commonmark, gfm, notion <!-- id: 10.1 -->

### 6.2 Sync Command Enhancement — `src/cli/sync_cmd.rs`
- [ ] Use IR for local ↔ Notion sync (markdown files → IR → Notion) <!-- id: 10.2 -->
- [ ] Watch mode: `notify` + debounced conversion <!-- id: 10.2 -->
- [ ] Conflict resolution: local wins, remote wins, merge <!-- id: 10.2 -->
- [ ] Support frontmatter for Notion properties <!-- id: 10.2 -->

### 6.3 TUI Enhancement — `src/cli/tui.rs`
- [ ] Browse pages/databases <!-- id: 10.3 -->
- [ ] Preview page as markdown (using IR → markdown) <!-- id: 10.3 -->
- [ ] Edit page in `$EDITOR` → convert → push to Notion <!-- id: 10.3 -->
- [ ] Search with live results <!-- id: 10.3 -->

### 6.4 Version Bump & Release
- [ ] `Cargo.toml`: `version = "0.2.0"` <!-- id: 10.4 -->
- [ ] `CHANGELOG.md` — document all changes <!-- id: 10.4 -->
- [ ] `README.md` — update with new features, converter usage, MCP setup <!-- id: 10.4 -->
- [ ] Publish to crates.io (if ready) <!-- id: 10.4 -->

---
*Updated: 2026-06-05 | Integrated Full Manifest ID-based Sync System.*
