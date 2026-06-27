#!/usr/bin/env python3
# tests/integration_test.py
import asyncio
import sys
from pathlib import Path

# Add the skills directory to the path to import the test client
sys.path.append(str(Path(__file__).parent.parent / "skills"))

from mcp_testing.scripts.mcp_test_client import MockMCPClient

async def main():
    print("🚀 Starting Integration Test for jules-mcp-server...")
    
    # Define the command to start our Rust MCP server
    server_binary = str(Path(__file__).parent.parent / "target/release/jules-mcp-server")
    if not Path(server_binary).exists():
        print(f"❌ Error: Server binary not found at {server_binary}")
        print("💡 Please run 'cargo build --release' first.")
        sys.exit(1)

    try:
        async with MockMCPClient(server_binary) as session:
            print("[1/3] ✅ Server Initialized Successfully.")
            
            tools = await session.list_tools()
            assert len(tools.tools) > 0, "Server did not expose any tools."
            print(f"[2/3] ✅ Server exposed {len(tools.tools)} tools.")
            
            # Example: Call the first available tool (adapt as needed)
            # In our case, it might be 'run-jules-command'
            first_tool = tools.tools[0].name
            print(f"🔬 Calling tool: '{first_tool}'...")
            
            # This is a placeholder call; we need to know the exact tool and args
            result = await session.call_tool(first_tool, {"command": "gh jules status"})
            assert not result.isError, f"Tool call failed: {result.content}"
            
            print(f"✔️ Tool response received: {result.content}")
            print("[3/3] ✅ Tool call successful.")

    except Exception as e:
        print(f"❌ Integration Test Failed: {e}")
        sys.exit(1)

    print("
🎉 Integration Test Passed!")

if __name__ == "__main__":
    asyncio.run(main())
