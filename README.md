# blink-md ⚡️

[![Crates.io](https://img.shields.io/crates/v/blink-md.svg)](https://crates.io/crates/blink-md)
[![Build Status](https://github.com/billlzzz26/blink-md/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/billlzzz26/blink-md/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.3.1-blue.svg)](https://github.com/billlzzz26/blink-md/releases/tag/v0.3.1)

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

Whether you are syncing a Notion database to a local Markdown folder, or converting a complex Google Doc into a clean, GitHub-flavored Markdown file, **blink-md** handles the structural mapping, rich-text annotations, and hierarchical nesting with lossless conversion.

### Why blink-md?
- **Universal IR First**: We don't just convert format-to-format; we map everything to a platform-neutral model first.
- **Offline-Friendly Pipeline**: Local file conversion and sync workflows are designed to be repeatable without losing source structure.
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
                 [ Local Files / CLI / TUI ]
                 [ MCP Server / AI Agents ]
```

---

## 🛠 Installation

### Prerequisites
- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (Edition 2021+)

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
For detailed instructions in **English** and **ภาษาไทย**, please see our [Quick Start Guide](docs/QUICKSTART.md).

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
| `LOG_LEVEL` | Logging verbosity (info, debug, trace) | `info` |

---

## 🗺 Roadmap (v0.3.1)
Track detailed progress in `TODO.md` and `docs/PLAN.md`.

- **Current release**: v0.3.1 — cross-platform release artifacts, Thai TUI hardening, Universal Data Adapters, self-update, installers, and CLI help polish.
- **Next engineering focus**: make every new feature update code, tests, docs, CI/package gates, and release notes together.
- **Upcoming converter work**: GitHub Markdown/GFM, HTML, Lark/Feishu, Google Docs, PDF, Docx, and Sheets/Excel adapters behind Universal IR.
- **Upcoming API work**: page markdown endpoints, data source CRUD, webhooks, search sort/filter, block position updates, and file upload polish.
- **Upcoming UX work**: deeper TUI preview/edit flows and better status/help surfaces.

---

## 🛠 Development

### Setup Environment
```bash
# Run tests
cargo test --workspace

# Run with color check
cargo check --workspace --color always

# Clean build artifacts
cargo clean
```

### Quality Gates
- All PRs must pass `make ci` locally before pushing.
- `make ci` runs formatting, clippy, tests, `cargo check`, and package hygiene.
- Integration tests require a valid `NOTION_TOKEN` or `wiremock` stubs.
- Cross-platform Android release builds are covered by `.github/workflows/cross-platform.yml`.
- `cargo package` must not include local agent data, secrets, or internal conductor docs.

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
*Built with Rust for Notion, Markdown, Lark, Google Docs, and local document workflows.*
