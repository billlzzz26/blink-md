# Database Dictionary: notion-rs

## Overview
This database schema is designed to serve as a local cache and sync engine for Notion data, ensuring 100% visual fidelity with the Notion UI. It uses a hybrid relational/JSONB approach to handle the dynamic nature of Notion's blocks and properties.

---

## Tables

### `users`
Stores metadata for humans and bots.
- `id`: UUID (PK).
- `type`: 'person' or 'bot'.
- `email`: User's email (null for some bots).
- `name`: Display name.
- `avatar_url`: Link to user's profile picture.

### `workspaces`
Highest level container.
- `id`: UUID (PK).
- `name`: Workspace name.
- `owner_id`: FK to `users.id`.

### `databases`
Schema definitions for collections of pages.
- `id`: UUID (PK).
- `workspace_id`: FK to `workspaces.id`.
- `title`: JSONB array of RichText (e.g., bold/italic title).
- `properties_schema`: JSONB defining the columns (Select, Multi-select, Date, etc.).
- `in_trash`: Boolean (soft delete).

### `database_views`
UI-specific configurations for a database.
- `id`: UUID (PK).
- `database_id`: FK to `databases.id`.
- `view_type`: Render type (table, board, gallery, list, calendar, timeline).
- `config`: JSONB containing column widths, groupings, and filters.
- `sort_order`: Double precision for tab positioning.

### `pages`
Container for blocks, can live in workspaces, databases, or other pages.
- `id`: UUID (PK).
- `parent_type`: Discriminator ('workspace', 'database', 'page').
- `parent_id`: ID of the parent resource.
- `sort_order`: Double precision for sequencing.
- `properties`: JSONB map of property values matching the database schema.
- `in_trash`: Boolean (soft delete).

### `blocks`
Atomic content elements (Paragraph, Heading, Image, etc.).
- `id`: UUID (PK).
- `page_id`: FK to `pages.id`.
- `parent_type`: Discriminator ('page', 'block').
- `parent_id`: ID of the parent (supports nested blocks like toggles/columns).
- `sort_order`: Double precision for vertical ordering.
- `block_type`: String identifier for the block logic.
- `content`: JSONB payload containing text and visual formatting.

### `comments`
Discussion threads.
- `id`: UUID (PK).
- `parent_id`: ID of the block or page.
- `rich_text`: JSONB content of the comment.

---

## Performance & Scaling
1. **Ordering**: `sort_order` uses `DOUBLE PRECISION` to allow O(1) reordering (LexoRank style).
2. **Search**: GIN indexes are applied to `blocks.content` and `pages.properties` to allow fast searching inside dynamic JSON data.
3. **Fidelity**: All formatting (colors, bold, etc.) is stored in `JSONB` to ensure the UI can reconstruct the exact look of the original Notion page.
