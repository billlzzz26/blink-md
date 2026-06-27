# setup.ps1: 1-Click Environment Setup (Windows)
# Version: 1.0.0
# Strategy: 3 Languages, 1.5 Shells, Dual Agents

Write-Host "🚀 Starting Project setup (Windows Edition)..." -ForegroundColor Cyan
Write-Host "=================================================="

# --- 1. Environment Analysis & Prereqs ---
Write-Host "🔍 Checking Core Dependencies..." -ForegroundColor Yellow

$Missing = $false
function Check-Cmd($cmd) {
    if (Get-Command $cmd -ErrorAction SilentlyContinue) {
        $version = & $cmd --version | Select-Object -First 1
        Write-Host "  [OK] $cmd: $version" -ForegroundColor Green
    } else {
        Write-Host "  [MISSING] $cmd is not installed." -ForegroundColor Red
        $global:Missing = $true
    }
}

Check-Cmd "cargo"
Check-Cmd "node"
Check-Cmd "npm"
Check-Cmd "python"
Check-Cmd "gh"

if ($Missing) {
    Write-Error "❌ Error: Please install missing core dependencies (using winget or manual) before running setup."
    exit 1
}

# --- 2. Agent Installation (Jules & Hermes) ---
Write-Host "`n🤖 Setting up AI Agents..." -ForegroundColor Yellow

# Jules CLI
if (!(Get-Command jules -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Installing @google/jules@latest..."
    npm install -g @google/jules@latest
} else {
    Write-Host "  [OK] Jules CLI is ready." -ForegroundColor Green
}

# Hermes Agent
if (!(Get-Command hermes -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Installing Hermes Agent..."
    python -m pip install hermes-agent
} else {
    Write-Host "  [OK] Hermes Agent is ready." -ForegroundColor Green
}

# --- 3. Build & Install MCP Server (Rust) ---
Write-Host "`n🛠 Building & Installing Jules MCP Server..." -ForegroundColor Yellow
cargo build --release

$BIN_DEST = "$HOME\.local\bin"
if (!(Test-Path $BIN_DEST)) { New-Item -ItemType Directory -Path $BIN_DEST }
Copy-Item "target\release\jules-mcp-server.exe" "$BIN_DEST\"
Write-Host "✅ MCP Server installed to: $BIN_DEST\jules-mcp-server.exe" -ForegroundColor Green

# --- 4. GitHub Extension Setup ---
Write-Host "`n🔌 Installing GitHub CLI Extension (v2.0)..." -ForegroundColor Yellow
if (Test-Path "gh-jules") {
    gh extension install .\gh-jules --force
    Write-Host "✅ GitHub Extension 'gh-jules' (Unified CLI) ready." -ForegroundColor Green
}

# --- 5. Final Health Check ---
Write-Host "`n🩺 Final Health Check..." -ForegroundColor Yellow

$Health = $true
Write-Host -NoNewline "  Testing Jules-Hermes Bridge... "
if (gh jules --help 2>&1 | Out-String -Stream | Select-String "Usage") {
    Write-Host "[PASSED]" -ForegroundColor Green
} else {
    Write-Host "[FAILED]" -ForegroundColor Red
    $Health = $false
}

if ($Health) {
    Write-Host "`n🎉 SUCCESS! Your command center is fully operational." -ForegroundColor Green
    Write-Host "--------------------------------------------------"
    Write-Host "Commands available:"
    Write-Host "  gh jules new `"<task>`"    (Jules + Auto-Conductor)"
    Write-Host "  gh jules hermes `"<task>`" (Hermes + Auto-Conductor)"
    Write-Host "--------------------------------------------------"
} else {
    Write-Host "`n⚠️ Setup completed with some warnings. Please check the logs." -ForegroundColor Yellow
}
