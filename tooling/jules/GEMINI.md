# Jules Project Context
<!-- PRIORITY:0 -->

## Project Architecture & Standards
<!-- PRIORITY:1 -->
- **Tech Stack**: ใช้ 'Triple-Language Core' (Rust สำหรับ backbone, TypeScript สำหรับ logic, Python สำหรับ analytics/research) และ '1.5 Shells' (PowerShell สำหรับ Windows, Bash สำหรับ Termux)
- **Planning Standard**: โปรเจกต์นี้ใช้ **Conductor Protocol** สำหรับการวางแผน และ **Constriction Protocol v3.0** สำหรับการจัดการ hooks
- **Track Registry**: [conductor/tracks.md](conductor/tracks.md)

## Core User Rules
<!-- PRIORITY:0 -->
- **User Role**: ผู้ใช้คือ **'Human Architect' (Non-coder)** เป็นผู้ออกแบบโครงสร้างพื้นฐาน และ Agent เป็นผู้ลงมือทำ (Execute) ด้วยความแม่นยำสูง
- **Memory Safety**: ต้องใช้ **'Staging Review Protocol'** ([docs/STAGING_REVIEW_PROTOCOL.md](docs/STAGING_REVIEW_PROTOCOL.md)) ก่อนทำการลบ (Pruning) หรือสรุป (Summarizing) ข้อมูลในโปรเจกต์เสมอ
- **Language**: สื่อสารและสรุปผลเป็นภาษาไทยเสมอ
- **Effort**: ผู้ใช้จะพยายามเตรียมทุกอย่างให้พร้อมเสมอ (尊重/Respect user's effort)

## Unified Control Commands
<!-- PRIORITY:1 -->
- **System Health**: `gh jules doctor` (Run diagnostics)
- **Status Overview**: `gh jules status` (Real-time dashboard)
- **Hermes Task**: `gh jules hermes "<task>"` (Autonomous background work)
- **New Task**: `gh jules new "<task>"` (Start Conductor track)

## Subdirectory Contexts
- [docs/](docs/) — Research, specifications, and memory rules.
- [src/](src/) — Main logic and implementation.
