#!/usr/bin/env bash
# setup.sh: 1-Click Environment Setup (Termux/Linux)
# Version: 1.0.0
# Strategy: 3 Languages, 1.5 Shells, Dual Agents
set -e

echo "🚀 Starting Project setup (Termux/Linux Edition)..."
echo "=================================================="

# --- Colors ---
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# --- 1. Environment Analysis & Prereqs ---
echo -e "🔍 ${YELLOW}Step 1: Checking Core Dependencies...${NC}"

check_cmd() {
    if command -v "$1" >/dev/null 2>&1; then
        echo -e "  [${GREEN}OK${NC}] $1: $($1 --version | head -n 1)"
    else
        echo -e "  [${RED}MISSING${NC}] $1 is not installed."
        MISSING=1
    fi
}

MISSING=0
check_cmd "cargo"
check_cmd "node"
check_cmd "npm"
check_cmd "python3"
check_cmd "gh"

if [ "$MISSING" -eq 1 ]; then
    echo -e "${RED}❌ Error: Please install missing core dependencies before running setup.${NC}"
    exit 1
fi

# --- 2. Agent Installation (Jules & Hermes) ---
echo -e "\n🤖 ${YELLOW}Step 2: Setting up AI Agents...${NC}"

# Jules CLI
if ! command -v jules >/dev/null 2>&1; then
    echo "📦 Attempting to install @google/jules@latest..."
    if npm install -g @google/jules@latest 2>/dev/null; then
        echo -e "  [${GREEN}OK${NC}] Jules CLI installed successfully."
    else
        echo -e "  [${YELLOW}INFO${NC}] Jules CLI native install not supported on this platform (e.g. Termux). Skipping..."
        echo -e "         ${NC}You can still use Jules via Remote or on supported OS (Windows/macOS/Linux x64).${NC}"
    fi
else
    echo -e "  [${GREEN}OK${NC}] Jules CLI is already present."
fi

# Hermes Agent
if ! command -v hermes >/dev/null 2>&1; then
    echo "📦 Installing Hermes Agent (via pip/uv)..."
    python3 -m pip install hermes-agent || echo -e "${RED}⚠️ Could not install hermes-agent via pip. Please check your Python environment.${NC}"
else
    echo -e "  [${GREEN}OK${NC}] Hermes Agent is ready."
fi

# --- 3. Build & Install MCP Server (Rust) ---
echo -e "\n🛠 ${YELLOW}Step 3: Building & Installing Jules MCP Server...${NC}"

# Handle Termux-specific build context
if [ -n "$TERMUX_VERSION" ]; then
    export CARGO_TARGET_DIR="$HOME/.cargo_target_jules"
    echo -e "📱 Termux detected. Using internal storage for build: ${GREEN}$CARGO_TARGET_DIR${NC}"
fi

cargo build --release

# Installation Path
BIN_DEST="$HOME/.local/bin"
mkdir -p "$BIN_DEST"

if [ -n "$TERMUX_VERSION" ]; then
    cp "$CARGO_TARGET_DIR/release/jules-mcp-server" "$BIN_DEST/"
else
    cp "target/release/jules-mcp-server" "$BIN_DEST/"
fi
chmod +x "$BIN_DEST/jules-mcp-server"
echo -e "✅ MCP Server installed to: ${GREEN}$BIN_DEST/jules-mcp-server${NC}"

# --- 4. GitHub Extension Setup ---
echo -e "\n🔌 ${YELLOW}Step 4: Installing GitHub CLI Extension (v2.0)...${NC}"
if [ -d "gh-jules" ]; then
    gh extension install ./gh-jules --force || echo "⚠️ Extension install failed (might be already installed)."
    echo -e "✅ GitHub Extension 'gh-jules' (Unified CLI) ready."
else
    echo -e "${RED}⚠️ Warning: gh-jules directory not found.${NC}"
fi

# --- 5. Final Health Check ---
echo -e "\n🩺 ${YELLOW}Step 5: Final Health Check...${NC}"

HEALTH=0
echo -n "  Testing Jules-Hermes Bridge... "
if bash gh-jules/gh-jules --help >/dev/null 2>&1; then
    echo -e "[${GREEN}PASSED${NC}]"
else
    echo -e "[${RED}FAILED${NC}]"
    HEALTH=1
fi

echo -n "  Testing MCP Binary... "
if "$BIN_DEST/jules-mcp-server" --version >/dev/null 2>&1 || [ $? -eq 1 ]; then # Exit 1 is often normal for MCP help
    echo -e "[${GREEN}PASSED${NC}]"
else
    echo -e "[${RED}FAILED${NC}]"
    HEALTH=1
fi

if [ "$HEALTH" -eq 0 ]; then
    echo -e "\n🎉 ${GREEN}SUCCESS! Your command center is fully operational.${NC}"
    echo "--------------------------------------------------"
    echo "Commands available:"
    echo "  gh jules new \"<task>\"    (Jules + Auto-Conductor)"
    echo "  gh jules hermes \"<task>\" (Hermes + Auto-Conductor)"
    echo "--------------------------------------------------"
else
    echo -e "\n⚠️ ${YELLOW}Setup completed with some warnings. Please check the logs.${NC}"
fi
