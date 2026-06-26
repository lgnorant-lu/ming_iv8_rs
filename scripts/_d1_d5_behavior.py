"""D1 (method return value semantics) + D5 (exception behavior) test suite.

D1: Verify that methods return the correct type (Promise vs undefined vs object
    vs primitive) per the Web IDL specification.

D5: Verify that `new XxxInterface()` throws TypeError for non-constructable
    interfaces, and that DOMException has the correct shape.

Usage:
    .venv\\Scripts\\python.exe scripts\\test_d1_d5_behavior.py
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent


# ---------------------------------------------------------------------------
# D1 test cases: method return value semantics
# ---------------------------------------------------------------------------
# Each entry: (test_id, description, js_expression, expected_predicate)
# The JS expression must return true for PASS, false for FAIL.
# For async tests, we drain microtasks inside the JS block.

D1_TESTS: list[tuple[str, str, str]] = [
    # --- Promise-returning methods (D1-01 through D1-07) ---

    (
        "D1-01",
        "navigator.clipboard.readText() returns Promise",
        r"""
        (function() {
            try {
                var r = navigator.clipboard.readText();
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-02",
        "navigator.clipboard.writeText() returns Promise",
        r"""
        (function() {
            try {
                var r = navigator.clipboard.writeText('test');
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-03",
        "navigator.credentials.get() returns Promise",
        r"""
        (function() {
            try {
                var r = navigator.credentials.get({password: true});
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-04",
        "navigator.getBattery() returns Promise",
        r"""
        (function() {
            try {
                var r = navigator.getBattery();
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-05",
        "navigator.permissions.query() returns Promise",
        r"""
        (function() {
            try {
                var r = navigator.permissions.query({name: 'notifications'});
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-06",
        "navigator.requestMIDIAccess() returns Promise",
        r"""
        (function() {
            try {
                var r = navigator.requestMIDIAccess();
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-07",
        "navigator.requestMediaKeySystemAccess() returns Promise",
        r"""
        (function() {
            try {
                var r = navigator.requestMediaKeySystemAccess('invalid-key-system');
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-08",
        "navigator.serviceWorker.register() returns Promise",
        r"""
        (function() {
            try {
                var r = navigator.serviceWorker.register('sw.js');
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),

    # --- D1 fixes: previously broken Promise returns ---

    (
        "D1-09",
        "Notification.requestPermission() returns Promise",
        r"""
        (function() {
            try {
                if (typeof Notification === 'undefined') return false;
                var r = Notification.requestPermission();
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-10",
        "navigator.getInstalledRelatedApps() returns Promise",
        r"""
        (function() {
            try {
                if (typeof navigator.getInstalledRelatedApps !== 'function') return false;
                var r = navigator.getInstalledRelatedApps();
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-11",
        "navigator.wakeLock.request('screen') returns Promise",
        r"""
        (function() {
            try {
                if (!navigator.wakeLock || typeof navigator.wakeLock.request !== 'function') return false;
                var r = navigator.wakeLock.request('screen');
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-12",
        "navigator.wakeLock.request() returns Promise (persistent across accesses)",
        r"""
        (function() {
            try {
                var wl1 = navigator.wakeLock;
                var wl2 = navigator.wakeLock;
                if (!wl1 || typeof wl1.request !== 'function') return false;
                if (typeof wl2.request !== 'function') return false;
                var r = wl2.request('screen');
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),

    # --- PaymentRequest ---
    (
        "D1-13",
        "PaymentRequest.prototype.canMakePayment() returns Promise",
        r"""
        (function() {
            try {
                if (typeof PaymentRequest === 'undefined') return false;
                var pr = new PaymentRequest([{supportedMethods: 'basic-card'}], {total: {label: 'x', amount: {currency: 'USD', value: '1'}}});
                var r = pr.canMakePayment();
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),

    # --- Observer takeRecords (D1 fix targets) ---

    (
        "D1-14",
        "IntersectionObserver.takeRecords() returns Array",
        r"""
        (function() {
            try {
                var obs = new IntersectionObserver(function() {});
                var r = obs.takeRecords();
                return Array.isArray(r);
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-15",
        "ResizeObserver.takeRecords() returns Array",
        r"""
        (function() {
            try {
                var obs = new ResizeObserver(function() {});
                var r = obs.takeRecords();
                return Array.isArray(r);
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-16",
        "ReportingObserver.takeRecords() returns Array",
        r"""
        (function() {
            try {
                var obs = new ReportingObserver(function() {});
                var r = obs.takeRecords();
                return Array.isArray(r);
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-17",
        "MutationObserver.takeRecords() returns Array",
        r"""
        (function() {
            try {
                var obs = new MutationObserver(function() {});
                var r = obs.takeRecords();
                return Array.isArray(r);
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-18",
        "PerformanceObserver.takeRecords() returns Array",
        r"""
        (function() {
            try {
                var obs = new PerformanceObserver(function() {});
                var r = obs.takeRecords();
                return Array.isArray(r);
            } catch(e) { return false; }
        })()
        """,
    ),

    # --- Non-Promise return types (boolean/object/primitive) ---

    (
        "D1-19",
        "navigator.sendBeacon() returns boolean",
        r"""
        (function() {
            try {
                var r = navigator.sendBeacon('https://example.com', '');
                return typeof r === 'boolean';
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-20",
        "window.matchMedia() returns MediaQueryList (not Promise)",
        r"""
        (function() {
            try {
                var r = window.matchMedia('(prefers-color-scheme: dark)');
                return typeof r === 'object' && r !== null && !(r instanceof Promise) && 'matches' in r && 'media' in r;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-21",
        "crypto.randomUUID() returns string",
        r"""
        (function() {
            try {
                var r = crypto.randomUUID();
                return typeof r === 'string' && /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(r);
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-22",
        "crypto.getRandomValues() returns same ArrayBufferView",
        r"""
        (function() {
            try {
                var arr = new Uint8Array(8);
                var r = crypto.getRandomValues(arr);
                return r === arr;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-23",
        "crypto.subtle.digest() returns Promise",
        r"""
        (function() {
            try {
                var r = crypto.subtle.digest('SHA-256', new Uint8Array([1,2,3]));
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-24",
        "crypto.subtle.generateKey() returns Promise",
        r"""
        (function() {
            try {
                var r = crypto.subtle.generateKey({name: 'AES-GCM', length: 256}, true, ['encrypt', 'decrypt']);
                return r instanceof Promise;
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-25",
        "navigator.usb is object with requestDevice method (known codegen gap)",
        r"""
        (function() {
            try {
                var usb = navigator.usb;
                return typeof usb === 'object' && usb !== null && typeof usb.requestDevice === 'function';
            } catch(e) { return false; }
        })()
        """,
    ),
    (
        "D1-26",
        "navigator.bluetooth is object with requestDevice method (known codegen gap)",
        r"""
        (function() {
            try {
                var bt = navigator.bluetooth;
                return typeof bt === 'object' && bt !== null && typeof bt.requestDevice === 'function';
            } catch(e) { return false; }
        })()
        """,
    ),
]


# ---------------------------------------------------------------------------
# D5 test cases: exception behavior
# ---------------------------------------------------------------------------
# Interfaces that should throw TypeError when constructed with `new`.
# These are Web IDL interfaces marked as non-constructable.

D5_ILLEGAL_CONSTRUCTOR_INTERFACES: list[str] = [
    # Core DOM abstract types — definitively non-constructable in Chrome
    "Navigator",
    "Screen",
    "WorkerNavigator",
    "Node",
    "Element",
    "HTMLElement",
    "Document",
    "CharacterData",
    "Text",
    "Comment",
    "DocumentFragment",
    "ShadowRoot",
    "DocumentType",
    "Attr",
    "NodeList",
    "HTMLCollection",
    "DOMTokenList",
    "NamedNodeMap",
    "DOMImplementation",
    "Range",
    "EventTarget",
    # Window / browsing context singletons
    "Window",
    "History",
    "Location",
    "Storage",
    "ScreenOrientation",
    "VisualViewport",
    "MediaQueryList",
    "Crypto",
    "SubtleCrypto",
    "CryptoKey",
    "Plugin",
    "MimeType",
    "PluginArray",
    "BarProp",
    # Performance (entry types, not constructable)
    "Performance",
    "PerformanceEntry",
    "PerformanceNavigationTiming",
    "PerformanceResourceTiming",
    # CSS OM non-constructable
    "CSSStyleDeclaration",
    "CSSRule",
    "CSSRuleList",
    "StyleSheetList",
    "MediaList",
    "StyleSheet",
    # DataTransfer / clipboard non-constructable
    "DataTransfer",
    "DataTransferItemList",
    "DataTransferItem",
    "Clipboard",
    # Audio nodes (non-constructable in Chrome, but overridden by audio_context.rs)
    # -- moved to D5_JS_SHIM_OVERRIDDEN
    "AudioListener",
    "AudioDestinationNode",
    # Media non-constructable
    "MediaStreamTrack",
    # Non-constructable singletons
    "PushManager",
    "PushSubscription",
    "ServiceWorker",
    "TextMetrics",
    "CanvasGradient",
    "CanvasPattern",
    "MutationRecord",
    "AbstractRange",
    "DOMStringMap",
    "ApplicationCache",
    "CacheStorage",
    "Cache",
]

# Interfaces that are non-constructable in real Chrome but are intentionally
# overridden by IV8 JS shims (audio_context.rs, window_extras.rs) with
# working constructors. These are design decisions, not bugs.
D5_JS_SHIM_OVERRIDDEN: list[str] = [
    "HTMLDocument",      # window_extras.rs:47
    "AudioParam",        # audio_context.rs:292
    "OscillatorNode",    # audio_context.rs:294
    "GainNode",          # audio_context.rs:293
    "DynamicsCompressorNode",  # audio_context.rs (inline)
    "AnalyserNode",      # audio_context.rs (inline)
]

# Interfaces that ARE constructable in real browsers — used as negative
# controls to verify we don't falsely claim TypeError.
# Each entry: (interface_name, constructor_args_js)
# constructor_args_js is the JS expression for arguments to `new`.
D5_CONSTRUCTABLE_INTERFACES: list[tuple[str, str]] = [
    ("Event", "'test'"),
    ("Headers", "{}"),
    ("Request", "'http://example.com'"),
    ("Response", "''"),
    ("URL", "'http://example.com'"),
    ("ArrayBuffer", "8"),
    ("Error", ""),
    ("TypeError", "''"),
    ("DOMException", "'x'"),
    ("Map", ""),
    ("Set", ""),
    ("Promise", "function(){}"),
    ("Date", ""),
    ("RegExp", "''"),
    ("FormData", ""),
]


def build_d5_constructor_test_js(interface_name: str, args: str = "") -> str:
    """Build JS that tests whether `new InterfaceName()` throws TypeError.

    Returns a JSON string: {"throws": true/false, "is_typeerror": true/false,
    "error_type": "...", "error_msg": "..."}
    """
    name = interface_name.replace("'", "\\'")
    args_js = args if args else ""
    return (
        "(function() {\n"
        "    try {\n"
        f"        var fn = window['{name}'];\n"
        "        if (typeof fn !== 'function') {\n"
        "            return JSON.stringify({throws: false, is_typeerror: false, not_function: true, error_type: '', error_msg: ''});\n"
        "        }\n"
        f"        var obj = new fn({args_js});\n"
        "        return JSON.stringify({throws: false, is_typeerror: false, not_function: false, error_type: '', error_msg: ''});\n"
        "    } catch(e) {\n"
        "        return JSON.stringify({\n"
        "            throws: true,\n"
        "            is_typeerror: (e instanceof TypeError),\n"
        "            not_function: false,\n"
        "            error_type: e.constructor ? e.constructor.name : 'Unknown',\n"
        "            error_msg: e.message || ''\n"
        "        });\n"
        "    }\n"
        "})()\n"
    )


def build_d5_domexception_tests() -> list[tuple[str, str, str]]:
    """Build D5 DOMException shape verification tests."""
    return [
        (
            "D5-DOM-01",
            "new DOMException('test', 'NotSupportedError').name === 'NotSupportedError'",
            r"""
            (function() {
                try {
                    var e = new DOMException('test', 'NotSupportedError');
                    return e.name === 'NotSupportedError' && e.message === 'test';
                } catch(ex) { return false; }
            })()
            """,
        ),
        (
            "D5-DOM-02",
            "new DOMException('test') has default name 'Error'",
            r"""
            (function() {
                try {
                    var e = new DOMException('test');
                    return e.name === 'Error';
                } catch(ex) { return false; }
            })()
            """,
        ),
        (
            "D5-DOM-03",
            "DOMException is constructable (does NOT throw TypeError)",
            r"""
            (function() {
                try {
                    var e = new DOMException('x');
                    return e instanceof DOMException;
                } catch(ex) { return ex instanceof TypeError ? false : false; }
            })()
            """,
        ),
        (
            "D5-DOM-04",
            "DOMException instances have .code property",
            r"""
            (function() {
                try {
                    var e = new DOMException('x', 'NotSupportedError');
                    return typeof e.code === 'number';
                } catch(ex) { return false; }
            })()
            """,
        ),
        (
            "D5-DOM-05",
            "new Navigator() throws TypeError",
            r"""
            (function() {
                try {
                    new Navigator();
                    return false;
                } catch(e) {
                    return e instanceof TypeError;
                }
            })()
            """,
        ),
        (
            "D5-DOM-06",
            "new Screen() throws TypeError",
            r"""
            (function() {
                try {
                    new Screen();
                    return false;
                } catch(e) {
                    return e instanceof TypeError;
                }
            })()
            """,
        ),
    ]


def main() -> int:
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext

    print("Initializing IV8 runtime...")
    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)
    print("Page loaded. Running D1 + D5 behavior tests...")
    print()

    total_pass = 0
    total_fail = 0
    total_skip = 0
    failures: list[tuple[str, str, str]] = []

    # ===================================================================
    # D1 TESTS
    # ===================================================================
    print("=" * 72)
    print(" D1: Method Return Value Semantics")
    print("=" * 72)

    d1_pass = 0
    d1_fail = 0

    for test_id, desc, js in D1_TESTS:
        try:
            result = ctx.eval(js)
            if result is True:
                d1_pass += 1
                print(f"  PASS  {test_id}: {desc}")
            elif result is False:
                d1_fail += 1
                print(f"  FAIL  {test_id}: {desc}")
                failures.append((test_id, desc, "returned false"))
            else:
                d1_fail += 1
                print(f"  FAIL  {test_id}: {desc} (unexpected result: {result!r})")
                failures.append((test_id, desc, f"unexpected: {result!r}"))
        except Exception as e:
            d1_fail += 1
            print(f"  FAIL  {test_id}: {desc} (exception: {e})")
            failures.append((test_id, desc, f"exception: {e}"))

    print()
    print(f"  D1 Summary: {d1_pass} PASS / {d1_fail} FAIL / {len(D1_TESTS)} total")
    print()

    # ===================================================================
    # D5 TESTS: Illegal Constructor (TypeError)
    # ===================================================================
    print("=" * 72)
    print(" D5: Exception Behavior - Illegal Constructor")
    print("=" * 72)

    d5_typeerror_pass = 0
    d5_typeerror_fail = 0
    d5_typeerror_skip = 0
    d5_typeerror_not_function = 0

    for iface in D5_ILLEGAL_CONSTRUCTOR_INTERFACES:
        js = build_d5_constructor_test_js(iface)
        try:
            raw = ctx.eval(js)
            data = json.loads(raw)

            if data.get("not_function"):
                d5_typeerror_not_function += 1
                print(f"  SKIP  D5-{iface}: not a function on window")
            elif data["throws"] and data["is_typeerror"]:
                d5_typeerror_pass += 1
            elif data["throws"] and not data["is_typeerror"]:
                d5_typeerror_fail += 1
                err_type = data.get("error_type", "?")
                err_msg = data.get("error_msg", "")
                print(f"  FAIL  D5-{iface}: threw {err_type}: {err_msg[:60]}")
                failures.append((f"D5-{iface}", "illegal constructor", f"threw {err_type}"))
            else:
                d5_typeerror_fail += 1
                print(f"  FAIL  D5-{iface}: construction succeeded (should throw TypeError)")
                failures.append((f"D5-{iface}", "illegal constructor", "constructed without error"))
        except Exception as e:
            d5_typeerror_fail += 1
            print(f"  FAIL  D5-{iface}: eval exception: {e}")
            failures.append((f"D5-{iface}", "illegal constructor", f"eval exception: {e}"))

    print()
    print(f"  D5 Illegal Constructor: {d5_typeerror_pass} PASS / {d5_typeerror_fail} FAIL / {d5_typeerror_not_function} SKIP(not-function) / {len(D5_ILLEGAL_CONSTRUCTOR_INTERFACES)} total")
    print()

    # ===================================================================
    # D5 TESTS: JS shim overridden (non-constructable in Chrome, but
    # intentionally constructable in IV8 via JS shim)
    # ===================================================================
    print("-" * 72)
    print(" D5: JS Shim Overridden (non-constructable in Chrome, constructable in IV8)")
    print("-" * 72)

    d5_shim_pass = 0
    d5_shim_fail = 0

    for iface in D5_JS_SHIM_OVERRIDDEN:
        js = build_d5_constructor_test_js(iface)
        try:
            raw = ctx.eval(js)
            data = json.loads(raw)

            if data.get("not_function"):
                print(f"  SKIP  D5-SHIM-{iface}: not a function on window")
            elif not data["throws"]:
                d5_shim_pass += 1
                print(f"  PASS  D5-SHIM-{iface}: constructable (JS shim override)")
            else:
                d5_shim_fail += 1
                print(f"  FAIL  D5-SHIM-{iface}: threw {data.get('error_type', '?')}")
                failures.append((f"D5-SHIM-{iface}", "JS shim override", "threw error"))
        except Exception as e:
            d5_shim_fail += 1
            print(f"  FAIL  D5-SHIM-{iface}: eval exception: {e}")
            failures.append((f"D5-SHIM-{iface}", "JS shim override", f"eval exception: {e}"))

    print()
    print(f"  D5 JS Shim Overridden: {d5_shim_pass} PASS / {d5_shim_fail} FAIL / {len(D5_JS_SHIM_OVERRIDDEN)} total")
    print()

    # ===================================================================
    # D5 TESTS: Constructable interfaces (negative controls)
    # ===================================================================
    print("-" * 72)
    print(" D5: Constructable Interfaces (negative controls)")
    print("-" * 72)

    d5_constructable_pass = 0
    d5_constructable_fail = 0
    d5_constructable_skip = 0

    for iface, args in D5_CONSTRUCTABLE_INTERFACES:
        js = build_d5_constructor_test_js(iface, args)
        try:
            raw = ctx.eval(js)
            data = json.loads(raw)

            if data.get("not_function"):
                d5_constructable_skip += 1
                print(f"  SKIP  D5-CTRL-{iface}: not a function on window")
            elif not data["throws"]:
                d5_constructable_pass += 1
            else:
                d5_constructable_fail += 1
                err_type = data.get("error_type", "?")
                print(f"  FAIL  D5-CTRL-{iface}: should be constructable but threw {err_type}")
                failures.append((f"D5-CTRL-{iface}", "should be constructable", f"threw {err_type}"))
        except Exception as e:
            d5_constructable_fail += 1
            print(f"  FAIL  D5-CTRL-{iface}: eval exception: {e}")
            failures.append((f"D5-CTRL-{iface}", "should be constructable", f"eval exception: {e}"))

    print()
    print(f"  D5 Constructable: {d5_constructable_pass} PASS / {d5_constructable_fail} FAIL / {d5_constructable_skip} SKIP / {len(D5_CONSTRUCTABLE_INTERFACES)} total")
    print()

    # ===================================================================
    # D5 TESTS: DOMException shape
    # ===================================================================
    print("-" * 72)
    print(" D5: DOMException Shape")
    print("-" * 72)

    dom_tests = build_d5_domexception_tests()
    d5_dom_pass = 0
    d5_dom_fail = 0

    for test_id, desc, js in dom_tests:
        try:
            result = ctx.eval(js)
            if result is True:
                d5_dom_pass += 1
                print(f"  PASS  {test_id}: {desc}")
            else:
                d5_dom_fail += 1
                print(f"  FAIL  {test_id}: {desc}")
                failures.append((test_id, desc, "returned false"))
        except Exception as e:
            d5_dom_fail += 1
            print(f"  FAIL  {test_id}: {desc} (exception: {e})")
            failures.append((test_id, desc, f"exception: {e}"))

    print()
    print(f"  D5 DOMException: {d5_dom_pass} PASS / {d5_dom_fail} FAIL / {len(dom_tests)} total")
    print()

    # ===================================================================
    # SUMMARY
    # ===================================================================
    total_pass = d1_pass + d5_typeerror_pass + d5_shim_pass + d5_constructable_pass + d5_dom_pass
    total_fail = d1_fail + d5_typeerror_fail + d5_shim_fail + d5_constructable_fail + d5_dom_fail
    total_skip = d5_typeerror_not_function + d5_constructable_skip
    total = total_pass + total_fail + total_skip

    print("=" * 72)
    print(" OVERALL SUMMARY")
    print("=" * 72)
    print(f"  D1 Return Value Semantics:     {d1_pass} PASS / {d1_fail} FAIL / {len(D1_TESTS)} total")
    print(f"  D5 Illegal Constructor:        {d5_typeerror_pass} PASS / {d5_typeerror_fail} FAIL / {d5_typeerror_not_function} SKIP / {len(D5_ILLEGAL_CONSTRUCTOR_INTERFACES)} total")
    print(f"  D5 JS Shim Overridden:         {d5_shim_pass} PASS / {d5_shim_fail} FAIL / {len(D5_JS_SHIM_OVERRIDDEN)} total")
    print(f"  D5 Constructable (neg ctrl):   {d5_constructable_pass} PASS / {d5_constructable_fail} FAIL / {d5_constructable_skip} SKIP / {len(D5_CONSTRUCTABLE_INTERFACES)} total")
    print(f"  D5 DOMException Shape:         {d5_dom_pass} PASS / {d5_dom_fail} FAIL / {len(dom_tests)} total")
    print()
    print(f"  TOTAL: {total_pass} PASS / {total_fail} FAIL / {total_skip} SKIP / {total} total")
    print()

    if failures:
        print("-" * 72)
        print(" FAILURE DETAILS")
        print("-" * 72)
        for test_id, desc, reason in failures:
            print(f"  {test_id}: {desc} -- {reason}")
        print()

    ctx.close()

    return 1 if total_fail > 0 else 0


if __name__ == "__main__":
    sys.exit(main())
