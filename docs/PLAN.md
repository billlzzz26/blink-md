# blink-md Plan — v0.3.1

## Current state
- Version: 0.3.1
- Main branch: clean delivery baseline for cross-platform release artifacts, Thai TUI hardening, Universal Data Adapters, self-update, installers, and CLI help polish.
- Source of truth: `TODO.md` for work status, `CHANGELOG.md` for released changes, `README.md` for user-facing guidance.
- Quality gates: `make ci` plus GitHub Actions CI and Cross-Platform Build.

## Completed in v0.3.1
- Workspace MCP servers: `src/mcp/core`, `src/mcp/jules`, `src/mcp/md`, `src/mcp/mmd`.
- Universal IR foundation for Notion and Markdown conversion.
- Markdown roundtrip tests and converter coverage.
- TUI theme system with 15 JSON themes and syntect-based syntax highlighting.
- Thai TUI hardening with grapheme-aware input and visual-width handling.
- Lark/Feishu Sheets and CSV adapters through Universal IR.
- Self-update command and global installers.
- Release workflow for Linux, macOS Intel/ARM, and Windows artifacts.
- Package hygiene guard to keep local agent data, secrets, and internal conductor docs out of `cargo package`.

## Active engineering focus
1. Keep CI green across stable CI, cross-platform release builds, and package hygiene.
2. Finish remaining platform adapters behind Universal IR:
   - GitHub Markdown/GFM
   - HTML
   - Lark/Feishu API
   - Google Docs
   - PDF
   - Docx
   - Sheets/Excel
3. Finish remaining Notion API surface:
   - page markdown endpoints
   - data source CRUD
   - webhooks
   - search sort/filter
   - block position updates
   - file upload polish
4. Improve TUI from browse-only to preview/edit workflows:
   - preview page as Markdown through IR
   - edit with `$EDITOR`
   - convert back and push to Notion
   - better status/help surfaces

## Definition of done for every new feature
- Code change is covered by tests that fail before the fix or feature exists.
- `make ci` passes locally.
- `scripts/check-package-hygiene.py` passes.
- README, TODO.md, CHANGELOG.md, and relevant docs are updated before merge.
- CI workflows are updated if the feature changes build, package, release, or cross-platform behavior.
- Secrets and local agent state stay ignored and are never included in packages or commits.
