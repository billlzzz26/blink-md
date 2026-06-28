# blink-md TODO.md — v0.3.1

## Overview
- Version: 0.3.1
- Current branch: `main`
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
### 0. Markdown + YAML Frontmatter ↔ Notion Database (TDD, in progress)
- [x] **Phase A — detection**: `detect_frontmatter()` extracts a `---`-delimited YAML block from the start of a Markdown file, leaving the rest as body. 15 unit tests cover: missing block, unterminated block, multi-line YAML, empty YAML, CRLF, body containing `---`, colons in values, and edge cases like empty input. Lives in [`src/api/markdown_frontmatter.rs`](src/api/markdown_frontmatter.rs); tested by [`tests/markdown_frontmatter.rs`](tests/markdown_frontmatter.rs).
- [x] **Phase B — property mapping**: `parse_frontmatter_to_properties()` / `properties_to_yaml()` translate between explicit `type:` tagged YAML and [`crate::ir::metadata::PropertyValue`]. 9 `PropertyType` variants (Title, RichText, Number, Select, MultiSelect, Date, Checkbox, Url, Email), 25 unit tests in [`tests/frontmatter_properties.rs`](tests/frontmatter_properties.rs). Adds `serde_yaml = "0.9"` dependency.
- [ ] **Phase C — converter**: `MarkdownWithFrontmatterConverter` that round-trips Markdown+YAML ↔ UniversalDocument with `metadata.properties` populated.
- [ ] **Phase D — sync glue**: teach `blink-md sync --dir <dir>` to read frontmatter from each `.md` file and write properties into the Notion page on `create_page`.
- [ ] **Phase E — export**: `export_page_to_md(page_id, out_dir)` writes one `<slug>-<page-id>.md` file per page with YAML header + body.

### 1. Platform adapters behind Universal IR
- [ ] GitHub Markdown/GFM extensions: footnotes, alerts, issue/PR refs, mentions, commit refs.
- [ ] HTML adapter: semantic tags, styles, images, links, and platform extensions.
- [ ] Full Lark/Feishu API adapter: 48 block types, mentions, topics, files, Bitable references.
- [ ] Google Docs adapter: paragraphs, tables, TOC, section breaks, inline objects, suggestions.
- [ ] PDF export: layout, fonts, pagination, headers/footers, TOC.
- [ ] Docx adapter: paragraphs, runs, tables, images, styles, numbering, headers/footers.
- [ ] Sheets/Excel adapter: merged cells, formulas, validation, conditional formatting.

### 2. Notion API surface
- [ ] Page markdown endpoints: GET/PATCH markdown.
- [ ] Data source CRUD: create, update, delete, list, query with pagination.
- [ ] Webhooks: event types, payload parsing, signature verification.
- [ ] Search enhancements: sort, filter, page_size, start_cursor.
- [ ] Block operations: update all block types, delete, get, append with position.
- [ ] File upload polish: multipart, external URL, base64, retry/error handling.

### 3. TUI preview/edit workflows
- [ ] Preview page as Markdown through IR.
- [ ] Edit page in `$EDITOR`, convert back, and push to Notion.
- [ ] Conflict resolution: local wins, remote wins, merge.
- [ ] Live search results and better status/help surfaces.

### 4. Integration tests
- [ ] Enable or intentionally document ignored wiremock tests.
- [ ] Add API tests for pages, databases, blocks, files, webhooks, search, and errors.
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

*Updated: 2026-06-18 | CI/package/docs sync pass added.*
