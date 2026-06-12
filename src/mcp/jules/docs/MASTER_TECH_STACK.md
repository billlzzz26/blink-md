# ยุทธศาสตร์การเลือกใช้เทคโนโลยี (Master Tech Stack Strategy) - REVISED
<!-- PRIORITY:0 -->
> **สถาปนิก:** ผู้ใช้ (Architect / Non-Coder)
> **หลักการ:** เลือกเครื่องมือตาม "หน้าที่" และ "สภาพแวดล้อม" เพื่อลดภาระและเพิ่มความแม่นยำ

---

## 🏗️ 1. กฎ 3 ภาษา (Triple-Language Usage)
<!-- PRIORITY:0 -->

### 🦀 Rust (The Local Power)
- **เมื่อไหร่:** งาน Local ทั้งหมด, Desktop App, CLI, Library, Engine, งานจัดการ Memory, Binary, และงานที่เน้น Performance
- **ทำไม:** พังยาก AI เขียนได้แม่นยำ และรันได้เสถียรที่สุดในเครื่อง

### 🐍 Python (The Data & Server Backbone)
- **เมื่อไหร่:** Backend, Server, **Modal.com (Serverless)**, Notebook, Embedding, Vector, Data Processing, และสคริปต์ทั่วไป
- **ทำไม:** ยืดหยุ่นที่สุดสำหรับงานประมวลผลข้อมูลและงานบน Cloud/Server

### 🔷 TypeScript / Node.js (The UI & Vercel Wrapper)
- **เมื่อไหร่:** งาน UI ทั้งหมด (Next.js, Vite, Shadcn), ส่วนต่อผสาน (Interface), ทำ Wrapper จาก Rust Engine/Binary, ProtoBuf, และ AI SDK ต่างๆ
- **หมายเหตุ:** ใช้เพราะความจำเป็นเรื่อง **Vercel Deployment** และ UI Ecosystem เท่านั้น (สถาปนิกไม่ชอบความเทอะทะของ node_modules และการลบยากบน Windows)

---

## 🛡️ 2. กฎการควบคุม (Control Protocols)
<!-- PRIORITY:0 -->

### 🗺️ Conductor Protocol
- **ลำดับ:** ต้องเรียก `conductor setup` ก่อนเสมอ เพื่อตรวจสอบไดเรกทอรี
- **เงื่อนไข:** หากมีโฟลเดอร์ `conductor/` อยู่แล้ว ให้หยุดและแจ้งผู้ใช้ให้ใช้ `conductor implement` ห้ามข้ามไปสร้าง Track ใหม่โดยพลการ

### 📦 Track Management
- **ห้ามลบโดยไม่ถาม:** เมื่อจบงาน ต้องถามผู้ใช้ทุกครั้งว่าจะ **"ลบ (Delete)"** หรือ **"เก็บถาวร (Archive)"**
- **Indexing:** ต้องมีระบบ Index ที่ชัดเจนใน `conductor/tracks.md`

### 🌀 Unified Bridge (Beyond heavy CLIs)
- หากเครื่องมือมาตรฐาน (เช่น jules cli) รันไม่ได้ในบางสภาพแวดล้อม (เช่น Termux) ให้เขียนสะพานเชื่อม (Bridge/CLI) ใหม่ที่เบาและรันได้ทุกที่ผ่าน API Key แทน
