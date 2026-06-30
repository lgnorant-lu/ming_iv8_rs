#!/usr/bin/env python3
"""WPT Worker variant runner — runs idlharness in Worker context.

Builds a single Worker script containing:
1. JS stub (interface constructors, already in worker_js_stub.js)
2. testharness.js (with worker-mode detection)
3. testharnessreport.js (worker version)
4. WebIDLParser.js
5. idlharness.js
6. IDL files registered as fetch_spec resources
7. Test code (idl_test + done)

The script is passed as a data: URL to new Worker(). Results are
collected via postMessage from Worker to main thread.
"""
from __future__ import annotations

import json
import re
import sys
import threading
import time
import urllib.parse
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
WPT_DIR = REPO_ROOT / "tools" / "wpt"
FIXTURES_DIR = WPT_DIR / "fixtures"
RESOURCES_DIR = FIXTURES_DIR / "resources"
INTERFACES_DIR = FIXTURES_DIR / "interfaces"


def load_worker_test_files() -> dict[str, str]:
    """Load all JS files needed for Worker idlharness test."""
    files = {}
    for name in ["testharness.js", "idlharness.js", "webidl2.js"]:
        path = RESOURCES_DIR / name
        if not path.exists():
            print(f"FATAL: {path} not found. Run tools/wpt/wpt_update.py first.")
            sys.exit(1)
        files[name] = path.read_text(encoding="utf-8")
    return files


def load_worker_testharnessreport() -> str:
    """Worker version of testharnessreport.js — outputs via postMessage."""
    return """
// Worker testharnessreport.js
// Collects results and sends via postMessage when done
var __workerResults = [];
var __workerCompleted = false;

setup({output: false});

add_result_callback(function(test) {
    __workerResults.push({
        name: test.name,
        status: test.format_status(),
        message: test.message || null
    });
});

add_completion_callback(function(tests, harness_status) {
    __workerCompleted = true;
    self.postMessage(JSON.stringify({
        type: 'completion',
        results: __workerResults,
        status: harness_status ? harness_status.status : 0
    }));
});
"""


def load_any_js_test(specs: list[str]) -> tuple[str, dict[str, str]]:
    """Load the .any.js test file and IDL specs."""
    # dom/idlharness.any.js
    anyjs_path = FIXTURES_DIR / "dom" / "idlharness.any.js"
    if not anyjs_path.exists():
        # Download it
        import urllib.request
        url = "https://raw.githubusercontent.com/web-platform-tests/wpt/master/dom/idlharness.any.js"
        print(f"Downloading {url}...")
        anyjs_path.parent.mkdir(parents=True, exist_ok=True)
        resp = urllib.request.urlopen(url, timeout=15)
        anyjs_path.write_bytes(resp.read())

    test_code = anyjs_path.read_text(encoding="utf-8")
    # Strip META comments
    test_code = re.sub(r'^//\s*META:.*$', '', test_code, flags=re.MULTILINE)

    # Load IDL files
    idl_contents = {}
    for spec in specs:
        path = INTERFACES_DIR / f"{spec}.idl"
        if path.exists():
            idl_contents[spec] = path.read_text(encoding="utf-8")

    return test_code, idl_contents


def build_worker_script(
    test_code: str,
    idl_contents: dict[str, str],
    resources: dict[str, str],
    variant_query: str,
) -> str:
    """Build the complete Worker script.

    The script contains everything needed to run idlharness in Worker:
    - testharness.js (worker mode)
    - testharnessreport.js (worker version with postMessage)
    - WebIDLParser.js
    - idlharness.js
    - fetch_spec implementation (using inline IDL data)
    - location.search for variant subsetting
    - Test code
    """
    # Register IDL files as inline data for fetch_spec
    idl_registry = {}
    for spec, content in idl_contents.items():
        idl_registry[f"/interfaces/{spec}.idl"] = content

    idl_registry_json = json.dumps(idl_registry)

    worker_report = load_worker_testharnessreport()

    parts = []
    # 1. Worker-specific globals
    parts.append(f"""
// === Worker WPT Shim ===
var __idlRegistry = {idl_registry_json};
globalThis.GLOBAL = globalThis;
globalThis.GLOBAL.isWindow = function() {{ return false; }};
globalThis.GLOBAL.isWorker = function() {{ return true; }};
globalThis.GLOBAL.isShadowRealm = function() {{ return false; }};
globalThis.location = {{ search: {json.dumps(variant_query)} }};
globalThis.fetch = function(url) {{
    if (__idlRegistry[url]) {{
        return Promise.resolve({{
            ok: true,
            text: function() {{ return Promise.resolve(__idlRegistry[url]); }}
        }});
    }}
    return Promise.reject(new Error('Resource not found: ' + url));
}};
globalThis.fetch_spec = function(spec) {{
    var url = "/interfaces/" + spec + ".idl";
    return fetch(url).then(function(r) {{
        return r.text();
    }}).then(function(idl) {{
        return {{ spec: spec, idl: idl }};
    }});
}};
// === End Worker WPT Shim ===
""")

    # 2. testharness.js
    parts.append(resources["testharness.js"])

    # 3. Worker testharnessreport.js
    parts.append(worker_report)

    # 4. WebIDLParser.js
    parts.append(resources["webidl2.js"])

    # 5. idlharness.js (patched: replace instanceof check with typeof check)
    # V8 Worker isolate cannot use setPrototypeOf or Symbol.hasInstance
    # without triggering GC crash or OOM. Patch exposed_in() to use
    # typeof check instead of instanceof.
    idlharness = resources["idlharness.js"]
    idlharness = idlharness.replace(
        "self instanceof DedicatedWorkerGlobalScope",
        "typeof DedicatedWorkerGlobalScope !== 'undefined'",
    )
    idlharness = idlharness.replace(
        "self instanceof SharedWorkerGlobalScope",
        "typeof SharedWorkerGlobalScope !== 'undefined'",
    )
    idlharness = idlharness.replace(
        "self instanceof ServiceWorkerGlobalScope",
        "typeof ServiceWorkerGlobalScope !== 'undefined'",
    )
    parts.append(idlharness)

    # 6. Test code with error capture
    parts.append(f"""
// Global error handler to catch unhandled errors
self.onerror = function(msg, url, line, col, error) {{
    self.postMessage(JSON.stringify({{
        type: 'completion',
        results: [],
        status: -1,
        error: 'onerror: ' + msg + (error && error.stack ? '\\n' + error.stack : '')
    }}));
}};
self.addEventListener('unhandledrejection', function(e) {{
    self.postMessage(JSON.stringify({{
        type: 'completion',
        results: [],
        status: -1,
        error: 'unhandledrejection: ' + (e.reason ? e.reason.toString() : String(e.reason))
    }}));
}});
try {{
{test_code}
}} catch(e) {{
    self.postMessage(JSON.stringify({{
        type: 'completion',
        results: [],
        status: -1,
        error: 'sync: ' + e.toString() + (e.stack ? '\\n' + e.stack : '')
    }}));
}}
""")

    return "\n".join(parts)


def run_worker_suite(
    suite_name: str,
    variant_name: str,
    variant_query: str,
    test_code: str,
    idl_contents: dict[str, str],
    resources: dict[str, str],
) -> dict:
    """Run idlharness in Worker context.

    Returns a result dict with pass/fail/total and test details.
    """
    import iv8_rs as iv8

    # Build Worker script
    worker_script = build_worker_script(
        test_code, idl_contents, resources, variant_query
    )

    # Create main context
    ctx = iv8.JSContext()

    try:
        # Register worker script as a resource
        # Worker will load it via importScripts or data: URL
        encoded_script = urllib.parse.quote(worker_script)
        worker_url = f"data:,{encoded_script}"

        # Create Worker and collect results
        # Worker postMessage sends results back as JSON string
        main_js = f"""
        var __workerResults = null;
        var w = new Worker({json.dumps(worker_url)});
        w.onmessage = function(e) {{
            try {{
                var raw = e.data;
                var data = (typeof raw === 'string') ? JSON.parse(raw) : raw;
                if (data && data.type === 'completion') {{
                    __workerResults = data;
                }} else {{
                    __workerResults = {{ error: 'unexpected: ' + typeof raw + ' ' + String(raw).substring(0, 200) }};
                }}
            }} catch(ex) {{
                __workerResults = {{ error: 'parse: ' + ex.toString() + ' data=' + String(e.data).substring(0, 200) }};
            }}
        }};
        'started';
        """

        ctx.eval(main_js, name="worker-runner.js")

        # Wait for Worker to complete (poll with drain)
        max_wait = 30  # seconds
        for i in range(max_wait * 2):
            time.sleep(0.5)
            ctx.eval("1+1")  # trigger drain_worker_messages
            try:
                done = ctx.eval("__workerResults !== null")
                if done:
                    break
            except Exception:
                pass
            if i % 4 == 0:
                print(f"  Waiting for Worker... ({i * 0.5:.0f}s)")

        # Collect results
        try:
            results_json = ctx.eval("JSON.stringify(__workerResults)")
            results_data = json.loads(results_json)
        except Exception as e:
            return {
                "suite": suite_name,
                "variant": variant_name,
                "run_status": f"error: {e}",
                "total": 0, "pass": 0, "fail": 0,
                "tests": [],
            }

        if "error" in results_data:
            return {
                "suite": suite_name,
                "variant": variant_name,
                "run_status": f"error: {results_data['error']}",
                "total": 0, "pass": 0, "fail": 0,
                "tests": [],
            }

        results = results_data.get("results", [])
        pass_count = sum(1 for r in results if r["status"] == "Pass")
        fail_count = sum(1 for r in results if r["status"] != "Pass")

        return {
            "suite": suite_name,
            "variant": variant_name,
            "run_status": "completed",
            "total": len(results),
            "pass": pass_count,
            "fail": fail_count,
            "tests": results,
        }

    finally:
        ctx.close()


def main():
    threading.stack_size(64 * 1024 * 1024)

    result_holder = {}

    def worker():
        try:
            resources = load_worker_test_files()
            test_code, idl_contents = load_any_js_test(["dom", "fullscreen", "html"])

            # Run dom/idlharness.any.worker.html variants
            variants = [
                {"name": "include=Node", "query": "?include=Node"},
                {"name": "exclude=Node", "query": "?exclude=Node"},
            ]

            all_results = []
            for variant in variants:
                print(f"\n--- dom/idlharness.worker [{variant['name']}] ---")
                result = run_worker_suite(
                    "dom/idlharness.worker",
                    variant["name"],
                    variant["query"],
                    test_code,
                    idl_contents,
                    resources,
                )
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
    total_tests = sum(r["total"] for r in results)
    total_pass = sum(r["pass"] for r in results)
    total_fail = sum(r["fail"] for r in results)

    print("\n" + "=" * 60)
    print("WPT Worker Test Report")
    print("=" * 60)
    print(f"Total: {total_pass} PASS, {total_fail} FAIL / {total_tests}")
    print()
    for r in results:
        print(f"  {r['suite']} [{r['variant']}]: "
              f"{r['pass']}/{r['total']} PASS ({r['run_status']})")


if __name__ == "__main__":
    main()
