# Plan: Hermes-MCP Integration

เป้าหมาย: เพิ่มความสามารถให้ `jules-mcp-server` สามารถเรียกใช้งาน Hermes Agent เพื่อทำงาน Background และ Cron Jobs

## Phase 1: Bridge Design (DONE)
- [x] ออกแบบ Tool Interface ใน Rust สำหรับเรียกใช้ `hermes` CLI
- [x] กำหนดโครงสร้าง JSON สำหรับการสื่อสารระหว่าง MCP และ Hermes

## Phase 2: Implementation (Rust) (DONE)
- [x] เพิ่ม Tool `hermes_query` ใน `src/main.rs`
- [x] เพิ่ม Tool `hermes_list_skills` และ `hermes_cron_status`
- [x] จัดการ Error Handling และความปลอดภัยในการเรียก Command

## Phase 3: Validation (DONE)
- [x] ทดสอบสั่งงาน Hermes ผ่าน MCP Client (สำเร็จผ่าน `hermes_query`)
- [x] ตรวจสอบความถูกต้องของการสร้าง Skill อัตโนมัติ (Verified CLI path and output capture)
- [x] บรรลุเป้าหมาย Zero Warnings ใน Rust codebase
