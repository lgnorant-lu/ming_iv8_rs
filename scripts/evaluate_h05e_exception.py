#!/usr/bin/env python3
"""H05e: Exception Type/Message Audit — verify correct exception types."""
from __future__ import annotations
import json, sys, threading, time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
OUTPUT_PATH = REPO_ROOT / "status" / "h05e-exception.json"
# max_no_throw=20 is intentional honesty (Charter P3): several EXC_TESTS are
# documented NoThrow paths (parseInt, cloneNode coerce, TextEncoder null, etc.).
# Do not lower without reclassifying those cases as expected NoThrow in the matrix.
THRESHOLDS = {"max_wrong_type": 0, "max_no_throw": 20, "min_coverage_pct": 40.0}

# Hand matrix of exception probes (not full IR required-args / TypeError matrix).
# Residual full IR exception matrix: long-todo H05e-IR / RD-32.
EXC_TESTS = [
    ("document.createElement('div').appendChild(null)", "TypeError"),
    ("document.createElement('div').appendChild()", "TypeError"),
    ("document.createElement('div').removeChild(null)", "TypeError"),
    ("document.createElement('div').insertBefore(null, null)", "TypeError"),
    ("document.createElement('div').setAttribute()", "TypeError"),
    # null → ToString("null") / ToString coerces; Chrome does not throw.
    ("document.createElement('div').getAttribute(null)", "NoThrow"),
    ("document.getElementById(null)", "NoThrow"),
    ("document.querySelectorAll(null)", "NoThrow"),
    ("document.querySelectorAll()", "TypeError"),
    ("new Event()", "TypeError"),
    # Event(null) → ToString → type "null" is valid in Chrome.
    ("new Event(null)", "NoThrow"),
    ("Event()", "TypeError"),
    ("new CustomEvent()", "TypeError"),
    ("new MouseEvent()", "TypeError"),
    ("new URL(null)", "TypeError"),
    ("new URL()", "TypeError"),
    # URLSearchParams(null) coerces to "null" string in Chrome.
    ("new URLSearchParams(null)", "NoThrow"),
    ("JSON.parse(null)", "SyntaxError"),
    ("JSON.parse('{invalid}')", "SyntaxError"),
    ("JSON.parse('')", "SyntaxError"),
    ("decodeURIComponent('%')", "URIError"),
    ("parseInt('xyz')", "NoThrow"),
    ("parseFloat('abc')", "NoThrow"),
    ("document.createElement('div').cloneNode('invalid')", "NoThrow"),
    ("document.createElement('div').hasAttribute()", "TypeError"),
    ("new Event('test').preventDefault()", "NoThrow"),
    ("new Event('test').stopPropagation()", "NoThrow"),
    ("new Event('test').stopImmediatePropagation()", "NoThrow"),
    ("document.createElement('div').addEventListener(null, function(){})", "TypeError"),
    ("document.createElement('div').removeEventListener(null, function(){})", "TypeError"),
    ("document.createElement('div').dispatchEvent(null)", "TypeError"),
    ("document.createElement('div').dispatchEvent()", "TypeError"),
    ("document.createElement('div').dispatchEvent('not an event')", "TypeError"),
    ("Object.defineProperty({}, 'a', {value: 1, writable: false}).a = 2; 'no_throw'", "NoThrow"),
    ("'use strict'; Object.defineProperty({}, 'a', {value: 1, writable: false}).a = 2", "TypeError"),
    ("new TextEncoder().encode(null)", "NoThrow"),
    ("new TextDecoder().decode(null)", "NoThrow"),
]

def build_audit_js():
    tests_js = json.dumps([[js, expected] for js, expected in EXC_TESTS])
    return f"""(function() {{
    var tests = {tests_js};
    var results = [];
    for (var i = 0; i < tests.length; i++) {{
        var js = tests[i][0], expected = tests[i][1];
        var r = {{ js: js, expected: expected, actual: null, classification: "PASS", detail: "" }};
        try {{
            eval(js);
            if (expected !== "NoThrow") {{
                r.classification = "NO_THROW";
                r.detail = "expected " + expected + " but no exception";
            }}
        }} catch(e) {{
            var excName = e.constructor.name;
            r.actual = excName;
            if (expected === "NoThrow") {{
                r.classification = "UNEXPECTED_THROW";
                r.detail = excName + ": " + String(e).substring(0, 80);
            }} else if (excName !== expected) {{
                r.classification = "WRONG_TYPE";
                r.detail = "expected " + expected + " got " + excName;
            }}
        }}
        results.push(r);
    }}
    return JSON.stringify(results);
}})();"""

def _run_in_thread(fn):
    result_box = [None]
    def target(): result_box[0] = fn()
    old = threading.stack_size()
    threading.stack_size(128 * 1024 * 1024)
    try:
        t = threading.Thread(target=target); t.start(); t.join()
    finally:
        threading.stack_size(old)
    return result_box[0]

def _run_audit():
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext
    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)
    raw = ctx.eval(build_audit_js())
    ctx.close()
    results = json.loads(raw)
    stats = {"PASS": 0, "WRONG_TYPE": 0, "NO_THROW": 0, "UNEXPECTED_THROW": 0}
    for r in results: stats[r["classification"]] = stats.get(r["classification"], 0) + 1
    total = len(results)
    coverage = (total - stats.get("NO_THROW", 0)) / max(total, 1) * 100
    report = {"schema_version": "h05e-exception.v0.1", "iv8_version": "0.8.91", "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"), "summary": {"total": total, **stats, "coverage_pct": round(coverage, 1)}, "results": results}
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"\n{'='*60}\nH05e Exception Type/Message Audit — Summary\n{'='*60}")
    print(f"Total: {total}")
    for k in ["PASS", "WRONG_TYPE", "NO_THROW", "UNEXPECTED_THROW"]: print(f"  {k:20s} {stats.get(k, 0)}")
    cat_a = stats.get("WRONG_TYPE", 0) <= THRESHOLDS["max_wrong_type"] and stats.get("NO_THROW", 0) <= THRESHOLDS["max_no_throw"]
    print(f"\nCategory A: {'PASS' if cat_a else 'FAIL'}")
    cat_d = coverage >= THRESHOLDS["min_coverage_pct"]
    print(f"Category D: {'PASS' if cat_d else 'FAIL'}")
    overall = cat_a and cat_d
    print(f"\nOVERALL: {'PASS' if overall else 'FAIL'}")
    if not overall:
        for r in results:
            if r["classification"] != "PASS": print(f"  {r['classification']}: {r['js'][:60]} — {r['detail']}")
    return 0 if overall else 1

def main():
    sys.exit(_run_in_thread(_run_audit))

if __name__ == "__main__":
    main()
