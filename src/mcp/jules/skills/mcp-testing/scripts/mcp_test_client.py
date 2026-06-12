import asyncio
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


async def run_test(server_command: str = "python", server_args: list[str] = ["your_server.py"]):
    server_params = StdioServerParameters(
        command=server_command,
        args=server_args,
    )

    async with stdio_client(server_params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()

            # ตรวจว่า server expose tools อย่างน้อย 1 ตัว
            tools = await session.list_tools()
            assert len(tools.tools) > 0, "Server ไม่มี tools — ตรวจ @mcp.tool() decorator"
            print(f"Tools: {[t.name for t in tools.tools]}")

            # ตัวอย่าง: เรียก tool แรกที่มีอยู่
            first_tool = tools.tools[0]
            result = await session.call_tool(first_tool.name, arguments={})
            assert not result.isError, f"Tool '{first_tool.name}' คืน error: {result.content}"
            print(f"Result from '{first_tool.name}': {result.content}")


if __name__ == "__main__":
    asyncio.run(run_test())

