# Master Plan: The Next Chapter (Testing & Learning)

นี่คือแผนการดำเนินงานหลักสำหรับ 2 ภารกิจสำคัญที่เราจะทำต่อไปนี้ครับ

---

## 🎯 **ภารกิจที่ 1: ติดตั้ง "Integration Test Suite" (จาก Skill `mcp-testing`)**

**เป้าหมาย:** สร้างระบบทดสอบอัตโนมัติสำหรับ `jules-mcp-server` เพื่อให้มั่นใจว่า Server ทำงานถูกต้อง 100% หลังมีการเปลี่ยนแปลงโค้ด

*   **Phase 1.1 (Setup):** สร้างไฟล์ตั้งต้น
    -   [ ] สร้างไฟล์ `tests/integration_test.py`
    -   [ ] สร้างไฟล์ `tests/run_tests.sh` เพื่อใช้เป็น "ปุ่มเดียว" ในการรันเทสทั้งหมด

*   **Phase 1.2 (Implementation):** พัฒนาสคริปต์ทดสอบ
    -   [ ] นำ Logic จาก `skills/mcp-testing/scripts/mcp_test_client.py` มาดัดแปลงใส่ใน `tests/integration_test.py`
    -   [ ] แก้ไขสคริปต์ให้ชี้เป้ามาที่ `jules-mcp-server` ของเรา
    -   [ ] เขียน Test Case เพื่อยืนยันว่า:
        -   Server สามารถสตาร์ทได้
        -   สามารถเรียก Tool `run-jules-command` (หรือ tool หลักอื่นๆ) ได้
        -   ได้รับ Response ที่ถูกต้องกลับมา

*   **Phase 1.3 (Execution):** ทำให้รันได้จริง
    -   [ ] แก้ไข `tests/run_tests.sh` ให้รันคำสั่ง `cargo test` (Unit Test) และ `python3 tests/integration_test.py` (Integration Test) ต่อเนื่องกัน

*   **Phase 1.4 (Documentation):** อัปเดตคู่มือ
    -   [ ] เพิ่มส่วน "Running Integration Tests" เข้าไปใน `README.md`

---

## 🧠 **ภารกิจที่ 2: เปิดใช้งาน "Agent Learning Loop"**

**เป้าหมาย:** ทำให้ Agent (Jules, Hermes) สามารถ "เรียนรู้" และ "ตกผลึก" บทเรียนจากการทำงานได้ด้วยตัวเอง

*   **Phase 2.1 (Core Skill):** สร้าง "แกนการเรียนรู้"
    -   [ ] สร้าง Wrapper Script ที่รับผิดชอบการรัน `omp skill learn`
    -   [ ] ให้สคริปต์นำผลลัพธ์ที่ได้ (Problem, Lesson, Trigger, Checklist) มาจัดเก็บเป็นไฟล์ Markdown ที่มีโครงสร้างชัดเจน

*   **Phase 2.2 (Trigger Mechanism):** สร้าง "กลไกการเรียนรู้"
    -   [ ] แก้ไข `gh-jules` CLI เพื่อเพิ่ม Logic ในการเรียกใช้ Wrapper Script นี้
    -   **แนวทาง:** หลังจากที่ Task ที่สั่งให้ Agent ทำงานเสร็จสิ้น (เช่น Hermes สแกน Slop เสร็จ) ระบบจะถามผู้ใช้ว่า "ต้องการบันทึกบทเรียนจากงานนี้หรือไม่?"

*   **Phase 2.3 (Memory Storage):** สร้าง "คลังความรู้"
    -   [ ] ออกแบบและสร้างโครงสร้างโฟลเดอร์ `shared_memory/lessons/`
    -   [ ] สร้างไฟล์ Index (`index.md`) สำหรับค้นหาและอ้างอิงบทเรียนที่เคยบันทึกไว้ได้

---

ผมได้วางแผนงานทั้งหมดเรียบร้อยแล้วครับ **อาจารย์อนุมัติแผนนี้หรือไม่ครับ?** ถ้าอนุมัติ ผมจะเริ่มลงมือทำ **Phase 1.1** (สร้างไฟล์ Test) เป็นลำดับแรกครับ
