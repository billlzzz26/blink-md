# 🏗️ สถาปัตยกรรมระบบ: Agent OS (Architecture Overview)
<!-- PRIORITY:0 -->

เอกสารฉบับนี้สรุปโครงสร้างของระบบ **Agent Command Center** เพื่อให้สถาปนิก (Architect) เข้าใจภาพรวมและการทำงานร่วมกันของทุกส่วนประกอบ

---

## 🌀 1. หัวใจหลัก (The Core)
- **Rust Backbone (`jules-mcp-server`):** ทำหน้าที่เป็นสะพานเชื่อมหลัก (Bridge) ระหว่างโลกของ MCP และเครื่องมือ CLI (Jules/Hermes) เน้นความเสถียรและ Zero Warnings
- **Bash Hooks (`.gemini/hooks/`):** "ยามเฝ้าประตู" ระดับสติปัญญา (v3.0) ที่คอยคัดกรองพฤติกรรมและควบคุมทิศทางการทำงานของ Agent ให้แม่นยำตามกฎเหล็ก

## 🤖 2. กองทัพ Agent (The Agents)
- **Jules (Google):** เน้นงานวิศวกรรมโค้ดที่ซับซ้อน (Rust/TS) และการวางแผนเชิงโครงสร้าง
- **Hermes (Nous Research):** เน้นงานวิจัย, งาน Background (Cron), และการสร้างความฉลาดใหม่ (Autonomous Skills)
- **Multi-Agent Synergy:** ทั้งสองทำงานร่วมกันผ่าน `shared_memory/` โดยมีโปรโตคอล JSON เป็นตัวแลกเปลี่ยนข้อมูล

## 📊 3. ระบบควบคุมและเฝ้าระวัง (Control & Monitoring)
- **Unified CLI (`gh jules`):** คำสั่งศูนย์กลางที่รวบรวมทุกความสามารถไว้ในที่เดียว
- **Python Dashboard (`scripts/agent_dashboard.py`):** ส่วนแสดงผลสถานะแบบ Real-time ดึงข้อมูลข้ามภาษามาสรุปให้เห็นภาพรวม
- **Memory Guard:** ระบบเฝ้าระวัง Context Overflow ที่จะสร้าง "Pruning Proposal" อัตโนมัติเมื่อความจำเริ่มล้น

## 🛡️ 4. ระบบกู้ภัยและวิวัฒนาการ (Self-Healing & Evolution)
- **Self-Healing Skill:** Hermes มีความสามารถในการ "ซ่อมแซมระบบ" เองเมื่อตรวจพบว่า Gateway หรือ Cron มีปัญหา
- **Autonomous Learning:** ระบบสามารถสร้าง Skill ใหม่ๆ เพิ่มเติมได้เองผ่านการสั่งงานระดับนโยบาย (Natural Language)

---

## 🚀 ทิศทางในอนาคต (The Roadmap)
1.  **Distributed Agents:** ขยายขอบเขตให้ Agent สามารถรันข้ามเครื่อง (Cloud/Local) ได้อย่างไร้รอยต่อ
2.  **Advanced Research Pipeline:** ใช้ Python ในการทำ RAG (Retrieval-Augmented Generation) ระดับลึกเพื่อวิจัยหัวข้อใหม่ๆ
3.  **UI Extension:** พัฒนาหน้า Dashboard จาก CLI ไปสู่ Web-based หรือ Mobile UI ที่ซับซ้อนขึ้น

*สถาปัตยกรรมนี้ถูกสร้างขึ้นเพื่อรองรับ "ความคิด" ของสถาปนิก มากกว่า "งาน" ของโปรแกรมเมอร์*
