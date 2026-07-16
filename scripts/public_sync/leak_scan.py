#!/usr/bin/env python3
"""LEAK scan for a tree (typically filter-repo dry-run output).

Exit 0 if clean, 1 if high-severity hits, 2 if only medium findings
(when --strict is set, medium also fails).

Usage:
  uv run python scripts/public_sync/leak_scan.py --root /path/to/filtered
  uv run python scripts/public_sync/leak_scan.py --root . --paths-file keep.txt
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

# (name, pattern, severity)
RULES: list[tuple[str, re.Pattern[str], str]] = [
    ("abs_path_d_dogepy", re.compile(r"D:\\\\dogepy|D:/dogepy", re.I), "high"),
    ("abs_path_users_lenovo", re.compile(r"C:\\\\Users\\\\Lenovo|C:/Users/Lenovo", re.I), "high"),
    ("abs_path_caches", re.compile(r"D:\\\\Caches|D:/Caches", re.I), "medium"),
    ("private_key_pem", re.compile(r"BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY"), "high"),
    ("github_pat", re.compile(r"ghp_[A-Za-z0-9]{20,}"), "high"),
    ("openai_sk", re.compile(r"sk-[A-Za-z0-9]{20,}"), "high"),
    ("aws_akia", re.compile(r"AKIA[0-9A-Z]{16}"), "high"),
    ("site_xhs", re.compile(r"xiaohongshu|\u5c0f\u7ea2\u4e66", re.I), "medium"),
    ("site_meituan", re.compile(r"meituan|\u7f8e\u56e2", re.I), "medium"),
    ("site_tcaptcha", re.compile(r"tcaptcha|TCaptcha", re.I), "medium"),
    ("cdp_local_9223", re.compile(r"127\.0\.0\.1:9223"), "medium"),
    ("sample_chrome_script", re.compile(r"sample_chrome_surface"), "high"),
    ("opencode_temp", re.compile(r"AppData\\\\Local\\\\Temp\\\\opencode", re.I), "medium"),
    ("roadmap_private", re.compile(r"docs/roadmap/v0\.8/(analysis|native-substrate)/"), "medium"),
    ("todo_private", re.compile(r"docs/todo/TODO-"), "medium"),
]

# Medium cross-links allowed in product changelog
ALLOW_MEDIUM_IN_PREFIX = (
    "CHANGELOG.md",
)

# Do not scan these path segments (binary/tool noise)
SKIP_DIR_NAMES = {
    ".git",
    ".venv",
    "venv",
    "node_modules",
    "target",
    "target-maturin",
    "__pycache__",
    ".uv",
}

SKIP_SUFFIXES = {
    ".png",
    ".jpg",
    ".jpeg",
    ".gif",
    ".webp",
    ".ico",
    ".woff",
    ".woff2",
    ".ttf",
    ".eot",
    ".pdf",
    ".dat",
    ".pyd",
    ".so",
    ".dll",
    ".exe",
    ".lock",
    ".bin",
    ".whl",
}

# Scanner / pipeline self-docs mention denylist patterns by design
SELF_PATH_PREFIXES = (
    "scripts/public_sync/",
)


def _is_self_path(rel: str) -> bool:
    return any(rel == p.rstrip("/") or rel.startswith(p) for p in SELF_PATH_PREFIXES)


def _iter_files(root: Path, paths_file: Path | None) -> list[Path]:
    if paths_file:
        rels = [
            ln.strip().replace("\\", "/")
            for ln in paths_file.read_text(encoding="utf-8").splitlines()
            if ln.strip() and not ln.strip().startswith("#")
        ]
        return [root / r for r in rels if (root / r).is_file()]
    out: list[Path] = []
    for p in root.rglob("*"):
        if not p.is_file():
            continue
        if any(part in SKIP_DIR_NAMES for part in p.parts):
            continue
        if p.suffix.lower() in SKIP_SUFFIXES:
            continue
        # skip extensionless binaries under .venv already covered; also skip large files
        try:
            if p.stat().st_size > 2_000_000:
                continue
        except OSError:
            continue
        out.append(p)
    return out


def scan(root: Path, paths_file: Path | None) -> list[tuple[str, str, str, int]]:
    """Return list of (severity, rule, path, line_no)."""
    hits: list[tuple[str, str, str, int]] = []
    for fp in _iter_files(root, paths_file):
        try:
            text = fp.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        rel = str(fp.relative_to(root)).replace("\\", "/")
        if _is_self_path(rel):
            continue
        for i, line in enumerate(text.splitlines(), 1):
            for name, pat, sev in RULES:
                if pat.search(line):
                    if sev == "medium" and any(
                        rel == p or rel.startswith(p + "/") for p in ALLOW_MEDIUM_IN_PREFIX
                    ):
                        continue
                    hits.append((sev, name, rel, i))
    return hits


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, required=True)
    ap.add_argument("--paths-file", type=Path, default=None)
    ap.add_argument(
        "--strict",
        action="store_true",
        help="Fail on medium findings as well as high",
    )
    ap.add_argument(
        "--report",
        type=Path,
        default=None,
        help="Write full report markdown",
    )
    args = ap.parse_args()

    root = args.root.resolve()
    if not root.is_dir():
        print(f"ERROR: root not a directory: {root}", file=sys.stderr)
        return 1

    hits = scan(root, args.paths_file)
    high = [h for h in hits if h[0] == "high"]
    medium = [h for h in hits if h[0] == "medium"]

    lines = [
        "# LEAK scan report",
        "",
        f"- root: `{root}`",
        f"- high: {len(high)}",
        f"- medium: {len(medium)}",
        "",
    ]
    if high:
        lines.append("## HIGH")
        for _sev, name, path, ln in high[:200]:
            lines.append(f"- `{path}:{ln}` [{name}]")
        lines.append("")
    if medium:
        lines.append("## MEDIUM")
        for _sev, name, path, ln in medium[:200]:
            lines.append(f"- `{path}:{ln}` [{name}]")
        lines.append("")

    report = "\n".join(lines)
    print(report)
    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(report + "\n", encoding="utf-8")

    if high:
        return 1
    if args.strict and medium:
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
