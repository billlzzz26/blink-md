# Research: ACP & A2A Integration for Jules Bridge

เอกสารฉบับนี้สรุปข้อมูลจากการค้นคว้าเกี่ยวกับโปรโตคอล **ACP** และ **A2A** เพื่อใช้เป็นแนวทางในการยกระดับความสามารถของ `jules-mcp-server`

---

## 1. ACP (AI Context Protocol / Agent Client Protocol)

โปรโตคอลที่เน้นการจัดการ "บริบท" และ "การสื่อสารมาตรฐาน" ระหว่าง Client และ AI Agent

### ข้อมูลทางเทคนิค
- **Crate:** `acp` (Context Indexing) และ `sacp` (SDK)
- **จุดเด่น:**
    - **Token-Efficient Indexing:** ใช้ `tree-sitter` สแกนโค้ดเบสเพื่อสร้างดัชนี JSON ที่ AI เข้าใจง่ายและประหยัด Token
    - **Streaming Standard:** กำหนดมาตรฐานการส่งข้อมูลแบบ Real-time ระหว่างเครื่องมือ (เช่น IDE) กับ Agent

### โอกาสในการนำมาใช้กับโปรเจกต์นี้
- **Smart Context Filling:** ก่อนส่งงานให้ `jules` (ผ่าน `remote new`), ตัว MCP Server สามารถรัน `acp` เพื่อสแกนโปรเจกต์ปัจจุบันและแนบโครงสร้างไฟล์ที่สำคัญไปกับ Prompt ได้เลย ช่วยให้ `jules` ไม่ต้อง "เดา" สภาพแวดล้อม
- **Native SDK:** เปลี่ยนจากการเขียน JSON ดิบๆ มาใช้ `sacp` เพื่อเพิ่มความเสถียรในการสื่อสาร

---

## 2. A2A (Agent-to-Agent Protocol)

โปรโตคอลที่ทำให้ AI Agents จากต่างค่าย (Interoperability) สามารถสื่อสาร สั่งงาน และทำงานร่วมกันได้

### ข้อมูลทางเทคนิค
- **Crate:** `a2a-rs`
- **จุดเด่น:**
    - **Capability Discovery:** ค้นหาว่า Agent อื่นทำอะไรได้บ้างผ่าน `agent.json`
    - **Task Lifecycle:** มาตรฐานการจัดการสถานะงาน (Submit -> Status -> Stream -> Pull)
    - **Agnostic:** ไม่ยึดติดกับ Model หรือผู้ให้บริการรายใดรายหนึ่ง

### โอกาสในการนำมาใช้กับโปรเจกต์นี้
- **Agent Orchestration:** ทำให้ `jules` สามารถรับคำสั่งจาก Agent ตัวอื่นได้โดยตรง เช่น ให้ `metaswarm` (หรือ Agent ที่เก่งเรื่องสถาปัตยกรรม) วิเคราะห์ปัญหาแล้วส่งงานผ่าน `a2a` มาให้ `jules` (ที่เป็น Coding Worker) ลงมือแก้โค้ดใน Cloud VM
- **Status Streaming:** ใช้มาตรฐาน A2A ในการดึงสถานะจาก `jules remote list --session` มาแสดงผลแบบ Real-time ในระบบของ Agent อื่นๆ

---

## 3. สรุปแนวทางการพัฒนา (Strategic Roadmap)

1. **ระยะสั้น (Optimization):** เพิ่ม `acp` เข้าไปใน MCP Server เพื่อทำ Context Indexing ก่อนส่งงานให้ `jules`
2. **ระยะกลาง (Standardization):** ปรับจูน API ของเราให้รองรับ `A2A Message Format` เพื่อเตรียมความพร้อมสำหรับทีม Agent (Multi-Agent Team)
3. **ระยะยาว (Integration):** สร้าง `Jules Capability Manifest` เพื่อให้ Agent ตัวอื่นๆ ในระบบนิเวศของ Rust ค้นพบและใช้งาน `jules` ได้โดยอัตโนมัติ

---
*หมายเหตุ: ข้อมูลนี้อ้างอิงจากการสำรวจ crates.io และ docs.rs เมื่อวันที่ 23 เมษายน 2026*
