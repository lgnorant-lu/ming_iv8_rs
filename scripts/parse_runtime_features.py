"""Parse Chromium runtime_enabled_features.json5 into a feature mapping.

Produces data/runtime_features_chrome148.json:
  { feature_name: { "enabled": bool, "status": str, "platforms": {...} } }

A feature is considered enabled in stable Chrome 148 (desktop, Windows) if:
  - status is "stable" (globally), OR
  - status is a dict with "Win": "stable" or "default": "stable"

Features with status "experimental", "test", or no status are disabled.

Usage: .venv\\Scripts\\python.exe scripts\\parse_runtime_features.py
"""

import json
import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
INPUT_PATH = REPO_ROOT / "data" / "runtime_enabled_features.json5"
OUTPUT_PATH = REPO_ROOT / "data" / "runtime_features_chrome148.json"


def strip_json5_comments(text: str) -> str:
    text = re.sub(r"//[^\n]*", "", text)
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
    return text


def fix_trailing_commas(text: str) -> str:
    text = re.sub(r",(\s*[}\]])", r"\1", text)
    return text


def parse_json5(path: Path) -> dict:
    raw = path.read_text(encoding="utf-8")
    cleaned = strip_json5_comments(raw)
    return cleaned


def extract_features_from_text(text: str) -> dict:
    features = {}
    pattern = re.compile(
        r'name:\s*"([^"]+)"\s*,(.*?)(?=\s*\{\s*name:|\s*\]\s*\}\s*$)',
        re.DOTALL,
    )
    for m in pattern.finditer(text):
        name = m.group(1)
        body = m.group(2)
        status_match = re.search(
            r'status:\s*(?:"([^"]+)"|\{([^}]*)\})',
            body,
        )
        status = None
        platforms = {}
        if status_match:
            if status_match.group(1):
                status = status_match.group(1)
                platforms = {"default": status}
            elif status_match.group(2):
                dict_body = status_match.group(2)
                pairs = re.findall(r'(\w+):\s*"([^"]+)"', dict_body)
                for key, val in pairs:
                    platforms[key] = val
                status = "dict"
        enabled = is_enabled_for_win(status if isinstance(status, str) else
                                     ({"Win": platforms.get("Win"),
                                       "default": platforms.get("default")}
                                      if platforms else None))
        features[name] = {
            "enabled": enabled,
            "status": status if status else "none",
            "platforms": platforms,
        }
    return features


def is_enabled_for_win(status) -> bool:
    if isinstance(status, str):
        return status == "stable"
    if isinstance(status, dict):
        win_status = status.get("Win")
        if win_status == "stable":
            return True
        default_status = status.get("default")
        if win_status is None and default_status == "stable":
            return True
        return False
    return False


def extract_features(data: dict) -> dict:
    features = {}
    for item in data.get("data", []):
        name = item.get("name")
        if not name:
            continue
        status = item.get("status")
        enabled = is_enabled_for_win(status)
        platforms = {}
        if isinstance(status, dict):
            platforms = {k: v for k, v in status.items() if k != "default"}
            default_status = status.get("default")
            if default_status:
                platforms["default"] = default_status
        elif isinstance(status, str):
            platforms = {"default": status}
        features[name] = {
            "enabled": enabled,
            "status": status if isinstance(status, str) else "dict",
            "platforms": platforms,
        }
    return features


def main():
    if not INPUT_PATH.exists():
        print(f"[ERROR] Input not found: {INPUT_PATH}")
        return 1

    text = parse_json5(INPUT_PATH)
    features = extract_features_from_text(text)

    enabled_count = sum(1 for f in features.values() if f["enabled"])
    disabled_count = len(features) - enabled_count

    output = {
        "chrome_version": "148.0.7778.96",
        "platform": "Win",
        "total_features": len(features),
        "enabled_count": enabled_count,
        "disabled_count": disabled_count,
        "features": features,
    }

    OUTPUT_PATH.write_text(json.dumps(output, indent=2), encoding="utf-8")

    print("=== Runtime Features (Chrome 148, Win) ===")
    print(f"  Total features: {len(features)}")
    print(f"  Enabled: {enabled_count}")
    print(f"  Disabled: {disabled_count}")
    print(f"  Output: {OUTPUT_PATH}")

    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
