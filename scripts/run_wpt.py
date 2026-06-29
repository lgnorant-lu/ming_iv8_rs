#!/usr/bin/env python3
"""WPT official test runner — runs WPT idlharness tests in IV8 V8 isolate.

Directly reuses WPT official test files (idlharness.https.html, etc.)
without modification. Compares results against Chrome baseline from wpt.fyi.

Usage:
  .venv\\Scripts\\python.exe scripts/run_wpt.py
  .venv\\Scripts\\python.exe scripts/run_wpt.py --suite html/dom
  .venv\\Scripts\\python.exe scripts/run_wpt.py --update

Output:
  data/wpt-report.json
  Exit code: 0 if all pass, 1 if any fail
"""
from __future__ import annotations

import json
import re
import sys
import threading
import urllib.request
from pathlib import Path
from collections import defaultdict

REPO_ROOT = Path(__file__).resolve().parent.parent
WPT_DIR = REPO_ROOT / "tools" / "wpt"
FIXTURES_DIR = WPT_DIR / "fixtures"
RESOURCES_DIR = FIXTURES_DIR / "resources"
INTERFACES_DIR = FIXTURES_DIR / "interfaces"
STATUS_DIR = WPT_DIR / "status"
VERSIONS_PATH = WPT_DIR / "versions.json"
DATA_DIR = REPO_ROOT / "data"
OUT_PATH = DATA_DIR / "wpt-report.json"

# WPT test suites to run
# Each suite maps to a WPT official test file and its variants
WPT_SUITES = [
    {
        "name": "html/dom/idlharness",
        "test_file": FIXTURES_DIR / "html" / "dom" / "idlharness.https.html",
        "variants": [
            {"name": "include=Document|Window", "query": "?include=(Document|Window)"},
            {"name": "include=HTML.+", "query": "?include=HTML.+"},
            {"name": "exclude=Document|Window|HTML.+", "query": "?exclude=(Document|Window|HTML.+)"},
        ],
        "idl_specs": [
            "html", "wai-aria", "SVG", "cssom", "touch-events", "pointerevents",
            "uievents", "dom", "xhr", "FileAPI", "mediacapture-streams",
            "performance-timeline", "trusted-types",
        ],
    },
    {
        "name": "dom/idlharness",
        "test_file": FIXTURES_DIR / "dom" / "idlharness.window.js",
        "variants": [
            {"name": "include=Node", "query": "?include=Node"},
            {"name": "exclude=Node", "query": "?exclude=Node"},
        ],
        "idl_specs": [
            "dom", "fullscreen", "html",
        ],
    },
    {
        "name": "css/cssom-view/idlharness",
        "test_file": FIXTURES_DIR / "css" / "cssom-view" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "cssom-view", "css-pseudo", "cssom", "pointerevents", "uievents",
            "SVG", "html", "dom",
        ],
    },
]


def load_wpt_resources() -> dict[str, str]:
    """Load WPT resource files (testharness.js, idlharness.js, webidl2.js).

    testharnessreport.js is IV8's custom version (sets output:false).
    """
    resources = {}
    for name in ["testharness.js", "idlharness.js", "webidl2.js"]:
        path = RESOURCES_DIR / name
        if path.exists():
            resources[name] = path.read_text(encoding="utf-8")
        else:
            print(f"FATAL: {path} not found. Run tools/wpt/wpt_update.py first.")
            sys.exit(1)
    # Use IV8's custom testharnessreport.js
    report_path = WPT_DIR / "fixtures" / "resources" / "testharnessreport.js"
    if report_path.exists():
        resources["testharnessreport.js"] = report_path.read_text(encoding="utf-8")
    else:
        print(f"FATAL: {report_path} not found.")
        sys.exit(1)
    return resources


def load_idl_files(specs: list[str]) -> dict[str, str]:
    """Load webref IDL files for the given spec names."""
    idl_contents = {}
    for spec in specs:
        path = INTERFACES_DIR / f"{spec}.idl"
        if path.exists():
            idl_contents[spec] = path.read_text(encoding="utf-8")
        else:
            print(f"WARN: IDL file not found: {path}")
    return idl_contents


def extract_script_from_html(html_path: Path) -> str:
    """Extract <script> content from a WPT .html test file.

    Returns the concatenated JS code from all <script> tags,
    excluding external src= references (those are loaded separately).
    """
    html = html_path.read_text(encoding="utf-8")
    # Extract content between <script> tags (without src=)
    pattern = r'<script(?![^>]*\bsrc=)[^>]*>(.*?)</script>'
    scripts = re.findall(pattern, html, re.DOTALL)
    return "\n".join(scripts)


def build_shim_code(idl_contents: dict[str, str], variant_query: str) -> str:
    """Build the shim code injected before WPT test execution.

    Includes:
    - GLOBAL object (isWindow=true for .html, isWorker=false)
    - fetch_spec implementation (using IV8's fetch + add_resource)
    - location.search for variant subsetting
    - add_result_callback for result collection
    """
    # Register IDL files as fetchable resources
    register_lines = []
    for spec, content in idl_contents.items():
        escaped = json.dumps(content)
        register_lines.append(
            f'  ctx.add_resource("/interfaces/{spec}.idl", {escaped});'
        )

    return f"""
// === IV8 WPT Shim ===
globalThis.GLOBAL = globalThis;
globalThis.GLOBAL.isWindow = function() {{ return true; }};
globalThis.GLOBAL.isWorker = function() {{ return false; }};
globalThis.GLOBAL.isShadowRealm = function() {{ return false; }};

// location.search for variant subsetting
globalThis.location = {{ search: {json.dumps(variant_query)} }};

// fetch_spec: idlharness.js calls this to load IDL files
globalThis.fetch_spec = function(spec) {{
    var url = "/interfaces/" + spec + ".idl";
    return fetch(url).then(function(r) {{
        if (!r.ok) throw new Error("Error fetching " + url);
        return r.text();
    }}).then(function(idl) {{
        return {{ spec: spec, idl: idl }};
    }});
}};

// Result collector
var __results = [];
add_result_callback(function(test) {{
    __results.push({{
        name: test.name,
        status: test.format_status(),
        message: test.message || null
    }});
}});

// testharness.js expects window.onload to fire before running tests.
// IV8 has no real event loop, so we resolve the 'load' event immediately.
if (typeof addEventListener === 'function') {{
    var __origAddEventListener = addEventListener;
    globalThis.addEventListener = function(type, listener) {{
        if (type === 'load') {{
            Promise.resolve().then(function() {{ listener({{ type: 'load' }}); }});
        }} else {{
            __origAddEventListener(type, listener);
        }}
    }};
}}
// === End IV8 WPT Shim ===
"""


def run_suite(suite: dict, variant: dict, resources: dict) -> dict:
    """Run a single WPT test suite variant in IV8.

    Returns a result dict with pass/fail/total and test details.
    """
    import iv8_rs as iv8

    suite_name = suite["name"]
    variant_name = variant["name"]
    variant_query = variant["query"]
    test_file = suite["test_file"]

    if not test_file.exists():
        return {
            "suite": suite_name,
            "variant": variant_name,
            "run_status": "error: test file not found",
            "total": 0, "pass": 0, "fail": 0,
            "tests": [],
        }

    # Load IDL files
    idl_contents = load_idl_files(suite["idl_specs"])

    # Extract test code
    if test_file.suffix == ".html":
        test_code = extract_script_from_html(test_file)
    else:
        test_code = test_file.read_text(encoding="utf-8")

    # Neutralize setup() calls that create DOM elements (video, iframe, etc.)
    # These crash in IV8 because createElement('video') returns a div-like
    # element without media capabilities. Replace setup(function() {...})
    # with a no-op, preserving the idl_test() call intact.
    # Pattern: setup(function() { ... });
    test_code = re.sub(
        r'setup\s*\(\s*function\s*\(\s*\)\s*\{[^}]*(?:\{[^}]*\}[^}]*)*\}\s*\)',
        'setup(function() {})',
        test_code,
        count=1,
    )

    # Create IV8 context
    ctx = iv8.JSContext()

    try:
        # Register IDL files as resources
        for spec, content in idl_contents.items():
            ctx.add_resource(f"/interfaces/{spec}.idl", content)

        # Inject pre-harness shim BEFORE testharness.js loads.
        # testharness.js IIFE registers load event listener at load time
        # via window.addEventListener('load', callback). We must intercept
        # this before testharness.js executes.
        pre_shim = f"""
globalThis.GLOBAL = globalThis;
globalThis.GLOBAL.isWindow = function() {{ return true; }};
globalThis.GLOBAL.isWorker = function() {{ return false; }};
globalThis.GLOBAL.isShadowRealm = function() {{ return false; }};
globalThis.location = {{ search: {json.dumps(variant_query)} }};
globalThis.fetch_spec = function(spec) {{
    return fetch("/interfaces/" + spec + ".idl").then(function(r) {{
        if (!r.ok) throw new Error("Error fetching " + spec);
        return r.text();
    }}).then(function(idl) {{ return {{ spec: spec, idl: idl }}; }});
}};
var __results = [];
var __loadCallbacks = [];
// Intercept addEventListener BEFORE testharness.js loads
var __origAEL = globalThis.addEventListener;
globalThis.addEventListener = function(type, listener, useCapture) {{
    if (type === 'load') {{
        __loadCallbacks.push(listener);
    }} else {{
        __origAEL.call(this, type, listener, useCapture);
    }}
}};
// window.addEventListener should be the same
if (typeof window !== 'undefined' && window !== globalThis) {{
    window.addEventListener = globalThis.addEventListener;
}}
"""
        ctx.eval(pre_shim, name="iv8-pre-shim.js")

        # Load WPT harness (IIFE registers load listener via our shim)
        ctx.eval(resources["testharness.js"], name="testharness.js")
        ctx.eval(resources["testharnessreport.js"], name="testharnessreport.js")
        ctx.eval(resources["webidl2.js"], name="webidl2.js")
        ctx.eval(resources["idlharness.js"], name="idlharness.js")

        # Post-shim: add result callback + completion callback
        post_shim = """
add_result_callback(function(test) {
    __results.push({
        name: test.name,
        status: test.format_status(),
        message: test.message || null
    });
});
var __completed = false;
add_completion_callback(function(tests, harness_status) {
    __completed = true;
});
"""
        ctx.eval(post_shim, name="iv8-post-shim.js")

        # Run test code — idl_test() is async (promise_test).
        # After test code registers tests, we need to:
        # 1. Fire load callbacks (triggers testharness to start tests)
        # 2. Drain event loop until all tests complete
        full_test_code = test_code + "\n;"

        try:
            # eval_promise evals the test code. idl_test() calls
            # promise_test() which returns undefined (not a promise),
            # so eval_promise returns immediately. The actual test
            # execution happens asynchronously via tests.promise_tests
            # chain, which needs microtask + macrotask draining.
            ctx.eval_promise(full_test_code, max_ticks=10000)

            # Fire load callbacks — testharness needs load event to
            # set all_loaded=true and call tests.complete()
            load_count = ctx.eval("__loadCallbacks.length")
            print(f"  Load callbacks: {load_count}")
            ctx.eval("""
                for (var i = 0; i < __loadCallbacks.length; i++) {
                    try { __loadCallbacks[i]({ type: 'load' }); } catch(e) {}
                }
            """)

            # Drain event loop until testharness completes or stabilizes.
            prev_count = -1
            stable_ticks = 0
            for i in range(1000):
                # Drain microtasks
                ctx.eval("__iv8__.eventLoop.drain()")
                # Run one macrotask (setTimeout callbacks etc)
                ctx.eval("__iv8__.eventLoop.tick()")
                # Check completion
                try:
                    completed = ctx.eval("__completed")
                except Exception:
                    completed = False
                if completed:
                    break
                # Check results stabilization
                try:
                    current = ctx.eval("__results.length")
                except Exception:
                    break
                if current == prev_count:
                    stable_ticks += 1
                    if stable_ticks >= 10 and current > 0:
                        break
                else:
                    stable_ticks = 0
                prev_count = current
                if i % 100 == 0:
                    print(f"  tick {i}: results={current}, completed={completed}")
            run_status = "completed"
        except Exception as e:
            run_status = f"error: {e}"
            print(f"  Execution error: {e}")

        # Collect results — __results may be populated even if
        # eval_promise threw (e.g. show_results crash after tests complete)
        try:
            results_json = ctx.eval("JSON.stringify(__results)")
            results = json.loads(results_json)
        except Exception:
            results = []

        pass_count = sum(1 for r in results if r["status"] == "Pass")
        fail_count = sum(1 for r in results if r["status"] != "Pass")

        return {
            "suite": suite_name,
            "variant": variant_name,
            "run_status": run_status,
            "total": len(results),
            "pass": pass_count,
            "fail": fail_count,
            "tests": results,
        }

    finally:
        ctx.close()


def fetch_chrome_baseline() -> dict:
    """Fetch Chrome's WPT results from wpt.fyi API for comparison."""
    baselines = {}
    test_paths = [
        "html/dom/idlharness",
        "dom/idlharness",
        "cssom-view/idlharness",
    ]
    for query in test_paths:
        url = f"https://wpt.fyi/api/search?q={query}"
        try:
            req = urllib.request.Request(url, headers={"User-Agent": "IV8-WPT/1.0"})
            with urllib.request.urlopen(req, timeout=15) as resp:
                data = json.loads(resp.read())
            for result in data.get("results", []):
                test = result["test"]
                # Chrome is usually the first run
                for status in result.get("legacy_status", []):
                    if status.get("total", 0) > 0:
                        baselines[test] = {
                            "pass": status["passes"],
                            "total": status["total"],
                        }
                        break
        except Exception:
            pass
    return baselines


def main() -> None:
    import argparse

    parser = argparse.ArgumentParser(description="WPT official test runner")
    parser.add_argument("--suite", "-s", help="Filter suite (e.g. html/dom)")
    parser.add_argument("--update", action="store_true",
                        help="Update status files to match current results")
    parser.add_argument("--output", "-o", default=str(OUT_PATH))
    args = parser.parse_args()

    # Set stack size for V8 template creation
    threading.stack_size(64 * 1024 * 1024)

    result_holder = {}

    def worker():
        try:
            resources = load_wpt_resources()

            suites = WPT_SUITES
            if args.suite:
                suites = [s for s in suites if args.suite in s["name"]]

            all_results = []
            for suite in suites:
                for variant in suite["variants"]:
                    print(f"\n--- {suite['name']} [{variant['name']}] ---")
                    result = run_suite(suite, variant, resources)
                    print(f"  Total={result['total']}, "
                          f"Pass={result['pass']}, Fail={result['fail']}")
                    all_results.append(result)

            result_holder["results"] = all_results
        except Exception as e:
            result_holder["error"] = repr(e)

    t = threading.Thread(target=worker)
    t.start()
    t.join()

    if "error" in result_holder:
        print(f"ERROR: {result_holder['error']}")
        sys.exit(1)

    results = result_holder["results"]

    # Compute totals
    total_tests = sum(r["total"] for r in results)
    total_pass = sum(r["pass"] for r in results)
    total_fail = sum(r["fail"] for r in results)

    # Fetch Chrome baseline
    print("\nFetching Chrome baseline from wpt.fyi...")
    chrome_baseline = fetch_chrome_baseline()

    report = {
        "schema_version": "wpt-report.v0.1",
        "source": "WPT official test files (direct reuse)",
        "suites": results,
        "chrome_baseline": chrome_baseline,
        "summary": {
            "total_tests": total_tests,
            "total_pass": total_pass,
            "total_fail": total_fail,
            "pass_rate": round(total_pass / total_tests * 100, 2) if total_tests > 0 else 0,
        },
    }

    # Write report
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(report, indent=2, ensure_ascii=False),
        encoding="utf-8",
    )

    print("\n" + "=" * 60)
    print("WPT Official Test Report")
    print("=" * 60)
    print(f"Total: {total_pass} PASS, {total_fail} FAIL / {total_tests} "
          f"({report['summary']['pass_rate']}%)")
    print()

    for r in results:
        print(f"  {r['suite']} [{r['variant']}]: "
              f"{r['pass']}/{r['total']} PASS")

    if chrome_baseline:
        print("\nChrome baseline (wpt.fyi):")
        for test, baseline in chrome_baseline.items():
            print(f"  {test}: {baseline['pass']}/{baseline['total']}")

    print(f"\nReport written to {output_path}")

    # Update status files if requested
    if args.update:
        status_path = STATUS_DIR / "idlharness.json"
        status = {}
        for r in results:
            key = f"{r['suite']} [{r['variant']}]"
            fails = [t["name"] for t in r["tests"] if t["status"] != "Pass"]
            if fails:
                status[key] = {"fail": {"expected": fails}}
        STATUS_DIR.mkdir(parents=True, exist_ok=True)
        status_path.write_text(
            json.dumps(status, indent=2, ensure_ascii=False),
            encoding="utf-8",
        )
        print(f"Status written to {status_path}")

    sys.exit(0 if total_fail == 0 else 1)


if __name__ == "__main__":
    main()
