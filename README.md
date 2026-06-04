# blink-md

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.86%2B-blue.svg)](https://www.rust-lang.org)

An unofficial Notion API SDK and TUI explorer for Rust. Built for performance, type safety, and ease of use.

## Features

- **Robust SDK**: Type-safe models and client for the Notion API (version 2026-03-11).
- **Interactive TUI**: A terminal user interface to explore your Notion workspace (Users, Pages, Blocks, Databases).
- **Comprehensive Models**: Detailed implementation of Blocks, Pages, Databases, and Users.
- **Async First**: Built on top of `tokio` and `reqwest` for high-performance asynchronous operations.

## Architecture

The project is structured as both a library and a binary:

- `src/lib.rs`: The main entry point for the SDK.
- `src/client.rs`: Core `NotionClient` implementation.
- `src/api/`: Module-specific API implementations (Blocks, Pages, Databases, etc.).
- `src/models/`: Strongly-typed Rust representations of Notion objects.
- `src/cli/tui.rs`: Interactive Terminal User Interface powered by `ratatui` 0.30.

## Getting Started

### Prerequisites

- Rust 1.86 or higher
- A Notion Integration Token (Internal or Public)

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/your-username/blink-md.git
cd blink-md
cargo build --release
```

### Usage

#### Library

Add `blink-md` to your `Cargo.toml` and use the client:

```rust
use notion_rs::NotionClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NotionClient::new("your_integration_token".to_string());
    let users = client.list_users().await?;
    println!("Found {} users", users.len());
    Ok(())
}
```

#### TUI Explorer

Run the built-in TUI to explore your workspace:

```bash
# Set your token as an environment variable
export NOTION_TOKEN="your_integration_token"

# Run the TUI
cargo run -- tui
```

> [!TIP]
> Use `Tab` / `BackTab` to switch between categories (Users, Pages, Blocks, etc.) and `j`/`k` or arrow keys to navigate lists.

## Development

### Running Tests

```bash
cargo test
```

### Dependency Management

The project uses modern Rust libraries:
- **TUI**: `ratatui` 0.30 with `crossterm` 0.28
- **Async**: `tokio` 1.0
- **HTTP**: `reqwest` 0.12
- **Serialization**: `serde` 1.0

## Status

> [!IMPORTANT]
> This is an unofficial SDK and is currently in active development. API coverage is expanding; check the `src/api/` directory for currently supported endpoints.
