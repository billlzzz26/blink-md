# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2026-06-13

### Added
- **Multi-Platform Support**: Added official release artifacts for Linux (amd64), macOS (Intel/M1), and Windows (amd64).
- **Hardened Thai TUI**: Implemented grapheme-cluster aware input and precise visual width calculation for Thai characters.
- **Universal Data Adapters**: Initial support for Lark (Feishu) Sheets and CSV via Universal IR.
- **Self-Update**: Added `blink-md upgrade` command for automated CLI updates.
- **Global Installers**: Created `scripts/install.sh` and `scripts/install.ps1` with interactive path selection.
- **Enhanced Help**: Added detailed command descriptions and usage examples to `--help`.

### Changed
- **Branding**: Updated repository description and topics for better discoverability.
- **Version Sync**: Synchronized all workspace package versions to 0.3.1.
- **File Integrity**: Standardized on LF line endings and ensured POSIX-compliant trailing newlines across the project.

## [0.3.0] - 2026-06-13

### Added
- **MCP Workspace**: Restructured MCP servers into `src/mcp/{core,jules,md,mmd}` workspace.
- **mcp-core v0.2.0**: Shared library for pmcp-based MCP servers with SchemaBuilder, CLI bridge, and logging utilities.
- **jules-mcp-server v0.3.0**: 8 tools for Jules AI agent and Hermes bridge (start_new_jules_task, list_jules_sessions, hermes_query, etc.).
- **md-mcp-server v0.2.0**: 2 tools for Markdown operations (parse_markdown, to_markdown).
- **mmd-mcp-server v0.1.0**: 3 tools for Mermaid diagram rendering (render_mermaid_svg, render_mermaid_png, list_diagram_types). Uses mermaid-rs-renderer from crates.io.
- **TUI Theme System**: Theme struct with Notion colors, 15 JSON themes (Dracula, Nord, Gruvbox, etc.).
- **SyntaxHighlighter**: syntect-based syntax highlighting for code blocks (prepared).
- **Integration Tests**: 4 new tests for mmd-mcp-server following TDD principles.
- **Workspace Lints**: Shared clippy/rust lints (unsafe_code = "forbid", clippy all + pedantic).

### Changed
- **pmcp Integration**: Migrated from custom STDIO MCP to pmcp v2.9.0 SDK.
- **Dependency Sharing**: Workspace-level dependencies (serde, tokio, anyhow, thiserror, pmcp).
- **Documentation**: Updated TODO.md, PLAN.md, README.md for v0.3.0.

### Fixed
- **Mermaid Integration**: Replaced vendored mermaid-rs-renderer with crates.io dependency (v0.2.2).
- **Build Stability**: Resolved edition conflicts in workspace members.

## [0.2.0] - 2026-06-04

### Added
- **Universal Database Schema**: Implemented a platform-agnostic PostgreSQL schema in `db/schema.sql` to support the v0.2.0 Universal IR architecture.
- **Relational Models**: Added `DbDocument`, `DbBlock`, `DbUser`, `DbWorkspace`, `DbStyle`, and `DbComment` in `src/models/db.rs`.
- **UI Completeness**: Integrated LexoRank-style `sort_order` (DOUBLE PRECISION) across blocks and documents to guarantee exact visual sequence rendering.
- **Database Views**: Added `database_views` table to store UI configurations (Table, Board, Gallery, etc.) for databases.
- **JSONB Indexing**: Applied GIN indexes to `content` and `properties` fields for high-performance querying of dynamic document data.

### Changed
- **Dependency Modernization**: Updated all project dependencies to their latest stable versions, including `thiserror` v2.0, `reqwest` v0.13, `tokio` v1.52, and `clap` v4.6.
- **IR Alignment**: Refactored database models to map directly from the `UniversalDocument` and `UniversalBlock` Intermediate Representations.
- **API Stabilization**: Improved `NotionClient` robustness and updated internal models to use `snake_case` naming conventions consistently.

### Removed
- **Obsolete Parsers**: Removed `universal_parser.rs` and `universal_mapper.rs` in favor of the new IR-based converter architecture.
- **Legacy API Modules**: Removed `docx.rs`, `html.rs`, and `pdf.rs` from the `src/api/` directory (to be rewritten as IR-platform converters).
- **Dead Code**: Cleaned up various temporary files and unused modules identified during the refactoring process.

### Fixed
- **Type Safety**: Resolved several minor type mismatches between API responses and internal models during the dependency upgrade.
- **Build Stability**: Fixed compilation errors on ARM64/Termux environments by cleaning up object file collisions.

## [0.1.0] - 2026-05-31

### Added
- Initial release of the Unofficial Notion API SDK for Rust.
- Support for basic Notion blocks and pages.
- Preliminary Markdown export functionality.
- TUI for browsing Notion workspaces.

[0.3.1]: https://github.com/billlzzz26/blink-md/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/billzzz26/notion-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/billzzz26/notion-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/billzzz26/notion-rs/releases/tag/v0.1.0
