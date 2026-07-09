#!/usr/bin/env python3
"""L3: Descriptor-level diff between Chrome and IV8 surface samples.

Reads data/surface_values_chrome.json and data/surface_values_iv8.json,
compares property values and descriptors, and produces a categorized diff
report.

Categories:
  MATCHED             — value type + descriptor shape both match
  VALUE_MISMATCH      — property exists in both but value/type differs
  DESCRIPTOR_MISMATCH — property exists in both but descriptor shape differs
                         (e.g. Chrome has accessor, IV8 has data property)
  MISSING_IN_IV8      — property exists in Chrome but not in IV8
  EXTRA_IN_IV8        — property exists in IV8 but not in Chrome

Usage:
  .venv\\Scripts\\python.exe scripts/descriptor_diff.py

Output:
  data/descriptor_diff.json
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = REPO_ROOT / "data"

DEFAULT_CHROME_PATH = DATA_DIR / "surface_values_chrome.json"
DEFAULT_IV8_PATH = DATA_DIR / "surface_values_iv8.json"
DEFAULT_OUTPUT = DATA_DIR / "descriptor_diff.json"


def load_surface(path: Path) -> dict:
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def normalize_descriptor(desc: dict | None) -> dict | None:
    """Normalize a descriptor dict for comparison.

    Returns a canonical dict with keys:
      location, isAccessor, isData, enumerable, configurable, writable
    """
    if desc is None:
        return None
    has_get = desc.get("hasGet", False)
    has_set = desc.get("hasSet", False)
    has_value = desc.get("hasValue", False)
    has_writable = desc.get("hasWritable", False)
    is_accessor = has_get or has_set
    is_data = has_value or has_writable
    return {
        "location": desc.get("location"),
        "isAccessor": is_accessor,
        "isData": is_data,
        "enumerable": desc.get("enumerable"),
        "configurable": desc.get("configurable"),
        "writable": desc.get("writable") if is_data else None,
        "hasGet": has_get,
        "hasSet": has_set,
    }


def compare_descriptors(chrome_desc: dict | None, iv8_desc: dict | None) -> list[str]:
    """Compare two descriptor dicts. Returns list of difference descriptions."""
    cn = normalize_descriptor(chrome_desc)
    iv8n = normalize_descriptor(iv8_desc)

    if cn is None and iv8n is None:
        return []
    if cn is None:
        return ["Chrome descriptor is null, IV8 has one"]
    if iv8n is None:
        return ["Chrome has descriptor, IV8 is null"]

    diffs = []

    if cn["location"] != iv8n["location"]:
        diffs.append(f"location: chrome={cn['location']}, iv8={iv8n['location']}")
    if cn["isAccessor"] != iv8n["isAccessor"]:
        diffs.append(
            f"isAccessor: chrome={cn['isAccessor']}, iv8={iv8n['isAccessor']}"
        )
    if cn["isData"] != iv8n["isData"]:
        diffs.append(f"isData: chrome={cn['isData']}, iv8={iv8n['isData']}")
    if cn["enumerable"] != iv8n["enumerable"]:
        diffs.append(
            f"enumerable: chrome={cn['enumerable']}, iv8={iv8n['enumerable']}"
        )
    if cn["configurable"] != iv8n["configurable"]:
        diffs.append(
            f"configurable: chrome={cn['configurable']}, iv8={iv8n['configurable']}"
        )
    if cn["isData"] and iv8n["isData"] and cn["writable"] != iv8n["writable"]:
        diffs.append(
            f"writable: chrome={cn['writable']}, iv8={iv8n['writable']}"
        )
    if cn["hasGet"] != iv8n["hasGet"]:
        diffs.append(f"hasGet: chrome={cn['hasGet']}, iv8={iv8n['hasGet']}")
    if cn["hasSet"] != iv8n["hasSet"]:
        diffs.append(f"hasSet: chrome={cn['hasSet']}, iv8={iv8n['hasSet']}")

    return diffs


def compare_values(chrome_val: dict, iv8_val: dict) -> str | None:
    """Compare two value dicts. Returns mismatch type string or None if matched."""
    ct = chrome_val.get("typeof")
    it = iv8_val.get("typeof")

    if ct != it:
        return "TYPE"

    if ct in ("string", "number", "boolean"):
        if chrome_val.get("value") != iv8_val.get("value"):
            return "VALUE"
        return None

    if ct == "undefined":
        return None

    if ct == "function":
        return None

    if ct == "object":
        ck = chrome_val.get("objectKeys")
        ik = iv8_val.get("objectKeys")
        if ck is not None and ik is not None:
            cs = set(ck)
            iss = set(ik)
            if cs != iss:
                return "OBJECT_KEYS"
        return None

    if ct == "symbol" or ct == "bigint":
        return None if chrome_val.get("value") == iv8_val.get("value") else "VALUE"

    return None


def diff_surfaces(chrome_data: dict, iv8_data: dict) -> dict:
    """Compute the full diff between Chrome and IV8 surface samples."""
    results = {
        "MATCHED": [],
        "VALUE_MISMATCH": [],
        "DESCRIPTOR_MISMATCH": [],
        "MISSING_IN_IV8": [],
        "EXTRA_IN_IV8": [],
    }

    chrome_interfaces = set(chrome_data.keys())
    iv8_interfaces = set(iv8_data.keys())

    for iface in sorted(chrome_interfaces | iv8_interfaces):
        c_iface = chrome_data.get(iface)
        i_iface = iv8_data.get(iface)

        if c_iface is None:
            results["EXTRA_IN_IV8"].append({
                "interface": iface,
                "reason": "interface only in IV8",
            })
            continue
        if i_iface is None:
            results["MISSING_IN_IV8"].append({
                "interface": iface,
                "reason": "interface only in Chrome",
            })
            continue

        if "__error" in c_iface:
            results["MISSING_IN_IV8"].append({
                "interface": iface,
                "reason": f"Chrome error: {c_iface['__error']}",
            })
            continue
        if "__error" in i_iface:
            results["MISSING_IN_IV8"].append({
                "interface": iface,
                "reason": f"IV8 error: {i_iface['__error']}",
            })
            continue

        c_props = set(c_iface.keys())
        i_props = set(i_iface.keys())

        for prop in sorted(c_props - i_props):
            results["MISSING_IN_IV8"].append({
                "interface": iface,
                "property": prop,
            })

        for prop in sorted(i_props - c_props):
            results["EXTRA_IN_IV8"].append({
                "interface": iface,
                "property": prop,
            })

        for prop in sorted(c_props & i_props):
            c_entry = c_iface[prop]
            i_entry = i_iface[prop]

            c_val = c_entry.get("value", {})
            i_val = i_entry.get("value", {})
            c_desc = c_entry.get("descriptor")
            i_desc = i_entry.get("descriptor")

            value_mismatch = compare_values(c_val, i_val)
            desc_diffs = compare_descriptors(c_desc, i_desc)

            entry = {
                "interface": iface,
                "property": prop,
                "chrome_value": c_val,
                "iv8_value": i_val,
                "chrome_descriptor": normalize_descriptor(c_desc),
                "iv8_descriptor": normalize_descriptor(i_desc),
            }

            if value_mismatch and desc_diffs:
                entry["value_mismatch_type"] = value_mismatch
                entry["descriptor_differences"] = desc_diffs
                results["DESCRIPTOR_MISMATCH"].append(entry)
            elif value_mismatch:
                entry["mismatch_type"] = value_mismatch
                results["VALUE_MISMATCH"].append(entry)
            elif desc_diffs:
                entry["differences"] = desc_diffs
                results["DESCRIPTOR_MISMATCH"].append(entry)
            else:
                results["MATCHED"].append({
                    "interface": iface,
                    "property": prop,
                })

    summary = {
        "total_properties": sum(len(v) for v in results.values()),
        "matched": len(results["MATCHED"]),
        "value_mismatch": len(results["VALUE_MISMATCH"]),
        "descriptor_mismatch": len(results["DESCRIPTOR_MISMATCH"]),
        "missing_in_iv8": len(results["MISSING_IN_IV8"]),
        "extra_in_iv8": len(results["EXTRA_IN_IV8"]),
    }

    return {
        "summary": summary,
        "details": results,
    }


def main():
    parser = argparse.ArgumentParser(
        description="L3: Descriptor-level diff between Chrome and IV8"
    )
    parser.add_argument(
        "--chrome-data",
        default=str(DEFAULT_CHROME_PATH),
        help=f"Chrome surface JSON (default: {DEFAULT_CHROME_PATH})",
    )
    parser.add_argument(
        "--iv8-data",
        default=str(DEFAULT_IV8_PATH),
        help=f"IV8 surface JSON (default: {DEFAULT_IV8_PATH})",
    )
    parser.add_argument(
        "--output",
        "-o",
        default=str(DEFAULT_OUTPUT),
        help=f"Output JSON (default: {DEFAULT_OUTPUT})",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Print all mismatches to stdout",
    )
    args = parser.parse_args()

    chrome_path = Path(args.chrome_data)
    iv8_path = Path(args.iv8_data)

    if not chrome_path.exists():
        print(f"ERROR: Chrome data not found: {chrome_path}")
        sys.exit(1)
    if not iv8_path.exists():
        print(f"ERROR: IV8 data not found: {iv8_path}")
        sys.exit(1)

    print(f"Loading Chrome surface: {chrome_path}")
    chrome_data = load_surface(chrome_path)
    print(f"Loading IV8 surface: {iv8_path}")
    iv8_data = load_surface(iv8_path)

    print("Computing diff...")
    report = diff_surfaces(chrome_data, iv8_data)

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(report, indent=2, default=str, ensure_ascii=False),
        encoding="utf-8",
    )

    s = report["summary"]
    print()
    print("=== Surface Diff Summary ===")
    print(f"  Total properties:   {s['total_properties']}")
    print(f"  MATCHED:            {s['matched']}")
    print(f"  VALUE_MISMATCH:     {s['value_mismatch']}")
    print(f"  DESCRIPTOR_MISMATCH:{s['descriptor_mismatch']}")
    print(f"  MISSING_IN_IV8:     {s['missing_in_iv8']}")
    print(f"  EXTRA_IN_IV8:       {s['extra_in_iv8']}")
    print()
    print(f"Report written to {output_path}")

    if args.verbose:
        for category in ("VALUE_MISMATCH", "DESCRIPTOR_MISMATCH", "MISSING_IN_IV8", "EXTRA_IN_IV8"):
            items = report["details"][category]
            if items:
                print()
                print(f"--- {category} ({len(items)}) ---")
                for item in items:
                    iface = item.get("interface", "?")
                    prop = item.get("property", item.get("reason", ""))
                    print(f"  {iface}.{prop}")

    sys.exit(0 if s["value_mismatch"] == 0 and s["descriptor_mismatch"] == 0 and s["missing_in_iv8"] == 0 else 1)


if __name__ == "__main__":
    main()
