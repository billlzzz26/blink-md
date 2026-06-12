# 📝 บันทึกการเปลี่ยนแปลง (Changelog)
<!-- PRIORITY:1 -->

การเปลี่ยนแปลงทั้งหมดในโปรเจกต์ **jules-mcp-server** จะถูกบันทึกไว้ที่นี่

## [0.2.1] - 2024-04-24
### ✨ เพิ่มฟีเจอร์ (Added)
- **Agent OS Infrastructure:** ยกระดับจาก MCP Server เป็นระบบปฏิบัติการ AI
- **Self-Healing Skill:** Hermes สามารถสร้างและรันสกิล `maintenance` เพื่อซ่อมแซมระบบได้เอง
- **Deep Health Check:** เพิ่มคำสั่ง `gh jules doctor` (Python-based)
- **Stability Guarantee:** ปรับจูน Hooks จนผ่านการทดสอบ 100% (7/7 Pass)
- **Architecture Blueprint:** เอกสารผังสถาปัตยกรรมระบบอย่างเป็นทางการ

## [0.2.0] - 2024-04-24
### ✨ เพิ่มฟีเจอร์ (Added)
- **Hermes Integration:** บูรณาการ Hermes Agent เข้ากับ MCP Server (Rust)
- **Unified CLI (gh-jules v2.0):** รวมคำสั่ง Jules และ Hermes ไว้ที่เดียว
- **Auto-Conductor:** ระบบสร้าง Track และ Plan อัตโนมัติเมื่อเริ่มงานใหม่
- **Shared Memory:** โครงสร้างพื้นที่กลางสำหรับแลกเปลี่ยนข้อมูลข้าม Agent
- **Unified Dashboard:** หน้าสรุปสถานะรวมผ่านคำสั่ง `gh jules status`

## [0.1.0] - 2024-04-24
### ✨ เพิ่มฟีเจอร์ (Added)
- **Rust Backbone:** พัฒนา Core MCP Server ด้วย Rust (Memory Safe)
- **Intelligence Hooks (v3.0):** ระบบเฝ้าประตูด้วย Constriction Protocol
- **Memory Protocol:** มาตรฐานการจัดการความจำและ Staging Review
- **1-Click Setup:** สคริปต์ติดตั้งสภาพแวดล้อมอัตโนมัติ (sh/ps1)
- **Zero Warnings:** เริ่มต้นโปรเจกต์ด้วยมาตรฐานโค้ดสะอาดสูงสุด

---
*บันทึกโดย Gemini CLI เพื่อสถาปนิก (Architect)*
