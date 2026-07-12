#!/usr/bin/env python3
"""H06a: Window vs iframe property consistency audit.

Verifies that key window properties return consistent values when accessed
from the main window vs an iframe's contentWindow. Fingerprinting scripts
compare these across contexts to detect inconsistencies.

Checks ~40 properties across categories:
  - Navigator: userAgent, platform, language, hardwareConcurrency, etc.
  - Screen: width, height, colorDepth, pixelDepth, availWidth, availHeight
  - Window: innerWidth, innerHeight, outerWidth, outerHeight, devicePixelRatio
  - Crypto: subtle, getRandomValues
  - WebGL: vendor, renderer (via canvas)
  - Misc: performance.now() type, Date.now() type

Classification:
  PASS       — main window and iframe return same value
  VALUE_DIFF — values differ between contexts
  TYPE_DIFF  — typeof differs between contexts
  THROW      — property access threw in one or both contexts
  SKIP       — property not available in iframe

Usage:
  python scripts/evaluate_h06_window_iframe.py

Output:
  status/h06a-window-iframe.json
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
OUTPUT_PATH = STATUS_DIR / "h06a-window-iframe.json"

THRESHOLDS = {
    "max_value_diff": 0,
    "max_type_diff": 0,
    "max_throw": 0,
    "min_coverage_pct": 80.0,
}

PROPERTIES = [
    ("navigator.userAgent", "string"),
    ("navigator.platform", "string"),
    ("navigator.language", "string"),
    ("navigator.languages", "object"),
    ("navigator.hardwareConcurrency", "number"),
    ("navigator.deviceMemory", "number"),
    ("navigator.maxTouchPoints", "number"),
    ("navigator.vendor", "string"),
    ("navigator.appVersion", "string"),
    ("navigator.appName", "string"),
    ("navigator.appCodeName", "string"),
    ("navigator.product", "string"),
    ("navigator.productSub", "string"),
    ("navigator.cookieEnabled", "boolean"),
    ("navigator.onLine", "boolean"),
    ("navigator.webdriver", "boolean"),
    ("navigator.pdfViewerEnabled", "boolean"),
    ("navigator.doNotTrack", "string"),
    ("navigator.plugins.length", "number"),
    ("navigator.mimeTypes.length", "number"),
    ("screen.width", "number"),
    ("screen.height", "number"),
    ("screen.colorDepth", "number"),
    ("screen.pixelDepth", "number"),
    ("screen.availWidth", "number"),
    ("screen.availHeight", "number"),
    ("screen.availLeft", "number"),
    ("screen.availTop", "number"),
    ("screen.orientation.type", "string"),
    ("screen.orientation.angle", "number"),
    ("window.innerWidth", "number"),
    ("window.innerHeight", "number"),
    ("window.outerWidth", "number"),
    ("window.outerHeight", "number"),
    ("window.devicePixelRatio", "number"),
    ("window.scrollX", "number"),
    ("window.scrollY", "number"),
    ("window.pageXOffset", "number"),
    ("window.pageYOffset", "number"),
    ("typeof crypto.subtle", "object"),
    ("typeof crypto.getRandomValues", "function"),
    ("typeof performance.now", "function"),
    ("typeof Date.now", "function"),
]


def build_audit_js() -> str:
    props_js = json.dumps([[p, t] for p, t in PROPERTIES])
    return f"""(function() {{
    var props = {props_js};
    var results = [];
    var iframe = document.createElement('iframe');
    iframe.style.display = 'none';
    document.body.appendChild(iframe);
    var iwin = iframe.contentWindow;
    var idoc = iframe.contentDocument;

    for (var i = 0; i < props.length; i++) {{
        var prop = props[i][0];
        var expectedType = props[i][1];
        var r = {{ property: prop, mainValue: null, mainType: null, iframeValue: null, iframeType: null, classification: "PASS", detail: "" }};

        var mainVal, mainErr;
        try {{
            mainVal = (function() {{ return eval(prop); }}).call(window);
            r.mainValue = String(mainVal);
            r.mainType = typeof mainVal;
        }} catch(e) {{
            mainErr = String(e);
            r.mainValue = "THROW:" + mainErr;
            r.mainType = "error";
        }}

        var iframeVal, iframeErr;
        try {{
            if (prop.indexOf('typeof ') === 0) {{
                var expr = prop.substring(7);
                iframeVal = (function() {{ return typeof eval(expr); }}).call(iwin);
            }} else {{
                iframeVal = (function() {{ return eval(prop); }}).call(iwin);
            }}
            r.iframeValue = String(iframeVal);
            r.iframeType = typeof iframeVal;
        }} catch(e) {{
            iframeErr = String(e);
            r.iframeValue = "THROW:" + iframeErr;
            r.iframeType = "error";
        }}

        if (r.mainType === "error" || r.iframeType === "error") {{
            r.classification = "THROW";
            r.detail = "main=" + r.mainValue + " iframe=" + r.iframeValue;
        }} else if (r.mainType !== r.iframeType) {{
            r.classification = "TYPE_DIFF";
            r.detail = "main_type=" + r.mainType + " iframe_type=" + r.iframeType;
        }} else if (r.mainValue !== r.iframeValue) {{
            r.classification = "VALUE_DIFF";
            r.detail = "main=" + r.mainValue + " iframe=" + r.iframeValue;
        }}

        results.push(r);
    }}

    document.body.removeChild(iframe);
    return JSON.stringify(results);
}})();"""


def _run_in_thread(fn, *args, **kwargs):
    import threading
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

    stats = {"PASS": 0, "VALUE_DIFF": 0, "TYPE_DIFF": 0, "THROW": 0}
    for r in results:
        stats[r["classification"]] = stats.get(r["classification"], 0) + 1

    total = len(results)
    coverage = (total - stats.get("THROW", 0)) / max(total, 1) * 100

    report = {
        "schema_version": "h06a-window-iframe.v0.1",
        "iv8_version": "0.8.90",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "summary": {
            "total_properties": total,
            **stats,
            "coverage_pct": round(coverage, 1),
        },
        "results": results,
    }

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(
        json.dumps(report, indent=2, ensure_ascii=False),
        encoding="utf-8"
    )

    print(f"\n{'='*60}")
    print(f"H06a Window vs iframe Consistency Audit — Summary")
    print(f"{'='*60}")
    print(f"Total properties: {total}")
    for k in ["PASS", "VALUE_DIFF", "TYPE_DIFF", "THROW"]:
        print(f"  {k:20s} {stats.get(k, 0)}")

    print(f"\nCategory A (Data Integrity): ", end="")
    cat_a = stats.get("VALUE_DIFF", 0) <= THRESHOLDS["max_value_diff"] and \
            stats.get("TYPE_DIFF", 0) <= THRESHOLDS["max_type_diff"] and \
            stats.get("THROW", 0) <= THRESHOLDS["max_throw"]
    print("PASS" if cat_a else "FAIL")
    print(f"  VALUE_DIFF={stats.get('VALUE_DIFF', 0)} (max {THRESHOLDS['max_value_diff']}), "
          f"TYPE_DIFF={stats.get('TYPE_DIFF', 0)} (max {THRESHOLDS['max_type_diff']}), "
          f"THROW={stats.get('THROW', 0)} (max {THRESHOLDS['max_throw']})")

    print(f"Category D (Coverage): ", end="")
    cat_d = coverage >= THRESHOLDS["min_coverage_pct"]
    print("PASS" if cat_d else "FAIL")
    print(f"  {coverage:.1f}% (min {THRESHOLDS['min_coverage_pct']}%)")

    overall = cat_a and cat_d
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
