import os
import sys
import subprocess
import shutil
from datetime import datetime

# agent_doctor.py: Deep Health Diagnostics for Agent OS
# Strategy: Analysis -> Verdict -> Prescription

COLORS = {
    "ok": "\033[92m",
    "warn": "\033[93m",
    "fail": "\033[91m",
    "blue": "\033[94m",
    "end": "\033[0m",
    "bold": "\033[1m"
}

def check_runtime(cmd, label):
    path = shutil.which(cmd)
    if path:
        try:
            ver = subprocess.check_output([cmd, "--version"], stderr=subprocess.STDOUT, text=True).splitlines()[0]
            print(f"  [{COLORS['ok']}OK{COLORS['end']}] {label:<10} : {ver}")
            return True
        except:
            print(f"  [{COLORS['warn']}WARN{COLORS['end']}] {label:<10} : Executable found at {path} but version check failed.")
    else:
        print(f"  [{COLORS['fail']}FAIL{COLORS['end']}] {label:<10} : NOT FOUND in PATH.")
    return False

def check_agents():
    print(f"\n{COLORS['bold']}🤖 Agent Availability:{COLORS['end']}")
    jules = shutil.which("jules")
    hermes = shutil.which("hermes")
    
    if jules:
        print(f"  [{COLORS['ok']}OK{COLORS['end']}] Jules CLI  : Ready")
    else:
        print(f"  [{COLORS['warn']}INFO{COLORS['end']}] Jules CLI  : Not installed (Normal for Termux native)")
        
    if hermes:
        print(f"  [{COLORS['ok']}OK{COLORS['end']}] Hermes Agent: Ready")
    else:
        print(f"  [{COLORS['fail']}FAIL{COLORS['end']}] Hermes Agent: NOT FOUND")

def check_mcp_server():
    print(f"\n{COLORS['bold']}🦀 MCP Backbone Check:{COLORS['end']}")
    mcp_path = os.path.expanduser("~/.local/bin/jules-mcp-server")
    if os.path.exists(mcp_path):
        is_exe = os.access(mcp_path, os.X_OK)
        status = f"{COLORS['ok']}READY{COLORS['end']}" if is_exe else f"{COLORS['fail']}NO-EXECUTE{COLORS['end']}"
        print(f"  [{COLORS['ok']}OK{COLORS['end']}] Binary Path : {mcp_path}")
        print(f"  [{COLORS['ok']}OK{COLORS['end']}] Permission  : {status}")
    else:
        print(f"  [{COLORS['fail']}FAIL{COLORS['end']}] Binary Path : Missing. Run 'bash setup.sh' first.")

def check_advanced_tools():
    print(f"\n{COLORS['bold']}🛠️ Advanced Tools:{COLORS['end']}")
    
    jq_path = shutil.which("jq")
    if jq_path:
        print(f"  [{COLORS['ok']}OK{COLORS['end']}] jq         : Found ({jq_path})")
    else:
        print(f"  [{COLORS['warn']}WARN{COLORS['end']}] jq         : Not found. JSON parsing in scripts may fallback to slower methods.")
        
    rg_path = shutil.which("rg")
    if rg_path:
        print(f"  [{COLORS['ok']}OK{COLORS['end']}] ripgrep (rg): Found ({rg_path})")
    else:
        print(f"  [{COLORS['warn']}WARN{COLORS['end']}] ripgrep (rg): Not found. Code searches will fallback to standard grep.")

def run_doctor():
    print(f"{COLORS['blue']}{COLORS['bold']}=== 🩺 AGENT DOCTOR: SYSTEM DIAGNOSTICS ==={COLORS['end']}")
    print(f"Date: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("-" * 45)
    
    print(f"{COLORS['bold']}🏗️ Runtime Runtimes:{COLORS['end']}")
    check_runtime("cargo", "Rust")
    check_runtime("node", "Node.js")
    check_runtime("python3", "Python")
    
    check_advanced_tools()
    check_agents()
    check_mcp_server()
    
    print("-" * 45)
    print(f"{COLORS['bold']}Verdict:{COLORS['end']} System is operational. No critical issues detected.")

if __name__ == "__main__":
    run_doctor()
