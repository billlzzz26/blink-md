---
name: add-memory
description: "Append a work summary to today's session log and .claude/MEMORY.md's Work Log, filed under a Conventional Commit type section."
---

# Add Memory

## 1. Usage

`.claude/hooks/add-memory.sh "type: work summary"`

The `type:` prefix is optional. If it matches one of `feat`, `fix`, `docs`,
`refactor`, `perf`, `test`, `ci`, `chore`, `deps` (the same vocabulary as
commit messages and labels — see `AGENTS.md`), the entry is filed under that
section. Otherwise it lands under `notes`.

## 2. What each file is for

1. `.claude/memory/session-<YYYY-MM-DD>.md` — one file per day, all hook
   calls that day accumulate into it instead of fragmenting into separate
   files. Raw, in the moment.
2. `.claude/MEMORY.md`'s `## Work Log` — the same entries, curated over
   time: prune anything no longer worth keeping, keep the rest. Each day
   heading links to that day's session file (`*Details: [...]*`), and each
   session file links back to `MEMORY.md`, so either file can be read alone.

## 3. Format

Entries are a GFM numbered list under a `####` (in `MEMORY.md`) or `##` (in
the session file) type heading:

```markdown
#### feat

1. add --sort flag to search (14:32)
2. add --limit flag to search (14:40)
```

## 4. Non-duplication

`MEMORY.md` and the session logs record project facts and history only —
architecture, current focus, what changed and why. They never restate:

1. `README.md`'s job — the user-facing pitch, install/usage, feature list.
2. `AGENTS.md`'s job — agent rules and workflow.

Reference those files by name instead of copying their content.

## 5. Rules

1. Do not overwrite existing entries; only append.
2. Do not duplicate an entry already logged today (the hook dedupes by
   text, ignoring the list number and timestamp).
3. Keep entries factual, concise, and past-tense.
