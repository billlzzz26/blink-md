# DATABASE SCHEMA DESIGN

## REQUIREMENTS ANALYSIS AND UI COMPLETENESS

โปรเจกต์ `notion-rs` ต้องการ Schema ที่ยืดหยุ่นเพื่อรองรับหน้าเพจแบบลำดับชั้น บล็อกหลายประเภท พร็อพเพอร์ตี้แบบไดนามิก และการโต้ตอบของผู้ใช้

ข้อกำหนดสำคัญเพื่อให้แน่ใจว่าการแปลงข้อมูลแสดงผลบน UI ได้ถูกต้องและสมบูรณ์:
1. Ordering & Sequencing: บล็อกและเพจต้องมีลำดับที่ชัดเจน (`sort_order`) เพื่อให้ UI วาดองค์ประกอบตามลำดับที่ถูกต้อง
2. Database Views: ฐานข้อมูลต้องรองรับมุมมองหลายแบบ (Table, Board, Gallery, Timeline)
3. Rich Format Metadata: สี การจัดหน้า และการตกแต่งข้อความถูกเก็บในฟิลด์ `JSONB` เพื่อรักษาความถูกต้องของการแสดงผล

## ENTITY RELATIONSHIP

```text
+--------------+       +---------------+       +------------------+
|    Users     | 1   N |  Workspaces   | 1   N |    Databases     |
|--------------|-------|---------------|-------|------------------|
| id (PK)      |       | id (PK)       |       | id (PK)          |
| email        |       | name          |       | workspace_id (FK)|
| name         |       | owner_id (FK) |       | title            |
| type         |       +---------------+       | properties_schema|
+--------------+               |               +------------------+
       |                       | 1                   | 1       | 1
       | 1                     |                     |         |
       |                       | N                   | N       | N
       |               +---------------+   +------------------+| +---------------+
       +---------------|    Pages      |---| Page Properties  || |Database Views |
       | 1           N |---------------|1 N|------------------|| |---------------|
       |               | id (PK)       |   | id (PK)          || | id (PK)       |
       |               | workspace_id  |   | page_id (FK)     || | db_id (FK)    |
       |               | parent_id     |   | property_name    || | view_type     |
       |               | sort_order    |   | property_value   || | config(JSONB) |
       |               +---------------+   +------------------+| +---------------+
       |                       | 1
       | 1                     |
       |                       | N
       |               +---------------+
       +---------------|    Blocks     |
                     N |---------------|
                       | id (PK)       |
                       | page_id (FK)  |
                       | parent_id (FK)|
                       | sort_order    |
                       | block_type    |
                       | content(JSONB)|
                       +---------------+
```

## NORMALIZATION APPROACH

ใช้รูปแบบ Hybrid (Relational + JSONB)
- Relational Integrity: โครงสร้างหลัก (ลำดับชั้น ความเป็นเจ้าของ การเรียงลำดับ) ใช้ตารางมาตรฐาน
- UI Preservation: ข้อมูลแบบไดนามิก (`block.content`, `page.properties`, `database.schema`, `view.config`) เก็บแบบ `JSONB` เพื่อป้องกันการสูญหายของข้อมูลการตกแต่งจากฝั่ง Frontend

## TABLE DEFINITIONS

```sql
-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type VARCHAR(20) NOT NULL CHECK (type IN ('person', 'bot')),
    email VARCHAR(255) UNIQUE,
    name VARCHAR(255),
    avatar_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Workspaces
CREATE TABLE workspaces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Databases
CREATE TABLE databases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    parent_page_id UUID,
    title JSONB NOT NULL DEFAULT '[]',
    description JSONB DEFAULT '[]',
    icon JSONB,
    cover JSONB,
    properties_schema JSONB NOT NULL DEFAULT '{}',
    is_inline BOOLEAN DEFAULT FALSE,
    in_trash BOOLEAN DEFAULT FALSE,
    created_by UUID NOT NULL REFERENCES users(id),
    last_edited_by UUID NOT NULL REFERENCES users(id),
    created_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_edited_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Database Views
CREATE TABLE database_views (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    database_id UUID NOT NULL REFERENCES databases(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    view_type VARCHAR(50) NOT NULL,
    config JSONB NOT NULL DEFAULT '{}',
    sort_order DOUBLE PRECISION NOT NULL,
    created_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Pages
CREATE TABLE pages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    parent_type VARCHAR(20) NOT NULL CHECK (parent_type IN ('workspace', 'database', 'page')),
    parent_id UUID NOT NULL,
    sort_order DOUBLE PRECISION NOT NULL DEFAULT 0,

    icon JSONB,
    cover JSONB,
    properties JSONB NOT NULL DEFAULT '{}',
    url TEXT,
    public_url TEXT,
    in_trash BOOLEAN DEFAULT FALSE,

    created_by UUID NOT NULL REFERENCES users(id),
    last_edited_by UUID NOT NULL REFERENCES users(id),
    created_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_edited_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Blocks
CREATE TABLE blocks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    page_id UUID NOT NULL REFERENCES pages(id) ON DELETE CASCADE,
    parent_type VARCHAR(20) NOT NULL CHECK (parent_type IN ('page', 'block')),
    parent_id UUID NOT NULL,

    sort_order DOUBLE PRECISION NOT NULL,
    block_type VARCHAR(50) NOT NULL,
    content JSONB NOT NULL DEFAULT '{}',
    has_children BOOLEAN DEFAULT FALSE,
    in_trash BOOLEAN DEFAULT FALSE,

    created_by UUID NOT NULL REFERENCES users(id),
    last_edited_by UUID NOT NULL REFERENCES users(id),
    created_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_edited_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Comments
CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    discussion_id UUID NOT NULL,
    parent_id UUID NOT NULL,
    parent_type VARCHAR(20) NOT NULL CHECK (parent_type IN ('page', 'block')),
    rich_text JSONB NOT NULL DEFAULT '[]',
    created_by UUID NOT NULL REFERENCES users(id),
    created_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## INDEXES FOR FAST UI RENDERING

```sql
-- Fetching blocks for a page in exact visual order
CREATE INDEX idx_blocks_render ON blocks(page_id, parent_id, sort_order) WHERE in_trash = FALSE;

-- Fetching views for a database
CREATE INDEX idx_db_views_render ON database_views(database_id, sort_order);

-- Pages tree building
CREATE INDEX idx_pages_parent ON pages(parent_type, parent_id, sort_order);

-- GIN index for querying JSONB properties efficiently
CREATE INDEX idx_pages_properties ON pages USING GIN (properties);
CREATE INDEX idx_blocks_content ON blocks USING GIN (content);
```

## GUARANTEEING UI COMPLETENESS

- LexoRank / Double Precision: ฟิลด์ `sort_order` ใช้ชนิดข้อมูล DOUBLE PRECISION เพื่อรองรับการลากและวาง (Drag and Drop) โดยไม่ต้องคำนวณอัปเดตข้อมูลทุกเรคคอร์ด
- View Configuration Isolation: การแยก `database_views` ออกมาช่วยรักษาความสมบูรณ์ของการจัดเรียง เช่น ความกว้างของคอลัมน์และกลุ่มข้อมูล ให้เหมือนบน Frontend ทุกประการ
- Lossless Transformation: การใช้ `JSONB` รับรองว่าข้อมูลการตกแต่งข้อความใดๆ จะไม่ถูกระบบฐานข้อมูลตัดทิ้งหรือขัดขวาง ทำให้ข้อมูลเหมือนกับต้นฉบับเสมอ