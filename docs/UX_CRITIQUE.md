# blink-md TUI UX Audit — v0.3.1

## Summary
- Overall score: 8/10
- Baseline: `docs/DESIGN.md`
- Critical issues remaining: 0
- Warnings remaining: 3

The basic TUI now uses the design tokens from `docs/DESIGN.md`, shows loading/error status in the footer, and has keyboard help. Remaining work is mostly workflow depth: preview, edit, conflict resolution, and richer live status.

## Resolved in v0.3.1
- Theme integration: `src/cli/theme.rs` maps Notion design tokens to Ratatui colors.
- TUI rendering uses theme colors/borders instead of hardcoded black/white styling.
- Footer shows keyboard hints and current status.
- Loading/error status is set during search/database loads.
- Help popup is implemented for keyboard shortcuts.

## Remaining recommendations
1. Preview page as Markdown through IR before editing or syncing.
2. Edit page in `$EDITOR`, convert back, and push to Notion.
3. Add conflict resolution: local wins, remote wins, merge.
4. Improve live search/status feedback for long-running API calls.

## Verification
- Run `make ci`.
- Run `blink-md tui` and verify theme, footer status, and help popup render correctly.
