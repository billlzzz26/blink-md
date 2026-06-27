.PHONY: all check test lint fmt fmt-fix clean build release setup hooks bench doc package-check mcp-build mcp-run ci

# Default: run all quality checks
all: fmt lint test check package-check

# Quality gates
check:
	cargo check --all-targets --all-features

test:
	cargo test --all-features

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all -- --check

fmt-fix:
	cargo fmt --all

# Build
build:
	cargo build --all-features

release:
	cargo build --release --all-features

# Clean
clean:
	cargo clean

# Documentation
doc:
	cargo doc --no-deps --all-features --open

# Package hygiene
package-check:
	python scripts/check-package-hygiene.py

# Benchmarks
bench:
	cargo bench --all-features

# Setup
setup:
	bash scripts/setup.sh

hooks:
	cp scripts/pre-commit .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit

# Unified MCP server (blink-md-mcp)
mcp-build:
	cargo build --release --features mcp --bin blink-md-mcp

mcp-run:
	cargo run --features mcp --bin blink-md-mcp

# Full CI simulation
ci: fmt lint test check package-check
	@echo "=== CI simulation passed ==="
