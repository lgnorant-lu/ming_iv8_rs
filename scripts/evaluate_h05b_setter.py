#!/usr/bin/env python3
"""H05b: Setter Side Effects Audit — attribute reflection check.

Verifies that setting an attribute produces the expected reflected value.
For example, setting element.className = "foo" should result in
element.className returning "foo".

Classification:
  PASS       — set value, get returns same value
  NO_SETTER  — setter exists but value not reflected (readonly)
  THROW      — setter threw
  SKIP       — no instance available

Usage:
  python scripts/evaluate_h05b_setter.py

Output:
  status/h05b-setter.json
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
OUTPUT_PATH = STATUS_DIR / "h05b-setter.json"

THRESHOLDS = {
    "max_throw": 0,
    "max_no_setter": 5,
    "min_coverage_pct": 80.0,
}

SETTER_TESTS = [
    ("document.title", "IV8 Test Title", "string"),
    ("document.title", "", "string"),
    ("element.className", "test-class", "string"),
    ("element.className", "", "string"),
    ("element.id", "test-id", "string"),
    ("element.id", "", "string"),
    ("element.lang", "en", "string"),
    ("element.dir", "rtl", "string"),
    ("element.hidden", True, "boolean"),
    ("element.hidden", False, "boolean"),
    ("element.tabIndex", 5, "number"),
    ("element.tabIndex", -1, "number"),
    ("element.title", "tooltip", "string"),
    ("element.draggable", True, "boolean"),
    ("element.draggable", False, "boolean"),
    ("element.contentEditable", "true", "string"),
    ("element.spellcheck", True, "boolean"),
    ("element.autocapitalize", "words", "string"),
    ("element.innerHTML", "<span>test</span>", "string"),
    ("element.outerHTML", None, "skip_get"),
    ("element.textContent", "hello", "string"),
    ("element.innerText", "world", "string"),
    ("input.value", "test value", "string"),
    ("input.value", "", "string"),
    ("input.type", "text", "string"),
    ("input.type", "checkbox", "string"),
    ("input.disabled", True, "boolean"),
    ("input.disabled", False, "boolean"),
    ("input.checked", True, "boolean"),
    ("input.checked", False, "boolean"),
    ("input.name", "username", "string"),
    ("input.placeholder", "Enter name", "string"),
    ("input.maxLength", 10, "number"),
    ("input.readOnly", True, "boolean"),
    ("input.required", True, "boolean"),
    ("input.autofocus", True, "boolean"),
    ("textarea.value", "textarea content", "string"),
    ("a.href", "https://example.com", "string"),
    ("a.target", "_blank", "string"),
    ("a.rel", "noopener", "string"),
    ("a.download", "file.txt", "string"),
    ("img.alt", "image alt", "string"),
    ("img.src", "https://example.com/img.png", "string"),
    ("img.width", 100, "number"),
    ("img.height", 200, "number"),
    ("option.value", "opt1", "string"),
    ("option.text", "Option 1", "string"),
    ("option.selected", True, "boolean"),
    ("option.disabled", True, "boolean"),
    ("label.htmlFor", "inputId", "string"),
]


def build_audit_js() -> str:
    tests_js = json.dumps([[p, v, t] for p, v, t in SETTER_TESTS])
    return f"""(function() {{
    var tests = {tests_js};
    var results = [];
    var div = document.createElement('div');
    document.body.appendChild(div);
    var input = document.createElement('input');
    document.body.appendChild(input);
    var textarea = document.createElement('textarea');
    document.body.appendChild(textarea);
    var a = document.createElement('a');
    document.body.appendChild(a);
    var img = document.createElement('img');
    document.body.appendChild(img);
    var option = document.createElement('option');
    document.body.appendChild(option);
    var label = document.createElement('label');
    document.body.appendChild(label);

    var ctx = {{document: document, element: div, input: input, textarea: textarea, a: a, img: img, option: option, label: label}};

    for (var i = 0; i < tests.length; i++) {{
        var prop = tests[i][0];
        var val = tests[i][1];
        var type = tests[i][2];
        var r = {{ property: prop, value: val, classification: "PASS", detail: "" }};

        try {{
            var obj = ctx[prop.split('.')[0]];
            var key = prop.split('.').slice(1).join('.');
            var setVal = val;
            if (type === 'boolean') setVal = Boolean(val);
            else if (type === 'number') setVal = Number(val);

            obj[key] = setVal;

            if (type !== 'skip_get') {{
                var gotVal = obj[key];
                if (type === 'boolean') gotVal = Boolean(gotVal);
                else if (type === 'number') gotVal = Number(gotVal);
                else gotVal = String(gotVal);

                var expectedVal = setVal;
                if (type === 'boolean') expectedVal = Boolean(val);
                else if (type === 'number') expectedVal = Number(val);
                else expectedVal = String(val);

                if (gotVal !== expectedVal) {{
                    if (gotVal === obj[key] && String(obj[key]) !== String(setVal)) {{
                        r.classification = "NO_SETTER";
                        r.detail = "set=" + String(setVal) + " got=" + String(gotVal);
                    }} else {{
                        r.classification = "NO_SETTER";
                        r.detail = "set=" + String(setVal) + " got=" + String(gotVal);
                    }}
                }}
            }}
        }} catch(e) {{
            r.classification = "THROW";
            r.detail = String(e);
        }}
        results.push(r);
    }}

    document.body.removeChild(div);
    document.body.removeChild(input);
    document.body.removeChild(textarea);
    document.body.removeChild(a);
    document.body.removeChild(img);
    document.body.removeChild(option);
    document.body.removeChild(label);
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
    stats = {"PASS": 0, "NO_SETTER": 0, "THROW": 0}
    for r in results:
        stats[r["classification"]] = stats.get(r["classification"], 0) + 1

    total = len(results)
    coverage = (total - stats.get("THROW", 0)) / max(total, 1) * 100

    report = {
        "schema_version": "h05b-setter.v0.1",
        "iv8_version": "0.8.90",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "summary": {"total": total, **stats, "coverage_pct": round(coverage, 1)},
        "results": results,
    }

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    print(f"\n{'='*60}")
    print(f"H05b Setter Side Effects Audit — Summary")
    print(f"{'='*60}")
    print(f"Total: {total}")
    for k in ["PASS", "NO_SETTER", "THROW"]:
        print(f"  {k:20s} {stats.get(k, 0)}")

    cat_a = stats.get("THROW", 0) <= THRESHOLDS["max_throw"]
    print(f"\nCategory A (Data Integrity): {'PASS' if cat_a else 'FAIL'}")
    print(f"  THROW={stats.get('THROW', 0)} (max {THRESHOLDS['max_throw']})")

    cat_b = stats.get("NO_SETTER", 0) <= THRESHOLDS["max_no_setter"]
    print(f"Category B (Setter Reflection): {'PASS' if cat_b else 'FAIL'}")
    print(f"  NO_SETTER={stats.get('NO_SETTER', 0)} (max {THRESHOLDS['max_no_setter']})")

    cat_d = coverage >= THRESHOLDS["min_coverage_pct"]
    print(f"Category D (Coverage): {'PASS' if cat_d else 'FAIL'}")
    print(f"  {coverage:.1f}% (min {THRESHOLDS['min_coverage_pct']}%)")

    overall = cat_a and cat_b and cat_d
    print(f"\n{'='*60}")
    print(f"OVERALL: {'PASS' if overall else 'FAIL'}")
    print(f"{'='*60}")

    if not overall:
        for r in results:
            if r["classification"] != "PASS":
                print(f"  {r['classification']}: {r['property']} — {r['detail']}")

    return 0 if overall else 1


def main():
    exit_code = _run_in_thread(_run_audit)
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
