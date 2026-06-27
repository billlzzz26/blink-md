# Plan: Hook Intelligence Refactor

ปรับปรุงระบบ Hooks (`boost_check.sh` และ `watchdog.sh`) ให้มีความฉลาดในการวิเคราะห์ Intent และประหยัดทรัพยากร

## Phase 1: Conductor Infrastructure (DONE)
- [x] สร้าง `conductor/tracks.md`
- [x] สร้างโครงสร้างโฟลเดอร์สำหรับ Track นี้

## Phase 2: Refactor boost_check.sh (DONE)
- [x] เปลี่ยนจาก Word Count เป็น **Structural Weighting Score**
- [x] ตรวจจับ Delimiters (`---`, `***`)
- [x] ตรวจจับ XML Tags (`<task>`, `<context>`)
- [x] ตรวจจับ Key-Value pairs (`Goal:`, `Action:`)
- [x] เพิ่ม **Conversational Ignore List** (อืม, โอเค, ลุย)

## Phase 3: Refactor watchdog.sh (DONE)
- [x] ปรับปรุง Loop Detection ให้ดูถึง Tool Arguments
- [x] เพิ่มระบบ Warning เมื่อใกล้ถึงขีดจำกัดเทิร์น (Soft Limit: 25, Hard Limit: 40)

## Phase 4: Constriction Protocol Integration (DONE)
- [x] เพิ่มระบบ **Top-Level Constraint Detection** (Must/Never)
- [x] เพิ่มระบบ **Output Formatting Spec Detection** (#, JSON)
- [x] เพิ่มระบบ **Proactive Skill Hinting** (แนะนำสกิลตามงานอัตโนมัติ)

## Phase 5: Validation (DONE)
- [x] อัปเดต `tests/validate_hooks.sh`
- [x] รันการทดสอบทั้งหมด (PASS 100%)
