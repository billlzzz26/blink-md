# Session Memory

## Log
- Merged PR #37 (webhook signature verification) and PR #38 (get_block, search_all pagination, fixed silent-truncation bug -> PaginationLimitExceeded) 21:50
- Built CLI UX overhaul: src/cli/output.rs table/JSON renderer, standard error output (error: prefix, exit codes, -v/--verbose), fixed offline commands (convert/diff/upgrade) wrongly requiring NOTION_TOKEN 21:50
- Opened issues #39 (Notion OAuth), #40 (webhook worker), #41 (remaining Notion API surface) for deferred Notion work 21:50
- Drafted and revised docs/ARCHITECTURE.md: Reader/Writer + Source/Sink + filters + Capabilities + ChangeSet design; platform survey (Notion, Lark/Feishu, GitHub Markdown, Obsidian, AppFlowy, Anytype, Craft) led to dropping Google Docs as write-path template (favor Lark batch_update) and adding frontmatter-based remote-ID addressing; PR #43 21:50
- Added paths-ignore to ci.yml/coverage.yml/cross-platform.yml/audit.yml so doc-only changes skip CI (tag pushes, daily audit schedule unaffected) 21:50
- Released v0.4.2: bumped Cargo.toml, cut CHANGELOG Unreleased into 0.4.2 section, synced README/TODO.md/docs/PLAN.md off stale 0.3.1 references; PR #43 merged to main; v0.4.2 git tag + crates.io publish still pending explicit user go-ahead 21:50
- Fixed add-memory.sh hook: PROJECT_ROOT path resolution was one level too shallow (wrote to phantom .claude/.claude/memory/) and the sed multi-line Work Log append was malformed (silently never ran); replaced with a portable awk insert 21:50
