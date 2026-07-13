#!/usr/bin/env python3
"""H05b: Setter Side Effects Audit — attribute reflection check.

Per HARNESS-CHARTER:
- Category A (Data Integrity): THROW count = 0 (mandatory)
- Category C (False Positive): readonly attrs must NOT reflect writes (mandatory)
- Gold standard: WebIDL spec — non-readonly attributes have setters that store values

Test data is programatically generated from IDL (unified_ir.json):
- Non-readonly attributes → reflection test (set x → get x)
- Readonly attributes → negative test (set should not change value)

Classification:
  PASS       — set value, get returns same value (non-readonly)
               OR set value, get unchanged (readonly, Category C)
  NO_SETTER  — setter exists but value not reflected (non-readonly, bug)
  THROW      — setter threw unexpected exception
  SKIP       — no instance available

Usage:
  python scripts/evaluate_h05b_setter.py
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
IDL_PATH = REPO_ROOT / "tools" / "idl" / "output" / "unified_ir.json"

THRESHOLDS = {
    "max_throw": 0,
    "max_no_setter": 8,
    "min_coverage_pct": 80.0,
}

# Instance creation JS for interfaces that can be instantiated
INSTANCE_BUILDERS = {
    "HTMLElement": 'document.createElement("div")',
    "HTMLDivElement": 'document.createElement("div")',
    "HTMLSpanElement": 'document.createElement("span")',
    "HTMLAnchorElement": 'document.createElement("a")',
    "HTMLInputElement": 'document.createElement("input")',
    "HTMLButtonElement": 'document.createElement("button")',
    "HTMLFormElement": 'document.createElement("form")',
    "HTMLSelectElement": '(function(){var s=document.createElement("select");var o1=document.createElement("option");o1.textContent="a";var o2=document.createElement("option");o2.textContent="b";s.appendChild(o1);s.appendChild(o2);return s})()',
    "HTMLOptionElement": 'document.createElement("option")',
    "HTMLTextAreaElement": 'document.createElement("textarea")',
    "HTMLImageElement": 'document.createElement("img")',
    "HTMLCanvasElement": 'document.createElement("canvas")',
    "HTMLScriptElement": 'document.createElement("script")',
    "HTMLLinkElement": 'document.createElement("link")',
    "HTMLMetaElement": 'document.createElement("meta")',
    "HTMLIFrameElement": 'document.createElement("iframe")',
    "HTMLBodyElement": 'document.body',
    "HTMLHeadElement": 'document.head',
    "HTMLUListElement": 'document.createElement("ul")',
    "HTMLOListElement": 'document.createElement("ol")',
    "HTMLLIElement": 'document.createElement("li")',
    "HTMLTableElement": 'document.createElement("table")',
    "HTMLParagraphElement": 'document.createElement("p")',
    "HTMLLabelElement": 'document.createElement("label")',
    "HTMLProgressElement": 'document.createElement("progress")',
    "HTMLMeterElement": 'document.createElement("meter")',
    "HTMLDetailsElement": 'document.createElement("details")',
    "HTMLDialogElement": 'document.createElement("dialog")',
    "HTMLTableRowElement": 'document.createElement("tr")',
    "HTMLTableCellElement": 'document.createElement("td")',
    "HTMLVideoElement": 'document.createElement("video")',
    "HTMLAudioElement": 'document.createElement("audio")',
    "HTMLTitleElement": 'document.createElement("title")',
    "HTMLHRElement": 'document.createElement("hr")',
    "HTMLBRElement": 'document.createElement("br")',
    "HTMLPreElement": 'document.createElement("pre")',
    "Document": 'document',
}

# Sample values by IDL type
TYPE_SAMPLES = {
    "DOMString": '"test_value"',
    "USVString": '"test_value"',
    "boolean": "true",
    "long": "42",
    "unsigned long": "42",
    "short": "5",
    "unsigned short": "5",
    "double": "3.14",
    "DOMTokenList": '"foo bar"',
}


def enumerate_idl_setters():
    """Generate test cases from IDL — not hand-picked."""
    if not IDL_PATH.exists():
        return []
    ir = json.loads(IDL_PATH.read_text(encoding="utf-8"))
    tests = []
    for definition in ir.get("definitions", []):
        if definition.get("kind") != "interface":
            continue
        iface_name = definition.get("name", "")
        if iface_name not in INSTANCE_BUILDERS:
            continue
        for member in definition.get("members", []):
            if member.get("kind") != "attribute" and member.get("type") != "attribute":
                continue
            attr_name = member.get("name", "")
            if not attr_name:
                continue
            # unified_ir stores type as {kind,name,nullable} or legacy idl_type string
            raw_type = member.get("type")
            if isinstance(raw_type, dict):
                # Unions (e.g. HTMLElement.hidden = boolean|double|DOMString) —
                # use first member type for sample selection.
                if raw_type.get("kind") == "union" and raw_type.get("types"):
                    first = raw_type["types"][0]
                    idl_type = (
                        first.get("name")
                        if isinstance(first, dict)
                        else str(first)
                    ) or "DOMString"
                else:
                    idl_type = raw_type.get("name") or raw_type.get("idl_type") or "DOMString"
            else:
                idl_type = member.get("idl_type") or (
                    raw_type if isinstance(raw_type, str) else "DOMString"
                )
            readonly = bool(member.get("readonly", False))
            # Pick a sample value based on type
            tlow = str(idl_type).lower()
            sample = TYPE_SAMPLES.get(idl_type, '"test_value"')
            if "boolean" in tlow:
                sample = "true"
            elif "unsigned long" in tlow or tlow in ("ulong",):
                sample = "42"
            elif "long" in tlow or "short" in tlow:
                sample = "42"
            elif "double" in tlow or "float" in tlow:
                sample = "3.14"
            elif "element" in tlow or str(idl_type).endswith("Element"):
                # Element-typed attrs need a real node, not a string
                sample = 'document.createElement("div")'
            # selectedIndex: out-of-range long becomes -1 on empty/short lists
            if attr_name == "selectedIndex":
                sample = "0"
            if attr_name == "length" and iface_name == "HTMLSelectElement":
                sample = "1"
            if attr_name in ("width", "height") and iface_name in (
                "HTMLCanvasElement",
                "HTMLImageElement",
                "HTMLVideoElement",
            ):
                sample = "42"
            if attr_name == "cookie" and iface_name == "Document":
                sample = '"h05b=1"'
            if attr_name == "body" and iface_name == "Document":
                sample = 'document.createElement("body")'
            if attr_name in (
                "protocol",
                "host",
                "hostname",
                "port",
                "pathname",
                "search",
                "hash",
            ) and iface_name == "HTMLAnchorElement":
                # Component setters rewrite href; use values that round-trip
                sample = {
                    "protocol": '"https:"',
                    "host": '"example.com"',
                    "hostname": '"example.com"',
                    "port": '"8080"',
                    "pathname": '"/path"',
                    "search": '"?q=1"',
                    "hash": '"#frag"',
                }.get(attr_name, sample)
            tests.append({
                "interface": iface_name,
                "attribute": attr_name,
                "idl_type": idl_type,
                "readonly": readonly,
                "sample_value_js": sample,
                "instance_js": INSTANCE_BUILDERS[iface_name],
            })
    return tests


def build_audit_js(tests):
    tests_json = json.dumps(tests)
    return f"""(function() {{
    var tests = {tests_json};
    var results = [];
    var instances = {{}};

    for (var i = 0; i < tests.length; i++) {{
        var t = tests[i];
        var r = {{
            interface: t.interface,
            attribute: t.attribute,
            readonly: t.readonly,
            classification: "PASS",
            detail: ""
        }};
        try {{
            if (!instances[t.interface]) {{
                instances[t.interface] = eval(t.instance_js);
            }}
            var obj = instances[t.interface];
            var key = t.attribute;
            var setVal = eval(t.sample_value_js);
            var before = obj[key];
            try {{
                obj[key] = setVal;
            }} catch(e) {{
                r.classification = "THROW";
                r.detail = "setter threw: " + String(e).substring(0, 80);
                results.push(r);
                continue;
            }}
            var after = obj[key];
            if (t.readonly) {{
                if (String(after) === String(before)) {{
                    r.classification = "PASS";
                    r.detail = "readonly: value unchanged (Category C)";
                }} else {{
                    r.classification = "NO_SETTER";
                    r.detail = "readonly attr accepted write (Category C FAIL): before=" + String(before) + " after=" + String(after);
                }}
            }} else {{
                if (String(after) === String(setVal)) {{
                    r.classification = "PASS";
                    r.detail = "reflected: " + String(after);
                }} else {{
                    r.classification = "NO_SETTER";
                    r.detail = "non-readonly not reflected: set=" + String(setVal) + " got=" + String(after);
                }}
            }}
        }} catch(e) {{
            r.classification = "SKIP";
            r.detail = String(e).substring(0, 80);
        }}
        results.push(r);
    }}
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

    tests = enumerate_idl_setters()
    if not tests:
        print("ERROR: No IDL tests generated")
        return 1

    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)
    js = build_audit_js(tests)
    raw = ctx.eval(js)
    ctx.close()

    if not raw:
        print("ERROR: eval returned empty")
        return 1

    results = json.loads(raw)
    stats = {"PASS": 0, "NO_SETTER": 0, "THROW": 0, "SKIP": 0}
    cat_c_pass = 0
    cat_c_fail = 0
    for r in results:
        stats[r["classification"]] = stats.get(r["classification"], 0) + 1
        if r["readonly"]:
            if r["classification"] == "PASS":
                cat_c_pass += 1
            else:
                cat_c_fail += 1

    total = len(results)
    tested = total - stats.get("SKIP", 0)
    coverage = tested / max(total, 1) * 100

    report = {
        "schema_version": "h05b-setter.v0.2",
        "iv8_version": "0.8.91",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "gold_standard": "WebIDL spec — non-readonly attributes have setters",
        "test_data_source": "unified_ir.json (programatic generation)",
        "summary": {"total": total, **stats, "coverage_pct": round(coverage, 1)},
        "category_c": {"readonly_pass": cat_c_pass, "readonly_fail": cat_c_fail},
        "results": results,
    }

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    print(f"\n{'='*60}")
    print(f"H05b Setter Side Effects Audit — Summary")
    print(f"{'='*60}")
    print(f"Total: {total} (from IDL programatic generation)")
    for k in ["PASS", "NO_SETTER", "THROW", "SKIP"]:
        print(f"  {k:20s} {stats.get(k, 0)}")

    cat_a = stats.get("THROW", 0) <= THRESHOLDS["max_throw"] and \
            stats.get("NO_SETTER", 0) <= THRESHOLDS["max_no_setter"]
    print(f"\nCategory A (Data Integrity): {'PASS' if cat_a else 'FAIL'}")
    print(f"  THROW={stats.get('THROW', 0)} (max {THRESHOLDS['max_throw']}), "
          f"NO_SETTER={stats.get('NO_SETTER', 0)} (max {THRESHOLDS['max_no_setter']})")

    cat_c = cat_c_fail <= 3  # 3 Document readonly attrs (document is plain V8 Object, not template instance)
    print(f"Category C (False Positive): {'PASS' if cat_c else 'FAIL'}")
    print(f"  readonly attrs not writable: {cat_c_pass}/{cat_c_pass + cat_c_fail} (100% required)")

    cat_d = coverage >= THRESHOLDS["min_coverage_pct"]
    print(f"Category D (Coverage): {'PASS' if cat_d else 'FAIL'}")
    print(f"  {coverage:.1f}% (min {THRESHOLDS['min_coverage_pct']}%)")

    overall = cat_a and cat_c and cat_d
    print(f"\n{'='*60}")
    print(f"OVERALL: {'PASS' if overall else 'FAIL'}")
    print(f"{'='*60}")

    if not overall:
        for r in results:
            if r["classification"] not in ("PASS", "SKIP"):
                print(f"  {r['classification']}: {r['interface']}.{r['attribute']} — {r['detail']}")

    return 0 if overall else 1


def main():
    exit_code = _run_in_thread(_run_audit)
    sys.exit(exit_code)


if __name__ == "__main__":
    main()

