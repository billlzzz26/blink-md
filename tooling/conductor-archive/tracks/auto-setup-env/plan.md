# Plan: Auto-Setup Environment (1-Click Installer)

เป้าหมาย: สร้างระบบติดตั้งสภาพแวดล้อมอัตโนมัติที่รองรับทั้ง Windows และ Termux ตามยุทธศาสตร์ "3 ภาษา 1.5 แชลล์"

## Phase 1: Environment Analysis (DONE)
- [x] วิเคราะห์สคริปต์ `install.sh` และ `install.ps1` เดิม
- [x] ตรวจสอบวิธีการตรวจวัด (Health Check) สำหรับ Rust, TS (Node.js), และ Python ในแต่ละ OS
- [x] ระบุช่องว่าง (Gaps): ขาด Python, Hermes, และ Health Check ระบบรวม

## Phase 2: Implementation (Shell/PS1) (DONE)
- [x] พัฒนาสคริปต์ `setup.sh` (สำเร็จ: รองรับการข้าม Jules บน Termux อย่างสง่างาม)
- [x] พัฒนาสคริปต์ `setup.ps1` (สำเร็จ: คู่แฝดบน Windows)
- [x] เพิ่มขั้นตอน "Final Health Check" เพื่อยืนยันการเชื่อมต่อ Agent ทั้งสอง

## Phase 3: Integration & Testing (DONE)
- [x] บูรณาการเข้ากับคำสั่ง `gh jules setup`
- [x] ทดสอบการรันจริงบน Termux (ผ่านฉลุยใน 2m 57s)
