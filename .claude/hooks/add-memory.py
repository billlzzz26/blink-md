#!/usr/bin/env python3
"""Append a work summary to today's session log and MEMORY.md's Work Log.

Entries are filed under a Conventional Commit type heading (feat/fix/docs/
refactor/perf/test/ci/chore/deps, matching the vocabulary already used for
commits and labels — see AGENTS.md), as a GFM numbered list within that
section. One session file per day, cross-linked with MEMORY.md, so multiple
hook calls in the same day accumulate instead of fragmenting into separate
files.

This only records project facts and history. It never restates what
README.md (user-facing pitch/usage) or AGENTS.md (agent rules/workflow)
already say — see AGENTS.md section 12.
"""
import re
import sys
from datetime import datetime
from pathlib import Path

TYPES = {"feat", "fix", "docs", "refactor", "perf", "test", "ci", "chore", "deps"}
DEFAULT_TYPE = "notes"

HEADING_RE = re.compile(r"^(#{1,6})\s+(.*?)\s*$")
NUMBERED_RE = re.compile(r"^\d+\.\s")


def single_line(text: str) -> str:
    return " ".join(text.split())


def parse_summary(summary: str) -> tuple[str, str]:
    summary = single_line(summary)
    m = re.match(r"^([a-z]+)(\([^)]*\))?:\s*(.+)$", summary)
    if m and m.group(1) in TYPES:
        return m.group(1), m.group(3).strip()
    return DEFAULT_TYPE, summary.strip()


def find_heading(lines, level, title, start=0, end=None):
    end = len(lines) if end is None else end
    for i in range(start, end):
        m = HEADING_RE.match(lines[i])
        if m and len(m.group(1)) == level and m.group(2) == title:
            return i
    return None


def section_end(lines, level, idx):
    for i in range(idx + 1, len(lines)):
        m = HEADING_RE.match(lines[i])
        if m and len(m.group(1)) <= level:
            return i
    return len(lines)


def next_item_number(lines, start, end):
    return sum(1 for l in lines[start:end] if NUMBERED_RE.match(l)) + 1


def insert_heading(lines, level, title, at):
    block = [f"{'#' * level} {title}\n", "\n"]
    return lines[:at] + block + lines[at:], at


def insert_numbered(lines, level, idx, text):
    end = section_end(lines, level, idx)
    n = next_item_number(lines, idx + 1, end)
    return lines[:end] + [f"{n}. {text}\n"] + lines[end:]


TIMESTAMP_SUFFIX_RE = re.compile(r"\s*\(\d{2}:\d{2}\)\s*$")


def already_logged(lines, text, start=0, end=None):
    end = len(lines) if end is None else end
    needle = TIMESTAMP_SUFFIX_RE.sub("", text).strip()
    for l in lines[start:end]:
        candidate = TIMESTAMP_SUFFIX_RE.sub("", NUMBERED_RE.sub("", l)).strip()
        if candidate == needle:
            return True
    return False


def normalize(lines):
    """Idempotent cleanup applied once before writing: drop a blank line
    wedged between two numbered-list items of the same list (an artifact of
    inserting into a section that later grew a new heading right after it),
    and guarantee exactly one blank line before every heading."""
    collapsed = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if (
            line.strip() == ""
            and collapsed
            and NUMBERED_RE.match(collapsed[-1])
            and i + 1 < len(lines)
            and NUMBERED_RE.match(lines[i + 1])
        ):
            i += 1
            continue
        collapsed.append(line)
        i += 1

    spaced = []
    for line in collapsed:
        if HEADING_RE.match(line) and spaced and spaced[-1].strip() != "":
            spaced.append("\n")
        if line.strip() == "" and spaced and spaced[-1].strip() == "":
            continue
        spaced.append(line)
    return spaced


def update_session_file(path: Path, day_iso: str, memory_rel_link: str, type_: str, entry: str) -> None:
    if path.exists():
        lines = path.read_text().splitlines(keepends=True)
    else:
        lines = [
            f"# Session Memory — {day_iso}\n",
            "\n",
            f"*Full project memory: [{memory_rel_link}]({memory_rel_link})*\n",
            "\n",
        ]
    if already_logged(lines, entry):
        return
    idx = find_heading(lines, 2, type_)
    if idx is None:
        lines, idx = insert_heading(lines, 2, type_, len(lines))
    lines = insert_numbered(lines, 2, idx, entry)
    path.write_text("".join(normalize(lines)))


def update_memory_file(path: Path, day_iso: str, session_link: str, type_: str, entry: str) -> None:
    if not path.exists():
        return
    lines = path.read_text().splitlines(keepends=True)
    wl_idx = find_heading(lines, 2, "Work Log")
    if wl_idx is None:
        return
    wl_end = section_end(lines, 2, wl_idx)
    day_idx = find_heading(lines, 3, day_iso, wl_idx + 1, wl_end)
    link_line = f"*Details: [{session_link}]({session_link})*\n"
    if day_idx is None:
        block = [f"### {day_iso}\n", "\n", link_line, "\n"]
        lines = lines[: wl_idx + 1] + block + lines[wl_idx + 1 :]
        day_idx = wl_idx + 1
    else:
        # dedupe only within today's section: identical text logged on a
        # different day is a legitimate distinct recurrence, not a repeat.
        day_end_check = section_end(lines, 3, day_idx)
        if already_logged(lines, entry, day_idx, day_end_check):
            return
        has_link = any(session_link in l for l in lines[day_idx : day_idx + 4])
        if not has_link:
            lines = lines[: day_idx + 1] + ["\n", link_line, "\n"] + lines[day_idx + 1 :]
    day_end = section_end(lines, 3, day_idx)
    type_idx = find_heading(lines, 4, type_, day_idx + 1, day_end)
    if type_idx is None:
        lines, type_idx = insert_heading(lines, 4, type_, day_end)
    lines = insert_numbered(lines, 4, type_idx, entry)
    path.write_text("".join(normalize(lines)))


def main() -> None:
    if len(sys.argv) < 2 or not sys.argv[1].strip():
        return
    summary = sys.argv[1].strip()
    type_, text = parse_summary(summary)

    now = datetime.now()
    day_iso = now.strftime("%Y-%m-%d")
    entry = f"{text} ({now.strftime('%H:%M')})"

    root = Path(__file__).resolve().parent.parent.parent
    memory_dir = root / ".claude" / "memory"
    memory_dir.mkdir(parents=True, exist_ok=True)

    session_file = memory_dir / f"session-{day_iso}.md"
    session_rel = f".claude/memory/session-{day_iso}.md"  # repo-root-relative, for the printed message only
    session_link = f"memory/session-{day_iso}.md"  # relative to MEMORY.md's own .claude/ directory
    memory_file = root / ".claude" / "MEMORY.md"
    memory_rel = "../MEMORY.md"  # relative to the session file's .claude/memory/ directory

    update_session_file(session_file, day_iso, memory_rel, type_, entry)
    update_memory_file(memory_file, day_iso, session_link, type_, entry)

    print(f"Memory: {session_rel} [{type_}]")


if __name__ == "__main__":
    main()
