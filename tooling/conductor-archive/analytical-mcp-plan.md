# Analytical Tools MCP (Rust) - Implementation Plan

## 🎯 Objective
สร้างและปรับใช้ 30 Analytical Tools (แบ่งเป็น 15 UI-Rendered และ 15 Text-Only) ผ่าน MCP Server ที่พัฒนาด้วย Rust เพื่อเป็น Standalone Engine แยกต่างหาก (เนื่องจากไม่เหมาะกับโปรเจกต์ Jules โดยตรง) โดยรองรับระบบ `json-render` Collection และยึด Design System ที่กำหนด (Prompt font, #00FFA1 primary, radius scale, shadow levels)

## 📁 Architecture & Location
- **Location**: สร้างโปรเจกต์ Rust ใหม่ชื่อ `analytical-tools-mcp` ที่ `/storage/emulated/0/Projects/mcps/analytical-tools-mcp`
- **MCP Server Protocol**: รันผ่าน `stdio` เพื่อให้ใช้งานร่วมกับ Agent CLI ได้ง่าย
- **Output Format**:
  - **UI-Rendered (15 Tools)**: คืนค่าเป็น JSON payload ที่มีโครงสร้าง `{ "component": "...", "props": { ... } }` เพื่อให้ React Renderer นำไปวาดต่อ (ตาม Design System)
  - **Text-Only (15 Tools)**: คืนค่าเป็น Markdown Text เชิงวิเคราะห์

## 🛠 Implementation Steps

### Phase 1: Project Setup (Rust)
1. รัน `cargo new analytical-tools-mcp`
2. เพิ่ม Dependencies: `serde`, `serde_json`, `tokio` (ถ้าจำเป็น), และโครงสร้าง MCP เบื้องต้น (อ้างอิงจาก `mermaid-rs-renderer/src/mcp_server.rs` เพื่อความรวดเร็ว)

### Phase 2: Define UI-Rendered Tools (15 Components)
แปลงโครงสร้าง Zod schema จาก TypeScript ให้เป็น JSON Schema ใน Rust สำหรับ `tools/list`
1. **Diagrams**: `FishboneDiagram`, `WhyChain`, `HierarchyTree`
2. **Matrices**: `SwotMatrix`, `DecisionTable`, `ImpactEffortMatrix`, `EisenhowerMatrix`, `TowsMatrix`
3. **Boards**: `SixHatsBoard`, `RiskBoard`, `ScamperCards`
4. **Charts**: `ParetoChart`, `CostBenefitChart`
5. **Canvases**: `StrategyCanvas`, `MeceTree`
- *Execution Handler*: รับ Parameters ตาม Schema แล้วประกอบร่างเป็น JSON Payload เพื่อส่งกลับไปให้ `json-render` ฝั่ง Frontend

### Phase 3: Define Text-Only Tools Catalog (15 Tools)
กำหนด JSON Schema สำหรับ 15 Tools และเขียน Logic ให้คืนค่าผลลัพธ์การวิเคราะห์แบบ Text (หรือ Prompt/Template)
- Tools: `prompt_compression`, `adversarial_test`, `analytical_advisor`, `red_team_blue_team`, `assumption_testing`, `blind_spot_analysis`, `devil_advocate`, `dialectical_inquiry`, `force_field_analysis`, `stakeholder_mapping`, `reverse_brainstorming`, `attribute_listing`, `triz_contradiction`, `value_chain_analysis`, `kano_model`
- *Execution Handler*: รับ Parameters และคืนค่าเป็น Text Markdown ที่มีโครงสร้างชัดเจน

### Phase 4: Design System & Integration Instructions
- กำหนด Instruction/Description ของ Tool แต่ละตัวใน MCP เพื่อย้ำให้ AI เข้าใจวิธีการสร้าง Content ให้เข้ากับ Design System:
  - Font: `Prompt`
  - Primary Color: `#00FFA1`
  - Background: `#0E0E0E`
  - Text Color: `#333333` / White
- (หมายเหตุ: ตัว UI Rendering แบบ React ถือว่าทำงานอยู่ที่ฝั่ง Client/Consumer ของ MCP Server โปรเจกต์นี้จะโฟกัสที่การสร้าง MCP Server เพื่อส่ง JSON Spec ออกไปให้ถูกต้อง)

## ✅ Verification & Testing
1. รัน `cargo check` และ `cargo test` เพื่อให้มั่นใจว่า Code ไม่มี Warning/Errors (ยึดหลัก Zero Warnings)
2. ทดสอบ Request `tools/list` เพื่อตรวจสอบความถูกต้องของ JSON Schema ทั้ง 30 Tools
3. ทดสอบ Request `tools/call` ด้วย Dummy Data เพื่อดูว่า `json-render` output ออกมาถูกต้องตรงตาม Component Name และ Props หรือไม่
