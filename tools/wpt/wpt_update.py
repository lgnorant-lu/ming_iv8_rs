#!/usr/bin/env python3
"""WPT file downloader — fetch WPT official test files and resources.

Downloads testharness.js, idlharness.js, WebIDLParser.js, and WPT official
test files (idlharness.https.html, etc.) from the WPT GitHub repository.
Also copies webref IDL files from the existing @webref/idl installation.

Usage:
  python tools/wpt/wpt_update.py            # download/update all
  python tools/wpt/wpt_update.py html/dom   # download specific test only
"""
from __future__ import annotations

import json
import shutil
import urllib.request
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
WPT_DIR = Path(__file__).resolve().parent
FIXTURES_DIR = WPT_DIR / "fixtures"
RESOURCES_DIR = FIXTURES_DIR / "resources"
INTERFACES_DIR = FIXTURES_DIR / "interfaces"
VERSIONS_PATH = WPT_DIR / "versions.json"
STATUS_DIR = WPT_DIR / "status"

WPT_BASE = "https://raw.githubusercontent.com/web-platform-tests/wpt/master/"
WEBREF_DIR = REPO_ROOT / "tools" / "idl" / "node_modules" / "@webref" / "idl"
WEBIDL2_PATH = REPO_ROOT / "tools" / "idl" / "node_modules" / "webidl2" / "dist" / "webidl2.js"

# WPT resource files to download
WPT_RESOURCES = [
    "resources/testharness.js",
    "resources/testharnessreport.js",
    "resources/idlharness.js",
    "resources/WebIDLParser.js",
]

# WPT test files to download (path in WPT repo -> local fixture path)
WPT_TEST_FILES = {
    "html/dom/idlharness.https.html": "html/dom/idlharness.https.html",
    "dom/idlharness.window.js": "dom/idlharness.window.js",
    "css/cssom-view/idlharness.html": "css/cssom-view/idlharness.html",
}

# IDL files needed (spec names) — must match WPT test file deps
# These are copied from @webref/idl
WPT_IDL_SPECS = [
    # html/dom/idlharness.https.html deps
    "html", "wai-aria", "SVG", "cssom", "touch-events", "pointerevents",
    "uievents", "dom", "xhr", "FileAPI", "mediacapture-streams",
    "performance-timeline", "trusted-types",
    # dom/idlharness.window.js deps
    "fullscreen",
    # css/cssom-view/idlharness.html deps
    "cssom-view", "css-pseudo",
]


def download_file(url: str, dest: Path) -> bool:
    """Download a file from URL to dest. Returns True on success."""
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "IV8-WPT/1.0"})
        with urllib.request.urlopen(req, timeout=30) as resp:
            data = resp.read()
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_bytes(data)
        return True
    except Exception as e:
        print(f"  FAIL: {url} -> {e}")
        return False


def get_wpt_commit() -> str:
    """Get the latest WPT master commit hash."""
    try:
        url = "https://api.github.com/repos/web-platform-tests/wpt/commits/master"
        req = urllib.request.Request(url, headers={
            "User-Agent": "IV8-WPT/1.0",
            "Accept": "application/vnd.github.v3+json",
        })
        with urllib.request.urlopen(req, timeout=15) as resp:
            data = json.loads(resp.read())
        return data["sha"][:12]
    except Exception as e:
        print(f"  WARN: could not get WPT commit: {e}")
        return "unknown"


def update_resources() -> dict:
    """Download WPT resource files (testharness.js, idlharness.js, etc.)."""
    print("=== Downloading WPT resources ===")
    sources = {}
    for wpt_path in WPT_RESOURCES:
        url = WPT_BASE + wpt_path
        dest = FIXTURES_DIR / wpt_path
        print(f"  {wpt_path}...")
        if download_file(url, dest):
            sources[wpt_path] = "downloaded"
        else:
            sources[wpt_path] = "failed"
    return sources


def update_webidl2() -> bool:
    """Copy webidl2.js from local node_modules to fixtures/resources/."""
    print("=== Copying webidl2.js ===")
    if WEBIDL2_PATH.exists():
        dest = RESOURCES_DIR / "webidl2.js"
        shutil.copy2(WEBIDL2_PATH, dest)
        print(f"  Copied from {WEBIDL2_PATH}")
        return True
    else:
        print(f"  FAIL: webidl2.js not found at {WEBIDL2_PATH}")
        print("  Run: cd tools/idl && npm install")
        return False


def update_test_files() -> dict:
    """Download WPT official test files."""
    print("=== Downloading WPT test files ===")
    sources = {}
    for wpt_path, local_path in WPT_TEST_FILES.items():
        url = WPT_BASE + wpt_path
        dest = FIXTURES_DIR / local_path
        print(f"  {wpt_path}...")
        if download_file(url, dest):
            sources[wpt_path] = "downloaded"
        else:
            sources[wpt_path] = "failed"
    return sources


def update_idl_files() -> dict:
    """Copy webref IDL files to fixtures/interfaces/."""
    print("=== Copying webref IDL files ===")
    sources = {}
    if not WEBREF_DIR.exists():
        print(f"  FAIL: @webref/idl not found at {WEBREF_DIR}")
        print("  Run: cd tools/idl && npm install")
        return sources
    for spec in WPT_IDL_SPECS:
        src = WEBREF_DIR / f"{spec}.idl"
        dest = INTERFACES_DIR / f"{spec}.idl"
        if src.exists():
            shutil.copy2(src, dest)
            sources[spec] = "copied"
        else:
            print(f"  WARN: {spec}.idl not found in @webref/idl")
            sources[spec] = "missing"
    return sources


def update_versions(commit: str, sources: dict) -> None:
    """Update versions.json with the current WPT commit and source status."""
    versions = {}
    if VERSIONS_PATH.exists():
        versions = json.loads(VERSIONS_PATH.read_text(encoding="utf-8"))
    versions["wpt_commit"] = commit
    versions["wpt_resources"] = {k: v for k, v in sources.items()
                                  if k.startswith("resources/")}
    versions["wpt_tests"] = {k: v for k, v in sources.items()
                             if "/" in k and not k.startswith("resources/")}
    versions["idl_files"] = {k: v for k, v in sources.items()
                             if not "/" in k}
    VERSIONS_PATH.write_text(
        json.dumps(versions, indent=2, ensure_ascii=False),
        encoding="utf-8",
    )
    print(f"\nVersions written to {VERSIONS_PATH}")
    print(f"  WPT commit: {commit}")


def main() -> None:
    print("WPT File Downloader")
    print("=" * 60)

    # Get WPT commit
    commit = get_wpt_commit()
    print(f"WPT commit: {commit}\n")

    # Download resources
    res_sources = update_resources()

    # Copy webidl2.js (idlharness.js redirects WebIDLParser.js to this)
    update_webidl2()

    # Download test files
    test_sources = update_test_files()

    # Copy IDL files
    idl_sources = update_idl_files()

    # Merge sources
    all_sources = {}
    all_sources.update({f"resources/{k}": v for k, v in res_sources.items()})
    all_sources.update(test_sources)
    all_sources.update(idl_sources)

    # Update versions
    update_versions(commit, all_sources)

    # Summary
    print("\n" + "=" * 60)
    ok = sum(1 for v in all_sources.values() if v in ("downloaded", "copied"))
    fail = sum(1 for v in all_sources.values() if v not in ("downloaded", "copied"))
    print(f"Total: {ok} OK, {fail} FAIL")
    if fail > 0:
        print("\nFailed files:")
        for k, v in all_sources.items():
            if v not in ("downloaded", "copied"):
                print(f"  {k}: {v}")


if __name__ == "__main__":
    main()
