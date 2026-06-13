# UX Critique Report: notion-rs TUI

## Summary
- **Overall Score**: 6.5/10
- **Baseline**: `DESIGN.md` (Notion-inspired system)
- **Critical Issues**: 2
- **Warnings**: 3

The TUI provides basic navigation and inspection capabilities for Notion entities. However, it lacks visual polish, consistent branding with the defined Design System, and accessibility features like proper focus indicators and color semantic mapping.

---

## Heuristic Evaluation (Nielsen's 10)

| # | Heuristic | Score | Finding |
|---|-----------|-------|---------|
| 1 | Visibility of system status | **WARN** | Loading states are missing during API calls (Users/Pages). The UI appears "frozen" until the response arrives. |
| 2 | Match between system and real world | **PASS** | Tab names and entity labels match Notion's domain language. |
| 3 | User control and freedom | **PASS** | 'q' and 'Esc' are standard for exiting. Tab switching is intuitive. |
| 4 | Consistency and standards | **FAIL** | Styles are hardcoded (Color::Black/White) and do not use the tokens defined in `DESIGN.md`. |
| 5 | Error prevention | **PASS** | Read-only TUI prevents accidental data modification. |
| 6 | Recognition rather than recall | **WARN** | Navigation keys (j/k, Tab) are not visible on screen. Users must remember them. |
| 7 | Flexibility and efficiency of use | **PASS** | Vim-style keys (j/k) are present for power users. |
| 8 | Aesthetic and minimalist design | **WARN** | Layout is functional but utilitarian. Lacks the "Notion Blue" accent and soft borders defined in the design system. |
| 9 | Help and recovery | **FAIL** | No error messages are displayed in the UI if an API call fails; it likely crashes or shows nothing. |
| 10| Help and documentation | **FAIL** | No help overlay ('?') or footer with keyboard hints. |

---

## Accessibility Audit (WCAG 2.1 - Terminal Context)

| Level | Criterion | Status | Detail |
|-------|-----------|--------|--------|
| AA | 1.4.3 Contrast | **WARN** | Uses default Terminal White/Black. May clash with user's theme if not careful. |
| A | 2.1.1 Keyboard | **PASS** | Fully keyboard navigable. |
| AA | 2.4.7 Focus Visible | **PASS** | Highlighting is clear, but uses "REVERSED" instead of accent colors. |
| A | 3.2.1 On Focus | **PASS** | Context changes predictably when selecting items. |

---

## Prioritized Recommendations

### 1. [CRITICAL] Token Integration (Consistency)
**Issue**: TUI uses hardcoded `Color::White` and `Color::Black` instead of matching `DESIGN.md`.
**Fix**: Define a `Theme` struct mapping `DESIGN.md` hex codes (mapped to nearest ANSI or RGB) to Ratatui styles.
```rust
// Proposed mapping
let ACCENT_BLUE = Color::Rgb(35, 131, 226); // #2383E2
let BORDER_GRAY = Color::Rgb(233, 233, 231); // #E9E9E7
```

### 2. [CRITICAL] Error Handling & System Status
**Issue**: No feedback during loading or network errors.
**Fix**: Add a `status_message` field to `App` and render it in a footer or popup. Use a spinner or "Loading..." text in the detail panel while async tasks run.

### 3. [WARNING] Keyboard Hints (Recognition)
**Issue**: Navigation is hidden.
**Fix**: Add a footer block showing `[q]uit [tab]switch [j/k]move [enter]expand [?]help`.

### 4. [IMPROVEMENT] Visual Hierarchy
**Issue**: The "Detail" view is a wall of plain text.
**Fix**: Use different colors for keys and values (e.g., `Accent` for Keys, `Primary` for values) and add padding.

---

## Technical Audit: Hardcoded Values vs Tokens

| File | Line | Hardcoded Value | Should be Token |
|------|------|-----------------|-----------------|
| `tui.rs` | 338 | `Color::White` | `--primary` |
| `tui.rs` | 344 | `Modifier::REVERSED` | `--accent` (Background) |
| `tui.rs` | 400 | `Color::Black/White` | `--surface` / `--primary` |
| `tui.rs` | 332 | `Borders::ALL` | `--border` |
