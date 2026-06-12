# Plan: Agent Learning Loop
*Status: On Hold (Awaiting Architect's Go-Signal)*

**เป้าหมาย:** บูรณาการ Skill `omp skill learn` เข้าไปใน Core Logic ของ Jules และ Hermes เพื่อสร้างวงจรการเรียนรู้อัตโนมัติ

## Phase 1: Core Integration
- [ ] พัฒนา Wrapper script สำหรับ `omp skill learn`
- [ ] สร้าง Trigger Condition ใน `gh jules` และ `hermes` ให้เรียกใช้ script นี้หลังจบภารกิจ

## Phase 2: Memory Storage
- [ ] ออกแบบโครงสร้างการจัดเก็บบทเรียนใน `shared_memory/lessons/`
- [ ] สร้างระบบ Indexing เพื่อให้สามารถค้นหาบทเรียนเก่าได้
