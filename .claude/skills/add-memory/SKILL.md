---
name: add-memory
description: "Append a work summary to today's session log and .claude/MEMORY.md's Work Log, filed under a Conventional Commit type section."
---

# Add Memory

## 1. Usage

`.claude/skills/add-memory/scripts/add-memory.sh "type: work summary"`

The `type:` prefix is optional. If it matches one of `feat`, `fix`, `docs`,
`refactor`, `perf`, `test`, `ci`, `chore`, `deps` (the same vocabulary as
commit messages and labels — see `AGENTS.md`), the entry is filed under that
section. Otherwise it lands under `notes`.

`.claude/hooks/add-memory.sh` is a thin forwarding stub for the
`session_end` hook (see `.claude/hooks/hooks.json`) — that trigger needs a
script at a fixed hooks/ path. Call the skill's own script directly for
everything else.

## 2. Portable by design

This skill (this whole `.claude/skills/add-memory/` folder — `scripts/`
and `templates/` included) is meant to be copied into other projects
as-is. It never assumes anything about the project it runs in:

1. `scripts/add-memory.py` finds the project root by walking up from its
   own location (looking for `.git`, or the `.claude` folder it lives
   under), not by a fixed number of parent directories.
2. `templates/MEMORY.md` and `templates/session.md` are what a missing
   `.claude/MEMORY.md` or session log gets bootstrapped from — generic,
   with no content specific to any one project. Edit these templates, not
   the bootstrap logic, when the starting shape needs to change.

## 3. What each file is for

1. `.claude/memory/session-<YYYY-MM-DD>.md` — one file per day, all hook
   calls that day accumulate into it instead of fragmenting into separate
   files. Raw, in the moment.
2. `.claude/MEMORY.md`'s `## Work Log` — the same entries, curated over
   time: prune anything no longer worth keeping, keep the rest. Each day
   heading links to that day's session file (`*Details: [...]*`), and each
   session file links back to `MEMORY.md`, so either file can be read alone.

## 4. Format

Entries are a GFM numbered list under a `####` (in `MEMORY.md`) or `##` (in
the session file) type heading:

```markdown
#### feat

1. add --sort flag to search (14:32)
2. add --limit flag to search (14:40)
```

## 5. Non-duplication

`MEMORY.md` and the session logs record project facts and history only —
architecture, current focus, what changed and why. They never restate:

1. `README.md`'s job — the user-facing pitch, install/usage, feature list.
2. `AGENTS.md`'s job — agent rules and workflow.

Reference those files by name instead of copying their content.

## 6. Rules

1. Do not overwrite existing entries; only append.
2. Do not duplicate an entry already logged today (the hook dedupes by
   text, ignoring the list number and timestamp).
3. Keep entries factual, concise, and past-tense.
