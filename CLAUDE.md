# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

For agent behavior, workflow, memory, security, file-placement, and commit/label conventions, read `AGENTS.md` first — this file only covers commands and code architecture, and does not repeat what's there.

## Commands

```bash
make ci                                    # fmt-check + clippy + test + check + package-check (run before every push)
cargo test --all-features                  # full test suite (wiremock-mocked HTTP, no NOTION_TOKEN needed)
cargo test --all-features <name>           # single test by name (matches across all test binaries)
cargo test --test integration_tests <name> # single test within one test file
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check                 # fmt --all (no --fix) to apply
cargo run --features mcp --bin blink-md-mcp
cargo build --release --all-features
python scripts/check-package-hygiene.py    # blocks local agent data/secrets from `cargo package`
```

MSRV is 1.75 (checked in `.github/workflows/cross-platform.yml`). Integration tests never need a real `NOTION_TOKEN` — they mock the HTTP layer with `wiremock`.

## Architecture

Single crate, two binaries. The `[workspace]` table in `Cargo.toml` exists only so the package can share `workspace.dependencies`/`workspace.lints` with itself (`members = ["."]`, no sub-crates). `blink-md` (`src/main.rs`, CLI + TUI) and `blink-md-mcp` (`src/bin/blink-md-mcp.rs`, behind the `mcp` feature) both link the same library code.

Everything routes through a Universal IR, never platform-to-platform directly. `src/ir/` (`UniversalDocument`, `UniversalBlock`, inline/style/table/metadata types) is the platform-neutral hub. Converting Notion to Markdown means Notion → IR → Markdown, going through `src/converter/`'s `FromPlatform`/`ToPlatform` traits (`src/converter/mod.rs`), never a direct mapping. `ConverterRegistry` currently type-erases through `Box<dyn Any>` because `FromPlatform::Input` varies per platform (`String` for file formats, `PageWithBlocks` for Notion) — a known limitation with a designed-but-not-yet-implemented fix in `docs/ARCHITECTURE.md` (Reader/Writer + Source/Sink split, M1–M5 migration plan). Read that doc before touching the converter/registry layer.

Two separate type layers for Notion, easy to conflate. `src/models/` are raw Notion API JSON types (wire format, one file per resource: `page.rs`, `block.rs`, `database.rs`, `datasource.rs`, `view.rs`, `db.rs`, `common.rs`). `src/ir/` is the platform-neutral IR. `src/converter/notion.rs` is the only place that bridges the two — API response shape changes belong in `src/models/`, IR shape changes belong in `src/ir/`.

`NotionClient` (`src/client.rs`) is just the HTTP/rate-limit layer. Its actual API surface is implemented as methods across `src/api/*.rs`, one file per Notion resource (`pages.rs`, `blocks.rs`, `databases.rs`, `search.rs`, `users.rs`, `comments.rs`, `views.rs`, `files.rs`, `trash.rs`, `webhooks.rs`), plus `markdown.rs`/`markdown_frontmatter.rs` for the Markdown-side parsing that `src/converter/` consumes.

CLI command dispatch order matters. `src/main.rs` checks for `NOTION_TOKEN` before running most commands, but offline commands (`convert`, `diff`, `upgrade`, `generate-skills`) are dispatched before that check since they never touch the network. When adding a new command that doesn't need Notion, add it to that early-dispatch list — it's an easy thing to regress.

`mcp-serve` is also dispatched before the token check, but for a different reason: starting the MCP server itself needs no token, since `src/mcp/server.rs` only registers the `notion_live` tools (`src/mcp/tools/notion_live.rs`) when `NOTION_TOKEN` is present in the environment. Once registered, those tools do make live Notion API calls through `NotionClient` — so "dispatched early" here means "doesn't need a token to start," not "never touches the network."

`src/sync/` backs `blink-md sync --dir`: `id_mapper.rs` maps local files to remote Notion page IDs, `builder.rs`/`schema.rs`/`json_schema.rs` support frontmatter-driven sync. `generate_skills.rs` is unrelated to `.claude/skills/` — it generates product-facing "persona/recipe" skill docs from `src/registry/{personas,recipes}.toml` (a CLI feature, not agent tooling).

`src/cli/output.rs` is the standard renderer for list/get commands (aligned tables by default, `--format json` for scripts) — new commands that return Notion objects should use it rather than dumping raw `serde_json`.

`src/mcp/` (feature-gated) bundles every platform's tools into one server: `core.rs` (shared `pmcp` helpers), `server.rs`, and `tools/{notion,notion_live,markdown,lark,mermaid}.rs`.
