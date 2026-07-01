# Cloud Platform API Reference

## Authentication Patterns

### Google Workspace OAuth2 (User Flow)
```
POST https://oauth2.googleapis.com/token
Params: client_id, client_secret, refresh_token, grant_type=refresh_token
Response: { access_token, expires_in, token_type }
```

Key traits from google-workspace-cli:
- Credential fallback: `GOOGLE_WORKSPACE_CLI_TOKEN` > `credentials_file` > encrypted > ADC
- Token caching to `~/.config/blink-md/token_cache.json`
- `yup-oauth2` with `InstalledFlowDelegate` for loopback server

### Lark/Feishu Tenant Token (App Flow)
```
POST https://open.larksuite.com/open-apis/auth/v3/tenant_access_token/internal
Body: { app_id, app_secret }
Response: { code, msg, tenant_access_token, expire }
```

Key traits from chyroc/lark:
- `cli := lark.New(lark.WithAppCredential("app-id", "app-secret"))`
- All calls include `tenant_access_token` header
- `feishu-sdk` crate available for Rust

## Service Mappings

### Google Workspace
| Alias | API | Version | Base URL | Description |
|-------|-----|---------|----------|-------------|
| docs | docs | v1 | docs.googleapis.com | Read/write documents |
| sheets | sheets | v4 | sheets.googleapis.com | Spreadsheets |
| drive | drive | v3 | www.googleapis.com | Files/folders |
| calendar | calendar | v3 | www.googleapis.com | Calendar events |
| chat | chat | v1 | www.googleapis.com | Chat spaces/messages |
| tasks | tasks | v1 | www.googleapis.com | Task lists |
| gmail | gmail | v1 | www.googleapis.com | Email |

### Lark/Feishu
| Alias | API | Version | Base URL | Description |
|-------|-----|---------|----------|-------------|
| docs | docx | v1 | open.larksuite.com/open-apis/docx | Documents |
| sheets | sheets | v3 | open.larksuite.com/open-apis/sheets | Spreadsheets |
| drive | drive | v1 | open.larksuite.com/open-apis/drive | Files |
| calendar | calendar | v1 | open.larksuite.com/open-apis/calendar | Calendar |
| im | im/v1 | v1 | open.larksuite.com/open-apis/im | Messages |
| bitable | bitable | v1 | open.larksuite.com/open-apis/bitable | Tables |
| wiki | wiki | v1 | open.larksuite.com/open-apis/wiki | Knowledge base |

## API Endpoints Reference

### Google Docs
- `GET https://docs.googleapis.com/v1/documents/{documentId}` - Get document
- `POST https://docs.googleapis.com/v1/documents` - Create document
- Body uses `StructuralElement` array with bullets, tables, paragraphs

### Lark Docs (docx)
- `GET /open-apis/docx/v1/documents/{document_id}/raw_content` - Get raw text
- `PATCH /open-apis/docx/v1/documents/{document_id}/blocks/{block_id}` - Update block
- Block types: text, table, grid, etc.

### Google Sheets
- `GET https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}` - Get spreadsheet
- `GET https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}` - Get values
- `PUT https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}` - Write values

### Lark Sheets
- `GET /open-apis/sheets/v3/spreadsheets/{spreadsheet_id}` - Get spreadsheet
- `POST /open-apis/sheets/v3/spreadsheets/{spreadsheet_id}/values/append` - Append values

### Google Calendar
- `GET https://www.googleapis.com/calendar/v3/calendars/{calendarId}/events` - List events
- `POST https://www.googleapis.com/calendar/v3/calendars/{calendarId}/events` - Create event

### Lark Calendar
- `POST /open-apis/calendar/v1/calendars` - Create calendar
- `POST /open-apis/calendar/v1/calendars/{calendar_id}/events` - Create event

## SDK Patterns

### Google (google-workspace-cli)
```rust
// Service discovery via Discovery Document JSON
discovery::fetch_discovery_document(&api_name, &version).await

// Auth flow
let auth = yup_oauth2::InstalledFlowDelegate::builder(client)
    .with_storage(Box::new(TokenStorage::new(path)))
    .build().await?;

let token = auth.token(&scopes).await?;
```

### Lark (open-lark / chyroc/lark)
```rust
// Client builder pattern
let client = LarkClient::builder("app_id", "app_secret")
    .with_app_type(AppType::SelfBuild)
    .build();

// API call pattern
let docs_raw = &client.docs.raw_content;
let resp = docs_raw.get(ctx, &GetDocxDocumentRawContentReq {
    document_id: doc_id
}).await?;
```

## Implementation Notes

### Google Docs Structure → IR
- `StructuralElement` contains `paragraph`, `table`, `sectionBreak`
- `Paragraph` has `elements[]` (text, rich link) + `paragraphStyle`
- Text has `textRun` with content + `textStyle`

### Lark Docs Structure → IR
- Block types: `TEXT`, `TABLE`, `GRID`, etc. (48 types)
- Each block has `block_id`, `parent_id`
- Text styling: `text_element` with `content` and `style`

### Token Storage (Android-compatible)
- Google: encrypted JSON cache with `aes-gcm`
- Lark: token cached in memory/file (2-hour expiry)
- Location: `~/.config/blink-md/` (respects XDG or HOME)