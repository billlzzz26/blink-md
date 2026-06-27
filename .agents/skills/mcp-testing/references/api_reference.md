# MCP Testing — API Reference

## MCP Inspector

```bash
npx @modelcontextprotocol/inspector <command> [args...]
```

| Flag | ความหมาย |
|------|----------|
| `--stdio` | เชื่อมผ่าน stdin/stdout (default สำหรับ local server) |
| `--port <n>` | HTTP mode, ระบุ port |
| `--verbose` | แสดง raw JSON request/response |

ตัวอย่าง:
```bash
npx @modelcontextprotocol/inspector python server.py
npx @modelcontextprotocol/inspector --verbose node server.js
```

---

## mcp Python SDK — Client API

### ClientSession

```python
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

server_params = StdioServerParameters(command="python", args=["server.py"])

async with stdio_client(server_params) as (read, write):
    async with ClientSession(read, write) as session:
        await session.initialize()
```

### Methods

| Method | Return | ความหมาย |
|--------|--------|----------|
| `session.initialize()` | `InitializeResult` | Handshake + รับ server capabilities |
| `session.list_tools()` | `ListToolsResult` | รายชื่อ tools ที่ server expose |
| `session.call_tool(name, arguments)` | `CallToolResult` | เรียก tool พร้อม arguments |
| `session.list_resources()` | `ListResourcesResult` | รายชื่อ resources |
| `session.read_resource(uri)` | `ReadResourceResult` | อ่าน resource ตาม URI |

### CallToolResult

```python
result = await session.call_tool("my_tool", {"param": "value"})
# result.content — list of ContentBlock
# result.isError — bool
for block in result.content:
    if block.type == "text":
        print(block.text)
```

---

## Error Types

| Exception | เกิดเมื่อ |
|-----------|---------|
| `McpError` | Server ส่ง error response กลับมา |
| `asyncio.TimeoutError` | Server ไม่ตอบภายใน timeout |
| `ConnectionResetError` | Server process ตายระหว่าง session |

---

## Transport Comparison

| Transport | ใช้เมื่อ | Command |
|-----------|---------|---------|
| stdio | local server, testing | `StdioServerParameters` |
| HTTP/SSE | remote server, production | `SseServerParameters(url=...)` |
