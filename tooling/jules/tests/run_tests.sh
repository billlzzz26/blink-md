#!/usr/bin/env bash
set -e

echo "--- 🦀 Running Rust Unit Tests ---"
cargo test

echo ""
echo "--- 🐍 Running Python Integration Tests ---"
python3 tests/integration_test.py
