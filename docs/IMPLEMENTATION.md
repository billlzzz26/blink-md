# IMPLEMENTATION PLAN: Notion-rs TUI & Design System

## 1. Phase Overview
1. **Phase 1: Design System Foundation** — Implement the core `Theme` struct mapping `DESIGN.md` tokens to Ratatui styles.
2. **Phase 2: TUI Layout & Typography** — Update the TUI layout to respect the grid, spacing, and typography scale constraints.
3. **Phase 3: Component Styling & State** — Apply semantic colors, borders, and interactive states to TUI components (e.g., tabs, lists, detail views).
4. **Phase 4: Polish & Accessibility** — Implement proper focus indicators, keyboard hints, and error/loading states (addressing UX critique).

---

## 2. Execution Phases

### Phase 1: Design System Foundation
**Objective**: Establish a centralized theme system that translates the `DESIGN.md` color palette and roles into usable Ratatui structures.
- **Deliverables**:
  - `src/cli/theme.rs`: A new module defining the `Theme` struct and color constants (Primary `#37352F`, Accent `#2383E2`, Surface `#FFFFFF`, etc.).
  - Update `src/cli/mod.rs` to expose the `theme` module.
- **Dependencies**: 
  - `ratatui` crate (already in `Cargo.toml`).
  - `DESIGN.md` palette definitions.
- **Verification criteria**: 
  - `cargo check` passes.
  - The `Theme` struct is successfully instantiated in `src/cli/tui.rs` without compilation errors.
  - *Command*: `cargo test --all-targets && cargo check`

### Phase 2: TUI Layout & Typography
**Objective**: Structure the terminal user interface to reflect the minimalist layout principles (spacing, grid constraints).
- **Deliverables**:
  - Refactored `src/cli/tui.rs`: Update the layout constraints using Ratatui's `Layout` to respect the 4px base unit (translated to terminal cell proportions, e.g., standard margins).
  - Implementation of a maximum width constraint (simulating the 900px max width where applicable in the terminal context).
- **Dependencies**:
  - Phase 1 (Theme Foundation).
- **Verification criteria**:
  - The TUI renders without overlapping elements.
  - Structural separation between the navigation list and the detail pane is clear.
  - *Command*: `cargo run -- tui` (Visual inspection).

### Phase 3: Component Styling & State
**Objective**: Apply the `Theme` tokens to all visible TUI components, removing hardcoded colors.
- **Deliverables**:
  - `src/cli/tui.rs`: Replace all instances of `Color::White`, `Color::Black`, and `Modifier::REVERSED` with explicit theme token references (e.g., `theme.accent`, `theme.primary_text`, `theme.surface`).
  - Style the Tabs to use the Notion Blue (`#2383E2`) for the active state and Stone Gray (`#9B9A97`) for inactive.
  - Style the Detail View to use Accent Blue for keys and Primary text for values.
- **Dependencies**:
  - Phase 1 & 2.
- **Verification criteria**:
  - No hardcoded ANSI colors exist in `tui.rs` for primary styling.
  - The UI visually reflects the Notion color palette.
  - *Command*: `cargo clippy --all-targets -- -D warnings` (Ensure no unused theme fields).

### Phase 4: Polish & Accessibility
**Objective**: Address the UX Critique findings by implementing system status feedback and keyboard navigation hints.
- **Deliverables**:
  - `src/cli/tui.rs`: Add a Footer block displaying keyboard hints (`[q]uit [tab]switch [j/k]move [enter]expand [?]help`).
  - `src/cli/tui.rs`: Implement a `status_message` area or popup to display Loading states and API errors using Semantic Status colors (Success `#448361`, Error `#D44C47`).
- **Dependencies**:
  - Phase 3.
- **Verification criteria**:
  - The footer is visible at the bottom of the screen.
  - Triggering an API call visually indicates a loading state.
  - *Command*: `cargo run -- tui` and perform navigation actions.

---

## 3. Journal

*(Empty - for progress tracking during execution)*
