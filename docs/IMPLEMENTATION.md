# blink-md Implementation Status — v0.3.1

## What is implemented
- Single-crate layout; the unified MCP server lives in-crate under `src/mcp/` (core, tools, server) behind the `mcp` feature.
- In-crate `src/mcp/core.rs` shared pmcp helpers and schema utilities (was the `mcp-core` crate).
- `blink-md-mcp` binary bundling Markdown, Notion/IR, Lark Sheets, and Mermaid tools, plus live Notion tools when `NOTION_TOKEN` is set.
- Markdown parse/to-markdown tools.
- Mermaid MCP server with integration tests.
- Universal IR types for documents, blocks, inline elements, styles, tables, and metadata.
- Notion ↔ IR conversion for pages, blocks, rich text, mentions, properties, and common Notion extensions.
- Markdown/GFM ↔ IR conversion with roundtrip tests.
- TUI theme system with JSON themes and syntect highlighting.
- Thai TUI hardening for grapheme clusters and visual width.
- Lark/Feishu Sheets and CSV adapters through Universal IR.
- `blink-md upgrade` self-update using GitHub releases.
- Install scripts and release artifacts for Linux, macOS, and Windows.

## What is not finished
- GitHub Markdown/GFM adapter extensions beyond the current Markdown IR path.
- HTML adapter.
- Full Lark/Feishu API adapter.
- Google Docs adapter.
- PDF export.
- Docx adapter.
- Sheets/Excel adapter.
- Page markdown endpoints.
- Data source CRUD.
- Webhook events and signature verification.
- Search sort/filter.
- Block position updates.
- File upload polish.
- TUI preview/edit flows.

## TUI status
- Theme tokens are implemented in `src/cli/theme.rs`.
- TUI uses theme colors and borders instead of hardcoded black/white styling.
- TUI has loading/error status text in the footer.
- TUI has keyboard help popup support.
- Remaining UX work is preview/edit flows, not the basic theme/status foundation.

## Engineering guardrails
- Do not add features without tests.
- Do not merge if `make ci` fails.
- Do not let `cargo package` include local agent data, secrets, or internal conductor docs.
- Keep `TODO.md`, `docs/PLAN.md`, `README.md`, and `CHANGELOG.md` in sync before release.
