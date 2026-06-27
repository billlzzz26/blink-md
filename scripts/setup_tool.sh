#!/usr/bin/env bash
set -euo pipefail

log() { printf '\033[1;32m[INFO]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[WARN]\033[0m %s\n' "$*"; }
err() { printf '\033[1;31m[ERROR]\033[0m %s\n' "$*"; }

have_cmd() { command -v "$1" >/dev/null 2>&1; }

ensure_path() {
  case ":$PATH:" in
    *":$1:"*) ;;
    *) export PATH="$1:$PATH" ;;
  esac
}

install_rust() {
  if have_cmd cargo && have_cmd rustc; then
    log "พบ Rust/Cargo แล้ว: $(cargo --version) / $(rustc --version)"
    return
  fi

  log "ไม่พบ Rust/Cargo กำลังติดตั้งผ่าน rustup..."
  export RUSTUP_INIT_SKIP_CHECK=1
  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | bash -s -- -y

  if [[ -f "$HOME/.cargo/env" ]]; then
    # shellcheck disable=SC1090
    source "$HOME/.cargo/env"
  else
    ensure_path "$HOME/.cargo/bin"
  fi

  ensure_path "$HOME/.cargo/bin"

  log "ติดตั้ง Rust/Cargo เสร็จแล้ว"
  cargo --version
  rustc --version
}

install_gh() {
  if have_cmd gh; then
    log "พบ gh แล้ว: $(gh --version | head -n 1)"
    return
  fi

  log "ไม่พบ gh กำลังพยายามติดตั้ง..."

  if have_cmd apt-get; then
    log "กำลังติดตั้ง gh ผ่าน apt-get"
    sudo apt-get update
    sudo apt-get install -y gh
  elif have_cmd dnf; then
    log "กำลังติดตั้ง gh ผ่าน dnf"
    sudo dnf install -y gh
  elif have_cmd yum; then
    log "กำลังติดตั้ง gh ผ่าน yum"
    sudo yum install -y gh
  elif have_cmd brew; then
    log "กำลังติดตั้ง gh ผ่าน brew"
    brew install gh
  else
    warn "ไม่พบตัวจัดการแพ็กเกจที่รองรับสำหรับ gh"
    warn "ข้ามการติดตั้ง gh"
  fi
}

main() {
  if ! have_cmd curl; then
    err "ไม่พบ curl กรุณาติดตั้ง curl ก่อน"
    exit 1
  fi

  install_rust
  ensure_path "$HOME/.cargo/bin"
  ensure_path "$HOME/.local/bin"

  log "ติดตั้ง Claude CLI..."
  curl -fsSL https://claude.ai/install.sh | bash

  log "ติดตั้ง Antigravity CLI..."
  curl -fsSL https://antigravity.google/cli/install.sh | bash

  install_gh

  log "ตรวจสอบผลลัพธ์..."
  have_cmd cargo && cargo --version || true
  have_cmd rustc && rustc --version || true
  have_cmd claude && claude --version || warn "ยังไม่พบ claude ใน PATH"
  have_cmd antigravity && antigravity --version || warn "ยังไม่พบ antigravity ใน PATH"
  have_cmd gh && gh --version | head -n 1 || warn "ยังไม่พบ gh ใน PATH"

  log "เสร็จสิ้น"
}

main "$@"
