# SPECIFICATION: High-Fidelity Universal IR & Cross-Platform Sync

## 1. PROJECT OBJECTIVE
สร้างระบบจัดการและแปลงเอกสารแบบ platform-agnostic ที่รับประกันความถูกต้องของข้อมูล (Lossless) และความสมบูรณ์ของการแสดงผล (Visual Fidelity) 100% โดยผู้ใช้ไม่ต้องตรวจสอบซ้ำ

## 2. CORE CONCEPTS

### 2.1 Universal Identity (The "Key")
- ทุกเอกสารไม่ว่าจะมาจาก Notion (Page ID), Google Docs (Doc ID), หรือ Lark (File Token) จะถูกแมพเข้ากับ Universal Key ในระบบ
- ระบบ ID Resolution ต้องสามารถระบุความสัมพันธ์ระหว่างต้นทางและปลายทางได้อย่างแม่นยำเพื่อป้องกันข้อมูลซ้ำซ้อนหรือการอัปเดตผิดพลาด

### 2.2 Universal Intermediate Representation (IR)
- IR Hub ทำหน้าที่เป็น "ศูนย์กลางความจริง" (Single Source of Truth)
- รองรับ Block Types ทั้งหมด (เช่น Lark 48 types) โดยใช้แนวทาง Dynamic Mapping
- บล็อกที่ไม่สามารถแมพได้โดยตรงจะถูกเก็บไว้ในรูปแบบ Raw Data (JSONB) เพื่อรักษาข้อมูลเดิมไว้ครบถ้วน (No Information Loss)

### 2.3 Structural Grammar & Syntax
- การแปลงต้องเคารพ Grammar และ Syntax ของแต่ละแพลตฟอร์มอย่างเคร่งครัด
- Notion: เคารพข้อจำกัด H1-H3 และการจัดเรียงบล็อกแบบลำดับชั้น
- Markdown: ใช้ GitHub Flavored Markdown (GFM) เป็นมาตรฐาน
- UI: แสดงผลผ่าน TUI/Preview โดยอ้างอิงจาก IR ที่ถูก Normalized แล้ว

## 3. TECHNICAL REQUIREMENTS

### 3.1 Persistence Layer (PostgreSQL)
- ใช้ relational schema สำหรับโครงสร้างหลัก (Hierarchy, Ordering)
- ใช้ JSONB สำหรับ Flexible Metadata, Styles, และ Platform-specific attributes
- LexoRank implementation สำหรับลำดับบล็อกที่แม่นยำ

### 3.2 Conversion Logic
- Source Adapter: ทำหน้าที่ดึงข้อมูลและทำ Parse ให้เป็น Universal IR
- Target Emitter: ทำหน้าที่ Generate เอกสารตาม Syntax ของแพลตฟอร์มปลายทาง
- Validation Gate: ตรวจสอบความสมบูรณ์ของข้อมูลหลังการแปลงทุกครั้ง

## 4. INTERFACES
- CLI: สำหรับการสั่งงานแบบ Orchestration
- TUI: สำหรับการจัดการและ Preview ข้อมูลใน IR Store
- MCP: สำหรับการเชื่อมต่อกับ AI Agents เพื่อทำ Agentic Document Manipulation
