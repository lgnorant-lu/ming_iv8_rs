#!/usr/bin/env python3
"""Download Chromium Blink IDL files via git sparse-checkout and parse them.

This script performs a sparse checkout of the Chromium source tree, limited to
the Blink renderer IDL directories, then invokes parse-chromium-idl.js (Node.js)
to produce chromium-148.ir.json.

Steps:
  1. git clone --filter=blob:none --depth=1 --sparse chromium-src
  2. git sparse-checkout set third_party/blink/renderer/core \
       third_party/blink/renderer/modules
  3. git fetch origin refs/tags/<version>:refs/tags/<version>
  4. git checkout refs/tags/<version>
  5. Collect all .idl file paths
  6. Run: node parse-chromium-idl.js --dir <idl-root> --recursive --ir \
       --output <chromium-148.ir.json>

Requirements:
  - git (with sparse-checkout support, git >= 2.25)
  - Node.js >= 18 with tools/idl/node_modules installed (npm install)

Usage:
  .venv\\Scripts\\python.exe scripts/download_chromium_idl.py [--version 148.0.7778.1] [--dest data/chromium-idl]

The checkout is large (~1-2 GB even sparse). Run only when network and disk
allow. This script is idempotent: if the checkout already exists, it reuses it.
"""
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
CHROMIUM_URL = "https://chromium.googlesource.com/chromium/src.git"
SPARSE_PATHS = [
    "third_party/blink/renderer/core",
    "third_party/blink/renderer/modules",
]
IDL_TOOL = REPO_ROOT / "tools" / "idl" / "parse-chromium-idl.js"

DEFAULT_VERSION = "148.0.7778.1"
DEFAULT_DEST = REPO_ROOT / "data" / "chromium-idl"


def run(cmd: list[str], cwd: Path | None = None, check: bool = True) -> str:
    print(f"  $ {' '.join(cmd)}")
    result = subprocess.run(
        cmd, cwd=cwd, capture_output=True, text=True, encoding="utf-8",
        errors="replace",
    )
    if result.stdout:
        print(result.stdout, end="")
    if result.stderr:
        print(result.stderr, end="", file=sys.stderr)
    if check and result.returncode != 0:
        raise RuntimeError(f"Command failed (exit {result.returncode}): {cmd[0]}")
    return result.stdout


def clone_sparse(dest: Path) -> None:
    """Clone with sparse filter if dest doesn't exist."""
    if dest.exists() and (dest / ".git").exists():
        print(f"[download_chromium_idl] Reusing existing checkout: {dest}")
        return
    if dest.exists():
        shutil.rmtree(dest)
    dest.parent.mkdir(parents=True, exist_ok=True)
    print("[download_chromium_idl] Step 1: sparse clone (this may take a while)...")
    run([
        "git", "clone",
        "--filter=blob:none",
        "--depth=1",
        "--sparse",
        "--no-checkout",
        CHROMIUM_URL,
        str(dest),
    ])


def configure_sparse(dest: Path) -> None:
    print("[download_chromium_idl] Step 2: configuring sparse-checkout paths...")
    run(["git", "sparse-checkout", "init"], cwd=dest)
    run(["git", "sparse-checkout", "set"] + SPARSE_PATHS, cwd=dest)


def checkout_version(dest: Path, version: str) -> None:
    print(f"[download_chromium_idl] Step 3: fetching tag {version}...")
    try:
        run([
            "git", "fetch", "--depth=1", "origin",
            f"refs/tags/{version}:refs/tags/{version}",
        ], cwd=dest)
    except RuntimeError:
        # Some tags may need full fetch; try without depth
        run([
            "git", "fetch", "origin",
            f"refs/tags/{version}:refs/tags/{version}",
        ], cwd=dest)
    print(f"[download_chromium_idl] Step 4: checking out {version}...")
    run(["git", "checkout", f"refs/tags/{version}"], cwd=dest)


def find_idl_files(dest: Path) -> list[Path]:
    """Collect all .idl files under the sparse paths."""
    idl_files: list[Path] = []
    for sp in SPARSE_PATHS:
        root = dest / sp
        if root.exists():
            idl_files.extend(sorted(root.rglob("*.idl")))
    return idl_files


def parse_with_node(idl_root: Path, output: Path) -> None:
    """Run parse-chromium-idl.js recursively to produce IR JSON."""
    print(f"[download_chromium_idl] Step 6: parsing IDL with Node.js...")
    output.parent.mkdir(parents=True, exist_ok=True)
    node = shutil.which("node")
    if not node:
        raise RuntimeError("node not found in PATH")
    if not IDL_TOOL.exists():
        raise RuntimeError(f"parse-chromium-idl.js not found: {IDL_TOOL}")
    run([
        node, str(IDL_TOOL),
        "--dir", str(idl_root),
        "--recursive",
        "--ir",
        "--output", str(output),
    ])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", default=DEFAULT_VERSION,
                        help=f"Chromium tag (default: {DEFAULT_VERSION})")
    parser.add_argument("--dest", type=Path, default=DEFAULT_DEST,
                        help="Destination directory for checkout")
    parser.add_argument("--output", type=Path,
                        default=REPO_ROOT / "tools" / "idl" / "output" / "chromium-148.ir.json",
                        help="Output IR JSON path")
    parser.add_argument("--skip-checkout", action="store_true",
                        help="Skip clone/checkout, only re-parse existing files")
    parser.add_argument("--list-only", action="store_true",
                        help="Only list .idl files, do not parse")
    args = parser.parse_args()

    dest = args.dest.resolve()

    if not args.skip_checkout:
        clone_sparse(dest)
        configure_sparse(dest)
        checkout_version(dest, args.version)

    print("[download_chromium_idl] Step 5: collecting .idl files...")
    idl_files = find_idl_files(dest)
    print(f"[download_chromium_idl] Found {len(idl_files)} .idl files")
    if not idl_files:
        print("[download_chromium_idl] ERROR: no .idl files found. "
              "Is the checkout complete?", file=sys.stderr)
        return 1

    # Write file list for reference
    file_list = dest.parent / "chromium-idl-files.txt"
    file_list.write_text(
        "\n".join(str(f.relative_to(dest)) for f in idl_files) + "\n",
        encoding="utf-8",
    )
    print(f"[download_chromium_idl] File list written to: {file_list}")

    if args.list_only:
        return 0

    # Use the common ancestor of all IDL files as the parse root
    idl_roots = set()
    for sp in SPARSE_PATHS:
        root = dest / sp
        if root.exists():
            idl_roots.add(root)

    # parse each sparse path separately, then merge — or use the topmost
    # common dir. Since both are under third_party/blink/renderer, use that.
    parse_root = dest / "third_party" / "blink" / "renderer"
    if not parse_root.exists():
        parse_root = dest

    try:
        parse_with_node(parse_root, args.output)
    except RuntimeError as e:
        print(f"[download_chromium_idl] Parse failed: {e}", file=sys.stderr)
        return 1

    print(f"[download_chromium_idl] Done. IR output: {args.output}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
