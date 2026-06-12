# notion-rs TODO.md — v0.2.0 Universal Document IR

## Overview
**Version**: 0.2.0
**Target**: Universal Document IR — platform-agnostic Intermediate Representation for document conversion
**Platforms**: Notion, GitHub Markdown, Lark/Feishu, Google Docs, PDF, Sheets, HTML, Docx, Markdown files
**Architecture**: Source Platform → Universal IR (platform-agnostic IR) → Target Platform

---

## Phase 1: Universal IR Foundation (Week 1-2)

### 1.1 Universal IR Types (`src/ir/`)
- [ ] `src/ir/mod.rs` — `UniversalDocument`, `Platform` enum, re-exports
- [ ] `src/ir/blocks.rs` — `UniversalBlock` enum (all block variants)
  - [ ] Text & Structure: Paragraph, Heading, CodeBlock, Quote
  - [ ] Lists: BulletList, OrderedList, TaskList, ListItem, TaskItem
  - [ ] Media: Image, Video, File (with `MediaSource` enum: External/Uploaded/Base64)
  - [ ] Table: Table, TableRow, TableCell, TableRowType (header/body)
  - [ ] Platform extensions: Callout, Toggle, Columns, PageBreak, TableOfContents, Embed, Mention
  - [ ] `Raw { platform: Platform, data: serde_json::Value }` — preserve platform-specific data
- [ ] `src/ir/inline.rs` — `InlineElement` enum
  - [ ] `TextRun { content, style: TextStyle }`
  - [ ] `Mention { mention_type, target, label }`
  - [ ] `MentionType`: User, Page, Database, Date, LinkPreview, UserGroup, DateRange
  - [ ] `TextStyle`: bold, italic, strikethrough, underline, code, color, link
- [ ] `src/ir/style.rs` — `StyleSheet`, `StyleRef`, `Style` (named styles), `TextStyle`, `BlockStyle`, `CodeStyle`, `TableStyle`
- [ ] `src/ir/table.rs` — `TableRow`, `TableCell`, `TableRowType` (header/body), `TableStyle`
- [ ] `src/ir/metadata.rs` — `DocumentMetadata` (title, author, created_time, last_edited_time, properties: HashMap<String, PropertyValue>)

### 1.2 Converter Traits (`src/converter/`)
- [ ] `src/converter/trait.rs` — `FromPlatform`, `ToPlatform` traits + `ConverterError`
- [ ] `src/converter/mod.rs` — re-exports, registry pattern for converters
- [ ] `src/converter/registry.rs` — `ConverterRegistry` for dynamic platform discovery

### 1.3 Notion ↔ IR Converter (`src/converter/notion.rs`)
- [ ] `NotionFromPlatform` — Notion API models → Universal IR
  - [ ] Page → UniversalDocument (metadata + blocks)
  - [ ] Block → UniversalBlock (all 25+ block types)
  - [ ] RichText → InlineElement (Text, Mention, Equation)
  - [ ] Properties → DocumentMetadata.properties
  - [ ] Handle `in_trash`, `has_children`, parent references
- [ ] `NotionToPlatform` — Universal IR → Notion API models
  - [ ] UniversalDocument → CreatePageRequest / UpdatePageRequest
  - [ ] UniversalBlock → Block (Notion BlockType)
  - [ ] InlineElement → RichText
  - [ ] Preserve Notion-specific blocks (Callout, Toggle, Columns) losslessly
- [ ] Roundtrip tests: Notion → IR → Notion (serialization equality)

### 1.4 Markdown (CommonMark + GFM) ↔ IR Converter (`src/converter/markdown.rs`)
- [ ] `MarkdownFromPlatform` — Markdown → Universal IR
  - [ ] Use `pulldown-cmark` with all Options
  - [ ] Parse: headings, paragraphs, code blocks, blockquotes, lists (bullet/ordered/task), tables
  - [ ] Parse GFM: task lists, strikethrough, tables, autolinks
  - [ ] Parse Notion-flavored extensions: `<callout>`, `<details>`, `<columns>`, `<mention-*>`
- [ ] `MarkdownToPlatform` — Universal IR → Markdown
  - [ ] Emit CommonMark + GFM
  - [ ] Emit Notion-flavored tags for platform extensions
  - [ ] Handle inline styles: bold, italic, strikethrough, code, underline, color, links
- [ ] Roundtrip tests: Markdown → IR → Markdown (semantic equality)

### 1.5 Roundtrip Test Infrastructure (`tests/ir_roundtrip.rs`)
- [ ] Test framework: `roundtrip_test<F, T>()`
- [ ] Notion → IR → Notion test cases (sample pages, databases)
- [ ] Markdown → IR → Markdown test cases (complex documents)
- [ ] Cross-platform: Notion → IR → Markdown → IR → Notion (lossless)

---

## Phase 2: Platform Converters (Week 2-4)

### 2.1 GitHub Markdown (GFM) ↔ IR (`src/converter/github_markdown.rs`)
- [ ] Extend Markdown converter for GFM specifics
- [ ] Tables with alignment, task lists, footnotes, alerts (blockquote `>[!NOTE]`)
- [ ] GitHub-specific: issue/PR references (`#123`), user mentions (`@user`), commit SHAs
- [ ] Roundtrip tests with real GitHub README samples

### 2.2 HTML ↔ IR (`src/converter/html.rs`)
- [ ] HTML → IR: parse with `tl` or `scraper`
  - [ ] Semantic tags: h1-h6, p, ul/ol/li, table, blockquote, pre/code, img, a, hr
  - [ ] Style attributes → TextStyle/BlockStyle
  - [ ] Custom data attributes for platform extensions
- [ ] IR → HTML: emit clean semantic HTML5
- [ ] CSS class mapping for styles

### 2.3 Lark/Feishu ↔ IR (`src/converter/lark.rs`)
- [ ] Lark JSON format → IR (blocks: text, heading, list, image, table, divider, callout, etc.)
- [ ] IR → Lark JSON (for Lark API create/update)
- [ ] Handle Lark-specific: `@mention`, `#topic`, file tokens, bitable references

### 2.4 Google Docs ↔ IR (`src/converter/google_docs.rs`)
- [ ] Use `google-docs1` crate
- [ ] Docs API response → IR (structural elements: paragraph, table, tableOfContents, sectionBreak)
- [ ] IR → Docs API requests (batchUpdate)
- [ ] Handle: named styles, inline objects, positioned objects, suggestions

### 2.5 PDF Export (`src/converter/pdf.rs`)
- [ ] IR → PDF using `genpdf` or `printpdf`
- [ ] Layout: page size, margins, headers/footers, TOC generation
- [ ] Fonts: embed Noto Sans / Noto Serif for Unicode
- [ ] Tables: pagination, header repeat

### 2.6 Docx ↔ IR (`src/converter/docx.rs`)
- [ ] Use `docx-rs`
- [ ] Docx → IR: paragraphs, runs, tables, images, styles
- [ ] IR → Docx: styles mapping, numbering, headers/footers

### 2.7 Sheets/Excel ↔ IR (`src/converter/sheets.rs`)
- [ ] Use `calamine` (read) + `rust_xlsxwriter` (write)
- [ ] IR Table blocks ↔ Sheet ranges
- [ ] Handle: merged cells, formulas, data validation, conditional formatting

---

## Phase 3: Notion API v0.2.0 Features (Parallel, Week 1-3)

### 3.1 File Uploads (3-step per Notion spec) — `src/api/files.rs`
- [ ] `create_file_upload(filename, content_type)` → `FileUpload { id, upload_url, expiry_time }`
- [ ] `upload_file_bytes(upload_url, bytes)` — PUT to upload_url
- [ ] `complete_file_upload(file_upload_id)` → `FileBlockContent`
- [ ] Convenience: `upload_file_from_path(path)` — all 3 steps
- [ ] Support: multipart (current), external URL, base64

### 3.2 Page Markdown Endpoints — `src/api/pages.rs`
- [ ] `get_page_markdown(page_id)` → `String` (GET /v1/pages/{id}/markdown)
- [ ] `patch_page_markdown(page_id, markdown)` → `Page` (PATCH /v1/pages/{id}/markdown)
- [ ] Update `CreatePageRequest` to support `markdown` field

### 3.3 Data Source CRUD — `src/api/databases.rs`
- [ ] `create_data_source(parent, title, properties)` → `DataSource` (POST /v1/data_sources)
- [ ] `update_data_source(id, title, properties)` → `DataSource` (PATCH /v1/data_sources/{id})
- [ ] `delete_data_source(id)` → `()` (DELETE /v1/data_sources/{id})
- [ ] `list_data_sources(parent_id?)` → `Vec<DataSource>` (GET /v1/data_sources with filter)
- [ ] Distinguish `database_id` (for page parent) vs `data_source_id` (for queries)

### 3.4 Webhook Events — `src/api/webhooks.rs`
- [ ] `WebhookEvent` enum: page.created, page.updated, page.deleted, block.created, block.updated, block.deleted, database.created, database.updated, comment.created, workspace.updated
- [ ] `WebhookPayload` with `event_type`, `timestamp`, `data`, `attempt`
- [ ] Signature verification helper: `verify_webhook_signature(secret, payload, signature)`
- [ ] Update `Webhook` model with `events: Vec<WebhookEventType>`

### 3.5 Pagination Helpers — `src/client.rs`
- [ ] `Paginated` trait with `next_page()`, `has_more()`, `start_cursor()`
- [ ] Implement for: `query_data_source`, `search`, `list_users`, `list_comments`, `get_block_children`, `list_webhooks`
- [ ] Auto-pagination: `collect_all_pages()` convenience method

### 3.6 Search Enhancements — `src/api/search.rs`
- [ ] `sort` parameter: `SortDirection` (asc/desc), `SortTimestamp` (last_edited_time, created_time)
- [ ] `filter` parameter: object type filter (page, database, data_source)
- [ ] `page_size` and `start_cursor` support

### 3.7 Block Operations — `src/api/blocks.rs`
- [ ] Verify `update_block` supports all block types
- [ ] `delete_block` — confirm sets `in_trash=true`
- [ ] `get_block(block_id)` — single block GET
- [ ] Block position: `append_block_children` with `after`/`before` position

---

## Phase 4: MCP Server (Week 3-4)

### 4.1 MCP Server Implementation — `src/cli/mcp.rs`
- [ ] Replace placeholder with full `pmcp` server
- [ ] Load `NOTION_API_TOKEN` from env at startup
- [ ] Health check endpoint

### 4.2 MCP Tools (Notion Capabilities)
| Tool | Description | Implementation |
|------|-------------|----------------|
| [ ] `notion_search` | Search pages/databases | `client.search()` |
| [ ] `notion_get_page` | Get page by ID | `client.get_page()` |
| [ ] `notion_get_page_markdown` | Get page as markdown | `client.get_page_markdown()` |
| [ ] `notion_create_page` | Create page in database/page | `client.create_page()` |
| [ ] `notion_update_page` | Update page properties | `client.update_page()` |
| [ ] `notion_query_database` | Query data source | `client.query_data_source()` |
| [ ] `notion_list_users` | List workspace users | `client.list_users()` |
| [ ] `notion_get_block_children` | Get block children | `client.get_block_children()` |
| [ ] `notion_append_blocks` | Append blocks to page | `client.append_block_children()` |
| [ ] `notion_upload_file` | Upload file to Notion | `client.upload_file()` |
| [ ] `notion_create_webhook` | Create webhook | `client.create_webhook()` |

### 4.3 MCP Resources (Optional)
- [ ] `notion://page/{id}` — page content as markdown
- [ ] `notion://database/{id}/schema` — database schema as JSON
- [ ] `notion://user/me` — current user info

### 4.4 MCP Testing
- [ ] Unit tests for each tool with wiremock
- [ ] Integration test: run server, call tools via MCP client
- [ ] Document MCP usage in README

---

## Phase 5: Integration Tests with wiremock (Week 3-4)

### 5.1 Enable All Ignored Tests
- [ ] Remove `#[ignore = "requires wiremock"]` from all 9 tests in `tests/integration_tests.rs`
- [ ] Run `cargo test --test integration_tests` — all pass

### 5.2 New Test Coverage
- [ ] `pages_tests`: create_page, update_page, move_page, duplicate_page, get_page_property, get_page_markdown, patch_page_markdown
- [ ] `databases_tests`: create_database, update_database, create_data_source, get_data_source, update_data_source, delete_data_source, query_data_source with pagination
- [ ] `blocks_tests`: update_block, delete_block, get_block, append with position
- [ ] `files_tests`: upload_file (multipart), create_file_upload, upload_file_bytes, complete_file_upload
- [ ] `webhooks_tests`: create_webhook, delete_webhook, webhook payload parsing, signature verification
- [ ] `search_tests`: search with sort, filter, pagination
- [ ] `error_tests`: 400, 401, 403, 404, 429 (rate limit), 500
- [ ] `converter_tests`: Notion ↔ IR, Markdown ↔ IR, HTML ↔ IR roundtrips

### 5.3 Test Infrastructure — `tests/common/`
- [ ] `mod.rs` — `mock_notion_server()`, `mock_page_response()`, `mock_database_response()`, `mock_block_children_response()`, `mock_error_response(status, code, message)`
- [ ] `fixtures/` — JSON fixtures for all API responses

---

## Phase 6: CLI Integration & Polish (Week 4-5)

### 6.1 Unified Convert Command — `src/cli/convert.rs`
- [ ] `--from <platform>` — source platform (notion, markdown, github, lark, gdoc, html, docx)
- [ ] `--to <platform>` — target platform
- [ ] `--input <spec>` — input spec (page_id=xxx, file=path, url=xxx, gdoc_id=xxx)
- [ ] `--output <spec>` — output spec (file=path, page_id=xxx, database_id=xxx)
- [ ] `--format <format>` — for markdown: commonmark, gfm, notion
- [ ] Examples:
  ```bash
  notion-rs convert --from notion --to markdown --input page_id=abc --output file=page.md
  notion-rs convert --from markdown --to notion --input file=doc.md --output database_id=db1
  notion-rs convert --from gdoc --to markdown --input gdoc_id=xxx --output file=out.md
  notion-rs convert --from html --to docx --input file=page.html --output file=page.docx
  ```

### 6.2 Sync Command Enhancement — `src/cli/sync_cmd.rs`
- [ ] Use IR for local ↔ Notion sync (markdown files → IR → Notion)
- [ ] Watch mode: `notify` + debounced conversion
- [ ] Conflict resolution: local wins, remote wins, merge
- [ ] Support frontmatter for Notion properties

### 6.3 TUI Enhancement — `src/cli/tui.rs`
- [ ] Browse pages/databases
- [ ] Preview page as markdown (using IR → markdown)
- [ ] Edit page in `$EDITOR` → convert → push to Notion
- [ ] Search with live results

### 6.4 Version Bump & Release
- [ ] `Cargo.toml`: `version = "0.2.0"`
- [ ] `CHANGELOG.md` — document all changes
- [ ] `README.md` — update with new features, converter usage, MCP setup
- [ ] Publish to crates.io (if ready)

---

## Dead Code Removal / Refactoring

| File/Module | Action | Replacement |
|-------------|--------|-------------|
| `src/api/universal_parser.rs` | **Remove** | `src/converter/markdown.rs` + `src/converter/notion.rs` |
| `src/models/universal.rs` | **Remove** | `src/ir/` |
| `src/models/universal_mapper.rs` | **Remove** | Converter traits |
| `src/api/docx.rs` | **Remove** | `src/converter/docx.rs` |
| `src/api/html.rs` | **Remove** | `src/converter/html.rs` |
| `src/api/pdf.rs` | **Remove** | `src/converter/pdf.rs` |
| `src/sync/builder.rs` | **Audit** | May become `src/converter/notion_sync.rs` |
| `src/sync/json_schema.rs` | **Audit** | May move to `src/ir/schema.rs` |
| `src/sync/schema.rs` | **Audit** | May move to `src/ir/schema.rs` |
| `src/cli/mcp.rs` | **Rewrite** | Full MCP implementation |
| `src/models/view.rs` | **Keep** | Used in CLI |
| `src/models/datasource.rs` | **Keep** | Used in databases.rs |

---

## Code Quality Gates (Every PR)

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features -D warnings`
- [ ] `cargo test --all-targets`
- [ ] `cargo test --test integration_tests` (with wiremock)
- [ ] `cargo doc --no-deps` — no warnings
- [ ] Roundtrip tests pass: Notion ↔ IR ↔ Notion, Markdown ↔ IR ↔ Markdown

---

## Timeline Summary

| Week | Primary Focus |
|------|---------------|
| **1** | Universal IR types + Notion ↔ IR converter |
| **2** | Markdown ↔ IR + Roundtrip tests + Notion API features (files, page markdown) |
| **3** | GitHub Markdown + HTML converters + Data sources + Webhooks + Pagination + wiremock tests |
| **4** | MCP server + Lark/Google Docs skeleton + CLI convert command |
| **5** | Docx/PDF/Sheets converters + Polish + Release v0.2.0 |

---

## Quick Wins (Can Start Immediately)

1. [ ] Create `src/ir/` module structure with empty files
2. [ ] Define `UniversalBlock`, `InlineElement`, `Platform` enums
3. [ ] Write Notion → IR converter for basic blocks (paragraph, heading, code, list)
4. [ ] Enable wiremock integration tests (remove ignores)
5. [ ] Version bump to 0.2.0 in Cargo.toml
6. [ ] Run dead code scan: `cargo clippy -W clippy::unused`

---

## Notes

- **Breaking changes allowed in 0.2.0** — IR replaces universal_parser/universal_mapper
- **Notion API version**: 2026-03-11 (current), plan for 2025-09-03 compatibility
- **Platform priority**: Notion ↔ Markdown (core), GitHub/HTML (P1), Lark/Google Docs (P2), PDF/Docx/Sheets (P3)
- **MCP**: Requires `pmcp` crate, Notion token at runtime
- **Testing**: wiremock for unit, real API for smoke tests (manual)

---

*Generated from revised v0.2.0 plan. Update as tasks progress.*