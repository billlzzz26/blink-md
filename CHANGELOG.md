# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Markdown YAML frontmatter detection (Phase A)**: new `blink_md::api::markdown_frontmatter::detect_frontmatter()` extracts a `---`-delimited YAML block from the start of a Markdown file and returns the raw YAML text plus the remaining body. Unterminated blocks are treated as plain Markdown. 15 unit tests cover happy paths, multi-line YAML, empty blocks, CRLF inputs, body text containing `---`, and edge cases. Phase B (property mapping), Phase C (converter), and Phase D (sync glue) are tracked in `TODO.md`.
- **Markdown YAML frontmatter property mapper (Phase B)**: new `blink_md::ir::frontmatter::{parse_frontmatter_to_properties, properties_to_yaml, PropertyType, FrontmatterError}` translates between explicit `type:` tagged YAML and `crate::ir::metadata::PropertyValue`. Supports 9 property types (Title, RichText, Number, Select, MultiSelect, Date, Checkbox, Url, Email) with structured errors for unknown types, missing fields, wrong field types, and invalid YAML. Adds `serde_yaml = "0.9"` dependency. 25 unit tests in `tests/frontmatter_properties.rs`.

## [0.4.1] - 2026-06-27

### Added
- **MCP Workspace Expansion**: Added `lark-mcp-server` and `notion-mcp-server` to the MCP workspace.
- **MCP Tool Schemas**: Added JSON schemas for all MCP servers in `docs/mcp/tools/`.
- **npm Package Wrapper**: Added `npm/package.json` and `npm/install.js` for Node.js installation.
- **GitHub Workflows**: Added audit, ci, coverage, stale, cla, policy, publish-skills, release-changesets, npm-publish workflows.

### Changed
- Bumped version from 0.3.1 to 0.4.1.

## [Unreleased]

### Added
- **Markdown YAML frontmatter detection (Phase A)**: new `blink_md::api::markdown_frontmatter::detect_frontmatter()` extracts a `---`-delimited YAML block from the start of a Markdown file and returns the raw YAML text plus the remaining body. Unterminated blocks are treated as plain Markdown. 15 unit tests cover happy paths, multi-line YAML, empty blocks, CRLF inputs, body text containing `---`, and edge cases. Phase B (property mapping), Phase C (converter), and Phase D (sync glue) are tracked in `TODO.md`.

### Changed
- **Single-crate consolidation**: collapsed the multi-crate Cargo workspace (`mcp-core`, `notion`/`md`/`mmd`/`lark`/`jules` MCP servers) back into the single `blink-md` crate. There are no sub-crates anymore.
- **Unified MCP server**: the per-platform MCP server binaries were merged into one `blink-md-mcp` binary (behind the `mcp` feature) that registers every Notion, Markdown, Lark Sheets, and Mermaid tool. The shared `mcp-core` helpers now live in `src/mcp/core.rs`.
- **Redesigned document deletion**: introduced a unified trash lifecycle (`src/api/trash.rs`) with a `Resource` enum and `trash` / `restore` / `delete_permanently` methods, plus a `Trashable` trait that unifies the legacy `archived` and current `in_trash` fields. `delete_block`/`delete_view` now delegate to it.
- **CI/CD**: removed the duplicate `rust-ci.yml`, and switched workflows, the `Makefile`, and helper scripts from `--workspace` to single-crate `--all-features` builds.
- Added `.qwen/` to `.gitignore` and package exclusion.

### Fixed
- **Android CI**: switched `self_update` to rustls so Android cross builds no longer depend on native-tls/OpenSSL sysroot.
- **Package hygiene**: added a package gate that blocks local agent data, secrets, and internal conductor docs from `cargo package`.
- **MSRV CI**: made the MSRV check fail loudly instead of masking failures.
- **Release CI**: Windows release artifacts now use PowerShell `Compress-Archive` instead of creating a gzip tarball with a `.zip` extension.
- **Release CI**: crates.io publish now fails the release workflow instead of being ignored.

### Removed
- **Jules/Hermes agent tooling** moved out of `src/mcp/jules` to `tooling/jules` (excluded from the crate and the build).
- Removed tracked local agent configuration/examples and obsolete Gemini docs from the repository.
- Removed tracked Jules anti-slop backup file.

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

[0.4.1]: https://github.com/billlzzz26/blink-md/compare/v0.3.1...v0.4.1
[0.3.1]: https://github.com/billlzzz26/blink-md/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/billlzzz26/blink-md/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/billlzzz26/blink-md/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/billlzzz26/blink-md/releases/tag/v0.1.0
