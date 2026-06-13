.PHONY: all check test lint fmt clean build release setup hooks bench doc

# Default: run all quality checks
all: fmt lint test check

# Quality gates
check:
	cargo check --workspace --all-targets

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets

fmt:
	cargo fmt --all -- --check

fmt-fix:
	cargo fmt --all

# Build
build:
	cargo build --workspace

release:
	cargo build --workspace --release

# Clean
clean:
	cargo clean

# Documentation
doc:
	cargo doc --workspace --no-deps --open

# Benchmarks
bench:
	cargo bench --workspace

# Setup
setup:
	bash scripts/setup.sh

hooks:
	cp scripts/pre-commit .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit

# MCP servers
mcp-build:
	cargo build -p jules-mcp-server -p md-mcp-server -p mmd-mcp-server

mcp-test:
	cargo test -p jules-mcp-server -p md-mcp-server -p mmd-mcp-server

# Full CI simulation
ci: fmt lint test check
	@echo "=== CI simulation passed ==="
