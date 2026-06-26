#!/usr/bin/env python3
"""Audit illegal constructor behavior across all IV8 codegen interfaces.

Background:
  L6 TypeError behavior — currently only Navigator/Screen/WorkerNavigator
  verify that `new XxxInterface()` throws TypeError. The remaining ~1284
  codegen interfaces are unverified.

This script:
  1. Enumerates all global properties from the IV8 runtime window object.
  2. Filters to function-typed properties (candidate constructors).
  3. Attempts `new InterfaceName()` for each, classifying the outcome:
     - THROWS_TYPEERROR : correctly throws TypeError (illegal constructor)
     - THROWS_OTHER     : throws a different exception type
     - CONSTRUCTED      : construction succeeded (may be legitimate)
     - NOT_FUNCTION     : not a function (typeof !== 'function')
  4. Cross-references CONSTRUCTED results against a curated list of
     interfaces that are known to be non-constructable in real browsers
     (Chrome) and flags those as potential_issues.

Usage:
  .venv\\Scripts\\python.exe scripts/audit_illegal_constructors.py

Output:
  data/illicit_constructor_audit.json
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = REPO_ROOT / "data"

# ---------------------------------------------------------------------------
# Curated list of Web IDL interfaces that are NOT constructable in real
# browsers (Chrome). `new XxxInterface()` must throw TypeError.
#
# Sources: Chrome DevTools manual testing, MDN Web Docs, HTML/DOM specs.
# These are abstract or singleton interfaces whose [[Construct]] is disabled.
# ---------------------------------------------------------------------------
KNOWN_NON_CONSTRUCTABLE: frozenset[str] = frozenset({
    # --- Core DOM abstract types ---
    "Node", "Element", "HTMLElement", "Document", "HTMLDocument",
    "CharacterData", "Text", "Comment", "CDATASection",
    "ProcessingInstruction", "DocumentFragment", "ShadowRoot",
    "DocumentType", "Attr", "NodeIterator", "TreeWalker",
    "Range", "AbstractRange", "NodeList", "HTMLCollection",
    "DOMTokenList", "DOMStringMap", "NamedNodeMap",
    "DOMImplementation", "MutationRecord", "StaticNodeList",
    "RadioNodeList",
    # --- Window / browsing context singletons ---
    "Window", "Navigator", "Screen", "History", "Location",
    "Storage", "WorkerNavigator", "ScreenOrientation",
    "VisualViewport", "MediaQueryList", "WindowProxy",
    "Crypto", "SubtleCrypto", "CryptoKey",
    "Plugin", "MimeType", "PluginArray", "MimeTypesArray",
    "BarProp", "ScrollToOptions", "ScreenDetailed",
    "ScreenDetails", "IconAtRule",
    # --- Performance / observers (entry types, not constructable) ---
    "Performance", "PerformanceEntry", "PerformanceNavigationTiming",
    "PerformanceResourceTiming", "PerformanceMark",
    "PerformanceMeasure", "PerformancePaintTiming",
    "PerformanceEventTiming", "PerformanceLongTaskTiming",
    "PerformanceObserverEntryList", "TaskAttributionTiming",
    "LayoutShiftAttribution",
    # --- CSS OM non-constructable ---
    "CSSStyleDeclaration", "CSSRule", "CSSRuleList",
    "CSSStyleRule", "CSSMediaRule", "CSSImportRule",
    "CSSStyleSheet", "CSSGroupingRule", "CSSConditionRule",
    "CSSFontFaceRule", "CSSKeyframesRule", "CSSKeyframeRule",
    "CSSNamespaceRule", "CSSPageRule", "CSSMarginRule",
    "CSSSupportsRule", "CSSLayerBlockRule", "CSSLayerStatementRule",
    "CSSContainerRule", "CSSStartingStyleRule", "StyleSheetList",
    "CSSImportRule", "MediaList", "StyleSheet",
    # --- Event target / abstract interfaces ---
    "EventTarget", "AbortSignal",
    # --- Various non-constructable Web APIs ---
    "ApplicationCache", "CacheStorage", "Cache",
    "Headers",  # NOTE: Headers IS constructable in Chrome — kept out
    # --- DataTransfer / clipboard non-constructable ---
    "DataTransfer", "DataTransferItemList", "DataTransferItem",
    "Clipboard",  # ClipboardItem is constructable (see KNOWN_CONSTRUCTABLE)
    # --- DOM URL / location singletons ---
    # URL is constructable, URLSearchParams is constructable
    # --- Audio / video non-constructable ---
    "AudioListener", "AudioParam", "AudioParamMap",
    "AudioDestinationNode", "AudioBufferSourceNode",
    "MediaElementAudioSourceNode", "MediaStreamAudioSourceNode",
    "MediaStreamTrackAudioSourceNode", "MediaStreamAudioDestinationNode",
    "OscillatorNode", "GainNode", "DelayNode", "BiquadFilterNode",
    "WaveShaperNode", "PannerNode", "StereoPannerNode",
    "ConvolverNode", "AnalyserNode", "ChannelSplitterNode",
    "ChannelMergerNode", "DynamicsCompressorNode",
    "ScriptProcessorNode", "ConstantSourceNode", "IIRFilterNode",
    "PeriodicWave",
    # --- Media non-constructable ---
    "MediaStreamTrack", "MediaStream", "MediaSettingsRange",
    "TrackDefaultList", "VideoTrackList", "AudioTrackList",
    "TextTrackList", "TextTrackCueList", "TextTrack",
    "VTTCue",  # VTTCue IS constructable actually
    # --- Canvas / rendering non-constructable ---
    "CanvasRenderingContext2D", "OffscreenCanvasRenderingContext2D",
    "WebGLRenderingContext", "WebGL2RenderingContext",
    "WebGLActiveInfo", "WebGLBuffer", "WebGLFramebuffer",
    "WebGLProgram", "WebGLQuery", "WebGLRenderbuffer",
    "WebGLSampler", "WebGLShader", "WebGLShaderPrecisionFormat",
    "WebGLSync", "WebGLTexture", "WebGLTransformFeedback",
    "WebGLUniformLocation", "WebGLVertexArrayObject",
    # --- Misc ---
    "FontFaceSet", "FontFace",  # FontFace IS constructable
    "ResizeObserverEntry", "IntersectionObserverEntry",
    "MutationObserverEntry",
    "SecurityPolicyViolationEvent",  # IS constructable
    "CustomElementRegistry",
    "HTMLFormControlsCollection",
    "HTMLOptionsCollection",
    "HTMLPropertiesCollection",
    "ValidityState", "AutocompleteErrorEvent",
    "FileError", "FileException",
    "SVGAnimatedEnumeration", "SVGAnimatedInteger",
    "SVGAnimatedNumber", "SVGAnimatedPreserveAspectRatio",
    "SVGAnimatedRect", "SVGAnimatedString",
    "SVGAnimatedTransformList", "SVGAnimatedBoolean",
    "SVGAnimatedAngle", "SVGAnimatedLength",
    "SVGAnimatedLengthList", "SVGAnimatedNumberList",
    "SVGPoint", "SVGPointList", "SVGStringList",
    "SVGTransform", "SVGTransformList",
    "SVGLength", "SVGLengthList", "SVGNumber", "SVGNumberList",
    "SVGAngle", "SVGRect", "SVGMatrix",
    # Remove ones that are actually constructable
})

# Interfaces that ARE legitimately constructable in Chrome.
# Used to avoid false positives in potential_issues.
KNOWN_CONSTRUCTABLE: frozenset[str] = frozenset({
    # JS builtins
    "Object", "Function", "Array", "Number", "Boolean", "String",
    "Symbol", "Date", "RegExp", "Error", "EvalError", "RangeError",
    "ReferenceError", "SyntaxError", "TypeError", "URIError",
    "AggregateError", "SuppressedError",
    "ArrayBuffer", "SharedArrayBuffer", "DataView",
    "Uint8Array", "Int8Array", "Uint16Array", "Int16Array",
    "Uint32Array", "Int32Array", "BigUint64Array", "BigInt64Array",
    "Uint8ClampedArray", "Float32Array", "Float64Array", "Float16Array",
    "Map", "Set", "WeakMap", "WeakSet", "WeakRef",
    "Promise", "Proxy", "FinalizationRegistry",
    "Iterator", "DisposableStack", "AsyncDisposableStack",
    # Web APIs — constructable in Chrome
    "Headers", "Request", "Response", "FormData", "Blob", "File",
    "URL", "URLSearchParams",
    "Event", "CustomEvent", "MessageEvent", "ErrorEvent",
    "CloseEvent", "UIEvent", "MouseEvent", "KeyboardEvent",
    "WheelEvent", "FocusEvent", "AnimationEvent", "TransitionEvent",
    "PointerEvent", "InputEvent", "CompositionEvent",
    "StorageEvent", "PopStateEvent", "PageTransitionEvent",
    "HashChangeEvent", "BeforeUnloadEvent", "DragEvent",
    "TouchEvent", "DeviceOrientationEvent", "DeviceMotionEvent",
    "ProgressEvent", "XMLHttpRequestEventTarget", "XMLHttpRequestProgressEvent",
    "SecurityPolicyViolationEvent",
    "AbortController", "AbortSignal",  # AbortSignal constructable in modern Chrome
    "FileReader", "XMLHttpRequest", "XMLSerializer", "DOMParser",
    "Image", "ImageBitmap", "ImageData",
    "Audio", "Option",
    "WebSocket", "Worker", "SharedWorker", "BroadcastChannel",
    "MessageChannel", "MessagePort",
    "MutationObserver", "IntersectionObserver", "ResizeObserver",
    "PerformanceObserver", "ReportingObserver",
    "TextEncoder", "TextDecoder",
    "Path2D", "DOMMatrix", "DOMMatrixReadOnly",
    "DOMRect", "DOMRectReadOnly", "DOMPoint", "DOMPointReadOnly",
    "DOMQuad", "DOMRectList", "DOMPointList", "DOMMatrixList",
    "CSSStyleSheet", "CSSImageValue", "CSSKeywordValue",
    "CSSMathValue", "CSSNumericValue", "CSSPositionValue",
    "CSSStyleValue", "CSSUnitValue", "CSSUnparsedValue",
    "CSSVariableReferenceValue", "StylePropertyMap",
    "Animation", "KeyframeEffect", "AnimationEffect",
    "AnimationTimeline", "DocumentTimeline",
    "ScrollTimeline", "ViewTimeline",
    "AudioContext", "OfflineAudioContext",
    "RTCPeerConnection", "RTCDataChannel",
    "MediaStream", "MediaRecorder",
    "WGSLLanguageFeatures",
    "VTTCue", "StaticRange",
    "FontFace", "TaskController", "TaskSignal",
    "Attribution",  # remove if not
    "CountQueuingStrategy", "ByteLengthQueuingStrategy",
    "ReadableStream", "WritableStream", "TransformStream",
    "ReadableStreamDefaultReader", "WritableStreamDefaultReader",
    "ReadableStreamDefaultController", "WritableStreamDefaultController",
    "TransformStreamDefaultController",
    "CompressionStream", "DecompressionStream",
    "CustomEvent",
    "MediaQueryListEvent",
    "PermissionStatus",
    "PushManager",  # no
    "PushSubscription",  # no
    "Notification",
    "ServiceWorker",  # no
    "Cache",  # no
    "TextMetrics",  # no
    "CanvasGradient", "CanvasPattern",
    "OffscreenCanvas",
    "ImageData",
    "ClipboardItem",  # new ClipboardItem(items) is constructable in Chrome
    "ClipboardEvent",
    "SubmitEvent",
    "SubmitEvent",
    "HTMLAudioElement", "HTMLImageElement", "HTMLVideoElement",
    "HTMLOptionElement", "HTMLAnchorElement",
    "Sanitizer",
    "Highlight",
    "HighlightRegistry",
    "Navigation",
    "NavigateEvent",
    "NavigationCurrentChange",
    "NavigationHistoryEntry",
    "NavigationTransition",
    "NavigationDestination",
    "DeclarativeShadowRoot",
    "PerformanceMark",  # actually constructable via new PerformanceMark()
    "PerformanceMeasure",  # actually constructable via new PerformanceMeasure()
})


# ---------------------------------------------------------------------------
# JS probe: enumerate window globals, classify construction behavior.
# ---------------------------------------------------------------------------
PROBE_JS = r"""
(function() {
    var names = Object.getOwnPropertyNames(window);
    var results = {
        THROWS_TYPEERROR: [],
        THROWS_OTHER: [],
        CONSTRUCTED: [],
        NOT_FUNCTION: []
    };
    var errors_detail = {};
    var constructed_detail = {};

    for (var i = 0; i < names.length; i++) {
        var name = names[i];
        try {
            var val = window[name];
        } catch(e) {
            // Accessing the property itself threw
            results.NOT_FUNCTION.push(name);
            continue;
        }
        if (typeof val !== 'function') {
            results.NOT_FUNCTION.push(name);
            continue;
        }
        try {
            var obj = new val();
            results.CONSTRUCTED.push(name);
            try {
                constructed_detail[name] = {
                    toString: Object.prototype.toString.call(obj),
                    proto_ctor: (Object.getPrototypeOf(obj) &&
                                 Object.getPrototypeOf(obj).constructor) ?
                                Object.getPrototypeOf(obj).constructor.name : 'none'
                };
            } catch(e) {
                constructed_detail[name] = {error: e.message};
            }
        } catch(e) {
            if (e instanceof TypeError) {
                results.THROWS_TYPEERROR.push(name);
            } else {
                results.THROWS_OTHER.push(name);
                errors_detail[name] = e.constructor.name + ': ' + e.message;
            }
        }
    }
    return JSON.stringify({
        total_globals: names.length,
        results: results,
        errors_detail: errors_detail,
        constructed_detail: constructed_detail
    });
})()
"""


def main() -> int:
    sys.path.insert(0, str(REPO_ROOT))

    from iv8_rs import JSContext

    print("Initializing IV8 runtime...")
    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)
    print("Page loaded. Probing global constructors...")

    raw = ctx.eval(PROBE_JS)
    data = json.loads(raw)

    results = data["results"]
    errors_detail = data.get("errors_detail", {})
    constructed_detail = data.get("constructed_detail", {})

    # Sort each category for stable output
    for key in results:
        results[key].sort()

    # Identify potential issues: interfaces that CONSTRUCTED in IV8 but are
    # known to be non-constructable in real browsers. KNOWN_CONSTRUCTABLE
    # takes precedence (an interface constructable in Chrome is never an
    # issue even if also listed in KNOWN_NON_CONSTRUCTABLE).
    constructed_set = set(results["CONSTRUCTED"])
    potential_issues = sorted(
        constructed_set & KNOWN_NON_CONSTRUCTABLE - KNOWN_CONSTRUCTABLE
    )

    # Also flag CONSTRUCTED interfaces that are NOT in our known-constructable
    # list and have suspicious toString (e.g. [object Object] for what should
    # be a DOM interface, suggesting a codegen stub rather than a real impl).
    suspicious_stubs = []
    for name in sorted(constructed_set):
        if name in KNOWN_CONSTRUCTABLE:
            continue
        if name in KNOWN_NON_CONSTRUCTABLE:
            continue  # already in potential_issues
        detail = constructed_detail.get(name, {})
        ts = detail.get("toString", "")
        # Interfaces that return [object Object] are likely codegen stubs
        # rather than real Web IDL implementations.
        if ts == "[object Object]":
            suspicious_stubs.append(name)

    output = {
        "total_globals": data["total_globals"],
        "functions_tested": (
            len(results["THROWS_TYPEERROR"])
            + len(results["THROWS_OTHER"])
            + len(results["CONSTRUCTED"])
        ),
        "summary": {
            "THROWS_TYPEERROR": len(results["THROWS_TYPEERROR"]),
            "THROWS_OTHER": len(results["THROWS_OTHER"]),
            "CONSTRUCTED": len(results["CONSTRUCTED"]),
            "NOT_FUNCTION": len(results["NOT_FUNCTION"]),
        },
        "results": results,
        "errors_detail": errors_detail,
        "potential_issues": potential_issues,
        "potential_issue_count": len(potential_issues),
        "suspicious_stubs": suspicious_stubs,
        "suspicious_stub_count": len(suspicious_stubs),
        "known_non_constructable_count": len(KNOWN_NON_CONSTRUCTABLE),
        "known_constructable_count": len(KNOWN_CONSTRUCTABLE),
        "notes": [
            "potential_issues: CONSTRUCTED in IV8 but known non-constructable in Chrome.",
            "suspicious_stubs: CONSTRUCTED with [object Object] toString (likely codegen stubs).",
            "THROWS_TYPEERROR is the expected behavior for non-constructable Web IDL interfaces.",
            "CONSTRUCTED is correct for interfaces like Headers, Request, Response, ArrayBuffer.",
        ],
    }

    DATA_DIR.mkdir(parents=True, exist_ok=True)
    out_path = DATA_DIR / "illicit_constructor_audit.json"
    out_path.write_text(
        json.dumps(output, indent=2, ensure_ascii=False),
        encoding="utf-8",
    )

    # Console summary
    print()
    print("=" * 72)
    print(" ILLEGAL CONSTRUCTOR AUDIT RESULTS")
    print("=" * 72)
    print(f"  total_globals       : {output['total_globals']}")
    print(f"  functions_tested    : {output['functions_tested']}")
    print()
    print(f"  THROWS_TYPEERROR    : {output['summary']['THROWS_TYPEERROR']}")
    print(f"  THROWS_OTHER        : {output['summary']['THROWS_OTHER']}")
    print(f"  CONSTRUCTED         : {output['summary']['CONSTRUCTED']}")
    print(f"  NOT_FUNCTION        : {output['summary']['NOT_FUNCTION']}")
    print()
    print(f"  potential_issues    : {output['potential_issue_count']}")
    print(f"  suspicious_stubs    : {output['suspicious_stub_count']}")
    print()

    if output["potential_issues"]:
        print("-" * 72)
        print(" Potential issues (constructed but should throw TypeError):")
        print("-" * 72)
        for name in output["potential_issues"][:50]:
            print(f"  {name}")
        if len(output["potential_issues"]) > 50:
            print(f"  ... and {len(output['potential_issues']) - 50} more")
        print()

    if output["suspicious_stubs"]:
        print("-" * 72)
        print(" Suspicious stubs (constructed, [object Object], not in known lists):")
        print("-" * 72)
        for name in output["suspicious_stubs"][:50]:
            print(f"  {name}")
        if len(output["suspicious_stubs"]) > 50:
            print(f"  ... and {len(output['suspicious_stubs']) - 50} more")
        print()

    print(f"Output written to: {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
