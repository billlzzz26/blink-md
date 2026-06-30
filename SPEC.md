# blink-md Workflow Goals

## md-sync: Bidirectional Markdown ↔ Notion Database Sync

### Goal
Complete Phase B-E (property mapping to sync glue) for YAML frontmatter ↔ Notion database synchronization.

### Context
Markdown files with YAML frontmatter should sync two-way with Notion database properties. YAML `type:` tag indicates Notion property type.

### Requirements
- Parse YAML `type:` tagged values to `PropertyValue` enum
- `MarkdownWithFrontmatterConverter` round-trips Markdown+YAML ↔ UniversalDocument
- `blink-md sync --dir <dir>` reads frontmatter and writes Notion properties
- `export_page_to_md(page_id, out_dir)` writes `.md` files with YAML headers

### Implementation Path
1. Extend `parse_frontmatter_to_properties` to handle type tags
2. Build converter in `src/converter/markdown_frontmatter.rs`
3. Add sync command in `src/cli/commands.rs`
4. Implement export in `src/api/notion/export.rs`

---

## db2sheet: Notion Database → CSV/JSON → OSC

### Goal
Query Notion databases and pipe to stdout as CSV/JSON, with optional OSC output.

### Context
Users can view Notion databases as terminal-compatible sheets; OSC output enables real-time integration with other tools.

### Requirements
- `blink-md db query --id <database_id> --format csv|json|osc`
- Handle pagination for large databases
- OSC output: emit `/row/update` messages for each row

### Implementation Path
1. Add `db_query` tool in `src/mcp/tools/`
2. Create formatter module for CSV/JSON serialization
3. Add OSC emitter using `rosc` crate (optional feature)

---

## msg2chan: Chat Message → Notion Page → Markdown

### Goal
Pipe chat messages to Notion pages, auto-export to markdown files.

### Context
Every chat message becomes a dated note in a Notion database, written to markdown in a vault directory.

### Requirements
- Accept stdin/file/webhook input
- Create Notion page from message content
- Auto-export to `.md` with slug+timestamp
- Optional: integrate with Telegram/Discord connectors

### Implementation Path
1. Add message handler in `src/api/message.rs`
2. Wire to Notion create_page API
3. Configure webhook server (axum/hyper)

---

## Google Workspace OAuth + API Adapter

### Goal
Integrate Google Workspace APIs (Docs, Sheets, Keep, Chat, Calendar, Tasks) using `yup-oauth2` pattern from google-workspace-cli.

### Context
blink-md already has `yup-oauth2` and `google-docs1` in Cargo.toml. Extend to full Google Workspace support.

### Architecture (adapted from google-workspace-cli)
- OAuth flow with `yup-oauth2::InstalledFlowDelegate` (loopback server)
- Token caching to `~/.config/blink-md/token_cache.json`
- Service registry mapping aliases → API names/versions

### Services to Add
| Alias | API | Version | Description |
|-------|-----|---------|-------------|
| docs | docs | v1 | Read/write Google Docs |
| sheets | sheets | v4 | Read/write spreadsheets |
| keep | keep | v1 | Manage notes |
| chat | chat | v1 | Manage spaces/messages |
| calendar | calendar | v3 | Manage calendars/events |
| tasks | tasks | v1 | Manage task lists |

### Implementation Path
1. Create `src/oauth.rs` - token provider trait + caching
2. Create `src/services.rs` - service registry pattern
3. Create `src/api/google/mod.rs` - Google API modules
4. Add `--google` feature to Cargo.toml

---

## Dependencies to Add

| Feature | Crates |
|---------|--------|
| Google OAuth | `yup-oauth2` (existing) |
| Token encryption | `aes-gcm`, `zeroize` |
| Google APIs | `google-docs1` (existing), `google-sheets4` (optional) |