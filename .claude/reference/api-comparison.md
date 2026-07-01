# API Platform Comparison: Obsidian, Notion, Lark, Google Workspace

## Summary Matrix

| Platform | Auth Model | Block Model | Pagination | Sync Pattern | Limitations |
|----------|------------|-------------|------------|--------------|-------------|
| Obsidian | Local plugin (no auth) | File-system | None | Direct file I/O | Local only, no conflict resolution |
| Notion | Bearer token (integration) | Flat blocks with parent_id | has_more + next_cursor | Real-time API | Rate limits, expensive for large docs |
| Lark | Tenant token (app) or User OAuth | Nested blocks (48 types) | page_token + has_more | Real-time API | Complex block model, limited docs |
| Google | OAuth2 (user) or Service Account | StructuralElement (hierarchical) | nextPageToken | REST API | Discovery doc overhead, rate limits |

---

## Authentication

### Obsidian Local REST API
- No authentication (localhost only)
- Plugin must be installed
- Headers optional for commands

### Notion API
- Bearer token (NOTION_API_KEY)
- Integration per workspace
- Token stored in integration settings
- Single token scope: all workspace access

### Lark/Feishu
```
POST /open-apis/auth/v3/tenant_access_token/internal
Body: { app_id, app_secret }
Response: { tenant_access_token, expire: 7200 }
```
- Tenant token for app-level access
- User OAuth available via `/auth/v3/oauth/`
- Token expires in 2 hours

### Google Workspace
```
GET https://oauth2.googleapis.com/token
Params: client_id, client_secret, refresh_token
Response: { access_token, expires_in: 3600 }
```
- OAuth2 user flow (loopback server)
- Service account (JWT) alternative
- Token cached, auto-refresh via refresh_token

---

## Data Models

### Obsidian
- Plain markdown files
- YAML frontmatter (parsed)
- Tags `#tag` inline
- No block structure, just lines

### Notion
```json
{
  "object": "block",
  "type": "paragraph",
  "paragraph": { "rich_text": [...] },
  "has_children": true
}
```
- Flat block list
- Parent via `parent_id` in query
- Property types: title, rich_text, select, multi_select, date, people, files, checkbox, url, email, phone_number, number, formula, created_time, last_edited_time, rollup

### Lark
- 48 block types (text, table, grid, etc.)
- Each block has unique `block_id`
- Parent-child relationships explicit
- Block types map to docx elements

### Google Docs
```json
{
  "structuralElements": [
    { "paragraph": { "paragraphStyle": {...}, "elements": [...] } },
    { "table": { "tableRows": [...] } }
  ]
}
```
- Hierarchical structure
- `StructuralElement` array in document body
- No explicit block IDs, positional

---

## API Patterns

### Obsidian Endpoints
| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | /vault/ | List root files |
| GET | /vault/{path}/ | List directory |
| GET | /vault/{file} | Get file content |
| PUT | /vault/{file} | Write file |
| PATCH | /vault/{file} | Append file |
| POST | /commands/{id} | Execute command |

### Notion Endpoints
| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | /v1/databases/{id} | Get database |
| POST | /v1/databases/{id}/query | Query with filter |
| GET | /v1/pages/{id} | Get page |
| PATCH | /v1/pages/{id} | Update page |
| GET | /v1/blocks/{id}/children | Get children |
| PATCH | /v1/blocks/{id} | Update block |
| POST | /v1/blocks/{id}/children | Append children |

### Lark Endpoints (open-lark)
| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | /open-apis/docx/v1/{id}/raw_content | Get doc text |
| PATCH | /open-apis/docx/v1/documents/{id}/blocks/{block} | Update block |
| GET | /open-apis/sheets/v3/spreadsheets/{id} | Get spreadsheet |
| POST | /open-apis/calendar/v1/calendars | Create calendar |

### Google Workspace Endpoints
| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | /v1/documents/{id} | Get document |
| GET | /v4/spreadsheets/{id} | Get spreadsheet |
| GET | /v3/calendars/{id}/events | List events |

---

## Pros / Cons Analysis

### Obsidian
**Pros:**
- Zero auth (local plugin)
- Plain markdown - human readable
- Frontmatter parsing built-in
- File-based sync (git friendly)

**Cons:**
- Local only (no cloud sync)
- No API for blocks/structure
- Plugin required on every machine
- No conflict resolution

### Notion
**Pros:**
- Clean REST API design
- Excellent developer docs
- Real-time sync capability
- Rich property types
- Good pagination pattern

**Cons:**
- Rate limits (3 req/sec)
- Expensive for large docs (many API calls)
- No partial updates (full block replacement)
- Integration tokens are workspace-wide

### Lark
**Pros:**
- 48 block types (more expressive than Notion)
- Tenant token model (app-level access)
- Sheets/Bitable integrated
- Chat/IM built-in

**Cons:**
- Complex block model (harder to learn)
- Chinese-dominant docs
- No official Rust SDK (community `feishu-sdk`)
- Two-token model (tenant or user)

### Google Workspace
**Pros:**
- Industry standard OAuth2
- Discovery doc auto-generates clients
- Familiar to most developers
- Good tooling (`google-apis-rs-client`)

**Cons:**
- Multiple APIs per service (docs, drive, sheets)
- No unified document model
- Quota per API (not shared)
- Discovery doc parsing overhead

---

## Recommendations for blink-md API Improvements

### Current Architecture
- Uses Notion API via `reqwest` directly
- Has Universal IR for document abstraction
- MCP server (`blink-md-mcp`) exists
- Missing: Google OAuth, Lark token management

### Proposed Improvements

1. **Unified Auth Layer**
   - `src/oauth.rs`: OAuth2 for Google
   - `src/lark_auth.rs`: Tenant token for Lark
   - Same interface: `token() -> String`

2. **Service Registry Pattern**
   - Map: `docs` → {Google: v1, Lark: v3}
   - Auto-select based on config
   - Single `get_document()` interface

3. **Batch Operations**
   - Notion: group block updates (reduce API calls)
   - Lark: use batch endpoints when available
   - Google: batch requests via multipart

4. **Caching Layer**
   - ETag-based caching for GET requests
   - Local file cache for offline mode
   - Cache invalidation on write

5. **CLI Consistency**
   ```
   blink-md cloud <platform> <service> <method>
   blink-md cloud google docs get --id xxx
   blink-md cloud lark sheets query --id xxx
   ```

6. **MCP Tool Unification**
   - Single `get_document(platform, id)` tool
   - Single `update_document(platform, id, content)` tool
   - Platform parameter determines API client