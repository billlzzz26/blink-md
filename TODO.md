# blink-md TODO.md — v0.4.2

## Overview
- Version: 0.4.2
- Current branch: `main`
- Quality gate: `make ci`
- Package gate: `python scripts/check-package-hygiene.py`
- CI: `.github/workflows/ci.yml`, `.github/workflows/coverage.yml`, and `.github/workflows/cross-platform.yml`

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
### 0. Markdown + YAML Frontmatter ↔ Notion Database (TDD, complete — Phases A–E)
- [x] **Phase A — detection**: `detect_frontmatter()` extracts a `---`-delimited YAML block from the start of a Markdown file, leaving the rest as body. 15 unit tests cover: missing block, unterminated block, multi-line YAML, empty YAML, CRLF, body containing `---`, colons in values, and edge cases like empty input. Lives in [`src/api/markdown_frontmatter.rs`](src/api/markdown_frontmatter.rs); tested by [`tests/markdown_frontmatter.rs`](tests/markdown_frontmatter.rs). _(merged #28)_
- [x] **Phase B — property mapping**: parse explicit `type:` tagged YAML values into [`crate::ir::metadata::PropertyValue`] (Title, RichText, Number, Select, MultiSelect, Date, Checkbox, Url, Email). Implemented as `parse_frontmatter_to_properties()` (plus `properties_to_yaml()` for the reverse) in [`src/ir/frontmatter.rs`](src/ir/frontmatter.rs). _(merged #30)_
- [x] **Phase C — converter**: `MarkdownWithFrontmatterConverter` that round-trips Markdown+YAML ↔ UniversalDocument with `metadata.properties` populated. Lives in [`src/converter/markdown_frontmatter.rs`](src/converter/markdown_frontmatter.rs). _(merged #27, fmt #29)_
- [x] **Phase D — sync glue**: `blink-md sync --dir <dir>` now reads frontmatter from each `.md` file via `MarkdownWithFrontmatterConverter::from_platform()` → `NotionToPlatform::to_platform()` and writes properties into the Notion page on `create_page`. The YAML block no longer leaks into the page body. When no `title`-typed property is present, the file stem is used as a `Name` title (preserving the prior default). Implemented in [`src/cli/sync_cmd.rs`](src/cli/sync_cmd.rs) with `ensure_title()` unit tests.
- [x] **Phase E — export**: `export_page_to_md(page_id, out_dir)` (CLI: `blink-md export-page <id> [--out-dir <dir>]`) writes one `<slug>-<page-id>.md` file per page with a typed YAML header + Markdown body — the reverse of Phase D, reusing `properties_to_yaml()`. Notion properties are mapped back to typed `PropertyValue`s (title, rich_text, number, select, multi_select, date, checkbox, url, email; unknown kinds fall back to `custom`). Implemented in [`src/cli/export_cmd.rs`](src/cli/export_cmd.rs) with `slugify()` / property-mapping unit tests.

### 1. Platform adapters behind Universal IR
> Architecture overhaul for making new platforms cheap to add (Reader/Writer + Source/Sink, filters, capabilities, ChangeSet write path) is designed in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md). New Notion-remainder work is tracked in issues #39 (OAuth), #40 (webhook worker), #41 (API surface).
- [x] **GFM tables**: `MarkdownConverter` round-trips pipe tables (parse → IR `Table`, render IR → aligned pipe table). `block_ir_to_notion` writes Notion `Table`/`TableRow` blocks (so `sync` pushes tables) and `NotionFromPlatform` regroups flattened API rows back into one IR table (so `export-page` renders a single table). Implemented in [`src/api/markdown.rs`](src/api/markdown.rs), [`src/converter/markdown.rs`](src/converter/markdown.rs), and [`src/converter/notion.rs`](src/converter/notion.rs).
- [ ] GitHub Markdown/GFM extensions: footnotes, alerts, issue/PR refs, mentions, commit refs. _(Cell alignment is lost on the md→Notion→IR path since the Notion table model has no per-column alignment.)_
- [ ] HTML adapter: semantic tags, styles, images, links, and platform extensions.
- [ ] Full Lark/Feishu API adapter: 48 block types, mentions, topics, files, Bitable references.
- [ ] Google Docs adapter: paragraphs, tables, TOC, section breaks, inline objects, suggestions.
- [ ] PDF export: layout, fonts, pagination, headers/footers, TOC.
- [ ] Docx adapter: paragraphs, runs, tables, images, styles, numbering, headers/footers.
- [ ] Sheets/Excel adapter: merged cells, formulas, validation, conditional formatting.

### 2. Notion API surface
- [ ] Page markdown endpoints: GET/PATCH markdown.
- [ ] Data source CRUD: create, update, delete, list, query with pagination. _(have: get/list/query data sources + query pagination; missing create/update/delete.)_
- [x] **Webhooks: event types, payload parsing, signature verification** — `WebhookEventType`, `WebhookEvent`/`WebhookPayload`, `parse_webhook_payload()`, and constant-time `verify_webhook_signature()` (HMAC-SHA256) in [`src/api/webhooks.rs`](src/api/webhooks.rs).
- [x] **Search enhancements: sort, filter, page_size, start_cursor** — already on `search()`; added `search_all()` auto-pagination helper in [`src/api/search.rs`](src/api/search.rs).
- [x] **Block operations: delete, get, append with position** — `get_block()` added; `append_block_children` already takes a `Position`; `update_block`/`delete_block` present in [`src/api/blocks.rs`](src/api/blocks.rs). _(per-block-type typed update still open.)_
- [ ] File upload polish: multipart, external URL, base64, retry/error handling.

### 3. CLI & TUI UX
- [x] **CLI output formatting** — list/get commands render aligned, unicode-width-aware tables by default; global `--format table|json` for machine output. Lives in [`src/cli/output.rs`](src/cli/output.rs). _(Increment 1.)_
- [x] **Industry-standard error output** — `error: <message>: <cause>` to stderr, non-zero exit, red on TTY (honors `NO_COLOR`); `-v/--verbose` for the full chain + backtrace.
- [ ] CLI ergonomics — `--limit`, `--sort`, `--filter` on `search`/list commands. _(Increment 2.)_
- [ ] TUI preview/edit: preview page as Markdown through IR; edit in `$EDITOR`, convert back, push to Notion.
- [ ] TUI conflict resolution: local wins, remote wins, merge.
- [ ] TUI live search results and better status/help surfaces. _(Theme system, footer hints, loading/error status, and `?` help overlay already shipped — see issue #1.)_

### 4. Integration tests
- [x] **Enable ignored wiremock tests** — removed `#[ignore]` from the whole suite in [`tests/integration_tests.rs`](tests/integration_tests.rs) so they run under `cargo test`; fixed 3 fixtures that had drifted from the models (`List.has_more`, PascalCase `ViewType`, `TableConfig.properties`).
- [x] **API tests for pages, databases, blocks, webhooks, search, and errors** — wiremock coverage for users/blocks (children + single get)/pages-search (+ `search_all` pagination)/databases/comments/views/webhooks/errors. _(files upload tests still open.)_
- [ ] Add converter roundtrips for Notion, Markdown, HTML, Google Docs, Lark, Docx, PDF, and Sheets.

### 5. Documentation
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

*Updated: 2026-07-01 | v0.4.2 released — CLI output/error formatting, webhooks, frontmatter Phases A–E, GFM tables, and the adapter architecture proposal (`docs/ARCHITECTURE.md`) landed.*
