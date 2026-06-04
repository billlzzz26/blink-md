-- UNIVERSAL DOCUMENT STORE SCHEMA FOR notion-rs
-- Target: PostgreSQL (with JSONB and UUID support)
-- This schema aligns with the v0.2.0 Universal Document IR architecture.

-- 1. EXTENSIONS
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 2. USERS
-- Stores people and bot information from any platform.
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    platform VARCHAR(50) NOT NULL, -- 'notion', 'github', 'lark', etc.
    external_id TEXT NOT NULL, -- Original ID from the platform
    email VARCHAR(255),
    name VARCHAR(255),
    avatar_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(platform, external_id)
);

-- 3. WORKSPACES
-- Containers for documents, linked to platforms.
CREATE TABLE workspaces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id),
    platform VARCHAR(50) NOT NULL,
    external_id TEXT, -- Original workspace ID if applicable
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 4. DOCUMENTS
-- Platform-agnostic documents (Pages, Files, Docs, etc.)
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    
    -- Metadata (Matches DocumentMetadata IR)
    title TEXT,
    author TEXT,
    source_platform VARCHAR(50) NOT NULL,
    source_id TEXT NOT NULL, -- Original ID (page_id, file_path, etc.)
    
    properties JSONB NOT NULL DEFAULT '{}', -- Platform-specific properties (PropertyValue IR)
    custom_metadata JSONB NOT NULL DEFAULT '{}', -- Custom metadata map
    
    created_time TIMESTAMP WITH TIME ZONE,
    last_edited_time TIMESTAMP WITH TIME ZONE,
    
    in_trash BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(source_platform, source_id)
);

-- 5. STYLES
-- Named styles for a document (StyleSheet IR)
CREATE TABLE styles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    style_type VARCHAR(50) NOT NULL, -- 'text', 'block', 'code', 'table'
    config JSONB NOT NULL DEFAULT '{}',
    UNIQUE(document_id, name)
);

-- 6. BLOCKS
-- Universal blocks within a document (UniversalBlock IR)
CREATE TABLE blocks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    
    -- Hierarchical Structure
    parent_id UUID, -- References another block ID if nested
    sort_order DOUBLE PRECISION NOT NULL, -- LexoRank for UI vertical ordering
    
    block_type VARCHAR(50) NOT NULL, -- 'paragraph', 'heading', 'code_block', etc.
    content JSONB NOT NULL DEFAULT '{}', -- Payload (InlineElements, MediaSource, etc.)
    
    -- Platform specific raw data (Optional preservation)
    raw_data JSONB,
    
    has_children BOOLEAN DEFAULT FALSE,
    in_trash BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 7. COMMENTS
-- Discussions on blocks or documents.
CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    discussion_id UUID NOT NULL,
    target_id UUID NOT NULL, -- Document or Block ID
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('document', 'block')),
    rich_text JSONB NOT NULL DEFAULT '[]',
    created_by UUID NOT NULL REFERENCES users(id),
    created_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 8. INDEXES
-- Optimized for Universal IR tree construction and search.

-- Documents
CREATE INDEX idx_docs_source ON documents(source_platform, source_id);
CREATE INDEX idx_docs_workspace ON documents(workspace_id) WHERE in_trash = FALSE;
CREATE INDEX idx_docs_properties ON documents USING GIN (properties);

-- Blocks
CREATE INDEX idx_blocks_document ON blocks(document_id, sort_order) WHERE in_trash = FALSE;
CREATE INDEX idx_blocks_parent ON blocks(parent_id, sort_order);
CREATE INDEX idx_blocks_content ON blocks USING GIN (content);

-- Styles
CREATE INDEX idx_styles_document ON styles(document_id);
