import os
import sys
import json
import subprocess

# jules_api_bridge.py: Production bridge for Google Jules API (v1alpha)
# Built from official documentation discovered by Hermes Agent.
# Strategy: Manage Sessions (Create/Message/List) via curl.

def get_api_key():
    try:
        with open("jules.key", "r") as f:
            return f.read().strip()
    except:
        print("❌ Error: jules.key not found.")
        sys.exit(1)

def call_jules(action, prompt=None, session_id=None, source=None):
    api_key = get_api_key()
    base_url = "https://jules.googleapis.com/v1alpha"
    
    headers = [
        "-H", f"X-Goog-Api-Key: {api_key}",
        "-H", "Content-Type: application/json"
    ]

    if action == "create":
        # Create a new session
        url = f"{base_url}/sessions"
        payload = {
            "prompt": prompt,
            "sourceContext": {
                "source": source if source else "sources/github/placeholder/repo",
                "githubRepoContext": {
                    "startingBranch": "main"
                }
            },
            "automationMode": "AUTO_CREATE_PR",
            "title": f"Session {session_id}" if session_id else "New Research Session"
        }
            
        cmd = ["curl", "-s", "-X", "POST", url] + headers + ["-d", json.dumps(payload)]

    elif action == "send":
        # Send message to existing session
        if not session_id: return "❌ Error: session_id required for 'send'"
        url = f"{base_url}/sessions/{session_id}:sendMessage"
        payload = {"prompt": prompt}
        cmd = ["curl", "-s", "-X", "POST", url] + headers + ["-d", json.dumps(payload)]

    elif action == "list":
        # List all sessions
        url = f"{base_url}/sessions?pageSize=10"
        cmd = ["curl", "-s", "-X", "GET", url] + [headers[0], headers[1]] # Only key and type

    else:
        return f"❌ Error: Unknown action '{action}'"

    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        if not result.stdout: return "⚠️  Empty response."
        return result.stdout
    except Exception as e:
        return f"❌ Bridge Error: {str(e)}"

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 jules_api_bridge.py list")
        print("       python3 jules_api_bridge.py create \"<prompt>\" [source]")
        print("       python3 jules_api_bridge.py send \"<prompt>\" <session_id>")
        sys.exit(1)

    cmd_type = sys.argv[1]
    
    if cmd_type == "list":
        print(call_jules("list"))
    elif cmd_type == "create":
        prompt = sys.argv[2]
        source = sys.argv[3] if len(sys.argv) > 3 else None
        print(call_jules("create", prompt=prompt, source=source))
    elif cmd_type == "send":
        prompt = sys.argv[2]
        sid = sys.argv[3]
        print(call_jules("send", prompt=prompt, session_id=sid))
    else:
        # Default behavior for gh-jules: create new session
        print(call_jules("create", prompt=sys.argv[1]))
