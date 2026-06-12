# Plan: Memory Manager (Cron Job)

ออกแบบและสร้างระบบจัดการความจำอัตโนมัติเพื่อป้องกันปัญหา Context Overflow

## Phase 1: Protocol Design & Manual Guardrails (DONE)
- [x] กำหนดมาตรฐานโครงสร้างไฟล์บริบท (`docs/CONTEXT_FILE_PROTOCOL.md`)
- [x] ออกแบบระบบ Tagging (Priority Levels) และใช้งานใน `GEMINI.md`
- [x] ออกแบบระบบ "Staging Review" (`docs/STAGING_REVIEW_PROTOCOL.md`)

## Phase 2: Monitoring & Risk Assessment (DONE)
- [x] เฝ้าสังเกตปริมาณความจำที่เพิ่มขึ้นในเซสชันจริง (Baseline: ~100K)
- [x] ประเมินความเสี่ยง: การลบข้อมูลสำคัญ (แก้ไขโดยใช้ระบบ Proposal แทนการลบจริง)

## Phase 3: Automation with Hermes (DONE)
- [x] พัฒนาสคริปต์ `scripts/memory_guard.sh` สำหรับตรวจวัดและแจ้งเตือน
- [x] ตั้งค่า Hermes Cron Job ให้รันสคริปต์ตรวจสอบทุกวัน (Job ID: f7468ef44042)
- [x] ระบบสร้าง "Pruning Proposal" อัตโนมัติเมื่อความจำเกินเกณฑ์
