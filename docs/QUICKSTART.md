# ⚡️ blink-md Quick Start Guide

[English](#english) | [ภาษาไทย](#thai)

---

<a name="english"></a>
## 🇺🇸 English

### 1. Installation
Ensure you have Rust installed. Clone the repository and build:
```bash
git clone https://github.com/billlzzz26/blink-md.git
cd blink-md
cargo build --release
```
The binary will be available at `./target/release/blink-md`.

### 2. Configure Credentials
**blink-md** prioritizes security. You MUST set your Notion API token as an environment variable:
```bash
export NOTION_TOKEN=your_secret_token_here
```

### 3. Common Commands
- **Search**: `blink-md search "document name"`
- **TUI (Interactive Mode)**: `blink-md tui`
- **Sync Folder**: `blink-md sync --dir ./docs --notion-db DATABASE_ID`
- **Convert File**: `blink-md convert --input file.md --output out.json --from markdown --to notion`

### 4. TUI Navigation
- `[q]` or `[Esc]`: Quit
- `[Tab]`: Switch between Users, Pages, Blocks, and Databases
- `[j/k]` or `[Arrow Keys]`: Move selection up/down
- `[Enter]`: Select item or expand/collapse block
- `⟳`: Loading indicator in title bar

### 5. AI Integration (MCP)
Start the MCP server to let AI agents interact with your Notion:
```bash
blink-md mcp-serve
```

---

<a name="thai"></a>
## 🇹🇭 ภาษาไทย

### 1. การติดตั้ง
ตรวจสอบว่าคุณได้ติดตั้ง Rust เรียบร้อยแล้ว จากนั้นทำการ Clone โปรเจกต์และคอมไพล์:
```bash
git clone https://github.com/billlzzz26/blink-md.git
cd blink-md
cargo build --release
```
คุณจะพบไฟล์โปรแกรมที่ `./target/release/blink-md`

### 2. ตั้งค่าการยืนยันตัวตน
**blink-md** ให้ความสำคัญกับความปลอดภัยระดับสูง คุณ "ต้อง" ตั้งค่า Notion API token ผ่าน Environment Variable เท่านั้น:
```bash
export NOTION_TOKEN=โค้ด_secret_ของคุณ
```

### 3. คำสั่งที่ใช้บ่อย
- **ค้นหา**: `blink-md search "ชื่อเอกสาร"`
- **TUI (โหมดโต้ตอบ)**: `blink-md tui`
- **ซิงค์โฟลเดอร์**: `blink-md sync --dir ./docs --notion-db ID_ของ_DATABASE`
- **แปลงไฟล์**: `blink-md convert --input file.md --output out.json --from markdown --to notion`

### 4. การใช้งาน TUI
- `[q]` หรือ `[Esc]`: ออกจากโปรแกรม
- `[Tab]`: สลับหน้า (Users, Pages, Blocks, Databases)
- `[j/k]` หรือ `[ปุ่มลูกศร]`: เลื่อนขึ้น/ลง
- `[Enter]`: เลือกรายการ หรือ ยืด/หด รายการบล็อก
- `⟳`: สัญลักษณ์กำลังโหลดข้อมูล (Loading) จะปรากฏที่แถบ Title

### 5. การเชื่อมต่อกับ AI (MCP)
รัน MCP Server เพื่อให้ AI เอเจนต์สามารถอ่านและจัดการ Notion ของคุณได้:
```bash
blink-md mcp-serve
```
