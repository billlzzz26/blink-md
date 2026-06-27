#!/bin/bash
# blink-md setup script
# Detects OS and installs required dependencies

set -e

echo "=== blink-md Setup ==="

# Detect OS
detect_os() {
    if [ -n "$TERMUX_VERSION" ]; then
        echo "termux"
    elif [ -f /etc/alpine-release ]; then
        echo "alpine"
    elif [ -f /etc/debian_version ]; then
        echo "debian"
    elif [ -f /etc/arch-release ]; then
        echo "arch"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    else
        echo "unknown"
    fi
}

OS=$(detect_os)
echo "Detected OS: $OS"

# Install system dependencies
install_deps() {
    case $OS in
        termux)
            echo "Installing Termux packages..."
            pkg update -y
            pkg install -y rust clang make git openssl
            ;;
        debian)
            echo "Installing Debian/Ubuntu packages..."
            sudo apt-get update
            sudo apt-get install -y build-essential pkg-config libssl-dev git curl
            ;;
        alpine)
            echo "Installing Alpine packages..."
            sudo apk add --no-cache build-base pkgconfig openssl-dev git curl
            ;;
        arch)
            echo "Installing Arch packages..."
            sudo pacman -Syu --noconfirm base-devel openssl git curl
            ;;
        macos)
            echo "Installing macOS dependencies..."
            if ! command -v brew &> /dev/null; then
                echo "Homebrew not found. Install from https://brew.sh"
                exit 1
            fi
            brew install openssl git
            ;;
        *)
            echo "Unknown OS. Please install: rust, git, make, openssl"
            ;;
    esac
}

# Install Rust if not present
install_rust() {
    if ! command -v cargo &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo "Rust already installed: $(rustc --version)"
    fi
}

# Verify tools
verify_tools() {
    echo "Verifying tools..."
    cargo --version || { echo "cargo not found"; exit 1; }
    rustc --version || { echo "rustc not found"; exit 1; }
    cargo clippy --version || rustup component add clippy
    cargo fmt --version || rustup component add rustfmt
    echo "All tools verified."
}

# Build and test
build_test() {
    echo "Building blink-md (all features)..."
    cargo build --all-features

    echo "Running tests..."
    cargo test --all-features

    echo "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    
    echo "=== Setup complete ==="
}

# Main
install_deps
install_rust
verify_tools
build_test

echo ""
echo "Setup complete! Run 'make hooks' to install pre-commit hooks."
