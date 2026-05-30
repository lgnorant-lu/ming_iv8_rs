# Changelog

All notable changes to iv8-rs are documented here.
This project adheres to [Semantic Versioning](https://semver.org/) and
[Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

## [0.2.0] - 2026-05-30

### Added

- `iv8_core::v8_extra` module providing `MarkAsUndetectable` and
  `SetCallAsFunctionHandler` bindings via cc crate, enabling real V8
  `[[IsHTMLDDA]]` semantics without requiring a forked v8 crate.
- 3 new integration tests for v8_extra (typeof/==/Boolean/if, callable,
  document.all combined pattern).
- `RustValue` enum gains four variants (`BigInt`, `DateTime`, `Map`, `Set`)
  produced when `strict_compat=False`. They map to Python `int` (any size),
  `datetime.datetime` (UTC), `dict` (insertion order), `set` respectively.
- `iv8_py::value_convert` helper module centralizes the new conversions and
  also handles round-trip back to V8 (`int -> BigInt`, `datetime -> Date`,
  `dict -> Map`, `set -> Set`).
- `RuntimeState::has` helper that returns `false` when no state is installed
  (used by conversion code that may run before/without a RuntimeState).

### Changed

- `__iv8__` tool object now has full `[[IsHTMLDDA]]` semantics:
  `typeof __iv8__ === 'undefined'`, `__iv8__ == null`, `Boolean(__iv8__) === false`.
  Property access (`__iv8__.page.load` etc.) remains unchanged.
- `document.all` now uses real `MarkAsUndetectable` (was JS-level workaround).
- `document` is now a real `EventTarget`: `addEventListener`, `removeEventListener`,
  and `dispatchEvent` are wired to the central `EventListenerRegistry` via the
  DOM tree's root `NodeId`. Listeners on `DOMContentLoaded`, `click`, etc. now
  fire correctly. Events from child nodes with `bubbles: true` reach `document`.
- `fetch()` requests are now recorded to `__iv8__.netLog.entries` alongside
  XHR. Same entry shape: `{ method, url, headers, body }`. Header names are
  lowercased; method is uppercased to match XHR semantics.
- When `strict_compat=False`, type conversion produces richer Python values:
  `BigInt -> int`, `Date -> datetime.datetime`, `Map -> dict`, `Set -> set`,
  `TypedArray -> list[int|float]` (11 typed array subtypes preserved).
  Previously these all degraded to strings, `None`, or raw bytes.
  `strict_compat=True` (default) is unchanged for v0.1 compatibility.
- `set_network_handler` is now documented as always-on regardless of
  `strict_compat`. The Python handler runs as the second tier of a three-layer
  fallback chain (ResourceBundle -> handler -> NetworkError) for both `fetch`
  and synchronous XHR. (No code change in v0.2 — this was already the case
  in v0.1; v0.2 just documents and tests the existing behavior explicitly.)
- Resolves L-01, L-03, L-04, L-09, L-10 known limitations from v0.1.

### Build

- iv8-core gains a `build.rs` that compiles `cxx/iv8_v8_extra.cc` via cc crate.
  Auto-locates V8 headers from cargo registry cache; override with
  `IV8_V8_CRATE_DIR` env var if needed.
- Requires C++20 compiler. On MSVC `/Zc:__cplusplus` is added so V8 headers
  detect the standard version correctly.

## [0.1.0] - 2026-05-30

### Added

- V8 147 kernel with eval pipeline, TryCatch, and strict_compat mode (default True)
- RuntimeState per-isolate container with slot-based storage
- IV8Error enum with 5 Python exception classes (IV8Error/EvalError/TypeError/TimeoutError/InternalError)
- safe_callback macro for catch_unwind in V8 callbacks
- 393 environment defaults injection via dot-path notation
- Type conversion matrix (D-3): JS primitives, objects, arrays, BigInt(->None), Date(->'[object Date]')
- GIL release for source >= 256 bytes
- Multiple JSContext coexistence with LIFO drop ordering

- MarkAsUndetectable JS shim for document.all (typeof -> "undefined" via shim)
- wrapNative: function.toString() -> "function name() { [native code] }"
- hookNative: dot-path function interception with Python callable
- window.chrome object (app/csi/loadTimes/runtime with connect/sendMessage error format)
- navigator.webdriver = false (strict_compat iv8 0.1.2 behavior)
- navigator/screen native getter (ObjectTemplate, getter.toString() -> [native code])
- __iv8__ DontEnum (Object.keys invisible)

- ego-tree DOM tree with html5ever HTML5 parsing
- selectors crate CSS Selector Level 4 engine
- FunctionTemplate prototype chain (31 element types)
- Node identity cache (same NodeId -> same V8 object)
- DOM query APIs: getElementById, querySelector, querySelectorAll, getElementsByTagName, getElementsByClassName
- DOM mutation APIs: appendChild, insertBefore, removeChild, replaceChild, cloneNode
- DOM attribute APIs: getAttribute, setAttribute, removeAttribute, classList, dataset
- DOM navigation: parentNode, childNodes, firstChild, lastChild, nextSibling, previousSibling, children
- innerHTML/outerHTML getter and setter with id index maintenance
- document.documentElement, document.body, document.head native getters

- EventLoop with microsecond precision (advance/sleep/tick/drain/getTime/reset)
- setTimeout, setInterval, clearTimeout, clearInterval, requestAnimationFrame, queueMicrotask
- DateInterceptor: Date.now() = EPOCH + eventLoop.getTime()
- EventTarget three-phase dispatch (capture -> target -> bubble)
- stopPropagation, stopImmediatePropagation, preventDefault, once option
- Event/CustomEvent/MouseEvent/KeyboardEvent/PointerEvent constructors

- page.load with HTML parsing, inline script execution, external script execution
- page.load snapshot API ({baseURL, html, resources})
- location object (href/origin/protocol/host/pathname/search/hash + assign/replace/reload)
- document.cookie read/write, document.referrer, document.hidden, document.visibilityState
- document.readyState lifecycle (loading -> interactive -> complete)

- SubtleCrypto: all 12 methods (digest/importKey/exportKey/generateKey/sign/verify/encrypt/decrypt/deriveBits/deriveKey/wrapKey/unwrapKey)
- Algorithms: SHA-1/256/384/512, HMAC, AES-GCM, AES-CBC, PBKDF2, HKDF, RSA-OAEP, RSA-PSS, ECDSA(P-256/P-384), ECDH(P-256/P-384)
- Key formats: raw, spki, pkcs8, jwk
- crypto.getRandomValues (BCryptGenRandom on Windows, getrandom elsewhere)
- crypto.randomUUID

- Canvas2D with tiny-skia real rendering (fillRect/strokeRect/clearRect/fillText/arc/path/transform)
- Canvas toDataURL with PNG encoding, deterministic noise (LCG seed), fixed fingerprint fallback
- Canvas save/restore state stack
- WebGL parameter table with environment-configured values and callLog

- fetch() with ResourceBundle -> Python network_handler -> NetworkError fallback
- XMLHttpRequest (sync + async modes) with network_handler fallback
- NetLog (XHR request recording via __iv8__.netLog.entries)
- eval_promise() for Promise/async function awaiting

- V8 Inspector with CDP WebSocket server (hand-rolled WebSocket with SHA1/base64)
- with_devtools(port, watch_apis) Python API
- vdebugger statement support
- Debugger class: trace_api/trace_apis/watch_property/eval_traced/snapshot/get_call_log/get_call_summary

- PyO3 binding: JSContext class with eval/eval_promise/expose/page.load/add_resource
- Python type stubs (_iv8.pyi + __init__.pyi)
- JSContext context manager (with statement)
- enable_logging() API (tracing subscriber, IV8_LOG env var)
- expose(callable, name) and expose(data, name) for Python interop

- atob/btoa, URL/URLSearchParams, TextEncoder/TextDecoder
- MessageChannel, localStorage/sessionStorage
- navigator.mimeTypes/plugins/permissions/mediaDevices
- history, AudioContext/OfflineAudioContext
- MutationObserver/IntersectionObserver/ResizeObserver (stubs)
- Blob, structuredClone, AbortController
- getComputedStyle, getBoundingClientRect (geometry from environment)
- console.log/warn/error with message capture + Python get_console_messages API

- GitHub Actions CI: lint + rust-test + python-test (Linux/macOS/Windows)
- cibuildwheel: 5 platforms x 2 wheels = 10 wheels
- criterion benchmark suite (context_lifecycle/eval/browser_api/dom/crypto/throughput)
- 198 diff-test fixtures across 19 categories
- 119 CreepJS/FingerprintJS anti-detection tests
- Memory stability tests (100-round long-run, <= 5MB drift)

### Known Limitations

See docs/PROGRESS.md section 7 (L-01..L-10) and docs/adr/001-mark-as-undetectable-deferred.md.

Key items:
- L-01: typeof __iv8__ === 'object' (MarkAsUndetectable not exposed in v8 crate)
- L-05: DOM wrapper without cppgc GC integration
- L-08: Windows context lifecycle ~9ms (Linux ~4.6ms)
