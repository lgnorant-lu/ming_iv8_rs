"""H03 L2: Surface accuracy evaluator — diff IV8 output against Chrome golden.

Usage: python scripts/evaluate_surface_accuracy.py [--golden golden.json] [--actual actual.json]
Exit code: 0 = all pass, 1 = any fail
"""

import json
import sys
import argparse
from pathlib import Path


def flatten(d, prefix=""):
    """Flatten nested dict to dot-path keys."""
    result = {}
    for k, v in d.items():
        key = f"{prefix}.{k}" if prefix else k
        if isinstance(v, dict) and "rangeMin" not in v:
            result.update(flatten(v, key))
        else:
            result[key] = v
    return result


def compare_values(golden_val, actual_val):
    """Compare two values, return (match, detail)."""
    if golden_val is None and actual_val is None:
        return True, "both null"
    if golden_val is None or actual_val is None:
        return False, f"golden={golden_val}, actual={actual_val}"
    if isinstance(golden_val, (int, float)) and isinstance(actual_val, (int, float)):
        match = abs(float(golden_val) - float(actual_val)) < 0.001
        return match, f"golden={golden_val}, actual={actual_val}"
    if isinstance(golden_val, list) and isinstance(actual_val, list):
        match = golden_val == actual_val
        return match, f"golden={golden_val}, actual={actual_val}"
    match = str(golden_val) == str(actual_val)
    return match, f"golden={golden_val!r}, actual={actual_val!r}"


def run(golden_path, actual_path):
    golden = json.loads(Path(golden_path).read_text(encoding="utf-8"))
    actual = json.loads(Path(actual_path).read_text(encoding="utf-8"))

    golden_flat = flatten(golden)
    actual_flat = flatten(actual)

    all_pass = True
    pass_count = 0
    fail_count = 0
    miss_count = 0

    print("=== H03 L2: Surface Accuracy Diff ===")
    print()

    # Group by section
    sections = {}
    for key in sorted(set(list(golden_flat.keys()) + list(actual_flat.keys()))):
        section = key.split(".")[0]
        if section not in sections:
            sections[section] = []

        golden_val = golden_flat.get(key, "__MISSING__")
        actual_val = actual_flat.get(key, "__MISSING__")

        if golden_val == "__MISSING__":
            sections[section].append((key, "MISS", f"not in golden"))
            miss_count += 1
        elif actual_val == "__MISSING__":
            sections[section].append((key, "MISS", f"not in actual (golden={golden_val})"))
            miss_count += 1
            all_pass = False
        else:
            match, detail = compare_values(golden_val, actual_val)
            if match:
                sections[section].append((key, "PASS", detail))
                pass_count += 1
            else:
                sections[section].append((key, "FAIL", detail))
                fail_count += 1
                all_pass = False

    for section in sorted(sections.keys()):
        items = sections[section]
        s_pass = sum(1 for _, s, _ in items if s == "PASS")
        s_fail = sum(1 for _, s, _ in items if s == "FAIL")
        s_miss = sum(1 for _, s, _ in items if s == "MISS")
        print(f"--- {section} ({s_pass} PASS, {s_fail} FAIL, {s_miss} MISS) ---")
        for key, status, detail in items:
            if status != "PASS":
                print(f"  [{status}] {key}: {detail}")
        print()

    total = pass_count + fail_count + miss_count
    print(f"Total: {pass_count} PASS, {fail_count} FAIL, {miss_count} MISS / {total}")
    print(f"Result: {'PASS' if all_pass else 'FAIL'}")

    return 0 if all_pass else 1


def main():
    parser = argparse.ArgumentParser(description="H03 L2: Surface accuracy evaluator")
    parser.add_argument("--golden", "-g", default="golden/chrome147_win10_rtx4060.json")
    parser.add_argument("--actual", "-a", default="actual_surface.json")
    args = parser.parse_args()

    if not Path(args.golden).exists():
        print(f"ERROR: Golden file not found: {args.golden}")
        print("Run scripts/sample_chrome_surface.py to generate golden data.")
        sys.exit(2)
    if not Path(args.actual).exists():
        print(f"ERROR: Actual file not found: {args.actual}")
        print("Run scripts/sample_iv8_surface.py to generate actual data.")
        sys.exit(2)

    sys.exit(run(args.golden, args.actual))


if __name__ == "__main__":
    main()
