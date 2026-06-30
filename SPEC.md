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