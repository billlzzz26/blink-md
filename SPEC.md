# blink-md Workflow Goals

## md-sync: Bidirectional Markdown ↔ Notion Database Sync (merged)

### Status: Phases A–E complete
- Detection, property mapping, converter, sync glue, and page export all landed.
- See merged PRs #27, #28, #30.

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

## Google Workspace OAuth + API Adapter (in progress on feature branch)

### Goal
Integrate Google Workspace APIs (Docs, Sheets, Keep, Chat, Calendar, Tasks) using `yup-oauth2` pattern from google-workspace-cli.

### Context
blink-md already has `yup-oauth2` and `google-docs1` in Cargo.toml. Extend to full Google Workspace support.

### Architecture (adapted from google-workspace-cli)
- OAuth flow with `yup-oauth2::InstalledFlowDelegate` (loopback server)
- Token caching to `~/.config/blink-md/token_cache.json`
- Service registry mapping aliases → API names/versions

### Services to Add
| Alias | API | Version |
|-------|-----|---------|
| docs | docs | v1 |
| sheets | sheets | v4 |
| keep | keep | v1 |
| chat | chat | v1 |
| calendar | calendar | v3 |
| tasks | tasks | v1 |

### Implementation Path
1. Create `src/oauth.rs` - token provider trait + caching
2. Create `src/services.rs` - service registry pattern
3. Create `src/api/google/mod.rs` - Google API modules
4. Add `--google` feature to Cargo.toml

### Related Issues
- #32 Google Workspace OAuth + API Adapter
- #33 OAuth token provider with caching
- #34 Google Docs IR adapter
- #35 Google Sheets CSV export
- #36 Google Chat, Keep, Calendar, Tasks API modules