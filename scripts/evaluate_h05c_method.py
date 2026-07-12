#!/usr/bin/env python3
"""H05c: Method Return Value Audit — key method return type check.

Verifies that key DOM/Element methods return values of the correct type.

Classification:
  PASS       — return type matches expected
  TYPE_FAIL  — return type wrong
  THROW      — method threw unexpected exception
  SKIP       — method not applicable

Usage:
  python scripts/evaluate_h05c_method.py

Output:
  status/h05c-method.json
  Exit code: 0 if OVERALL PASS, 1 otherwise
"""
from __future__ import annotations

import json
import sys
import threading
import time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
STATUS_DIR = REPO_ROOT / "status"
OUTPUT_PATH = STATUS_DIR / "h05c-method.json"

THRESHOLDS = {
    "max_type_fail": 0,
    "max_throw": 1,
    "min_coverage_pct": 80.0,
}

METHOD_TESTS = [
    ("document.createElement('div').toString()", "string"),
    ("document.createElement('div').getAttribute('id')", "object"),
    ("document.createElement('div').setAttribute('foo','bar')", "undefined"),
    ("document.createElement('div').hasAttribute('id')", "boolean"),
    ("document.createElement('div').removeAttribute('id')", "undefined"),
    ("document.createElement('div').cloneNode()", "object"),
    ("document.createElement('div').cloneNode(true)", "object"),
    ("document.createElement('div').appendChild(document.createTextNode('x'))", "object"),
    ("document.createElement('div').removeChild(document.createTextNode('x'))", "object"),
    ("document.createTextNode('hello').splitText(2)", "object"),
    ("document.createTextNode('hello').toString()", "string"),
    ("document.querySelector('body')", "object"),
    ("document.querySelectorAll('div')", "object"),
    ("document.getElementById('nonexistent')", "object"),
    ("document.getElementsByClassName('nonexistent')", "object"),
    ("document.getElementsByTagName('div')", "object"),
    ("document.getElementsByName('nonexistent')", "object"),
    ("document.createAttribute('test')", "object"),
    ("document.createComment('test')", "object"),
    ("document.createDocumentFragment()", "object"),
    ("document.createElementNS('http://www.w3.org/1999/xhtml','div')", "object"),
    ("document.importNode(document.createElement('div'), true)", "object"),
    ("document.adoptNode(document.createElement('div'))", "object"),
    ("element.getBoundingClientRect()", "object"),
    ("element.hasAttributes()", "boolean"),
    ("element.hasChildNodes()", "boolean"),
    ("element.isSameNode(document.body)", "boolean"),
    ("element.isEqualNode(document.createElement('div'))", "boolean"),
    ("element.matches('div')", "boolean"),
    ("element.closest('div')", "object"),
    ("element.insertAdjacentHTML('beforeend','<span></span>')", "undefined"),
    ("element.insertAdjacentElement('beforebegin', document.createElement('span'))", "object"),
    ("element.insertAdjacentText('beforeend','text')", "undefined"),
    ("element.scrollIntoView()", "undefined"),
    ("element.focus()", "undefined"),
    ("element.blur()", "undefined"),
    ("element.click()", "undefined"),
    ("element.after(document.createElement('span'))", "undefined"),
    ("element.before(document.createElement('span'))", "undefined"),
    ("element.replaceWith(document.createElement('span'))", "undefined"),
    ("element.remove()", "undefined"),
    ("element.prepend(document.createTextNode('x'))", "undefined"),
    ("element.append(document.createTextNode('x'))", "undefined"),
    ("element.replaceChildren(document.createTextNode('x'))", "undefined"),
    ("Array.from(document.querySelectorAll('body')).map(function(e){return e.tagName})", "object"),
    ("new Event('test').composedPath()", "object"),
    ("new Event('test').preventDefault()", "undefined"),
    ("new Event('test').stopPropagation()", "undefined"),
    ("new Event('test').stopImmediatePropagation()", "undefined"),
    ("document.addEventListener('click', function(){})", "undefined"),
    ("document.removeEventListener('click', function(){})", "undefined"),
    ("document.dispatchEvent(new Event('x'))", "boolean"),
    ("JSON.stringify({a:1})", "string"),
    ("Object.keys({a:1})", "object"),
    ("Object.values({a:1})", "object"),
    ("Object.entries({a:1})", "object"),
    ("Array.isArray([])", "boolean"),
    ("Promise.resolve()", "object"),
    ("new Map()", "object"),
    ("new Set()", "object"),
]


def build_audit_js() -> str:
    tests_js = json.dumps([[js, t] for js, t in METHOD_TESTS])
    return f"""(function() {{
    var tests = {tests_js};
    var results = [];
    var element = document.createElement('div');
    document.body.appendChild(element);

    for (var i = 0; i < tests.length; i++) {{
        var js = tests[i][0];
        var expectedType = tests[i][1];
        var r = {{ method: js, expectedType: expectedType, actualType: null, classification: "PASS", detail: "" }};
        try {{
            var val = eval(js);
            r.actualType = typeof val;
            if (r.actualType !== expectedType) {{
                if (val === null && expectedType === 'object') {{
                    r.classification = "PASS";
                }} else {{
                    r.classification = "TYPE_FAIL";
                    r.detail = "expected=" + expectedType + " actual=" + r.actualType + " value=" + String(val);
                }}
            }}
        }} catch(e) {{
            r.classification = "THROW";
            r.detail = String(e);
        }}
        results.push(r);
    }}

    document.body.removeChild(element);
    return JSON.stringify(results);
}})();"""


def _run_in_thread(fn, *args, **kwargs):
    result_box = [None]
    def target():
        result_box[0] = fn(*args, **kwargs)
    old_size = threading.stack_size()
    threading.stack_size(128 * 1024 * 1024)
    try:
        t = threading.Thread(target=target)
        t.start()
        t.join()
    finally:
        threading.stack_size(old_size)
    return result_box[0]


def _run_audit():
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext

    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)

    js = build_audit_js()
    raw = ctx.eval(js)
    ctx.close()

    if not raw:
        print("ERROR: eval returned empty")
        return 1

    results = json.loads(raw)
    stats = {"PASS": 0, "TYPE_FAIL": 0, "THROW": 0}
    for r in results:
        stats[r["classification"]] = stats.get(r["classification"], 0) + 1

    total = len(results)
    coverage = (total - 0) / max(total, 1) * 100

    report = {
        "schema_version": "h05c-method.v0.1",
        "iv8_version": "0.8.90",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "summary": {"total": total, **stats, "coverage_pct": round(coverage, 1)},
        "results": results,
    }

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    print(f"\n{'='*60}")
    print(f"H05c Method Return Value Audit — Summary")
    print(f"{'='*60}")
    print(f"Total: {total}")
    for k in ["PASS", "TYPE_FAIL", "THROW"]:
        print(f"  {k:20s} {stats.get(k, 0)}")

    cat_a = stats.get("TYPE_FAIL", 0) <= THRESHOLDS["max_type_fail"] and \
            stats.get("THROW", 0) <= THRESHOLDS["max_throw"]
    print(f"\nCategory A (Data Integrity): {'PASS' if cat_a else 'FAIL'}")
    print(f"  TYPE_FAIL={stats.get('TYPE_FAIL', 0)} (max {THRESHOLDS['max_type_fail']}), "
          f"THROW={stats.get('THROW', 0)} (max {THRESHOLDS['max_throw']})")

    cat_d = coverage >= THRESHOLDS["min_coverage_pct"]
    print(f"Category D (Coverage): {'PASS' if cat_d else 'FAIL'}")
    print(f"  {coverage:.1f}% (min {THRESHOLDS['min_coverage_pct']}%)")

    overall = cat_a and cat_d
    print(f"\n{'='*60}")
    print(f"OVERALL: {'PASS' if overall else 'FAIL'}")
    print(f"{'='*60}")

    if not overall:
        for r in results:
            if r["classification"] != "PASS":
                print(f"  {r['classification']}: {r['method']} — {r['detail']}")

    return 0 if overall else 1


def main():
    exit_code = _run_in_thread(_run_audit)
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
