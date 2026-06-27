# รายการคลังเทคโนโลยี (Tech Stack Inventory)
<!-- PRIORITY:0 -->
> **โปรเจกต์:** jules-mcp-server
> **สถานะ:** ใช้งานจริง (Active)

สแตคเทคโนโลยี (Tech Stack) ของโปรเจกต์นี้ถูกคัดสรรมาเพื่อความปลอดภัย (Safety) และความรวดเร็ว (Speed) ในสภาพแวดล้อม Termux/Android โดยมีรายละเอียดดังนี้:

---

## 🏗️ 1. Core Development Stack (ระดับโปรแกรมหลัก)
<!-- PRIORITY:0 -->
- **Language:** **Rust (Edition 2021)**
    - *บทบาท:* ใช้เขียนตรรกะหลักของโปรแกรม เพื่อความปลอดภัยของหน่วยความจำสูงสุด
- **AI Interface:** **pmcp (v1)**
    - *บทบาท:* ใช้จัดการโปรโตคอล Model Context Protocol (MCP) เพื่อคุยกับ AI Agents
- **Async Runtime:** **Tokio (v1)**
    - *บทบาท:* ใช้จัดการการทำงานหลายอย่างพร้อมกัน (Non-blocking I/O) เช่น การอ่านไฟล์และคุยกับ API พร้อมกัน
- **Serialization:** **Serde & Serde_JSON (v1)**
    - *บทบาท:* ใช้แปลงข้อมูลระหว่าง JSON (ที่ AI คุย) และตัวแปรใน Rust (ที่โปรแกรมเข้าใจ)
- **Error Handling:** **Anyhow (v1)**
    - *บทบาท:* ใช้จัดการและรายงานข้อผิดพลาดอย่างละเอียด

## 🛡️ 2. Tooling & Control Stack (ระดับเครื่องมือควบคุม)
<!-- PRIORITY:1 -->
- **Automation:** **Bash (Shell Scripts)**
    - *ไฟล์:* `.gemini/hooks/boost_check.sh`, `.gemini/hooks/watchdog.sh`
    - *บทบาท:* ใช้ทำ Intelligence Hook เพื่อตรวจสอบและเพิ่มความฉลาดให้คำสั่งก่อนทำงานจริง
- **Test Framework:** **Custom Bash Test (`tests/validate_hooks.sh`)**
    - *บทบาท:* ใช้ทดสอบตรรกะของสคริปต์ Hook เพื่อให้แน่ใจว่าทำงานได้ถูกต้อง 100%

## 🗺️ 3. Workflow & Knowledge Stack (ระดับการจัดการงาน)
<!-- PRIORITY:1 -->
- **Project Planning:** **Conductor Protocol**
    - *บทบาท:* ใช้ Markdown (`.md`) ในการวางแผนงาน (Tracks) และเก็บประวัติสถานะของโปรเจกต์
- **Documentation:** **Markdown Standard**
    - *บทบาท:* ใช้สื่อสารระหว่าง Agent และผู้ใช้ เพื่อความง่ายในการอ่านและแก้ไข

## 🐍 4. Python & Advanced Analytics (ระดับวิจัยและวิเคราะห์)
<!-- PRIORITY:1 -->
- **Scripts:**
    - `scripts/agent_dashboard.py`: ระบบสรุปสถานะรวม (Unified Dashboard)
    - `scripts/agent_doctor.py`: ระบบตรวจสุขภาพเชิงลึก (Deep Health Check)
    - `scripts/cron_reporter.py`: ระบบรายงานผลงานอัตโนมัติ (Cron Reporter)
- **บทบาท:** ใช้ประมวลผลข้อมูลที่ซับซ้อนข้ามภาษา และสร้างรายงานแบบ High-fidelity

## 🌀 5. Unified Command Stack (ระดับศูนย์บัญชาการ)
<!-- PRIORITY:1 -->
- **Extension:** **gh-jules (v2.1)**
    - *บทบาท:* รวมศูนย์คำสั่ง Jules และ Hermes ไว้ภายใต้ GitHub CLI เพื่อการควบคุมที่ง่ายที่สุด

---

## 💡 สรุปสถานะระบบ
โปรเจกต์ของอาจารย์ใช้ **"Triple-Language Core"**:
- **Rust** ทำหน้าที่เป็น "กล้ามเนื้อและสมองส่วนลึก" (Heavy Lifting & Protocol)
- **Python** ทำหน้าที่เป็น "ศูนย์วิเคราะห์และรายงาน" (Analytics & Reporting Layer)
- **Bash** ทำหน้าที่เป็น "ดวงตาและสัญชาตญาณ" (Monitoring & Intelligence Layer)

*เอกสารฉบับนี้ถือเป็นกฎระดับ Priority 0 เพื่อให้อาจารย์ตรวจสอบได้ตลอดเวลาครับ*
