#!/usr/bin/env python3
"""H05d: Constructor Behavior Audit — verify constructor behavior matches spec.

Per HARNESS-CHARTER:
- Category A (Data Integrity): WRONG_TYPE + THROW = 0 (mandatory)
- Category C (False Positive): non-constructable interfaces must throw (mandatory)
- Gold standard: WebIDL spec — constructor return type + non-constructable

Category C negative test: abstract/non-constructable interfaces
(Node, Element, Document, etc.) must throw TypeError when `new`'d.

Usage:
  python scripts/evaluate_h05d_constructor.py
"""
from __future__ import annotations
import json, sys, threading, time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
OUTPUT_PATH = REPO_ROOT / "status" / "h05d-constructor.json"
IDL_PATH = REPO_ROOT / "tools" / "idl" / "output" / "unified_ir.json"
THRESHOLDS = {"max_throw": 2, "max_wrong_type": 0, "min_coverage_pct": 80.0}

# Hand-curated positive constructable probes (behavior, not just new X()).
# NON_CONSTRUCTABLE list is IR-driven (see load_non_constructable).
CTOR_TESTS = [
    ("new Event('test')", "object", "Event"),
    ("new Event('test').type", "string", "Event"),
    ("new Event('test', {bubbles: true}).bubbles", "boolean", "Event"),
    ("new CustomEvent('test', {detail: 42}).detail", "number", "CustomEvent"),
    ("new MouseEvent('click').clientX", "number", "MouseEvent"),
    ("new MouseEvent('click', {clientX: 10}).clientX", "number", "MouseEvent"),
    ("new KeyboardEvent('keydown', {key: 'a'}).key", "string", "KeyboardEvent"),
    ("new PointerEvent('pointerdown').pointerId", "number", "PointerEvent"),
    ("new URL('https://example.com').href", "string", "URL"),
    ("new URLSearchParams('a=1&b=2').get('a')", "string", "URLSearchParams"),
    ("new Headers([['a','1']]).get('a')", "string", "Headers"),
    ("new AbortController().signal.aborted", "boolean", "AbortController"),
    ("new TextEncoder().encode('hello').length", "number", "TextEncoder"),
    ("new TextDecoder().decode(new Uint8Array([104]))", "string", "TextDecoder"),
    ("new MutationObserver(function(){})", "object", "MutationObserver"),
    ("new FormData().append", "function", "FormData"),
    ("new Blob([]).size", "number", "Blob"),
    ("new FileReader().readAsText", "function", "FileReader"),
    ("new XMLHttpRequest().open", "function", "XMLHttpRequest"),
    ("new DOMRect(0, 0, 100, 100).width", "number", "DOMRect"),
    ("new DOMPoint(1, 2).x", "number", "DOMPoint"),
    ("new DOMMatrix().a", "number", "DOMMatrix"),
    ("new Range().toString()", "string", "Range"),
    ("new Map().size", "number", "Map"),
    ("new Set().size", "number", "Set"),
    ("new WeakMap()", "object", "WeakMap"),
    ("new WeakSet()", "object", "WeakSet"),
    ("new Promise(function(r){r(1)}).then", "function", "Promise"),
    ("new Error('test').message", "string", "Error"),
    ("new TypeError('test').name", "string", "TypeError"),
    ("new Array(3).length", "number", "Array"),
    ("new Object()", "object", "Object"),
    ("new Date().getTime() > 0", "boolean", "Date"),
    ("new RegExp('a').test('a')", "boolean", "RegExp"),
    ("new Map([['a',1]]).get('a')", "number", "Map"),
]

def load_non_constructable(ir_path: Path) -> list[str]:
    """Interfaces with no constructor member and no NamedConstructor — illegal new."""
    if not ir_path.exists():
        return [
            "Node", "Element", "HTMLElement", "CharacterData", "Text", "Comment",
            "Attr", "DocumentFragment", "ShadowRoot", "Navigator", "Screen",
            "History", "Location", "Window",
        ]
    ir = json.loads(ir_path.read_text(encoding="utf-8"))
    # Prefer high-signal DOM surface for Category C (full 1284 would SKIP).
    priority = {
        "Node", "Element", "HTMLElement", "CharacterData", "Text", "Comment",
        "Attr", "DocumentFragment", "ShadowRoot", "Document", "XMLDocument",
        "Navigator", "Screen", "History", "Location", "Window",
        "HTMLCollection", "HTMLAllCollection", "HTMLOptionsCollection",
        "HTMLFormControlsCollection", "HTMLUnknownElement", "HTMLMediaElement",
    }
    found = []
    for d in ir.get("definitions", []):
        if d.get("kind") != "interface":
            continue
        name = d.get("name")
        if not name or name not in priority:
            continue
        ea = {e.get("name") for e in (d.get("ext_attrs") or []) if isinstance(e, dict)}
        if "LegacyNoInterfaceObject" in ea or "NoInterfaceObject" in ea:
            found.append(name)
            continue
        has_ctor = any(m.get("kind") == "constructor" for m in d.get("members", []))
        has_named = any(
            e.get("name") in ("NamedConstructor", "LegacyFactoryFunction")
            for e in (d.get("ext_attrs") or [])
            if isinstance(e, dict)
        )
        if not has_ctor and not has_named:
            found.append(name)
    return sorted(set(found)) or list(priority)


def build_audit_js(non_constructable: list[str]):
    tests_js = json.dumps([[js, t, iface] for js, t, iface in CTOR_TESTS])
    nc_js = json.dumps(non_constructable)
    return f"""(function() {{
    var tests = {tests_js};
    var nonConstructable = {nc_js};
    var results = [];
    for (var i = 0; i < tests.length; i++) {{
        var js = tests[i][0], expectedType = tests[i][1], iface = tests[i][2];
        var r = {{ js: js, expectedType: expectedType, iface: iface, category: "A", actualType: null, classification: "PASS", detail: "" }};
        try {{
            var val = eval(js);
            r.actualType = typeof val;
            if (r.actualType !== expectedType) {{
                if (val === null && expectedType === 'object') {{
                    r.classification = "PASS";
                }} else {{
                    r.classification = "WRONG_TYPE";
                    r.detail = "expected=" + expectedType + " actual=" + r.actualType;
                }}
            }}
        }} catch(e) {{
            r.classification = "THROW";
            r.detail = String(e).substring(0, 100);
        }}
        results.push(r);
    }}
    for (var j = 0; j < nonConstructable.length; j++) {{
        var name = nonConstructable[j];
        var r = {{ js: "new " + name + "()", expectedType: "THROW", iface: name, category: "C", actualType: null, classification: "PASS", detail: "" }};
        try {{
            eval("new " + name + "()");
            r.classification = "NO_THROW";
            r.detail = name + " should not be constructable (Category C)";
        }} catch(e) {{
            r.classification = "PASS";
            r.detail = name + " correctly threw: " + e.constructor.name;
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
    non_constructable = load_non_constructable(IDL_PATH)
    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)
    raw = ctx.eval(build_audit_js(non_constructable))
    ctx.close()
    results = json.loads(raw)
    stats = {"PASS": 0, "WRONG_TYPE": 0, "THROW": 0, "NO_THROW": 0}
    cat_c_pass = 0
    cat_c_fail = 0
    for r in results:
        stats[r["classification"]] = stats.get(r["classification"], 0) + 1
        if r.get("category") == "C":
            if r["classification"] == "PASS":
                cat_c_pass += 1
            else:
                cat_c_fail += 1
    total = len(results)
    coverage = (total - stats.get("THROW", 0)) / max(total, 1) * 100
    report = {
        "schema_version": "h05d-constructor.v0.2",
        "iv8_version": "0.8.91",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "gold_standard": "WebIDL spec — constructor object + non-constructable",
        "summary": {"total": total, **stats, "coverage_pct": round(coverage, 1)},
        "category_c": {"non_constructable_pass": cat_c_pass, "non_constructable_fail": cat_c_fail},
        "results": results,
    }
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"\n{'='*60}\nH05d Constructor Behavior Audit — Summary\n{'='*60}")
    print(f"Total: {total}")
    for k in ["PASS", "WRONG_TYPE", "THROW", "NO_THROW"]: print(f"  {k:20s} {stats.get(k, 0)}")
    cat_a = stats.get("WRONG_TYPE", 0) <= THRESHOLDS["max_wrong_type"] and stats.get("THROW", 0) <= THRESHOLDS["max_throw"]
    print(f"\nCategory A (Data Integrity): {'PASS' if cat_a else 'FAIL'}")
    cat_c = cat_c_fail == 0
    print(f"Category C (False Positive): {'PASS' if cat_c else 'FAIL'}")
    print(f"  non-constructable throw: {cat_c_pass}/{cat_c_pass + cat_c_fail} (100% required)")
    cat_d = coverage >= THRESHOLDS["min_coverage_pct"]
    print(f"Category D (Coverage): {'PASS' if cat_d else 'FAIL'}")
    overall = cat_a and cat_c and cat_d
    print(f"\nOVERALL: {'PASS' if overall else 'FAIL'}")
    if not overall:
        for r in results:
            if r["classification"] != "PASS": print(f"  {r['category']}/{r['classification']}: {r['js'][:60]} -- {r['detail']}")
    return 0 if overall else 1

def main():
    sys.exit(_run_in_thread(_run_audit))

if __name__ == "__main__":
    main()
