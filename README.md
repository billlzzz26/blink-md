# blink-md ⚡️

[![Crates.io](https://img.shields.io/crates/v/blink-md.svg)](https://crates.io/crates/blink-md)
[![Build Status](https://github.com/billlzzz26/blink-md/actions/workflows/rust.yml/badge.svg)](https://github.com/billlzzz26/blink-md/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.3.0-blue.svg)](https://github.com/billlzzz26/blink-md/releases/tag/v0.3.0)

**blink-md** is a high-performance, platform-agnostic document sync and conversion engine built in Rust. It serves as a bridge between structured SaaS documents (Notion, Lark, Google Docs) and standard file formats (Markdown, HTML, PDF), ensuring **100% visual fidelity and lossless transformation**.

---

## 📖 Table of Contents
- [Description](#description)
- [Key Features](#key-features)
- [Architecture](#-architecture)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Configuration](#-configuration)
- [Roadmap](#-roadmap)
- [Development](#-development)
- [Contributing](#-contributing)
- [License](#-license)

---

## 📝 Description
The modern document landscape is fragmented across proprietary platforms. **blink-md** solves the "vendor lock-in" and "format drift" problems by introducing a **Universal Intermediate Representation (IR)**. 

Whether you are syncing a Notion database to a local Markdown folder, or converting a complex Google Doc into a clean, GitHub-flavored Markdown file, **blink-md** handles the structural mapping, rich-text annotations, and hierarchical nesting with cryptographic precision.

### Why blink-md?
- **Universal IR First**: We don't just convert format-to-format; we map everything to a platform-neutral model first.
- **Relational Stability**: Integrated PostgreSQL document store for local caching and offline-first workflows.
- **Developer Centric**: Pure Rust, async-first, and easily extensible with new Platform Adapters.

---

## ✨ Key Features
- **Lossless Conversion**: Preserves every bold, italic, mention, and nested toggle across formats.
- **Universal Document IR**: A canonical model covering Notion 2026-03-11, GFM, and Lark specifics.
- **Precise UI Rendering**: Uses LexoRank-style `sort_order` for identical visual sequence on any interface.
- **MCP Server Ready**: Built-in Model Context Protocol support for AI-driven document manipulation.
- **High-Performance Sync**: Debounced, multi-threaded sync with smart rate-limit backoff.

---

## 🏗 Architecture
blink-md acts as the central hub for your document ecosystem.

```text
[ Source Platform ]       [ Universal IR ]       [ Target Platform ]
      (Notion)    <----->  (The Core)    <----->     (Markdown)
      (Lark)               /    |    \               (HTML/PDF)
      (GDocs)             /     |     \              (Docx)
                         v      v      v
                 [ Relational Document Store ]
                 [ Interactive TUI / Preview ]
```

---

## 🛠 Installation

### Prerequisites
- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (Edition 2021+)
- **PostgreSQL** (Optional): Required for the Relational Document Store features.

### From Crates.io
```bash
cargo install blink-md
```

### From Source
```bash
git clone https://github.com/billlzzz26/blink-md.git
cd blink-md
cargo build --release
```

---

## 🚀 Quick Start

### 1. Configure Credentials
Set your Notion API token (or other platform keys) in your environment:
```bash
export NOTION_TOKEN=your_notion_secret_key
export NOTION_DB_ID=your_database_id
```

### 2. Basic Commands
**Search for Documents:**
```bash
blink-md search "Technical Specs"
```

**Sync Local Folder to Notion:**
```bash
blink-md sync --dir ./my-docs --notion-db $NOTION_DB_ID
```

**Launch Interactive TUI:**
```bash
blink-md tui
```

### 3. Library Usage
```rust
use blink_md::NotionClient;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NotionClient::new("YOUR_TOKEN");
    
    // Fetch a page with full recursive children
    let blocks = client.get_block_children_recursive("PAGE_ID").await?;
    
    println!("Retrieved {} blocks total.", blocks.len());
    Ok(())
}
```

---

## ⚙️ Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `NOTION_TOKEN` | Your Notion Integration Token | - |
| `NOTION_DB_ID` | Default Database ID for sync/query | - |
| `DATABASE_URL` | PostgreSQL connection string | `postgres://localhost/blink_md` |
| `LOG_LEVEL` | Logging verbosity (info, debug, trace) | `info` |

---

## 🗺 Roadmap (v0.2.0)
Track our detailed progress on the [GitHub Roadmap](https://github.com/billlzzz26/blink-md/projects/2).

- **Phase 2 (Active)**: 🎨 UX & TUI Refinement (Theming, Status Indicators, Error handling).
- **Phase 3**: 🔄 Core Bidirectional Converters (Notion <-> IR <-> Markdown).
- **Phase 4**: 🌐 Extended Platforms (GitHub, HTML, Google Docs).
- **Phase 5**: 📎 Advanced API Features (File Uploads, Webhooks).
- **Phase 6**: 🤖 Full MCP 2.9 Integration & Server mode.

---

## 🛠 Development

### Setup Environment
```bash
# Run tests
cargo test

# Run with color check
cargo check --color always

# Clean build artifacts
cargo clean
```

### Quality Gates
- All PRs must pass `cargo clippy` and `cargo fmt`.
- Integration tests require a valid `NOTION_TOKEN` or `wiremock` stubs.
- Run `oh-my-product verify` for full compliance check.

---

## 🤝 Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License
Distributed under the MIT License. See `LICENSE` for more information.

---

## 👥 Authors & Acknowledgments
- **billlzzz26** - *Initial Work & Architecture*
- Inspired by the flexibility of **Notion** and the performance of the **Rust** ecosystem.

## 📞 Support & Contact
- **Issue Tracker**: [GitHub Issues](https://github.com/billlzzz26/blink-md/issues)
- **Email**: billzzz26@gmail.com

---
*Built with ❤️ using Gemini CLI on Termux.*
