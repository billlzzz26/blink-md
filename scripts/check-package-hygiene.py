#!/usr/bin/env python3
"""Fail CI if cargo package includes local-agent or internal documentation files."""

from __future__ import annotations

import subprocess
import sys

FORBIDDEN_PREFIXES = (
    ".gemini/",
    ".qwen/",
    "docs/mcp/conductor/",
)

FORBIDDEN_SUFFIXES = (
    ".key",
    ".pem",
    ".secret",
)


def main() -> int:
    try:
        output = subprocess.check_output(
            ["cargo", "package", "--list", "--allow-dirty"],
            text=True,
            stderr=subprocess.STDOUT,
        )
    except subprocess.CalledProcessError as exc:
        print(exc.output, end="")
        return exc.returncode

    paths = output.splitlines()
    bad = [
        path
        for path in paths
        if path.startswith(FORBIDDEN_PREFIXES) or path.endswith(FORBIDDEN_SUFFIXES)
    ]

    if bad:
        print("Forbidden files would be included in cargo package:")
        for path in bad:
            print(f"- {path}")
        return 1

    print("Package hygiene check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
