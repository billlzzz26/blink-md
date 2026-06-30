# blink-md Plan — v0.4.1

## Current state
- Version: 0.4.1
- Current branch: `feature/google-workspace-oauth`
- Source of truth: `TODO.md` for work status, `CHANGELOG.md` for released changes, `README.md` for user-facing guidance.
- Quality gates: `make ci` plus GitHub Actions CI and Cross-Platform Build.

## Completed in v0.4.1
### MCP server (unified)
- Single `blink-md-mcp` binary (feature `mcp`) bundling all tools.
- In-crate `src/mcp/core.rs` shared pmcp utilities (was `mcp-core`).
- Markdown, Notion/IR, Lark Sheets, and Mermaid tool groups.
- Live Notion tools shared with `blink-md mcp-serve`.
- Jules/Hermes bridge relocated to `tooling/jules` (outside the build).

### Universal IR and converters
- Universal IR document/block/inline/style/table/metadata types.
- Notion ↔ IR conversion for pages, blocks, rich text, mentions, properties, and platform extensions.
- Markdown/GFM ↔ IR conversion with roundtrip tests.
- Lark/Feishu Sheets and CSV adapters through Universal IR.

### Frontmatter integration (merged in main)
- Phases A–E complete: detection, property mapping, converter, sync glue, and page export all landed.
- See merged PRs #27, #28, #30.

---

## Active workflow goals

### md-sync (completed)
- [x] Phases A–E complete (merged)

### db2sheet (pending)
- [ ] Add `db_query` tool with --format csv|json|osc
- [ ] Support pagination via `start_cursor` and `page_size`
- [ ] OSC output: emit `/row/update` messages for each row

### msg2chan (pending)
- [ ] Create `src/api/message.rs` module
- [ ] Accept text input from stdin, file, or webhook
- [ ] Auto-export to `.md` with slug+timestamp

---

## Next work: Cloud Platform OAuth + API Adapters

### Google Workspace
- [ ] OAuth foundation (src/oauth.rs, src/services.rs)
- [ ] Docs/Sheets API modules
- [ ] IR converters

### Lark/Feishu
- [ ] Tenant token provider
- [ ] Docs/Sheets adapters (feishu-sdk)
- [ ] IR converters

### Shared infrastructure
- [ ] Unified `cloud` CLI command
- [ ] Service registry pattern
- [ ] Batch operations for efficiency

---

## Dependencies to Add

| Feature | Crates |
|---------|--------|
| Google OAuth | `yup-oauth2` (existing) |
| Token encryption | `aes-gcm`, `zeroize` |
| OSC output | `rosc` (optional) |
| Lark/Feishu | `feishu-sdk` (existing) |