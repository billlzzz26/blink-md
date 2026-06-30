# blink-md Plan — v0.3.1

## Current state
- Version: 0.3.1
- Main branch: clean delivery baseline for cross-platform release artifacts, Thai TUI hardening, Universal Data Adapters, self-update, installers, and CLI help polish.
- Source of truth: `TODO.md` for work status, `CHANGELOG.md` for released changes, `README.md` for user-facing guidance.
- Quality gates: `make ci` plus GitHub Actions CI and Cross-Platform Build.

## Completed in v0.3.1
- Unified MCP server: single `blink-md-mcp` binary (feature `mcp`) with in-crate `src/mcp/` modules (core, tools, server). The former per-platform servers were merged; Jules/Hermes tooling moved to `tooling/jules`.
- Universal IR foundation for Notion and Markdown conversion.
- Markdown roundtrip tests and converter coverage.
- TUI theme system with 15 JSON themes and syntect-based syntax highlighting.
- Thai TUI hardening with grapheme-aware input and visual-width handling.
- Lark/Feishu Sheets and CSV adapters through Universal IR.
- Self-update command and global installers.
- Release workflow for Linux, macOS Intel/ARM, and Windows artifacts.
- Package hygiene guard to keep local agent data, secrets, and internal conductor docs out of `cargo package`.

---

## Workflow Goals (md-sync, db2sheet, msg2chan)

### Phase 1: md-sync (Markdown ↔ Notion Database)
- [ ] **Phase B — property mapping**: parse explicit `type:` tagged YAML values into `PropertyValue` (Title, RichText, Number, Select, MultiSelect, Date, Checkbox, Url, Email, Relation)
- [ ] **Phase C — converter**: `MarkdownWithFrontmatterConverter` round-trips Markdown+YAML ↔ UniversalDocument with `metadata.properties` populated
- [ ] **Phase D — sync glue**: teach `blink-md sync --dir <dir>` to read frontmatter from each `.md` file and write properties into the Notion page on `create_page`
- [ ] **Phase E — export**: `export_page_to_md(page_id, out_dir)` writes one `<slug>-<page-id>.md` file per page with YAML header + body

### Phase 2: db2sheet (Database to Sheet)
- [ ] Add `db_query` tool with --format csv|json|osc
- [ ] Support pagination via `start_cursor` and `page_size`
- [ ] OSC output: emit `/row/update` messages for each row

### Phase 3: msg2chan (Message to Channel)
- [ ] Create `src/api/message.rs` module
- [ ] Accept text input from stdin, file, or webhook
- [ ] Auto-export to `.md` with slug+timestamp

---

## Google Workspace Integration (OAuth + API Adapter)

### Phase 1: OAuth Foundation
- [ ] Create `src/oauth.rs` - token provider trait + caching (adapted from google-workspace-cli)
- [ ] Create `src/services.rs` - service registry mapping aliases to API names/versions
- [ ] Add `--google` feature to Cargo.toml with `yup-oauth2` config

### Phase 2: API Modules
- [ ] Create `src/api/google/mod.rs` - common Google API utilities
- [ ] Create `src/api/google/docs.rs` - Google Docs read/write
- [ ] Create `src/api/google/sheets.rs` - Spreadsheet API
- [ ] Create `src/api/google/keep.rs` - Google Keep notes
- [ ] Create `src/api/google/chat.rs` - Chat spaces/messages
- [ ] Create `src/api/google/calendar.rs` - Calendar events
- [ ] Create `src/api/google/tasks.rs` - Task lists

### Phase 3: IR Adapters
- [ ] Extend IR with Google property types (text styles, suggestions, section breaks)
- [ ] Build `GoogleDocConverter` for lossless Docs ↔ Universal IR
- [ ] Build `GoogleSheetConverter` for Sheets ↔ IR tables
- [ ] Wire converters to `blink-md convert` command

---

## Dependencies

| Feature | Crates Needed |
|---------|---------------|
| Google OAuth | `yup-oauth2` (existing) |
| Token encryption | `aes-gcm`, `zeroize` |
| Google APIs | `google-docs1` (existing), `google-sheets4` (optional) |
| OSC output | `rosc` (optional) |