# SPECIFICATION: High-Fidelity Universal IR & Cross-Platform Sync

## 1. PROJECT OBJECTIVE
Create a platform-agnostic document conversion and sync engine that preserves source structure, rich text, ordering, and platform-specific metadata through a Universal Intermediate Representation (IR).

## 2. CORE CONCEPTS

### 2.1 Universal Identity (The "Key")
- Every document source, such as Notion Page ID, Google Docs Doc ID, Lark File Token, or local file path, should map to a stable identity in the conversion pipeline.
- ID resolution must distinguish source identity from target identity to avoid duplicate or conflicting updates.

### 2.2 Universal Intermediate Representation (IR)
- IR is the central conversion model for source adapters and target emitters.
- Block types are represented as typed variants, with `Raw` data preserving platform-specific fields that cannot be mapped directly.
- Inline elements preserve text, links, mentions, styles, and code spans.
- Styles and metadata stay attached to documents and blocks during conversion.

### 2.3 Structural Grammar & Syntax
- Source adapters parse platform-specific structures into normalized IR.
- Target emitters generate platform-specific syntax from IR.
- Roundtrip tests validate that conversion preserves semantic structure and visible content.

## 3. CURRENT IMPLEMENTATION SCOPE
- CLI orchestration for search, TUI, convert, sync, diff, upgrade, and MCP server modes.
- Notion API client and common page/database/block operations.
- Notion ↔ IR and Markdown/GFM ↔ IR conversion.
- Local file conversion and sync workflows.
- MCP servers for Jules, Markdown, and Mermaid workflows.

## 4. PLANNED EXTENSIONS
- Relational persistence or local cache for offline-first sync.
- HTML, Google Docs, Lark/Feishu API, PDF, Docx, and Sheets/Excel adapters.
- Webhooks, data sources, file upload polish, and advanced search filters.
- TUI preview/edit workflows with conflict resolution.

## 5. INTERFACES
- CLI: orchestration and local workflows.
- TUI: browsing, status, help, and future preview/edit flows.
- MCP: AI-agent document manipulation.
- Tests: unit, integration, roundtrip, package hygiene, and CI release gates.
