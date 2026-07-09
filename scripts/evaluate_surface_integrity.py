#!/usr/bin/env python3
"""H04: Surface Integrity Matrix evaluator.

Parses idlharness report and classifies each FAIL into L0-L13 layers per
Web IDL spec section 3.7. Produces a matrix report mapping
[interface, layer] to PASS/FAIL counts.

Layers (per H04-surface-integrity-matrix.md section 2.2.1):
  L0  Existence
  L1  Value correctness
  L2  Value consistency (cross-field)
  L3  Descriptor correctness
  L4  toString completeness
  L5  Recursive toString
  L6  TypeError behavior
  L7  Prototype chain correctness
  L8  Cross-context
  L9  Interface object properties
  L10 Named constructor
  L11 Static operations
  L12 Stringifier
  L13 Iterable/Setlike/Maplike

Usage:
  python scripts/evaluate_surface_integrity.py
  python scripts/evaluate_surface_integrity.py --report data/idlharness-report.json

Output:
  data/surface-integrity-report.json
  Exit code: 0 if no FAIL, 1 if any FAIL
"""
from __future__ import annotations

import argparse
import json
import sys
from collections import defaultdict
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = REPO_ROOT / "data"
DEFAULT_REPORT = DATA_DIR / "idlharness-report.json"
DEFAULT_WPT_REPORT = DATA_DIR / "wpt-report.json"
DEFAULT_OUTPUT = DATA_DIR / "surface-integrity-report.json"


def classify_fail(test_name: str, message: str) -> str:
    """Classify a FAIL test into an L-layer.

    Mapping based on Web IDL spec sections and idlharness test patterns.
    Returns layer identifier (e.g. "L3", "L7") or "other".
    """
    name = test_name
    msg = message or ""

    # L13 - Iterable/Setlike/Maplike
    if any(k in name.lower() for k in
           ["iterable", "setlike", "maplike",
            "entries equality", "symbol.iterator"]):
        return "L13"

    # L12 - Stringifier
    if "stringifier" in name.lower():
        return "L12"

    # L10 - Named constructor
    if "named constructor" in name.lower():
        return "L10"

    # L11 - Static operations
    if "static operation" in msg.lower() or "static operation" in name.lower():
        return "L11"

    # L7 - Prototype chain correctness
    if "prototype of" in msg and "is not" in msg:
        return "L7"
    if "instanceof" in msg and "expected true got false" in msg:
        return "L7"

    # L0 - Existence (interface/prototype/property missing or extra)
    if "existence" in name.lower():
        return "L0"
    if "expected property" in msg and "missing" in msg:
        return "L0"
    if "global object must have a property" in msg:
        return "L0"
    if "prototype object must have a property" in msg:
        return "L0"
    if "prototype object should not have a property" in msg:
        return "L0"
    if "should not have a" in msg and "prototype" in msg:
        return "L0"

    # L3 - Descriptor correctness (writable/enumerable/configurable/name/length
    #       + setter existence + extra getter)
    if any(k in msg for k in [
        "should be writable", "should not be writable",
        "getter must have the name", "setter must have the name",
        "setter length must be", "getter length must be",
        "property has wrong .length", "property has wrong .name",
        "assert_not_equals: wrong value in",
        "property must be configurable",
        "property must be enumerable",
        "assert_true: property must be",
        "assert_false: property should not be",
        "property should be enumerable",
        "property should not have a getter",
        "setter must be function",
    ]):
        return "L3"

    # L6 - TypeError behavior
    if any(k in msg for k in [
        "Illegal invocation", "assert_throws",
        "must throw", "assert_throws_js",
        "calling operation with this",
        "calling getter with this",
        "calling setter with this",
    ]):
        if "Illegal invocation" in msg:
            return "L6"
        if "must throw" in msg.lower():
            return "L6"
        if "assert_throws" in msg:
            return "L6"
        if "calling" in msg and "this" in msg:
            return "L6"

    # L9 - Interface object properties
    if any(k in name.lower() for k in [
        "interface object length", "interface object name",
        "constant on interface object",
        "interface prototype object's",
    ]):
        return "L9"
    if "interface object length" in msg:
        return "L9"

    # L0 - Existence
    if "existence" in name.lower():
        if "expected property" in msg and "missing" in msg:
            return "L0"
        return "L0"

    # L4 - toString completeness (class string, [native code])
    if any(k in msg for k in [
        "assert_class_string", "class string of",
        "[native code]", "toString",
    ]):
        return "L4"

    # L1 - Value correctness (type mismatch, value mismatch)
    if any(k in msg for k in [
        'expected "function" but got',
        'expected "string" but got',
        'expected "number" but got',
        'expected "boolean" but got',
        'expected "object" but got',
        'expected "undefined" but got',
    ]):
        return "L1"

    # Remaining value mismatches
    if "expected" in msg and "but got" in msg:
        if "wrong value for" in msg:
            return "L10"  # named constructor value
        return "L1"

    # Runtime errors (stub returns wrong type)
    if "Cannot convert" in msg or "Cannot read" in msg:
        return "L1"

    # Missing property
    if "expected property" in msg and "missing" in msg:
        if "global object" in msg or "The global object" in msg:
            return "L0"
        return "L0"

    return "other"


def classify_pass(test_name: str) -> str:
    """Classify a PASS test into an L-layer (for coverage counting)."""
    name = test_name.lower()
    if "iterable" in name or "setlike" in name or "maplike" in name:
        return "L13"
    if "stringifier" in name:
        return "L12"
    if "named constructor" in name:
        return "L10"
    if "static operation" in name:
        return "L11"
    if "existence" in name:
        return "L0"
    if "interface object length" in name or "interface object name" in name:
        return "L9"
    if "constant" in name:
        return "L9"
    if "writable" in name or "getter must have" in name or "setter" in name:
        return "L3"
    if "property has wrong" in name:
        return "L3"
    if "prototype of" in name:
        return "L7"
    if "must throw" in name or "assert_throws" in name:
        return "L6"
    if "inherit" in name:
        return "L7"
    if "attribute" in name or "operation" in name:
        return "L1"
    return "other"


def evaluate(report: dict) -> dict:
    """Evaluate idlharness report into L-layer matrix.

    Supports both old format (interfaces[]) and WPT format (suites[]).
    Returns a dict with:
      layers: {layer: {pass, fail, total}}
      interfaces: {interface: {layer: {pass, fail}}}
      summary: {total_pass, total_fail, pass_rate, layer_coverage}
    """
    layers = defaultdict(lambda: {"pass": 0, "fail": 0, "total": 0})
    interfaces = defaultdict(lambda: defaultdict(
        lambda: {"pass": 0, "fail": 0}))

    # Support WPT official report format (suites[])
    if "suites" in report:
        for suite in report["suites"]:
            suite_name = suite.get("suite", "unknown")
            variant = suite.get("variant", "")
            iface_name = f"{suite_name} [{variant}]"
            for t in suite.get("tests", []):
                if t["status"] == "Pass":
                    layer = classify_pass(t["name"])
                    layers[layer]["pass"] += 1
                    layers[layer]["total"] += 1
                    interfaces[iface_name][layer]["pass"] += 1
                else:
                    msg = t.get("message", "") or ""
                    layer = classify_fail(t["name"], msg)
                    layers[layer]["fail"] += 1
                    layers[layer]["total"] += 1
                    interfaces[iface_name][layer]["fail"] += 1
    else:
        # Old format (interfaces[])
        for iface in report.get("interfaces", []):
            iface_name = iface["name"]
            for t in iface["tests"]:
                if t["status"] == "Pass":
                    layer = classify_pass(t["name"])
                    layers[layer]["pass"] += 1
                    layers[layer]["total"] += 1
                    interfaces[iface_name][layer]["pass"] += 1
                else:
                    msg = t.get("message", "") or ""
                    layer = classify_fail(t["name"], msg)
                    layers[layer]["fail"] += 1
                    layers[layer]["total"] += 1
                    interfaces[iface_name][layer]["fail"] += 1

    total_pass = sum(l["pass"] for l in layers.values())
    total_fail = sum(l["fail"] for l in layers.values())
    total = total_pass + total_fail
    pass_rate = round(total_pass / total * 100, 2) if total > 0 else 0

    layer_coverage = {}
    for layer in sorted(layers.keys()):
        l = layers[layer]
        rate = round(l["pass"] / l["total"] * 100, 2) if l["total"] > 0 else 0
        layer_coverage[layer] = {
            "pass": l["pass"],
            "fail": l["fail"],
            "total": l["total"],
            "pass_rate": rate,
        }

    return {
        "schema_version": "surface-integrity-report.v0.1",
        "source": "idlharness-report.json",
        "idlharness_layers": ["L0", "L1", "L3", "L4", "L6", "L7",
                              "L9", "L10", "L11", "L12", "L13"],
        "separate_script_layers": ["L2", "L5", "L8",
                                   "D1", "D2", "D3", "D4", "D5", "D6"],
        "separate_script_sources": {
            "L2": "scripts/evaluate_env_consistency.py",
            "L5": "(not yet implemented)",
            "L8": "(not yet implemented)",
            "L14": "scripts/run_creepjs_lies.py (stack trace)",
            "L15": "(not yet implemented)",
            "L16": "scripts/run_creepjs_lies.py (resistance)",
            "D1": "scripts/_d1_d5_behavior.py",
            "D2": "scripts/_d1_d5_behavior.py",
            "D3": "scripts/_d1_d5_behavior.py",
            "D4": "scripts/_d1_d5_behavior.py",
            "D5": "scripts/run_creepjs_lies.py",
            "D6": "scripts/_d1_d5_behavior.py",
        },
        "layers": dict(layer_coverage),
        "interfaces": {
            name: {layer: dict(counts)
                   for layer, counts in sorted(layers.items())}
            for name, layers in sorted(interfaces.items())
        },
        "summary": {
            "total_pass": total_pass,
            "total_fail": total_fail,
            "total": total,
            "pass_rate": pass_rate,
        },
    }


def print_report(result: dict) -> None:
    """Print human-readable matrix report with all 20 layers."""
    print("=" * 70)
    print("H04: Surface Integrity Matrix Report")
    print("=" * 70)
    print()
    total_pass = result["summary"]["total_pass"]
    total_fail = result["summary"]["total_fail"]
    total = result["summary"]["total"]
    rate = result["summary"]["pass_rate"]
    print(f"idlharness Total: {total_pass} PASS, {total_fail} FAIL / {total} ({rate}%)")
    print()

    # All 20 layers in order
    all_layers = [
        ("L0", "Existence"),
        ("L1", "Value correctness"),
        ("L2", "Value consistency (cross-field)"),
        ("L3", "Descriptor correctness"),
        ("L4", "toString completeness"),
        ("L5", "Recursive toString"),
        ("L6", "TypeError behavior"),
        ("L7", "Prototype chain + Proxy detection"),
        ("L8", "Cross-context (Worker vs Window)"),
        ("L9", "Interface object properties"),
        ("L10", "Named constructor"),
        ("L11", "Static operations"),
        ("L12", "Stringifier"),
        ("L13", "Iterable/Setlike/Maplike"),
        ("L14", "Stack trace shape"),
        ("L15", "Enumeration order"),
        ("L16", "Timing resolution"),
        ("D1", "Method return value semantics"),
        ("D2", "Promise semantics"),
        ("D3", "Event trigger timing"),
        ("D4", "State transition"),
        ("D5", "Exception behavior"),
        ("D6", "Async ordering"),
    ]
    # Layers not covered by idlharness (need separate scripts)
    separate_script_layers = {"L2", "L5", "L8", "L14", "L15", "L16",
                              "D1", "D2", "D3", "D4", "D5", "D6"}

    print("--- Layer Breakdown (idlharness) ---")
    print(f"{'Layer':<6} {'Name':<40} {'Pass':>6} {'Fail':>6} {'Rate':>8}")
    print("-" * 70)
    for layer_id, layer_name in all_layers:
        if layer_id in separate_script_layers:
            print(f"{layer_id:<6} {layer_name:<40} {'N/A':>6} {'N/A':>6} {'(separate script)':>8}")
        else:
            l = result["layers"].get(layer_id, {"pass": 0, "fail": 0, "total": 0, "pass_rate": 0})
            print(f"{layer_id:<6} {layer_name:<40} {l['pass']:>6} {l['fail']:>6} {l['pass_rate']:>7.1f}%")
    if "other" in result["layers"]:
        o = result["layers"]["other"]
        print(f"{'other':<6} {'Uncategorized':<40} {o['pass']:>6} {o['fail']:>6} {o['pass_rate']:>7.1f}%")
    print()

    print("--- Interface Breakdown (top FAIL) ---")
    iface_fails = []
    for name, layers in result["interfaces"].items():
        fail = sum(l["fail"] for l in layers.values())
        if fail > 0:
            iface_fails.append((name, fail))
    iface_fails.sort(key=lambda x: -x[1])
    for name, fail in iface_fails[:10]:
        print(f"  {name}: {fail} FAIL")
    print()


def main() -> None:
    parser = argparse.ArgumentParser(
        description="H04: Surface Integrity Matrix evaluator")
    parser.add_argument(
        "--report", "-r",
        default=str(DEFAULT_WPT_REPORT),
        help=f"Test report JSON (default: {DEFAULT_WPT_REPORT} — WPT official)",
    )
    parser.add_argument(
        "--output", "-o",
        default=str(DEFAULT_OUTPUT),
        help=f"Output JSON (default: {DEFAULT_OUTPUT})",
    )
    args = parser.parse_args()

    report_path = Path(args.report)
    if not report_path.exists():
        print(f"ERROR: Report not found: {report_path}")
        print("Run scripts/run_wpt.py first (WPT official runner).")
        print("Or run scripts/run_idlharness.py for legacy runner.")
        sys.exit(2)

    report = json.loads(report_path.read_text(encoding="utf-8"))
    result = evaluate(report)

    # Add Chrome baseline if available
    if "chrome_baseline" in report:
        chrome_total = sum(b.get("total", 0) for b in report["chrome_baseline"].values())
        chrome_pass = sum(b.get("pass", 0) for b in report["chrome_baseline"].values())
        result["chrome_baseline"] = {
            "total": chrome_total,
            "pass": chrome_pass,
            "pass_rate": round(chrome_pass / chrome_total * 100, 2) if chrome_total > 0 else 0,
        }

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(result, indent=2, ensure_ascii=False),
        encoding="utf-8",
    )

    print_report(result)

    # Print Chrome baseline if available
    if "chrome_baseline" in result:
        cb = result["chrome_baseline"]
        print(f"Chrome 151 baseline: {cb['pass']}/{cb['total']} ({cb['pass_rate']}%)")
        print()

    print(f"Report written to {output_path}")

    sys.exit(0 if result["summary"]["total_fail"] == 0 else 1)


if __name__ == "__main__":
    main()

