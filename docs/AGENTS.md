# AGENTS.md (Single Source of Truth)

## 1. PROJECT WORKFLOW & PROTOCOLS
- **Step 1: Define (SPEC.md)** - ระบุสิ่งที่จะทำและเหตุผล <!-- Imported from: SPEC.md -->
- **Step 2: Design (PLAN.md)** - เขียนขั้นตอนและงานที่จะทำเป็น Phase <!-- Imported from: PLAN.md -->
- **Step 3: Implement (TODO.md)** - เช็คลิสงานที่ต้องทำอย่างละเอียดก่อนเริ่มงาน <!-- Imported from: TODO.md -->
- **Step 4: Design System (DESIGN.md)** - มาตรฐาน UI และ Token <!-- Imported from: DESIGN.md -->

## 2. CORE PRINCIPLES & CONTINUITY (META-RULES)
- **Merge, Never Overwrite (ผสาน ห้ามทำลาย)**: เมื่อผู้ใช้เสนอแนะแนวทางใหม่ หรือชี้ข้อผิดพลาด ห้ามลบโครงสร้างเดิม (เช่น TODO.md, PLAN.md) ทิ้งเพื่อเริ่มใหม่ทั้งหมด ต้องใช้วิธีการผสานความต้องการใหม่เข้ากับรากฐานเดิมเพื่อรักษาประวัติการทำงาน (Historical Continuity)
- **Context Before Execution (สร้างกฎก่อนลงมือ)**: เมื่อได้รับคำตักเตือนเรื่องพฤติกรรม ต้องหยุดเพื่อนำบทเรียนที่ได้ไปฝังเป็น "กฎการทำงาน" (Protocol) ในไฟล์ Context หลัก (เช่น AGENTS.md) ให้เสร็จสมบูรณ์ก่อนเสนอตัวรับงาน
- **Anti-Panic Protocol (ห้ามลนลานทำลายงานเก่า)**: ห้ามทำลายงานเก่าเพื่อเอาใจผู้ใช้ การแก้ปัญหาต้องทำผ่านการวิเคราะห์อย่างมีเหตุผล (Structural Refinement) ไม่ใช่การตอบสนองแบบตื่นตระหนก (Reactive Destruction)
- **Lexical Precision**: ในการจัดการ Universal IR ต้องเคารพความแตกต่างของแต่ละแพลตฟอร์มอย่างเคร่งครัด (เช่น Lark 48 types vs Notion H1-H3) โดยเป้าหมายคือการแปลงที่สูญเสียข้อมูลเป็นศูนย์ (Lossless) และไม่ต้องพึ่งพามนุษย์ในการตรวจสอบซ้ำ
- **Skill Compliance**: ต้องอ่านและปฏิบัติตามคู่มือของ Skill ที่เกี่ยวข้องอย่างเคร่งครัด (เช่น การใช้ `fileKey` ใน `figma-generate-diagram` หรือการใช้ Hybrid Workflow) ห้ามเพิกเฉยต่อกระบวนการที่ออกแบบมาแล้ว

## 3. AGENT BEHAVIOR MANDATES
- Agent Role: ดำเนินการตามแผนด้วยความแม่นยำสูง (High Fidelity)
- ห้ามขอโทษในทุกกรณี เมื่อทำผิดต้องลงมือแก้ไขเชิงระบบ
- ห้ามใช้ตัวหนา ในการจัดรูปแบบทุกพื้นที่
- Header ต้องเป็นภาษาอังกฤษล้วน สั้น และกระชับ

## 4. COMMIT MESSAGES & LABELS
- Commit message และ PR title ต้องใช้รูปแบบ Conventional Commits: `type(scope): summary` เช่น `feat(cli): add --sort flag`, `fix(api): handle empty search results`, `docs: update README roadmap`
- Types ต้องตรงกับ Type of Change ใน `.github/PULL_REQUEST_TEMPLATE.md`: `feat` (New feature), `fix` (Bug fix), `perf` (Performance optimization), `refactor` (Refactoring), `docs` (Documentation update), `test`, `ci`, `chore`
- ป้ายทั้งหมดใช้ชื่อสั้นตรงกับ type/scope ข้างต้น (`docs` ไม่ใช่ `documentation`, `deps` ไม่ใช่ `dependencies`, `test` ไม่ใช่ `tests`) เพื่อให้ป้ายกับคำนำหน้า commit เป็นคำเดียวกัน
- `.github/labels.yml` ติดป้ายตามพื้นที่ที่เปลี่ยนโดยอัตโนมัติ (`docs`, `ci`, `rust`, `test`, `deps`, `scripts`, `tooling`) — path glob บอกความหมายเชิง feature/fix/refactor ไม่ได้
- ป้ายเชิงความหมาย (`feat`/`fix`/`refactor`/`perf`/`chore`) ต้องติดเองให้ตรงกับ Conventional Commit type ที่ใช้ในหัวข้อ PR อย่ารอให้ผู้ใช้ติดแทน และอย่าติดป้ายที่ไม่ตรงกับสิ่งที่เปลี่ยนจริง
- สีและคำอธิบายของทุกป้าย (ทั้งเชิงพื้นที่และเชิงความหมาย) กำหนดไว้ที่เดียวใน `.github/label-definitions.yml` ใช้กลุ่มสีแยกตามหมวด: แดง = ด่วน/อันตราย (`fix`), ขาว = เอกสาร (`docs`), ม่วงเข้ม = CI/CD (`ci`), ที่เหลือแยกสีตามหมวดของตัวเอง — `.github/workflows/sync-labels.yml` เป็นตัว sync ชื่อ/สี/คำอธิบายจริงเข้า repo
