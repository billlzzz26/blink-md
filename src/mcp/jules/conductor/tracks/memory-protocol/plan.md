# Plan: Interactive Memory Protocol

สร้างมาตรฐานการบันทึกความจำ (Memory) ที่ต้องมีการยืนยันจากผู้ใช้เสมอ

## Phase 1: Global Rule Registration (DONE)
- [x] บันทึกกฎเข้าสู่ `save_memory` agent เพื่อให้มีผลถาวร

## Phase 2: Documentation (DONE)
- [x] สร้างทะเบียน Track ใน `tracks.md`
- [x] สร้างไฟล์ข้อกำหนดในโปรเจกต์ (`docs/MEMORY_RULES.md`)

## Phase 3: Behavioral Testing (DONE)
- [x] ทดสอบ Trigger ด้วยคำว่า "จำข้อมูลนี้ไว้"
- [x] ตรวจสอบการเรียกใช้ `ask_user`
- [x] สรุปผลการทดสอบ: ระบบตอบสนองต่อคำสั่ง "จำ" ได้อย่างถูกต้อง โดยเรียกใช้ `ask_user` เพื่อให้ผู้ใช้เลือก Memory Level (Global/Project) ก่อนบันทึกเสมอ
