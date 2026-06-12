# Project Learnings & Durable Patterns

## Session: 2026-06-05 (Structural Integrity & Protocol Adherence)

### 🚨 The Problem (Reactive Destruction)
When confronted with errors, user corrections, or strict tool validation rules (e.g., Figma architecture rules), the default agentic response was **Reactive Destruction** — throwing away existing work, creating new artifact files instead of updating them, and overwriting historical context (like `TODO.md` and `PLAN.md`) just to quickly appease the user. This caused workspace clutter and loss of project continuity.

### 💡 The Durable Lesson
Stewardship of the workspace and strict adherence to operational protocols (like reading skill instructions deeply and merging changes) are vastly more important than simply forcing a task to completion. As stated in the Figma skill: *"One good question beats one wasted diagram."*

### ⚡ Trigger Conditions (When to use this pattern)
- Receiving negative feedback, harsh critique, or corrections from the user.
- Encountering strict validation errors from external tools (e.g., Figma MCP, Compilers).
- Being asked to change the direction of an implementation plan or architecture.
- Feeling the urge to run a destructive command (`rm`, full file overwrite, spawning duplicate drafts).

### ✅ Reusable Checklist (The Anti-Panic Protocol)
1. **[ ] Stop & Absorb**: Do not immediately execute or propose a solution. Read the error or feedback carefully. Do not apologize; focus on the technical failure.
2. **[ ] Consult Protocols**: Re-read relevant skill instructions (`SKILL.md`) or core project mandates (`AGENTS.md`) before proceeding. 
3. **[ ] Merge, Never Overwrite**: If the plan changes, integrate the new requirements into the existing `PLAN.md` or `TODO.md` without deleting historical context.
4. **[ ] Reuse Artifacts**: Always use identifiers (like `fileKey` for Figma) to update existing external artifacts instead of spawning new ones.
5. **[ ] Context Before Execution**: Update the rules in `AGENTS.md` or `LEARN.md` *before* attempting the task again to ensure the behavior is permanently corrected.
