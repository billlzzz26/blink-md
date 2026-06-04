# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.2.0]: https://github.com/billzzz26/notion-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/billzzz26/notion-rs/releases/tag/v0.1.0
