# Jules CLI Specification

Jules CLI เป็นเครื่องมือสำหรับใช้งาน Google's asynchronous AI coding agent ที่ช่วยในการเขียนโค้ด แก้ไขบั๊ก และเพิ่มฟีเจอร์ต่างๆ ผ่านระบบ Cloud VM โดยไม่ต้องทำในเครื่องตัวเอง (Remote Task Delegation)

## Core Commands

### 1. Authentication
ใช้สำหรับเข้าสู่ระบบด้วย Google Account
```bash
jules login
```

### 2. Remote Task Management
เป็นกลุ่มคำสั่งหลักที่ใช้ส่งงานให้ Jules ทำงานจากระยะไกล

#### Start a New Task
สร้างเซสชันใหม่เพื่อสั่งให้ Jules ทำงานใน Repository ที่กำหนด
```bash
jules remote new --repo <repo_name> --session "<task_description>"
```
- `--repo`: ชื่อ GitHub repository (เช่น `google/jules` หรือ `.` สำหรับ repo ปัจจุบัน)
- `--session`: รายละเอียดงานที่ต้องการให้ทำ (Prompt)

#### List Repositories
แสดงรายชื่อ repository ที่เชื่อมต่อกับบัญชี Jules
```bash
jules remote list --repo
```

#### List Sessions
แสดงรายการเซสชัน (ทั้งที่กำลังทำและเสร็จแล้ว) เพื่อตรวจสอบสถานะ
```bash
jules remote list --session
```

#### Pull Changes
ดึงโค้ดที่ Jules แก้ไขเสร็จแล้วลงมาที่เครื่อง Local
```bash
jules remote pull --session <session_id>
```

## Related Features
- **Asynchronous Execution**: Jules จะสร้าง VM ขึ้นมาทำงานแยกต่างหาก ทำให้เราทำงานอื่นต่อได้เลย
- **Parallel Sessions**: สามารถรันงานหลายๆ อย่างพร้อมกันได้โดยใช้ flag `--parallel <number>`
- **Integration**: สามารถใช้ร่วมกับ `gh cli` เพื่อส่ง Issue เข้าไปให้ Jules ทำงานได้โดยตรง

## Installation & Setup
- **Install**: `npm install -g @google/jules`
- **Setup**: ต้องเชื่อมต่อ GitHub repository ผ่านหน้าเว็บ [jules.google.com](https://jules.google.com) ก่อนใช้งานคำสั่ง remote
