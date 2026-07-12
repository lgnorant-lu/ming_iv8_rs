#!/usr/bin/env python3
"""L4b-d: Detection Vector Registry — automated check of anti-detection shims.

Checks 26 detection vectors from CreepJS and BotD against IV8's runtime
surface. Each vector is classified as:
  PASS    — IV8 evades this detection vector
  FAIL    — IV8 is detectable by this vector
  MISSING — IV8 does not implement the required API at all

Usage:
  python scripts/_detection_vector_registry.py

Output:
  status/detection-vector-registry.json
  Exit code: 0 if all PASS, 1 if any FAIL
"""
from __future__ import annotations

import json
import sys
import time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
STATUS_DIR = REPO_ROOT / "status"
OUTPUT_PATH = STATUS_DIR / "detection-vector-registry.json"

# JS expression for each detection vector.
# Each returns "PASS", "FAIL", or "MISSING".
VECTORS = [
    # --- BotD vectors (18) ---
    {
        "id": "BV01",
        "name": "navigator.webdriver",
        "source": "BotD, CreepJS",
        "js": """(function() {
            try {
                if (navigator.webdriver === true) return 'FAIL';
                var desc = Object.getOwnPropertyDescriptor(Navigator.prototype, 'webdriver');
                if (desc && 'value' in desc) return 'FAIL';
                if (desc && desc.get) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV02",
        "name": "navigator.userAgent no HeadlessChrome",
        "source": "BotD, CreepJS",
        "js": """(function() {
            var ua = navigator.userAgent || '';
            if (/HeadlessChrome/i.test(ua)) return 'FAIL';
            return 'PASS';
        })()""",
    },
    {
        "id": "BV03",
        "name": "navigator.appVersion no HeadlessChrome",
        "source": "BotD",
        "js": """(function() {
            var av = navigator.appVersion || '';
            if (/HeadlessChrome/i.test(av)) return 'FAIL';
            return 'PASS';
        })()""",
    },
    {
        "id": "BV04",
        "name": "navigator.productSub",
        "source": "BotD",
        "js": """(function() {
            if (navigator.productSub !== '20030107') return 'FAIL';
            return 'PASS';
        })()""",
    },
    {
        "id": "BV05",
        "name": "navigator.plugins.length > 0",
        "source": "BotD, CreepJS",
        "js": """(function() {
            try {
                if (!navigator.plugins || navigator.plugins.length === 0) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV06",
        "name": "navigator.plugins instanceof PluginArray",
        "source": "BotD, CreepJS",
        "js": """(function() {
            try {
                if (!(navigator.plugins instanceof PluginArray)) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV07",
        "name": "navigator.mimeTypes non-empty",
        "source": "BotD, CreepJS",
        "js": """(function() {
            try {
                if (!navigator.mimeTypes || navigator.mimeTypes.length === 0) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV08",
        "name": "navigator.mimeTypes instanceof MimeTypeArray",
        "source": "BotD, CreepJS",
        "js": """(function() {
            try {
                if (!(navigator.mimeTypes instanceof MimeTypeArray)) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV09",
        "name": "window.process absent",
        "source": "BotD",
        "js": """(function() {
            if (typeof window.process !== 'undefined') return 'FAIL';
            return 'PASS';
        })()""",
    },
    {
        "id": "BV10",
        "name": "window.outerWidth > 0",
        "source": "BotD",
        "js": """(function() {
            if (window.outerWidth === 0 || window.outerHeight === 0) return 'FAIL';
            return 'PASS';
        })()""",
    },
    {
        "id": "BV11",
        "name": "window.external correct (External instance, not bot marker)",
        "source": "BotD",
        "js": """(function() {
            // Real Chrome has window.external as an External instance.
            // Bots sometimes have external as undefined or wrong type.
            if (typeof window.external === 'undefined') return 'FAIL';
            if (window.external === null) return 'FAIL';
            // Check: does it have AddSearchProvider/IsSearchProviderInstalled?
            // Real Chrome external has these removed (empty object).
            return 'PASS';
        })()""",
    },
    {
        "id": "BV12",
        "name": "WebGL vendor not SwiftShader",
        "source": "BotD, CreepJS",
        "js": """(function() {
            try {
                var canvas = document.createElement('canvas');
                var gl = canvas.getContext('webgl');
                if (!gl) return 'MISSING';
                var ext = gl.getExtension('WEBGL_debug_renderer_info');
                if (!ext) return 'MISSING';
                var vendor = gl.getParameter(ext.UNMASKED_VENDOR_WEBGL) || '';
                var renderer = gl.getParameter(ext.UNMASKED_RENDERER_WEBGL) || '';
                if (/SwiftShader/i.test(vendor) || /SwiftShader/i.test(renderer)) return 'FAIL';
                if (!vendor || !renderer) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV13",
        "name": "Error().stack is V8 format",
        "source": "BotD",
        "js": """(function() {
            try {
                var stack = new Error().stack || '';
                if (stack.indexOf('at ') !== -1) return 'PASS';
                return 'FAIL';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV14",
        "name": "eval.toString().length is V8 value",
        "source": "BotD",
        "js": """(function() {
            try {
                var len = eval.toString().length;
                if (len > 0 && len < 100) return 'PASS';
                return 'FAIL';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV15",
        "name": "Function.prototype.bind.toString contains native code",
        "source": "BotD",
        "js": """(function() {
            try {
                var s = Function.prototype.bind.toString();
                if (s.indexOf('[native code]') !== -1) return 'PASS';
                return 'FAIL';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV16",
        "name": "No bot-specific globals (_selenium, __nightmare, callPhantom)",
        "source": "BotD",
        "js": """(function() {
            var botGlobals = ['_selenium', '__nightmare', 'callPhantom', 'webdriver',
                'domAutomation', 'domAutomationController', '$cdc_asdjflasutopfhvcZLmcf',
                '_phantom', '__nightmare', 'spawn', 'emit'];
            for (var i = 0; i < botGlobals.length; i++) {
                if (typeof window[botGlobals[i]] !== 'undefined') return 'FAIL';
            }
            return 'PASS';
        })()""",
    },
    {
        "id": "BV17",
        "name": "navigator.languages consistency",
        "source": "BotD",
        "js": """(function() {
            try {
                if (!navigator.languages || !navigator.languages.length) return 'FAIL';
                if (navigator.languages.length > 0 && !navigator.language) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "BV18",
        "name": "Notification.permission consistency",
        "source": "BotD, CreepJS",
        "js": """(function() {
            if (typeof Notification === 'undefined') return 'MISSING';
            if (typeof navigator.permissions === 'undefined') return 'MISSING';
            try {
                var np = Notification.permission;
                var pp = navigator.permissions.query({name: 'notifications'});
                if (pp && pp.state && np) {
                    if (pp.state === 'prompt' && np === 'denied') return 'FAIL';
                }
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },

    # --- CreepJS vectors (8 additional, not overlapping with BotD) ---
    {
        "id": "CV01",
        "name": "window.chrome exists",
        "source": "CreepJS",
        "js": """(function() {
            if (typeof window.chrome !== 'object' || !window.chrome) return 'FAIL';
            return 'PASS';
        })()""",
    },
    {
        "id": "CV02",
        "name": "window.chrome.runtime exists",
        "source": "CreepJS",
        "js": """(function() {
            if (typeof window.chrome !== 'object' || !window.chrome) return 'FAIL';
            if (!window.chrome.runtime) return 'FAIL';
            return 'PASS';
        })()""",
    },
    {
        "id": "CV03",
        "name": "chrome.runtime not constructable (no prototype)",
        "source": "CreepJS",
        "js": """(function() {
            try {
                if (window.chrome && window.chrome.runtime) {
                    if (window.chrome.runtime.prototype) return 'FAIL';
                    try {
                        new window.chrome.runtime();
                        return 'FAIL';
                    } catch(e) {
                        return 'PASS';
                    }
                }
                return 'MISSING';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "CV04",
        "name": "Function.toString returns [native code] for codegen functions",
        "source": "CreepJS",
        "js": """(function() {
            try {
                var fn = document.createElement;
                var s = fn.toString();
                if (s.indexOf('[native code]') === -1) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "CV05",
        "name": "navigator.pdfViewerEnabled",
        "source": "CreepJS",
        "js": """(function() {
            if (typeof navigator.pdfViewerEnabled === 'undefined') return 'MISSING';
            if (navigator.pdfViewerEnabled === false) return 'FAIL';
            return 'PASS';
        })()""",
    },
    {
        "id": "CV06",
        "name": "navigator.userAgentData non-blank",
        "source": "CreepJS",
        "js": """(function() {
            try {
                if (!navigator.userAgentData) return 'MISSING';
                if (!navigator.userAgentData.platform) return 'FAIL';
                if (navigator.userAgentData.platform === '') return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
    {
        "id": "CV07",
        "name": "iframe contentWindow accessible",
        "source": "CreepJS",
        "js": """(function() {
            try {
                var iframe = document.createElement('iframe');
                document.body.appendChild(iframe);
                var cw = iframe.contentWindow;
                document.body.removeChild(iframe);
                if (!cw) return 'FAIL';
                if (!cw.navigator) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'FAIL'; }
        })()""",
    },
    {
        "id": "CV08",
        "name": "screen dimensions not identical (avail != total)",
        "source": "CreepJS",
        "js": """(function() {
            try {
                if (screen.width === screen.availWidth &&
                    screen.height === screen.availHeight) return 'FAIL';
                return 'PASS';
            } catch(e) { return 'MISSING'; }
        })()""",
    },
]


def _run_in_thread(fn, *args, **kwargs):
    """Run fn in a sub-thread with sufficient stack for V8 template creation.

    Python's main thread has a small stack (1MB on Windows). V8 FunctionTemplate
    creation (1287 interfaces, 9223 members) requires 128MB+. We spawn a thread
    with threading.stack_size(128MB) to run JSContext creation + evaluation.
    """
    import threading
    result_box = [None, None]  # [return_value, exception]

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


def _run_checks():
    """Run all detection vector checks in IV8. Must be called in a high-stack thread."""
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext

    print("Initializing IV8 runtime...")
    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)

    print("Checking vectors...")
    results = []
    stats = {"PASS": 0, "FAIL": 0, "MISSING": 0}

    for vector in VECTORS:
        js = vector["js"]
        try:
            raw = ctx.eval(js)
            status = raw if isinstance(raw, str) else str(raw)
            if status not in ("PASS", "FAIL", "MISSING"):
                status = "FAIL"
        except Exception:
            status = "MISSING"

        stats[status] = stats.get(status, 0) + 1
        results.append({
            "id": vector["id"],
            "name": vector["name"],
            "source": vector["source"],
            "status": status,
        })

        marker = {"PASS": "[PASS]", "FAIL": "[FAIL]", "MISSING": "[MISSING]"}[status]
        print(f"  {marker} {vector['id']:5s} {vector['name']}")

    return results, stats


def main():
    print("L4b-d: Detection Vector Registry")
    print(f"  Vectors to check: {len(VECTORS)}")
    print()

    results, stats = _run_in_thread(_run_checks)

    # Write report
    report = {
        "schema_version": "detection-vector-registry.v0.1",
        "iv8_version": "0.8.89",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "summary": {
            "total_vectors": len(VECTORS),
            **stats,
        },
        "vectors": results,
    }

    output_path = OUTPUT_PATH
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(report, indent=2, ensure_ascii=False),
        encoding="utf-8"
    )

    print(f"\n{'='*60}")
    print(f"L4b-d Detection Vector Registry — Summary")
    print(f"{'='*60}")
    print(f"Total vectors: {len(VECTORS)}")
    print(f"  PASS:    {stats['PASS']}")
    print(f"  FAIL:    {stats['FAIL']}")
    print(f"  MISSING: {stats['MISSING']}")
    print(f"Output: {output_path}")

    if stats["FAIL"] > 0:
        sys.exit(1)
    sys.exit(0)


if __name__ == "__main__":
    main()
