# Plan: CLI Unification & Conductor Automation

เป้าหมาย: อัปเกรด `gh-jules` ให้เป็นเครื่องมือเดียวที่คุมได้ทุก Agent และบันทึกงานเข้า Conductor โดยอัตโนมัติ

## Phase 1: Hermes Support (DONE)
- [x] เพิ่มคำสั่ง `gh jules hermes "<task>"` เพื่อสั่งงานผ่าน Hermes CLI
- [x] ทดสอบการรันและ capture ผลลัพธ์สำเร็จ

## Phase 2: Conductor Integration (DONE)
- [x] ทำให้ `gh jules new` สร้าง Folder และ `plan.md` ใน `conductor/tracks/` ให้อัตโนมัติ
- [x] ระบบ Auto-registration ใน `tracks.md` (Hermes ทำสำเร็จด้วยตัวเอง!)

## Phase 3: Cross-Agent Sync & Dashboard (DONE)
- [x] สร้างโครงสร้าง `shared_memory/` สำหรับแลกเปลี่ยนข้อมูลระหว่าง Agent
- [x] พัฒนา `scripts/agent_dashboard.py` เพื่อสรุปสถานะรวมของทุก Agent (Python)
- [x] บูรณาการ Dashboard เข้ากับคำสั่ง `gh jules status` (v2.1)
- [x] ทดสอบการส่งงานต่อ (Hand-off) ระหว่าง Jules และ Hermes (สำเร็จผ่าน JSON protocol)
