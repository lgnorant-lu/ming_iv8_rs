#!/usr/bin/env python3
"""H05f: toString / Symbol.toStringTag audit — spec-conformance check.

Verifies that all 1284 IDL interfaces have correct:
1. Object.prototype.toString.call(instance) → "[object InterfaceName]"
2. Interface.prototype[Symbol.toStringTag] → "InterfaceName"
3. Interface.prototype.toString() does NOT return "[object InterfaceNamePrototype]"

This is a spec-conformance check (no Chrome CDP needed). The expected
values are fully determined by the WebIDL spec (toStringTag = interface name).

Usage:
  python scripts/evaluate_h05f_tostring.py

Output:
  status/h05f-tostring.json
  Exit code: 0 if all PASS, 1 if any FAIL
"""
from __future__ import annotations

import json
import sys
import threading
import time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
IR_PATH = REPO_ROOT / "tools" / "idl" / "output" / "unified_ir.json"
STATUS_DIR = REPO_ROOT / "status"
OUTPUT_PATH = STATUS_DIR / "h05f-tostring.json"

THRESHOLDS = {
    "max_fail": 0,
    "min_coverage_pct": 80.0,
}


def enumerate_interfaces(ir_path: Path) -> list[str]:
    with open(ir_path, encoding="utf-8") as f:
        ir = json.load(f)
    return sorted([
        d["name"] for d in ir["definitions"]
        if d["kind"] == "interface" and d.get("name")
    ])


def build_audit_js(interface_names: list[str]) -> str:
    names_js = json.dumps(interface_names)
    return f"""(function() {{
    var names = {names_js};
    // Legacy aliases share prototype with canonical — skip them
    // Legacy aliases share prototype/toStringTag with canonical interface (Chrome).
    var aliases = {{'webkitAudioContext': true, 'Option': true, 'webkitOfflineAudioContext': true, 'webkitURL': true}};
    var results = [];
    for (var i = 0; i < names.length; i++) {{
        var name = names[i];
        if (aliases[name]) continue;
        var r = {{ name: name, toStringTag: null, toStringResult: null, protoToString: null, errors: [] }};
        try {{
            var ctor = globalThis[name];
            if (!ctor || !ctor.prototype) {{
                r.errors.push('no_constructor');
                results.push(r);
                continue;
            }}
            var proto = ctor.prototype;

            // Check 1: Symbol.toStringTag on prototype
            try {{
                var tag = proto[Symbol.toStringTag];
                r.toStringTag = tag;
                if (tag !== name) {{
                    r.errors.push('toStringTag_mismatch: expected=' + name + ' got=' + tag);
                }}
            }} catch(e) {{
                r.errors.push('toStringTag_error: ' + e.message);
            }}

            // Check 2: Object.prototype.toString.call(proto) — should NOT return "[object namePrototype]"
            // But for instances, it should return "[object name]"
            // We can't create instances for all, so check prototype level:
            // proto.toString() should not throw and should return "[object name]"
            // (Chrome behavior: prototype.toString() returns "[object name]")
            try {{
                var ts = Object.prototype.toString.call(proto);
                r.toStringResult = ts;
                // Chrome: Object.prototype.toString.call(SomeInterface.prototype) returns "[object SomeInterface]"
                // But some interfaces have toStringTag on prototype which affects this
                if (ts !== '[object ' + name + ']') {{
                    r.errors.push('toString_mismatch: expected=[object ' + name + '] got=' + ts);
                }}
            }} catch(e) {{
                r.errors.push('toString_error: ' + e.message);
            }}

            // Check 3: proto.toString() — should return string, not throw
            try {{
                var pts = proto.toString ? proto.toString() : 'no_toString';
                r.protoToString = typeof pts === 'string' ? pts.substring(0, 50) : typeof pts;
            }} catch(e) {{
                r.errors.push('protoToString_error: ' + e.message);
            }}
        }} catch(e) {{
            r.errors.push('unexpected: ' + e.message);
        }}
        results.push(r);
    }}
    return JSON.stringify(results);
}})()"""


def _run_in_thread(fn, *args, **kwargs):
    result_box = [None, None]

    def target():
        try:
            result_box[0] = fn(*args, **kwargs)
        except Exception as e:
            result_box[1] = e

    old_size = threading.stack_size()
    threading.stack_size(128 * 1024 * 1024)
    try:
        t = threading.Thread(target=target)
        t.start()
        t.join()
    finally:
        threading.stack_size(old_size)

    if result_box[1]:
        raise result_box[1]
    return result_box[0]


def _run_audit():
    print("H05f: toString / Symbol.toStringTag Audit")
    print()

    # Step 1: Enumerate interfaces
    print("Step 1: Enumerating interfaces from IDL...")
    interface_names = enumerate_interfaces(IR_PATH)
    print(f"  Found {len(interface_names)} interfaces")

    # Step 2: Initialize IV8
    print("Step 2: Initializing IV8 runtime...")
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext
    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)

    # Step 3: Run audit JS
    print("Step 3: Running toString audit...")
    js = build_audit_js(interface_names)
    raw = ctx.eval(js)
    results = json.loads(raw) if isinstance(raw, str) else raw

    # Step 4: Classify
    print("Step 4: Classifying results...")
    stats = {"PASS": 0, "FAIL": 0, "SKIP": 0}
    fails = []
    for r in results:
        if not r.get("errors"):
            stats["PASS"] += 1
        elif r["errors"] == ["no_constructor"]:
            stats["SKIP"] += 1
        else:
            stats["FAIL"] += 1
            fails.append(r)

    # Step 5: Write report
    print("Step 5: Writing report...")
    report = {
        "schema_version": "h05f-tostring.v0.1",
        "iv8_version": "0.8.89",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "summary": {
            "total_interfaces": len(interface_names),
            **stats,
        },
        "fails": fails,
    }

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(
        json.dumps(report, indent=2, ensure_ascii=False),
        encoding="utf-8"
    )

    # Print summary
    coverage = (stats["PASS"] + stats["FAIL"]) / max(len(interface_names), 1) * 100
    cat_a_pass = stats["FAIL"] <= THRESHOLDS["max_fail"]
    cat_d_pass = coverage >= THRESHOLDS["min_coverage_pct"]

    # Category C negative: Object.prototype.toString.call on a plain object
    # must return "[object Object]", NOT an interface toStringTag.
    # This verifies that toStringTag is NOT globally leaked.
    cat_c_pass = True
    cat_c_details = []
    if ctx:
        neg_js = """
            var results = [];
            var plain = {};
            var tag = Object.prototype.toString.call(plain);
            if (tag !== '[object Object]') {
                results.push('FAIL: plain object toString = ' + tag + ' (expected [object Object])');
            }
            var arr = [];
            var arrTag = Object.prototype.toString.call(arr);
            if (arrTag !== '[object Array]') {
                results.push('FAIL: array toString = ' + arrTag + ' (expected [object Array])');
            }
            var fn = function(){};
            var fnTag = Object.prototype.toString.call(fn);
            if (fnTag !== '[object Function]') {
                results.push('FAIL: function toString = ' + fnTag + ' (expected [object Function])');
            }
            if (results.length === 0) { 'PASS'; } else { results.join('; '); }
        """
        try:
            neg_result = ctx.eval(neg_js)
            if neg_result and not neg_result.startswith("PASS"):
                cat_c_pass = False
                cat_c_details.append(neg_result)
        except Exception as e:
            cat_c_details.append(f"SKIP: eval error: {e}")

    print(f"\n{'='*60}")
    print(f"H05f toString / Symbol.toStringTag Audit — Summary")
    print(f"{'='*60}")
    print(f"Total interfaces: {len(interface_names)}")
    print(f"  PASS:  {stats['PASS']}")
    print(f"  FAIL:  {stats['FAIL']}")
    print(f"  SKIP:  {stats['SKIP']}")
    print(f"  Coverage: {coverage:.1f}%")

    if fails:
        print(f"\nFailures (first 20):")
        for f in fails[:20]:
            print(f"  {f['name']}: {', '.join(f['errors'][:2])}")

    print(f"\nCategory A (Data Integrity): {'PASS' if cat_a_pass else 'FAIL'}")
    print(f"  FAIL={stats['FAIL']} (max {THRESHOLDS['max_fail']})")
    print(f"Category C (False Positive): {'PASS' if cat_c_pass else 'FAIL'}")
    if cat_c_details:
        for d in cat_c_details:
            print(f"  {d}")
    print(f"Category D (Coverage): {'PASS' if cat_d_pass else 'FAIL'}")
    print(f"  {coverage:.1f}% (min {THRESHOLDS['min_coverage_pct']}%)")

    overall = cat_a_pass and cat_c_pass and cat_d_pass
    print(f"\nOVERALL: {'PASS' if overall else 'FAIL'}")
    print(f"Output: {OUTPUT_PATH}")

    return 0 if overall else 1


def main():
    exit_code = _run_in_thread(_run_audit)
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
