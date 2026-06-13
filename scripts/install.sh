#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}==>${NC} blink-md Installer"

# Platform detection
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux*)   PLATFORM="linux" ;;
  darwin*)  PLATFORM="darwin" ;;
  *)        echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64)   ARCH_NAME="amd64" ;;
  aarch64|arm64) ARCH_NAME="arm64" ;;
  *)        echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Special case for Termux
if [ -n "$TERMUX_VERSION" ]; then
    INSTALL_DIR="$PREFIX/bin"
    echo -e "${BLUE}==>${NC} Detected Termux environment"
else
    INSTALL_DIR="/usr/local/bin"
fi

ARTIFACT="blink-md-${PLATFORM}-${ARCH_NAME}.tar.gz"
REPO="billlzzz26/blink-md"
LATEST_RELEASE=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "Failed to fetch latest release version."
    exit 1
fi

echo -e "${BLUE}==>${NC} Installing blink-md ${LATEST_RELEASE} for ${PLATFORM}/${ARCH_NAME}"

# Path selection
echo -e "${BLUE}==>${NC} Choose installation path:"
echo "1) $INSTALL_DIR (Standard)"
echo "2) $HOME/.local/bin (User)"
echo "3) Custom path"
read -p "Select [1-3, default: 1]: " choice

case "$choice" in
  2) INSTALL_DIR="$HOME/.local/bin" ;;
  3) read -p "Enter custom path: " INSTALL_DIR ;;
  *) ;;
esac

echo -e "${BLUE}==>${NC} Installing to: ${INSTALL_DIR}"

# Set global alias/path
case "$SHELL" in
  */zsh)  RC_FILE="$HOME/.zshrc" ;;
  */bash) RC_FILE="$HOME/.bashrc" ;;
  *)      RC_FILE="$HOME/.profile" ;;
esac

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${BLUE}==>${NC} Adding ${INSTALL_DIR} to PATH in ${RC_FILE}"
    echo "export PATH=\"\$PATH:${INSTALL_DIR}\"" >> "$RC_FILE"
    echo -e "${BLUE}==>${NC} Please run 'source ${RC_FILE}' or restart terminal"
fi

# Download
TMP_DIR=$(mktemp -d)
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/$ARTIFACT"

curl -L "$DOWNLOAD_URL" -o "$TMP_DIR/$ARTIFACT"
tar -xzf "$TMP_DIR/$ARTIFACT" -C "$TMP_DIR"

# Install
if [ -n "$TERMUX_VERSION" ]; then
    mv "$TMP_DIR/blink-md" "$INSTALL_DIR/"
else
    sudo mv "$TMP_DIR/blink-md" "$INSTALL_DIR/"
fi

chmod +x "$INSTALL_DIR/blink-md"

rm -rf "$TMP_DIR"

echo -e "${GREEN}==>${NC} Successfully installed blink-md!"
echo -e "${GREEN}==>${NC} Run 'blink-md --help' to get started."
