#!/usr/bin/env python3
"""Download mdn-bcd-results for Chrome on Windows and extract API existence data.

The mdn-bcd-results JSON schema:
  {
    "__version": "...",
    "userAgent": "...",
    "results": {
      "<collector-url>": [
        {"name": "api.Navigator", "exposure": "Window", "result": true},
        {"name": "api.Navigator.userAgent", "exposure": "Window", "result": true},
        ...
      ]
    }
  }

Outputs:
  data/bcd-chrome148.json                 - all api.* entries (deduplicated)
  data/bcd-chrome148-interfaces.json      - interface-level entries only
"""

import json
import os
import re
import sys
import urllib.request

REPO = "openwebdocs/mdn-bcd-results"
BRANCH = "main"
API_TREE_URL = f"https://api.github.com/repos/{REPO}/git/trees/{BRANCH}?recursive=1"
RAW_BASE = f"https://raw.githubusercontent.com/{REPO}/{BRANCH}/"

DEFAULT_CHROME_VERSION = "148.0.0.0"
DEFAULT_PLATFORM = "windows-10"

DATA_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "data")


def http_get_json(url):
    req = urllib.request.Request(
        url, headers={"User-Agent": "bcd-collector-downloader", "Accept": "application/json"}
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.loads(resp.read().decode("utf-8"))


def http_get_raw(url):
    req = urllib.request.Request(url, headers={"User-Agent": "bcd-collector-downloader"})
    with urllib.request.urlopen(req, timeout=120) as resp:
        return resp.read()


def list_repo_files():
    data = http_get_json(API_TREE_URL)
    if isinstance(data, dict) and "tree" in data:
        return [e["path"] for e in data["tree"] if e["type"] == "blob"]
    raise RuntimeError(f"Unexpected tree response from {API_TREE_URL}")


def find_file(files, chrome_version, platform):
    chrome_pat = re.compile(
        rf"^[\d.]+-chrome-{re.escape(chrome_version)}-{re.escape(platform)}-[0-9a-f]+\.json$"
    )
    matches = [f for f in files if chrome_pat.match(f)]
    if matches:
        return sorted(matches)[0]
    return None


def find_nearest_file(files, chrome_major, platform):
    pat = re.compile(
        rf"^[\d.]+-chrome-(\d+)\.\d+\.\d+\.\d+-{re.escape(platform)}-[0-9a-f]+\.json$"
    )
    candidates = []
    for f in files:
        m = pat.match(f)
        if m:
            major = int(m.group(1))
            candidates.append((major, f))
    if not candidates:
        return None
    candidates.sort(key=lambda x: x[0], reverse=True)
    for major, f in candidates:
        if major <= chrome_major:
            return f
    return candidates[-1][1]


def parse_results(data):
    """Flatten the results dict-of-arrays into a single list, deduplicating
    by (name, exposure) keeping the last non-null result."""
    results_obj = data.get("results", {})
    flat = {}
    if isinstance(results_obj, dict):
        for url, entries in results_obj.items():
            if not isinstance(entries, list):
                continue
            for r in entries:
                if not isinstance(r, dict):
                    continue
                name = r.get("name")
                exposure = r.get("exposure")
                result = r.get("result")
                if name is None or exposure is None:
                    continue
                key = (name, exposure)
                prev = flat.get(key)
                if prev is not None and prev.get("result") is not None:
                    continue
                flat[key] = {
                    "name": name,
                    "result": result,
                    "exposure": exposure,
                }
    elif isinstance(results_obj, list):
        for r in results_obj:
            if not isinstance(r, dict):
                continue
            name = r.get("name")
            exposure = r.get("exposure")
            if name is None:
                continue
            flat[(name, exposure)] = {
                "name": name,
                "result": r.get("result"),
                "exposure": exposure,
            }
    return list(flat.values())


def extract_api_entries(all_entries):
    api_entries = [e for e in all_entries if str(e["name"]).startswith("api.")]
    api_entries.sort(key=lambda e: (e["name"], e["exposure"]))
    interface_entries = []
    for e in api_entries:
        rest = e["name"][len("api."):]
        if rest and "." not in rest:
            interface_entries.append(e)
    interface_entries.sort(key=lambda e: (e["name"], e["exposure"]))
    return api_entries, interface_entries


def parse_metadata(filename, default_chrome, default_platform):
    parts = filename.replace(".json", "").split("-")
    chrome_version = default_chrome
    platform = default_platform
    try:
        ci = parts.index("chrome")
        chrome_version = parts[ci + 1]
        platform = "-".join(parts[ci + 2:-1])
    except (ValueError, IndexError):
        pass
    return chrome_version, platform


def write_outputs(filename, chrome_version, platform, api_entries, interface_entries):
    os.makedirs(DATA_DIR, exist_ok=True)
    full_path = os.path.join(DATA_DIR, "bcd-chrome148.json")
    iface_path = os.path.join(DATA_DIR, "bcd-chrome148-interfaces.json")

    with open(full_path, "w", encoding="utf-8") as f:
        json.dump(
            {
                "chrome_version": chrome_version,
                "platform": platform,
                "source_file": filename,
                "total_api_entries": len(api_entries),
                "entries": api_entries,
            },
            f,
            ensure_ascii=False,
            indent=2,
        )
    with open(iface_path, "w", encoding="utf-8") as f:
        json.dump(
            {
                "chrome_version": chrome_version,
                "platform": platform,
                "source_file": filename,
                "total_interface_entries": len(interface_entries),
                "entries": interface_entries,
            },
            f,
            ensure_ascii=False,
            indent=2,
        )
    return full_path, iface_path


def process_file(filename, default_chrome, default_platform):
    url = RAW_BASE + filename
    print(f"[fetch] {url}")
    raw = http_get_raw(url)
    print(f"[ok] downloaded {len(raw)} bytes")
    data = json.loads(raw.decode("utf-8"))

    all_entries = parse_results(data)
    print(f"[info] total result entries (deduplicated): {len(all_entries)}")

    api_entries, interface_entries = extract_api_entries(all_entries)
    print(f"[info] api.* entries: {len(api_entries)}")
    print(f"[info] interface-level entries: {len(interface_entries)}")

    chrome_version, platform = parse_metadata(filename, default_chrome, default_platform)
    full_path, iface_path = write_outputs(
        filename, chrome_version, platform, api_entries, interface_entries
    )
    print(f"[write] {full_path} ({len(api_entries)} entries)")
    print(f"[write] {iface_path} ({len(interface_entries)} entries)")
    return len(all_entries), len(api_entries), len(interface_entries)


def main():
    chrome_version = os.environ.get("CHROME_VERSION", DEFAULT_CHROME_VERSION)
    platform = os.environ.get("PLATFORM", DEFAULT_PLATFORM)
    print(f"[config] chrome_version={chrome_version} platform={platform}")

    # Try direct known filename first.
    if chrome_version == DEFAULT_CHROME_VERSION and platform == DEFAULT_PLATFORM:
        known_file = "10.19.1-chrome-148.0.0.0-windows-10-1148dbfe77.json"
        try:
            process_file(known_file, chrome_version, platform)
            return 0
        except Exception as e:
            print(f"[warn] direct URL failed: {e}; falling back to repo listing")

    print(f"[list] enumerating repo files via {API_TREE_URL}")
    try:
        files = list_repo_files()
    except Exception as e:
        print(f"[error] failed to list repo: {e}", file=sys.stderr)
        return 2

    filename = find_file(files, chrome_version, platform)
    if not filename:
        try:
            chrome_major = int(chrome_version.split(".")[0])
        except ValueError:
            chrome_major = 0
        nearest = find_nearest_file(files, chrome_major, platform)
        if nearest:
            print(f"[warn] exact match not found; using nearest: {nearest}")
            filename = nearest
        else:
            print(
                f"[error] no file matching chrome-{chrome_version}-{platform}",
                file=sys.stderr,
            )
            return 3

    process_file(filename, chrome_version, platform)
    return 0


if __name__ == "__main__":
    sys.exit(main())
