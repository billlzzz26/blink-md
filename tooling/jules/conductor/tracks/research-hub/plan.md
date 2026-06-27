# Plan: Multi-Agent Research Hub

เป้าหมาย: สร้างกระบวนการวิจัยอัตโนมัติที่ใช้จุดแข็งของทุก Agent ร่วมกันเพื่อผลิตรายงานคุณภาพสูง

## Phase 1: Research Workflow & Slim Bridge Design
- [ ] พัฒนา `scripts/jules_api_bridge.py` (เรียก API โดยตรง ไม่ใช้ CLI หนัก)
- [ ] ออกแบบโปรโตคอลการวิจัย:
    1. Jules: แมพบริบทในเครื่อง (Local Context)
    2. Hermes: วิจัยข้อมูลภายนอก (External Research / Simulating Search)
    3. Python: วิเคราะห์และสังเคราะห์ข้อมูล (Synthesis)
- [ ] สร้าง Template รายงาน Markdown ที่สวยงาม

## Phase 2: Python Synthesis Engine
- [ ] พัฒนา `scripts/research_synthesizer.py`
- [ ] ระบบดึงข้อมูลจาก `shared_memory/handoff/` มาสร้างเป็นรายงาน

## Phase 3: CLI Integration
- [ ] เพิ่มคำสั่ง `gh jules research "<topic>"`
- [ ] ระบบบันทึกรายงานลงใน `docs/research/` อัตโนมัติ
