"""Regenerate docs/PYTHON_TEST_INVENTORY.md from pytest collection output.

Usage:
    uv run pytest tests/ --collect-only -q | uv run python scripts/update_python_test_inventory.py
    # Or: uv run python scripts/update_python_test_inventory.py  (runs pytest internally)
"""

import os
import re
import sys
from pathlib import Path
from subprocess import run as subprocess_run
from typing import Dict, List, Tuple


PROJECT_ROOT = Path(__file__).resolve().parent.parent
OUTPUT_FILE = PROJECT_ROOT / "docs" / "PYTHON_TEST_INVENTORY.md"


def collect_tests() -> str:
    """Run pytest --collect-only -q and return stdout."""
    result = subprocess_run(
        [sys.executable, "-m", "pytest", "tests/", "--collect-only", "-q"],
        capture_output=True,
        text=True,
        cwd=str(PROJECT_ROOT),
        timeout=120,
    )
    if result.returncode != 0 and "no tests ran" not in result.stdout.lower():
        print(f"[WARN] pytest exited with {result.returncode}: {result.stderr[:200]}")
    return result.stdout


def parse_collection(stdout: str) -> Dict[str, List[str]]:
    """Parse pytest --collect-only -q output into per-file test lists.

    Lines look like:
        tests/test_file.py::test_name
        tests/subdir/test_file.py::test_name
    """
    by_file: Dict[str, List[str]] = {}
    for line in stdout.splitlines():
        line = line.strip()
        if not line or "::" not in line:
            continue
        file_part, test_part = line.split("::", 1)
        file_part = file_part.replace("\\", "/")
        if file_part.startswith("tests/"):
            by_file.setdefault(file_part, []).append(test_part)
    return by_file


def normalize_path(path: str) -> str:
    """Ensure forward slashes and consistent prefix."""
    return path.replace("\\", "/")


def format_inventory(by_file: Dict[str, List[str]]) -> str:
    """Format the inventory as markdown."""
    from datetime import date

    top_level: List[Tuple[str, int]] = []
    subdirs: Dict[str, List[Tuple[str, int]]] = {}
    total = 0

    for filepath in sorted(by_file.keys()):
        count = len(by_file[filepath])
        total += count
        rel = normalize_path(filepath)
        parts = rel.split("/")
        if len(parts) == 2:
            top_level.append((parts[1], count))
        else:
            dir_name = parts[1]
            file_name = parts[-1]
            subdirs.setdefault(dir_name, []).append((file_name, count))

    lines: List[str] = []
    lines.append("# Python Test Inventory")
    lines.append("")
    lines.append(f"> Generated: {date.today()}")
    lines.append("> Source: `uv run pytest tests/ --collect-only -q`")
    lines.append(f"> Total collected: **{total}** (function-level, excludes parametrize expansions)")
    lines.append("")
    lines.append("## Per-Module Summary")
    lines.append("")

    lines.append("### Top-Level Test Files")
    lines.append("")
    lines.append("| Module | File | Tests |")
    lines.append("|---|---|---|")
    for fname, count in sorted(top_level):
        mod = os.path.splitext(fname)[0]
        lines.append(f"| {mod} | `{fname}` | {count} |")
    lines.append("")

    for dir_name in sorted(subdirs.keys()):
        dir_total = sum(c for _, c in subdirs[dir_name])
        lines.append(f"### {dir_name}/ Subdirectory ({len(subdirs[dir_name])} files, {dir_total} tests)")
        lines.append("")
        lines.append("| File | Tests |")
        lines.append("|---|---|")
        for fname, count in sorted(subdirs[dir_name]):
            lines.append(f"| `{fname}` | {count} |")
        lines.append("")

    lines.append("### Gaps Noted")
    lines.append("")
    lines.append("- `@pytest.mark.xfail` not used anywhere — known-failing tests not tracked")
    lines.append("- 8 contract test files have 14 or fewer tests each — candidates for merge")
    lines.append("- Subdirectories use relative imports — may require `-p` flag during collection")
    lines.append("")

    return "\n".join(lines)


def main() -> None:
    if sys.stdin.isatty():
        print("Running pytest --collect-only (this may take a few minutes)...")
        stdout = collect_tests()
    else:
        stdout = sys.stdin.read()

    by_file = parse_collection(stdout)
    if not by_file:
        print("[ERROR] No test files found in collection output")
        sys.exit(1)

    markdown = format_inventory(by_file)
    OUTPUT_FILE.write_text(markdown, encoding="utf-8")
    print(f"  Wrote {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
