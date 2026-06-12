import os
import json
import subprocess
from datetime import datetime

# agent_dashboard.py: The Command Center Overview
# Strategy: Python (Connector) + JSON Status Sync

COLORS = {
    "header": "\033[95m",
    "jules": "\033[94m",
    "hermes": "\033[92m",
    "warning": "\033[93m",
    "fail": "\033[91m",
    "end": "\033[0m",
    "bold": "\033[1m"
}

def get_jules_status():
    try:
        # Check for active sessions (placeholder logic for now)
        # In real case, we'd list .jules-session-* files
        return "Active (Monitoring...)"
    except:
        return "Offline"

def get_hermes_status():
    try:
        result = subprocess.run(["hermes", "cron", "list"], capture_output=True, text=True)
        if "No active jobs" in result.stdout:
            return "Online (Idle)"
        return f"Online ({len(result.stdout.splitlines()) - 1} Jobs)"
    except:
        return "Offline"

def get_memory_stats():
    try:
        size_kb = int(subprocess.check_output(["du", "-sk", "conductor/", "docs/", "GEMINI.md"]).split()[0])
        status = "HEALTHY" if size_kb < 200 else "EXCEEDED"
        return f"{size_kb}KB [{status}]"
    except:
        return "Unknown"

def print_dashboard():
    print(f"{COLORS['header']}{COLORS['bold']}=== 🌀 AGENT COMMAND CENTER DASHBOARD ==={COLORS['end']}")
    print(f"Timestamp: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("-" * 40)
    
    print(f"{COLORS['jules']}🚀 Jules Agent:{COLORS['end']}  {get_jules_status()}")
    print(f"{COLORS['hermes']}⚕️  Hermes Agent:{COLORS['end']} {get_hermes_status()}")
    print(f"{COLORS['warning']}🧠 Memory State:{COLORS['end']} {get_memory_stats()}")
    
    print("-" * 40)
    print(f"Available Commands: {COLORS['bold']}gh jules <new|hermes|list|pull>{COLORS['end']}")

if __name__ == "__main__":
    print_dashboard()
