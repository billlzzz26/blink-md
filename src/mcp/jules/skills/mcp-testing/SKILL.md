---
name: mcp-testing-debugging
description: "Techniques and tools for testing and debugging MCP servers. Use for: verifying tool outputs, debugging connection issues, and performing automated testing of MCP servers."
---

# MCP Testing & Debugging

## Testing Strategies

### 1. Manual Testing with MCP Inspector

```bash
npx @modelcontextprotocol/inspector python your_server.py
```

ใช้ Inspector เพื่อเรียก tool โดยตรง ตรวจ input/output schema และดู error ก่อน integrate กับ client จริง

### 2. Unit Testing (pytest)

ทดสอบ tool logic แยกออกจาก transport layer:

```python
def test_my_tool_logic():
    from your_server import my_tool_logic
    result = my_tool_logic("test input")
    assert "expected" in result
```

### 3. Integration Testing

Mock client-server round-trip โดยใช้ `scripts/mcp_test_client.py`:

```python
# ตัวอย่าง: เรียก tool ผ่าน mock client และตรวจ response schema
client = MockMCPClient("python your_server.py")
response = client.call_tool("my_tool", {"input": "test"})
assert response["status"] == "ok"
```

## Debugging — Common Issues

| Issue | Cause | Fix |
|-------|-------|-----|
| Connection Refused | Server ไม่ได้รันหรือ port ผิด | ตรวจ process + port ที่ server bind |
| Tool Not Found | ขาด `@mcp.tool()` หรือ typo ใน name | เปรียบเทียบ decorator name กับ call site |
| Timeout | Tool ทำ I/O แบบ blocking | เปลี่ยน `def` เป็น `async def` |
| Schema Error | Pydantic model ไม่ตรงกับ input | Validate payload ก่อน pass เข้า tool |

## Resources

- `scripts/mcp_test_client.py` — mock client สำหรับ integration test
- `references/debugging_guide.md` — error traces, transport-level logs, stdio vs HTTP debugging
