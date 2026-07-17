#!/usr/bin/env python3
"""Classify staged (or listed) paths against public keep rules.

Fast gate for pre-commit: does NOT run full filter-repo dry-run.
Prints KEEP / DROP / UNTRACKED-RULE for each path so agents verify
public-surface files are on the keep list before commit.

Exit 0 always by default (advisory). Exit 1 if --strict and any staged
path under a public-ish prefix is not kept (docs/api, README, crates, ...).

Usage:
  uv run python scripts/public_sync/check_staged_paths.py
  uv run python scripts/public_sync/check_staged_paths.py --strict
  uv run python scripts/public_sync/check_staged_paths.py path1 path2
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(Path(__file__).resolve().parent))
from build_keep_paths import (  # noqa: E402
    KEEP_SCRIPTS,
    KEEP_TOOLS,
    KEEP_TOP,
    _match_prefix,
    _parse_keep_file,
    build_keep_set,
)

# Prefixes that usually belong on the public surface when added/edited
PUBLICISH_PREFIXES = (
    "crates/",
    "python/",
    "tests/",
    ".github/",
    "README.md",
    "README.zh-CN.md",
    "LICENSE",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    "docs/api/",
    "docs/conventions/",
    "docs/source/",
    "docs/releases/",
    "docs/GUIDE.public.md",
    "docs/quality-harness/",
    "scripts/public_sync/",
    "pyproject.toml",
    "Cargo.toml",
    "Cargo.lock",
)


def _staged_paths() -> list[str]:
    raw = subprocess.check_output(
        ["git", "diff", "--cached", "--name-only", "-z"],
        cwd=ROOT,
    )
    return [p.replace("\\", "/") for p in raw.decode().split("\0") if p]


def _is_publicish(path: str) -> bool:
    for pref in PUBLICISH_PREFIXES:
        if path == pref.rstrip("/") or path.startswith(pref):
            return True
    return False


def _classify(path: str, keep: set[str], rules: list[str]) -> str:
    if path in keep:
        return "KEEP"
    for rule in rules:
        if _match_prefix(path, rule):
            # rule matches prefix but file not in keep set (e.g. not tracked yet)
            return "KEEP-RULE"
    return "DROP"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("paths", nargs="*", help="paths (default: staged)")
    ap.add_argument(
        "--strict",
        action="store_true",
        help="fail if public-ish staged path is DROP",
    )
    args = ap.parse_args()
    paths = [p.replace("\\", "/") for p in args.paths] if args.paths else _staged_paths()
    if not paths:
        print("check_staged_paths: no staged paths")
        return 0

    keep = build_keep_set()
    rules = (
        _parse_keep_file(KEEP_TOP)
        + _parse_keep_file(KEEP_SCRIPTS)
        + _parse_keep_file(KEEP_TOOLS)
    )

    bad: list[str] = []
    print("path classification (public keep filter):")
    for path in sorted(set(paths)):
        kind = _classify(path, keep, rules)
        mark = ""
        if kind == "DROP" and _is_publicish(path):
            mark = "  << public-ish but DROP: add to keep-top/scripts/tools or do not expect on public"
            bad.append(path)
        elif kind == "DROP":
            mark = "  (private-only; OK if intentional)"
        print(f"  {kind:10} {path}{mark}")

    if args.strict and bad:
        print("FAIL: public-ish paths not on keep list:", ", ".join(bad), file=sys.stderr)
        print("Update scripts/public_sync/manifests/keep-*.txt then rebuild keep-paths.", file=sys.stderr)
        return 1
    if bad:
        print("WARN: public-ish DROP paths present (non-strict). Review before funnel.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
