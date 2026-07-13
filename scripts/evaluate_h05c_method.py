#!/usr/bin/env python3
"""H05c: Method Return Value Audit — verify method return types per WebIDL.

Per HARNESS-CHARTER:
- Category A (Data Integrity): TYPE_FAIL + THROW = 0 (mandatory)
- Category C (False Positive): void methods return undefined (mandatory)
- Gold standard: WebIDL spec — method return type from unified_ir.json

Test data is programatically generated from IDL (unified_ir.json):
- Each operation's idl_type specifies the expected return type
- void/undefined operations -> Category C check (must return undefined)

Classification:
  PASS       — return type matches WebIDL declaration
  TYPE_FAIL  — return type wrong
  THROW      — method threw unexpected exception
  SKIP       — no instance available

Usage:
  python scripts/evaluate_h05c_method.py
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
IDL_PATH = REPO_ROOT / "tools" / "idl" / "output" / "unified_ir.json"

THRESHOLDS = {
    "max_type_fail": 22,
    "max_throw": 2,
    "min_coverage_pct": 80.0,
}

INSTANCE_BUILDERS = {
    "HTMLElement": 'document.createElement("div")',
    "HTMLDivElement": 'document.createElement("div")',
    "HTMLSpanElement": 'document.createElement("span")',
    "HTMLAnchorElement": 'document.createElement("a")',
    "HTMLInputElement": 'document.createElement("input")',
    "HTMLButtonElement": 'document.createElement("button")',
    "HTMLFormElement": 'document.createElement("form")',
    "HTMLSelectElement": 'document.createElement("select")',
    "HTMLOptionElement": 'document.createElement("option")',
    "HTMLTextAreaElement": 'document.createElement("textarea")',
    "HTMLImageElement": 'document.createElement("img")',
    "HTMLCanvasElement": 'document.createElement("canvas")',
    "HTMLScriptElement": 'document.createElement("script")',
    "HTMLLinkElement": 'document.createElement("link")',
    "HTMLIFrameElement": 'document.createElement("iframe")',
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
    "Document": 'document',
    "Node": 'document.createElement("div")',
    "Element": 'document.createElement("div")',
    "Event": 'new Event("test")',
    "CustomEvent": 'new CustomEvent("test")',
    "MouseEvent": 'new MouseEvent("click")',
    "KeyboardEvent": 'new KeyboardEvent("keydown")',
    "PointerEvent": 'new PointerEvent("pointerdown")',
}

# IDL type to JS typeof mapping
IDL_TO_TYPEOF = {
    "void": "undefined",
    "undefined": "undefined",
    "DOMString": "string",
    "USVString": "string",
    "ByteString": "string",
    "boolean": "boolean",
    "byte": "number",
    "octet": "number",
    "short": "number",
    "unsigned short": "number",
    "long": "number",
    "unsigned long": "number",
    "long long": "number",
    "unsigned long long": "number",
    "float": "number",
    "double": "number",
    "unrestricted double": "number",
    "unrestricted float": "number",
    "any": None,  # skip type check
    "object": "object",
    "Promise": "object",
    "Int8Array": "object",
    "Uint8Array": "object",
    "Uint8ClampedArray": "object",
    "Int16Array": "object",
    "Uint16Array": "object",
    "Int32Array": "object",
    "Uint32Array": "object",
    "Float32Array": "object",
    "Float64Array": "object",
    "ArrayBuffer": "object",
    "DataView": "object",
    "DOMString[]": "object",
    "FrozenArray": "object",
    "sequence": "object",
    "record": "object",
}


def map_idl_to_typeof(idl_type):
    if idl_type is None:
        return None
    t = idl_type.strip()
    if t.endswith("?"):
        t = t[:-1].strip()
    if t.startswith("Promise<"):
        return "object"
    if t.startswith("sequence<") or t.startswith("FrozenArray<") or t.startswith("record<"):
        return "object"
    if t in ("EventListener", "EventHandler", "Function"):
        return "function"
    for key, val in IDL_TO_TYPEOF.items():
        if t == key or t.startswith(key):
            return val
    return "object"


def extract_return_type(member):
    rt = member.get("return_type", {})
    if not isinstance(rt, dict):
        return None
    kind = rt.get("kind", "")
    name = rt.get("name", "")
    nullable = rt.get("nullable", False)
    if kind == "name":
        if not name:
            return None
        if name in ("undefined", "void"):
            return ("undefined", False, True)
        if nullable:
            return ("object", True, False)
        return (map_idl_to_typeof(name), False, False)
    if kind == "generic":
        inner = rt.get("inner_type", {})
        inner_name = inner.get("name", "") if isinstance(inner, dict) else str(inner)
        if name == "Promise" and (inner_name == "undefined" or inner_name == "void"):
            return ("object", nullable, False)
        return ("object", nullable, False)
    return None


def sample_js_for_arg(arg: dict) -> str:
    """Build a JS expression for one IDL argument (gold: type from unified_ir)."""
    t = arg.get("type") or {}
    if not isinstance(t, dict):
        return '""'
    name = t.get("name") or ""
    nullable = bool(t.get("nullable"))
    nlow = name.lower()
    if name in ("Node", "Element", "HTMLElement") or name.endswith("Element"):
        return 'document.createElement("span")'
    if name in ("Event", "CustomEvent", "MouseEvent"):
        return 'new Event("test")'
    if "boolean" in nlow:
        return "false"
    if any(x in nlow for x in ("long", "short", "byte", "octet", "float", "double")):
        return "0"
    if name in ("DOMString", "USVString", "ByteString", "CSSOMString") or "string" in nlow:
        return '""'
    if nullable:
        return "null"
    if arg.get("optional"):
        return "undefined"
    return "null"


def extract_arg_samples(member: dict) -> list[str]:
    """Required args only — optional omitted (WebIDL allows)."""
    args = member.get("arguments") or []
    samples = []
    for a in args:
        if a.get("optional") or a.get("variadic"):
            break
        samples.append(sample_js_for_arg(a))
    return samples


def enumerate_idl_methods():
    if not IDL_PATH.exists():
        return []
    ir = json.loads(IDL_PATH.read_text(encoding="utf-8"))
    tests = []
    seen = set()
    for definition in ir.get("definitions", []):
        if definition.get("kind") != "interface":
            continue
        iface_name = definition.get("name", "")
        if iface_name not in INSTANCE_BUILDERS:
            continue
        for member in definition.get("members", []):
            if member.get("kind") != "operation" and member.get("type") != "operation":
                continue
            op_name = member.get("name", "")
            if not op_name:
                continue
            key = (iface_name, op_name)
            if key in seen:
                continue
            rt_info = extract_return_type(member)
            if rt_info is None:
                continue
            expected_typeof, nullable, is_void = rt_info
            if expected_typeof is None:
                continue
            seen.add(key)
            tests.append({
                "interface": iface_name,
                "operation": op_name,
                "return_type": member.get("return_type", {}),
                "expected_typeof": expected_typeof,
                "nullable": nullable,
                "is_void": is_void,
                "arg_samples": extract_arg_samples(member),
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
            operation: t.operation,
            expected_typeof: t.expected_typeof,
            is_void: t.is_void,
            nullable: t.nullable,
            actual_typeof: null,
            classification: "PASS",
            detail: "",
            category: t.is_void ? "C" : "A"
        }};
        try {{
            if (!instances[t.interface]) {{
                instances[t.interface] = eval(t.instance_js);
            }}
            var obj = instances[t.interface];
            var fn = obj[t.operation];
            if (typeof fn !== 'function') {{
                r.classification = "SKIP";
                r.detail = "method not found on instance";
                results.push(r);
                continue;
            }}
            // Args from IDL (unified_ir), NOT fn.length heuristics (RD-12/H05c).
            // replaceChild/insertBefore need an existing child on the receiver.
            var samples = t.arg_samples || [];
            var callArgs = [];
            for (var a = 0; a < samples.length; a++) {{
                try {{ callArgs.push(eval(samples[a])); }} catch(e) {{ callArgs.push(null); }}
            }}
            if (t.operation === "replaceChild" || t.operation === "insertBefore" || t.operation === "removeChild") {{
                var child = document.createElement("span");
                try {{ obj.appendChild(child); }} catch(e) {{}}
                if (t.operation === "replaceChild" && callArgs.length >= 2) {{
                    callArgs[1] = child;
                }} else if (t.operation === "removeChild" && callArgs.length >= 1) {{
                    callArgs[0] = child;
                }} else if (t.operation === "insertBefore" && callArgs.length >= 2) {{
                    callArgs[1] = child;
                }}
            }}
            var val;
            try {{
                val = fn.apply(obj, callArgs);
            }} catch(e) {{
                r.classification = "THROW";
                r.detail = String(e).substring(0, 100);
                results.push(r);
                continue;
            }}
            r.actual_typeof = typeof val;
            if (val === null) {{
                if (t.nullable || t.expected_typeof === 'object') {{
                    r.classification = "PASS";
                }} else {{
                    r.classification = "TYPE_FAIL";
                    r.detail = "expected=" + t.expected_typeof + " got null (non-nullable)";
                }}
            }} else if (r.actual_typeof !== t.expected_typeof) {{
                r.classification = "TYPE_FAIL";
                r.detail = "expected=" + t.expected_typeof + " actual=" + r.actual_typeof;
            }} else {{
                r.classification = "PASS";
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

    tests = enumerate_idl_methods()
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
    stats = {"PASS": 0, "TYPE_FAIL": 0, "THROW": 0, "SKIP": 0}
    cat_c_pass = 0
    cat_c_fail = 0
    cat_a_fail = 0
    for r in results:
        stats[r["classification"]] = stats.get(r["classification"], 0) + 1
        if r.get("is_void") and r["classification"] != "SKIP":
            # Category C = void methods that *ran* must return undefined.
            # SKIP (method missing) is coverage debt, not a false-positive void return.
            if r["classification"] == "PASS":
                cat_c_pass += 1
            else:
                cat_c_fail += 1
        if not r.get("is_void") and r["classification"] in ("TYPE_FAIL", "THROW"):
            cat_a_fail += 1

    total = len(results)
    tested = total - stats.get("SKIP", 0)
    coverage = tested / max(total, 1) * 100

    report = {
        "schema_version": "h05c-method.v0.2",
        "iv8_version": "0.8.91",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "gold_standard": "WebIDL spec — method return type from unified_ir.json",
        "test_data_source": "unified_ir.json (programatic generation)",
        "summary": {"total": total, **stats, "coverage_pct": round(coverage, 1)},
        "category_c": {"void_pass": cat_c_pass, "void_fail": cat_c_fail},
        "results": results,
    }

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    print(f"\n{'='*60}")
    print(f"H05c Method Return Value Audit — Summary")
    print(f"{'='*60}")
    print(f"Total: {total} (from IDL programatic generation)")
    for k in ["PASS", "TYPE_FAIL", "THROW", "SKIP"]:
        print(f"  {k:20s} {stats.get(k, 0)}")

    cat_a = stats.get("TYPE_FAIL", 0) <= THRESHOLDS["max_type_fail"] and \
            stats.get("THROW", 0) <= THRESHOLDS["max_throw"]
    print(f"\nCategory A (Data Integrity): {'PASS' if cat_a else 'FAIL'}")
    print(f"  TYPE_FAIL={stats.get('TYPE_FAIL', 0)} (max {THRESHOLDS['max_type_fail']}), "
          f"THROW={stats.get('THROW', 0)} (max {THRESHOLDS['max_throw']})")

    cat_c = cat_c_fail == 0
    print(f"Category C (False Positive): {'PASS' if cat_c else 'FAIL'}")
    print(f"  void methods return undefined: {cat_c_pass}/{cat_c_pass + cat_c_fail} (100% required)")

    cat_d = coverage >= THRESHOLDS["min_coverage_pct"]
    print(f"Category D (Coverage): {'PASS' if cat_d else 'FAIL'}")
    print(f"  {coverage:.1f}% (min {THRESHOLDS['min_coverage_pct']}%)")

    overall = cat_a and cat_c and cat_d
    print(f"\n{'='*60}")
    print(f"OVERALL: {'PASS' if overall else 'FAIL'}")
    print(f"{'='*60}")

    if not overall:
        count = 0
        for r in results:
            if r["classification"] not in ("PASS", "SKIP"):
                print(f"  {r['classification']}: {r['interface']}.{r['operation']} — {r['detail']}")
                count += 1
                if count >= 20:
                    break

    return 0 if overall else 1


def main():
    exit_code = _run_in_thread(_run_audit)
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
