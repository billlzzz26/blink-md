# วิจัยเชิงลึก: Hermes Agent ☤
> **ผู้เขียน:** Gemini CLI (Constriction Protocol v3.0)
> **แหล่งข้อมูล:** Source Code อ้างอิงจาก `~/.hermes/hermes-agent/` (Nous Research)

Hermes Agent คือ AI Agent ที่เน้นการ **"เรียนรู้และพัฒนาตัวเอง" (Self-Improving)** ซึ่งถูกออกแบบมาให้ทำงานได้ทุกที่ (Platform Agnostic) และมีความยืดหยุ่นสูงสุดในระดับวิศวกรรม

---

## 🛡️ 5 จุดเด่นเชิงลึก (Deep Dive)

### 1. Closed Learning Loop & Autonomous Skills
Hermes ไม่ได้เป็นเพียงแค่ Bot ที่รับคำสั่งแล้วจบไป แต่มีระบบ **"วงจรการเรียนรู้แบบปิด"**:
- **Skill Creation:** เมื่อ Hermes ทำงานที่ซับซ้อนสำเร็จ มันจะวิเคราะห์ขั้นตอนนั้นและบันทึกเป็น "Skill" ใหม่ลงใน `skills/` เพื่อให้เรียกใช้ได้ในอนาคตโดยไม่ต้องเสีย Context ในการอธิบายใหม่
- **Honcho Dialectic:** ใช้ระบบ Honcho ในการสร้าง Model ของผู้ใช้ (User Profiling) เพื่อจดจำสไตล์การทำงาน ความชอบ และบริบทส่วนตัว ทำให้การตอบสนอง "รู้ใจ" มากขึ้นเรื่อยๆ ตามจำนวนเซสชัน

### 2. Flexible Execution Backends
ความสามารถในการรัน (Run) ที่ไม่ได้จำกัดอยู่แค่ในเครื่องคอมพิวเตอร์:
- **Environment Isolation:** รองรับการทำงานใน **Docker** และ **Daytona** เพื่อแยกสภาพแวดล้อมการทำงานให้ปลอดภัย
- **Serverless Persistence (Modal):** สามารถรันบน Modal ซึ่งจะ "จำศีล" (Hibernate) เมื่อไม่มีงาน และ "ตื่น" (Wake up) ทันทีเมื่อได้รับคำสั่ง ช่วยประหยัดค่าใช้จ่ายได้มหาศาล (Pay-per-use)
- **Termux Support:** ปรับจูนมาเป็นพิเศษสำหรับ Android (Termux) โดยมีการจัดการ Dependencies ที่เหมาะสมกับสถาปัตยกรรมมือถือ

### 3. Natural Language Automations (Cron Scheduler)
Hermes มีระบบ **Cron** ในตัวที่แตกต่างจากระบบสากล:
- **Natural Language Trigger:** ผู้ใช้สามารถสั่งว่า *"ทุกวันจันทร์ตอน 8 โมงเช้า ให้สรุปอีเมลสำคัญให้ฉันหน่อย"* และ Hermes จะแปลคำสั่งนี้เป็นตารางงานอัตโนมัติ
- **Unattended Execution:** งานเหล่านี้จะรันอยู่ใน Background และส่งผลลัพธ์กลับมายังช่องทางที่กำหนด (CLI หรือ Gateway) โดยไม่ต้องเปิดหน้าจอค้างไว้

### 4. Model-Agnostic Infrastructure
ความเป็นอิสระจากผู้ผลิต Model รายใดรายหนึ่ง (No Vendor Lock-in):
- **Fast Switching:** ใช้คำสั่ง `hermes model` เพื่อสลับระหว่าง OpenAI, Anthropic, NVIDIA NIM หรือ Local LLM (ผ่าน Ollama/Llama.cpp) ได้ทันที
- **Standardized Tooling:** ไม่ว่าจะเป็น Model ไหน Hermes จะใช้ชุดเครื่องมือ (Tools) ชุดเดียวกันในการทำงาน ทำให้ผลลัพธ์มีความคงเส้นคงวา

### 5. Research-Ready Trajectories
จุดเด่นที่ทำมาเพื่อนักพัฒนาและนักวิจัย AI โดยเฉพาะ:
- **Trajectory Capture:** ทุกการตัดสินใจและผลลัพธ์ของ Tool จะถูกบันทึกเป็น "Trajectory" (เส้นทางการทำงาน)
- **Model Training Data:** ข้อมูลเหล่านี้สามารถนำไปเข้ากระบวนการ `trajectory_compressor.py` เพื่อใช้เป็นชุดข้อมูลในการเทรน (Fine-tuning) AI Model รุ่นถัดไปให้เก่งการใช้ Tool มากยิ่งขึ้น

---

## 🛠️ การนำมาใช้งาน (Usage Guide)

### 1. การติดตั้งในระบบ Termux (Android)
```bash
# รัน Installer (แนะนำ)
curl -fsSL https://raw.githubusercontent.com/NousResearch/hermes-agent/main/scripts/install.sh | bash

# หลังจากติดตั้งให้รัน setup
hermes setup
```

### 2. คำสั่งหลักที่ควรรู้
- `hermes` : เข้าสู่โหมด Chat (TUI)
- `hermes model` : ตั้งค่า Provider และ Model
- `hermes tools` : เปิด/ปิด เครื่องมือที่จะให้ Agent ใช้
- `hermes doctor` : ตรวจสอบสภาพแวดล้อมว่ามีอะไรพังหรือไม่

### 3. การเพิ่มความสามารถด้วย MCP
Hermes รองรับ **Model Context Protocol (MCP)** เต็มรูปแบบ:
- สามารถเชื่อมต่อ MCP Server อื่นๆ (รวมถึง `jules-mcp-server`) ได้ผ่านการตั้งค่าใน `cli-config.yaml` หรือผ่านคำสั่ง `hermes config set`

---

## 💡 สรุปมุมมองสำหรับโปรเจกต์เรา
การนำ Hermes มาใช้ร่วมกับโปรเจกต์นี้ จะช่วยให้เรามี **"แรงงาน AI"** ที่สามารถรันงาน Background ระยะยาว (Cron) และสามารถสร้าง Skill ใหม่ๆ ที่เฉพาะเจาะจงกับงานของอาจารย์ได้เองในระยะยาวครับ
