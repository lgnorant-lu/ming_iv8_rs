#!/usr/bin/env python3
"""Build the public path-keep list from keep manifests + git ls-files.

Writes paths (relative to repo root, forward slashes) suitable for:
  git filter-repo --paths-from-file <out>

Usage:
  uv run python scripts/public_sync/build_keep_paths.py
  uv run python scripts/public_sync/build_keep_paths.py -o /tmp/keep.txt
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
KEEP_TOP = ROOT / "docs/roadmap/v0.8/analysis/public-paths-keep.txt"
KEEP_SCRIPTS = ROOT / "docs/roadmap/v0.8/analysis/public-paths-scripts-keep.txt"
KEEP_TOOLS = ROOT / "docs/roadmap/v0.8/analysis/public-paths-tools-keep.txt"

# Explicit drops even if under a kept prefix (defense in depth)
EXPLICIT_DROP = {
    "scripts/sample_chrome_surface.py",
}


def _parse_keep_file(path: Path) -> list[str]:
    if not path.is_file():
        return []
    out: list[str] = []
    for ln in path.read_text(encoding="utf-8").splitlines():
        s = ln.strip()
        if not s or s.startswith("#"):
            continue
        out.append(s.replace("\\", "/"))
    return out


def _git_ls_files() -> list[str]:
    raw = subprocess.check_output(
        ["git", "ls-files", "-z"],
        cwd=ROOT,
    )
    return [p.replace("\\", "/") for p in raw.decode("utf-8", errors="replace").split("\0") if p]


def _match_prefix(path: str, rule: str) -> bool:
    rule = rule.rstrip("/")
    if path == rule:
        return True
    if path.startswith(rule + "/"):
        return True
    return False


def build_keep_set() -> set[str]:
    top = _parse_keep_file(KEEP_TOP)
    scripts = _parse_keep_file(KEEP_SCRIPTS)
    tools = _parse_keep_file(KEEP_TOOLS)
    rules = top + scripts + tools

    kept: set[str] = set()
    for path in _git_ls_files():
        if path in EXPLICIT_DROP:
            continue
        for rule in rules:
            if _match_prefix(path, rule):
                kept.add(path)
                break
    return kept


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "-o",
        "--output",
        type=Path,
        default=ROOT / "scripts/public_sync/generated/keep-paths.txt",
        help="Output paths-from-file list",
    )
    ap.add_argument(
        "--print-stats",
        action="store_true",
        help="Print counts to stdout",
    )
    args = ap.parse_args()

    kept = build_keep_set()
    if not kept:
        print("ERROR: keep set is empty", file=sys.stderr)
        return 1

    args.output.parent.mkdir(parents=True, exist_ok=True)
    lines = sorted(kept)
    args.output.write_text("\n".join(lines) + "\n", encoding="utf-8")

    if args.print_stats:
        tops: dict[str, int] = {}
        for p in lines:
            top = p.split("/", 1)[0]
            tops[top] = tops.get(top, 0) + 1
        print(f"keep_paths={len(lines)} -> {args.output}")
        for k in sorted(tops):
            print(f"  {k}: {tops[k]}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
