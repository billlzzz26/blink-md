# IMPLEMENTATION PLAN: High-Fidelity Infrastructure

## PHASE 0: INFRASTRUCTURE & MCP (v0.3.0) — COMPLETE
- [x] Workspace restructure: `src/mcp/{core,jules,md,mmd}`
- [x] mcp-core v0.2.0 — shared pmcp utilities (SchemaBuilder, run_cli_command)
- [x] jules-mcp-server v0.3.0 — 8 tools (Jules/Hermes bridge)
- [x] md-mcp-server v0.2.0 — 2 tools (parse_markdown, to_markdown)
- [x] mermaid-rs-renderer v0.2.2 — from crates.io, 4 integration tests
- [x] Shared workspace dependencies & lints
- [x] TUI Theme system (15 JSON themes)
- [x] SyntaxHighlighter (syntect) — prepared

## PHASE 1: IR STORE & SCHEMA STABILIZATION — COMPLETE
- [x] บล็อกข้อมูลโครงสร้างพื้นฐาน (Relational + JSONB)
- [x] ออกแบบ Universal IR Types (Blocks, Inlines, Styles)
- [x] พัฒนาฐานข้อมูล PostgreSQL รองรับ LexoRank

## PHASE 2: UNIVERSAL CONVERTERS (CURRENT)
เป้าหมาย: สร้างการเชื่อมต่อที่สมบูรณ์ระหว่าง Platform-specific JSON และ Universal IR

### 2.1 Notion Adapter Enhancement
- [ ] พัฒนา Recursive child block fetching แบบ lossless
- [ ] แมพ Notion Blocks ทั้งหมด (รวมถึงพวก Column, Callout, Toggle) เข้ากับ IR
- [ ] ระบบ ID Mapping สำหรับการทำ Sync แบบ bidirectional

### 2.2 Markdown Adapter (GFM)
- [ ] พัฒนา Parser (From Markdown) และ Emitter (To Markdown)
- [ ] รองรับ Notion-flavored markdown extensions
- [ ] จัดการเรื่อง Escaping characters และ Rich text formatting ให้ตรงตามมาตรฐาน

## PHASE 3: UX & TUI REFINEMENT
เป้าหมาย: สร้าง Interface ที่สะท้อนความถูกต้องของข้อมูลจาก IR Store

- [x] พัฒนา TUI Theme ตาม DESIGN.md (Accent Blue, Primary Text)
- [ ] ระบบ Preview โหมด ที่เรนเดอร์ข้อมูลจาก Universal IR โดยตรง
- [ ] ระบบสถานะและการแจ้งเตือน (Status Message) เมื่อเกิดข้อผิดพลาดในการแปลง

## PHASE 4: EXTENDED PLATFORMS & MCP
เป้าหมาย: ขยายการรองรับแพลตฟอร์มอื่นและการเชื่อมต่อกับ AI

- [ ] พัฒนา Adapter สำหรับ Lark (Handle 48 block types)
- [ ] พัฒนา Adapter สำหรับ Google Docs
- [x] พัฒนา MCP Server เต็มรูปแบบ (jules, md, mmd)

## PHASE 5: VALIDATION & SYNC ENHANCEMENT
เป้าหมาย: รับประกันความถูกต้อง 100%

- [ ] พัฒนาชุดทดสอบ Roundtrip (Notion -> IR -> Markdown -> IR -> Notion)
- [ ] ระบบเปรียบเทียบความต่าง (Diff Engine) สำหรับ Universal Document
- [ ] ฟีเจอร์ Sync แบบ Debounced และ multi-threaded
