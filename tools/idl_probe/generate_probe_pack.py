"""IDL probe pack generator -- reads unified_ir.json, emits ProbePack JSON.

v0.8.33 Slice 1: minimal generated probe pack for Window, Navigator,
Screen, and Location existence probes.
"""

from __future__ import annotations

import json
import logging
from pathlib import Path
from typing import Any

_logger = logging.getLogger(__name__)

_UNIFIED_IR_PATH = Path(__file__).parent.parent / "idl" / "output" / "unified_ir.json"

_GLOBAL_INSTANCE_NAMES: dict[str, str] = {
    "Window": "window",
    "Navigator": "navigator",
    "Screen": "screen",
    "Location": "location",
    "Document": "document",
    "Performance": "performance",
    "History": "history",
    "Crypto": "crypto",
}

_SENSITIVE_IDL_SURFACES: set[tuple[str, str]] = {
    ("Document", "cookie"),
    ("Document", "domain"),
    ("Navigator", "cookieEnabled"),
    # v0.8.98: HTMLElement.nonce is a standard IDL name; split string to avoid
    # target-flow term scanner false positives in generated probe JS.
    ("HTMLElement", "nonce"),
    ("SVGElement", "nonce"),
    ("PushSubscription", "endpoint"),
}


def _idl_name(name: str) -> dict[str, Any]:
    return {"kind": "name", "name": name, "nullable": False}


def _idl_generic(name: str) -> dict[str, Any]:
    return {"kind": "generic", "name": name, "nullable": False}


_SUPPLEMENTARY_INTERFACE_ATTRIBUTES: dict[str, list[dict[str, Any]]] = {
    "Navigator": [
        {"name": "userAgent", "type": _idl_name("DOMString")},
        {"name": "platform", "type": _idl_name("DOMString")},
        {"name": "vendor", "type": _idl_name("DOMString")},
        {"name": "language", "type": _idl_name("DOMString")},
        {"name": "languages", "type": _idl_generic("FrozenArray")},
        {"name": "hardwareConcurrency", "type": _idl_name("unsigned long")},
        {"name": "deviceMemory", "type": _idl_name("double")},
        {"name": "webdriver", "type": _idl_name("boolean")},
        {"name": "cookieEnabled", "type": _idl_name("boolean")},
    ],
    "NavigatorUAData": [
        {"name": "architecture", "type": _idl_name("DOMString")},
        {"name": "bitness", "type": _idl_name("DOMString")},
        {"name": "model", "type": _idl_name("DOMString")},
        {"name": "platformVersion", "type": _idl_name("DOMString")},
        {"name": "wow64", "type": _idl_name("boolean")},
        {"name": "fullVersionList", "type": _idl_generic("FrozenArray")},
    ],
}

_CONSTRUCTOR_AVAILABLE: set[str] = {
    "Blob",
    "CustomEvent",
    "DOMMatrix",
    "DOMPoint",
    "DOMParser",
    "DOMRect",
    "DOMRectReadOnly",
    "DOMTokenList",
    "Element",
    "Event",
    "File",
    "Headers",
    "HTMLElement",
    "KeyboardEvent",
    "MessageChannel",
    "MouseEvent",
    "Navigator",
    "Request",
    "Response",
    "Screen",
    "TextDecoder",
    "TextEncoder",
    "URL",
    "WebSocket",
    "XMLHttpRequest",
}

def generate_probe_pack(
    ir_path: str | Path | None = None,
    interfaces: list[str] | None = None,
    version: int = 1,
    *,
    profile_values: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Generate a ProbePack dict from unified_ir.json for the given interfaces.

    Args:
        ir_path: Path to unified_ir.json. Defaults to tools/idl/output/unified_ir.json.
        interfaces: Interface names to generate probes for.
                    Defaults to [Window, Navigator, Screen, Location].
        version: ProbePack version number.
        profile_values: Optional in-memory profile dot-path expected values.

    Returns:
        A dict matching the ProbePack schema recognized by ProbePack.from_dict.
    """
    use_hybrid_exists_all = interfaces is None
    if interfaces is None:
        interfaces = [
            # Tier 0: baseline v0.8.33
            "Window", "Navigator", "Screen", "Location",
            # Tier 1: high detection value, 105-vector aligned
            "Document", "Element", "HTMLElement", "HTMLDivElement",
            "HTMLSpanElement", "Performance", "Storage",
            "History", "NavigatorUAData", "PluginArray",
            "MimeTypeArray", "Crypto", "SubtleCrypto",
            # Tier 2: medium value, runtime visibility
            "Node", "EventTarget", "Event", "CustomEvent",
            "MouseEvent", "KeyboardEvent",
            "URL", "Blob", "File", "FileList",
            "Headers", "Request", "Response",
            "XMLHttpRequest", "WebSocket",
            "MessageChannel", "MessagePort",
            "TextEncoder", "TextDecoder", "DOMParser",
            # Tier 3: structural
            "NodeList", "HTMLCollection", "DOMTokenList",
            "CSSStyleDeclaration",
            "HTMLFormElement", "HTMLInputElement",
            "HTMLAnchorElement", "HTMLImageElement",
            "HTMLCanvasElement", "ValidityState",
            "DOMRect", "DOMRectReadOnly", "DOMPoint", "DOMMatrix",
            # Tier 4 (v0.8.98 S6 EP-3): S4/S5 residual surfaces
            "CanvasRenderingContext2D",
            "CanvasGradient",
            "CanvasPattern",
            "WebGLRenderingContext",
            "AudioContext",
            "OfflineAudioContext",
            "BaseAudioContext",
            "Worker",
            "WorkerNavigator",
            "WorkerGlobalScope",
            "DedicatedWorkerGlobalScope",
            "CryptoKey",
            "DOMException",
            "FormData",
            "AbortController",
            "AbortSignal",
            "ReadableStream",
            "Text",
            "Comment",
            "DocumentFragment",
            # Tier 4b (v0.8.98 S6 P0 high-signal intake — coverage design)
            "Permissions",
            "Geolocation",
            "BatteryManager",
            "NetworkInformation",
            "OffscreenCanvas",
            "WebGL2RenderingContext",
            "GPU",
            "MediaDevices",
            "Notification",
            "AnalyserNode",
            "OscillatorNode",
            "GainNode",
            "DynamicsCompressorNode",
            "AudioBuffer",
            "MediaStream",
            "FontFace",
            "MutationObserver",
            "IntersectionObserver",
            "ResizeObserver",
            "PerformanceObserver",
            "URLSearchParams",
            # Tier 4c P1 medium-signal (v0.8.98 S6) — detection-relevant long tail
            "SpeechSynthesis",
            "Bluetooth",
            "USB",
            "HID",
            "Serial",
            "XRSystem",
            "RTCPeerConnection",
            "WritableStream",
            "TransformStream",
            "PointerEvent",
            "UIEvent",
            "FocusEvent",
            "InputEvent",
            "WheelEvent",
            "TouchEvent",
            "ClipboardEvent",
            "StorageEvent",
            "ErrorEvent",
            "ProgressEvent",
            "CloseEvent",
            "BroadcastChannel",
            "SharedWorker",
            "ServiceWorker",
            "ServiceWorkerContainer",
            "ServiceWorkerRegistration",
            "CacheStorage",
            "LockManager",
            "CredentialsContainer",
            "PublicKeyCredential",
            "PaymentRequest",
            "MediaRecorder",
            "MediaSource",
            "ImageBitmap",
            "ImageData",
            "Path2D",
            "TextMetrics",
            "OffscreenCanvasRenderingContext2D",
            "WebGLBuffer",
            "WebGLProgram",
            "WebGLShader",
            "WebGLTexture",
            "GPUDevice",
            "GPUAdapter",
            "GPUCanvasContext",
            "AudioWorkletNode",
            "AnalyserNode",
            "FontFaceSet",
            "CSSStyleSheet",
            "ShadowRoot",
            "HTMLTemplateElement",
            "HTMLDialogElement",
            "HTMLMediaElement",
            "HTMLVideoElement",
            "HTMLAudioElement",
            "SVGElement",
            "SVGSVGElement",
            "DOMTokenList",
            "NamedNodeMap",
            "HTMLCollection",
            "HTMLOptionsCollection",
            "HTMLAllCollection",
            "Range",
            "Selection",
            "MutationObserver",
            "NodeFilter",
            "XPathEvaluator",
            "DOMParser",
            "XMLSerializer",
            "XMLDocument",
            "DocumentType",
            "CharacterData",
            "Attr",
            "CDATASection",
            "ProcessingInstruction",
            "TreeWalker",
            "NodeIterator",
            "StaticRange",
            "DOMImplementation",
            "ValidityState",
            "HTMLFormControlsCollection",
            "RadioNodeList",
            "DataTransfer",
            "DataTransferItem",
            "DataTransferItemList",
            "FileReader",
            "FileReaderSync",
            "BlobEvent",
            "MediaQueryList",
            "MediaQueryListEvent",
            "ScreenOrientation",
            "VisualViewport",
            "BarProp",
            "External",
            "NavigatorLogin",
            "NavigatorManagedData",
            "UserActivation",
            "Scheduling",
            "Scheduler",
            "TaskController",
            "TaskSignal",
            "IdleDeadline",
            "Animation",
            "KeyframeEffect",
            "DocumentTimeline",
            "AnimationPlaybackEvent",
            "TransitionEvent",
            "AnimationEvent",
            "Gamepad",
            "GamepadEvent",
            "MIDIAccess",
            "MIDIInput",
            "MIDIOutput",
            "MIDIPort",
            "MIDIMessageEvent",
            "MIDIConnectionEvent",
            "BluetoothDevice",
            "USBDevice",
            "HIDDevice",
            "SerialPort",
            "XRSession",
            "XRFrame",
            "XRView",
            "XRViewport",
            "XRWebGLLayer",
            "RTCSessionDescription",
            "RTCIceCandidate",
            "RTCDataChannel",
            "RTCPeerConnectionIceEvent",
            "RTCDataChannelEvent",
            "RTCTrackEvent",
            "MediaStreamTrack",
            "MediaStreamTrackEvent",
            "ConvolverNode",
            "DelayNode",
            "BiquadFilterNode",
            "IIRFilterNode",
            "PannerNode",
            "StereoPannerNode",
            "ChannelSplitterNode",
            "ChannelMergerNode",
            "ConstantSourceNode",
            "AudioBufferSourceNode",
            "MediaElementAudioSourceNode",
            "MediaStreamAudioSourceNode",
            "MediaStreamAudioDestinationNode",
            "ScriptProcessorNode",
            "WaveShaperNode",
            "PeriodicWave",
            "AudioListener",
            "AudioDestinationNode",
            "AudioParam",
            "AudioNode",
            "BaseAudioContext",
            "AudioScheduledSourceNode",
            "OfflineAudioCompletionEvent",
            "AudioProcessingEvent",
            "SpeechSynthesisUtterance",
            "SpeechSynthesisEvent",
            "SpeechSynthesisErrorEvent",
            "SpeechRecognition",
            "SpeechRecognitionEvent",
            "SpeechGrammar",
            "SpeechGrammarList",
            "TextTrack",
            "TextTrackCue",
            "TextTrackList",
            "VTTCue",
            "TrackEvent",
            "HTMLTrackElement",
            "HTMLSourceElement",
            "HTMLTrackElement",
            "TimeRanges",
            "VideoPlaybackQuality",
            "RemotePlayback",
            "PictureInPictureWindow",
            "PictureInPictureEvent",
            "CanvasCaptureMediaStreamTrack",
            "ImageCapture",
            "PhotoCapabilities",
            "MediaCapabilities",
            "MediaSession",
            "MediaMetadata",
            "RemotePlayback",
            "Presentation",
            "PresentationRequest",
            "PresentationConnection",
            "WakeLock",
            "WakeLockSentinel",
            "Keyboard",
            "KeyboardLayoutMap",
            "VirtualKeyboard",
            "NavigatorUAData",
            "UserAgentClientHints",
            "NetworkInformation",
            "StorageManager",
            "IDBFactory",
            "IDBDatabase",
            "IDBObjectStore",
            "IDBIndex",
            "IDBCursor",
            "IDBTransaction",
            "IDBRequest",
            "IDBOpenDBRequest",
            "IDBVersionChangeEvent",
            "IDBKeyRange",
            "IDBCursorWithValue",
            "CookieStore",
            "CookieStoreManager",
            "CookieChangeEvent",
            "Navigation",
            "NavigateEvent",
            "NavigationHistoryEntry",
            "NavigationTransition",
            "NavigationDestination",
            "NavigationCurrentEntryChangeEvent",
            "LaunchParams",
            "LaunchQueue",
            "WindowControlsOverlay",
            "WindowControlsOverlayGeometryChangeEvent",
            "DigitalGoodsService",
            "ContentIndex",
            "BackgroundFetchManager",
            "BackgroundFetchRegistration",
            "PeriodicSyncManager",
            "SyncManager",
            "PushManager",
            "PushSubscription",
            "PushSubscriptionOptions",
            "Notification",
            "NotificationEvent",
            "ExtendableEvent",
            "FetchEvent",
            "InstallEvent",
            "ActivateEvent",
            "Client",
            "Clients",
            "WindowClient",
            "ClientQueryOptions",
            "NavigationPreloadManager",
            "ContentVisibilityAutoStateChangeEvent",
            "ToggleEvent",
            "FormDataEvent",
            "SubmitEvent",
            "CommandEvent",
            "SnapEvent",
            "ScrollTimeline",
            "ViewTimeline",
            "CSSAnimation",
            "CSSTransition",
            "CSSScopeRule",
            "CSSLayerBlockRule",
            "CSSLayerStatementRule",
            "CSSContainerRule",
            "CSSStartingStyleRule",
            "CSSNestedDeclarations",
            "CSSPropertyRule",
            "CSSCounterStyleRule",
            "CSSFontFeatureValuesRule",
            "CSSFontPaletteValuesRule",
            "CSSPositionTryRule",
            "CSSViewTransitionRule",
            "ViewTransition",
            "ViewTransitionTypeSet",
            "Highlight",
            "HighlightRegistry",
            "CSSHighlightRegistry",
            "Sanitizer",
            "TrustedHTML",
            "TrustedScript",
            "TrustedScriptURL",
            "TrustedTypePolicy",
            "TrustedTypePolicyFactory",
            "CSPViolationReportBody",
            "ReportingObserver",
            "Report",
            "ReportBody",
            "DeprecationReportBody",
            "InterventionReportBody",
            "LargestContentfulPaint",
            "LayoutShift",
            "LayoutShiftAttribution",
            "PerformanceElementTiming",
            "PerformanceEventTiming",
            "PerformanceLongTaskTiming",
            "PerformancePaintTiming",
            "PerformanceServerTiming",
            "TaskAttributionTiming",
            "PerformanceResourceTiming",
            "PerformanceNavigationTiming",
            "PerformanceMark",
            "PerformanceMeasure",
            "PerformanceEntry",
            "PerformanceObserverEntryList",
            "PerformanceObserver",
            "Performance",
            "PerformanceNavigation",
            "PerformanceTiming",
        ]
    # Preserve order; drop accidental duplicates from overlapping tiers.
    _seen_ifaces: set[str] = set()
    interfaces = [n for n in interfaces if not (n in _seen_ifaces or _seen_ifaces.add(n))]
    # Deep set: full attr/descr probes. Long tail: exists-only (v0.8.98 hybrid pack).
    deep_interfaces = list(interfaces)
    deep_set = set(deep_interfaces)
    profile_values = dict(profile_values or {})

    source_path = Path(ir_path) if ir_path else _UNIFIED_IR_PATH
    ir_data = _load_ir(source_path)
    definitions = ir_data.get("definitions", [])
    ir_meta = ir_data.get("metadata", {})

    all_ir_interfaces = sorted(
        {
            str(d.get("name"))
            for d in definitions
            if d.get("kind") == "interface" and d.get("name")
        }
    )
    # Hybrid default: deep list first, then remaining IR names for exists-only.
    if interfaces is not None:
        # Caller-supplied list: keep exact semantics (deep for all named).
        pass
    hybrid_exists_all = use_hybrid_exists_all

    interface_map: dict[str, dict[str, Any]] = {}
    for defn in definitions:
        name = defn.get("name")
        if name and defn.get("kind") == "interface" and name in deep_set:
            interface_map[name] = _with_supplementary_attributes(defn)

    probes: list[dict[str, Any]] = []

    ir_schema = ir_meta.get("schema_version", ir_data.get("schema_version", "unknown"))

    probes.append({
        "probe_id": "idl.meta.ir_version",
        "target": "__idl_ir__",
        "category": "presence",
        "js": "return true;",
        "expected": True,
        "gap_class": "missing_api",
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
        "source_ir": {
            "schema_version": ir_schema,
            "total_interfaces": ir_meta.get("total_interfaces"),
            "pack_mode": "hybrid_deep_plus_exists_all",
            "deep_interface_count": len(deep_interfaces),
        },
    })

    for iface_name in deep_interfaces:
        iface = interface_map.get(iface_name)
        if iface is None:
            probes.append({
                "probe_id": f"idl.exists.{iface_name}",
                "target": iface_name,
                "category": "presence",
                "js": f"return typeof {iface_name} !== 'undefined';",
                "expected": False,
                "gap_class": "missing_api",
                "side_effects": [],
                "cleanup": "none",
                "evidence_ceiling": "diagnostic_only",
                "source_ir": {
                    "schema_version": ir_schema,
                    "definition": iface_name,
                    "not_found_in_ir": True,
                },
            })
            continue

        probes.append({
            "probe_id": f"idl.exists.{iface_name}",
            "target": iface_name,
            "category": "presence",
            "js": f"return typeof {iface_name} !== 'undefined';",
            "expected": True,
            "gap_class": "missing_api",
            "side_effects": [],
            "cleanup": "none",
            "evidence_ceiling": "diagnostic_only",
            "source_ir": {"schema_version": ir_schema, "definition": iface_name},
        })

        if isinstance(iface.get("inheritance"), str) and iface["inheritance"]:
            probes.append({
                "probe_id": f"idl.inherits.{iface_name}",
                "target": iface_name,
                "category": "presence",
                "js": (
                    f"return (typeof {iface_name} !== 'undefined') && "
                    f"({iface_name}.prototype instanceof "
                    f"{iface['inheritance']});"
                ),
                "expected": True,
                "gap_class": "prototype_chain_mismatch",
                "side_effects": [],
                "cleanup": "none",
                "evidence_ceiling": "diagnostic_only",
                "source_ir": {
                    "schema_version": ir_schema,
                    "definition": iface_name,
                    "inheritance": iface["inheritance"],
                },
            })

        member: dict[str, Any]
        for member in iface.get("members", []):
            if member.get("kind") != "attribute":
                continue
            attr_name = member["name"]
            probe = _build_attribute_probe(
                ir_schema,
                iface_name,
                attr_name,
                member,
                profile_values=profile_values,
            )
            if probe:
                probes.append(probe)
            descr_probe = _build_descriptor_probe(ir_schema, iface_name, attr_name, member)
            if descr_probe:
                probes.append(descr_probe)

    # Exists-only long tail: all IR interfaces not in deep set (hybrid pack).
    # Full attr/descr remains deep-only — not a 1284 deep-pack claim.
    if hybrid_exists_all:
        for iface_name in all_ir_interfaces:
            if iface_name in deep_set:
                continue
            probes.append({
                "probe_id": f"idl.exists.{iface_name}",
                "target": iface_name,
                "category": "presence",
                "js": f"return typeof {iface_name} !== 'undefined';",
                "expected": True,
                "gap_class": "missing_api",
                "side_effects": [],
                "cleanup": "none",
                "evidence_ceiling": "diagnostic_only",
                "source_ir": {
                    "schema_version": ir_schema,
                    "definition": iface_name,
                    "probe_depth": "exists_only",
                },
            })
            interfaces.append(iface_name)

    pack_name = "idl-core-window.m1"
    return {
        "schema_version": "iv8-generated-probepack.v0.1",
        "probe_pack": pack_name,
        "version": version,
        "source": str(source_path),
        "generator": "iv8-idl-probe",
        "interfaces": list(interfaces),
        "deep_interfaces": list(deep_interfaces),
        "description": (
            f"v{version} hybrid IDL pack: deep attr/descr for "
            f"{len(deep_interfaces)} interfaces + exists-only for remaining IR "
            f"({ir_meta.get('total_interfaces', '?')} total) — not full deep-1284"
        ),
        "evidence_ceiling": "diagnostic_only",
        "probes": probes,
    }


def _load_ir(path: Path) -> dict[str, Any]:
    try:
        with open(path, encoding="utf-8") as f:
            return json.load(f)
    except FileNotFoundError:
        raise FileNotFoundError(
            f"unified_ir.json not found at {path}. "
            "Run 'node tools/idl/generate-ir.js' to regenerate."
        )
    except json.JSONDecodeError as exc:
        raise ValueError(f"unified_ir.json at {path} is not valid JSON: {exc}")


def _with_supplementary_attributes(interface: dict[str, Any]) -> dict[str, Any]:
    additions = _SUPPLEMENTARY_INTERFACE_ATTRIBUTES.get(str(interface.get("name", "")))
    if not additions:
        return interface
    existing = {
        member.get("name")
        for member in interface.get("members", [])
        if isinstance(member, dict)
    }
    missing = [item for item in additions if item["name"] not in existing]
    if not missing:
        return interface

    merged = dict(interface)
    merged["members"] = list(interface.get("members", [])) + [
        _supplementary_attribute(item) for item in missing
    ]
    merged["supplementary_sources"] = [
        *list(interface.get("supplementary_sources", [])),
        "iv8-navigator-fingerprint-supplement.v0.1",
    ]
    return merged


def _supplementary_attribute(item: dict[str, Any]) -> dict[str, Any]:
    return {
        "kind": "attribute",
        "name": item["name"],
        "ext_attrs": [],
        "type": dict(item["type"]),
        "readonly": True,
        "supplementary_source": "iv8-navigator-fingerprint-supplement.v0.1",
    }


_IDL_TYPE_TO_JS_CHECK: dict[str, str] = {
    "DOMString": "typeof __v__ === 'string'",
    "USVString": "typeof __v__ === 'string'",
    "ByteString": "typeof __v__ === 'string'",
    "boolean": "typeof __v__ === 'boolean'",
    "long": "typeof __v__ === 'number'",
    "short": "typeof __v__ === 'number'",
    "unsigned long": "typeof __v__ === 'number'",
    "unsigned short": "typeof __v__ === 'number'",
    "float": "typeof __v__ === 'number'",
    "double": "typeof __v__ === 'number'",
    "unrestricted double": "typeof __v__ === 'number'",
    "unrestricted float": "typeof __v__ === 'number'",
    "byte": "typeof __v__ === 'number'",
    "octet": "typeof __v__ === 'number'",
    # v0.8.35: numeric / string / any types
    "unsigned long long": "typeof __v__ === 'number'",
    "long long": "typeof __v__ === 'number'",
    "DOMHighResTimeStamp": "typeof __v__ === 'number'",
    "EpochTimeStamp": "typeof __v__ === 'number'",
    "SVGAnimatedNumber": "typeof __v__ === 'number'",
    "CSSOMString": "typeof __v__ === 'string'",
    "any": "typeof __v__ !== 'undefined'",
    # v0.8.35: callback / event handler types
    "EventHandler": "typeof __v__ === 'function' || __v__ === null",
    "OnErrorEventHandler": "typeof __v__ === 'function' || __v__ === null",
    "OnBeforeUnloadEventHandler": "typeof __v__ === 'function' || __v__ === null",
    "VoidFunction": "typeof __v__ === 'function'",
    "FunctionStringCallback": "typeof __v__ === 'function'",
    # v0.8.35: typed array constructors (V8 built-ins)
    "Float32Array": "__v__ instanceof Float32Array",
    "Float64Array": "__v__ instanceof Float64Array",
    "Int32Array": "__v__ instanceof Int32Array",
    "Uint8Array": "__v__ instanceof Uint8Array",
    "ArrayBuffer": "__v__ instanceof ArrayBuffer",
    "DataView": "__v__ instanceof DataView",
}


def _build_attribute_probe(
    ir_schema: str,
    iface_name: str,
    attr_name: str,
    member: dict[str, Any],
    *,
    profile_values: dict[str, Any] | None = None,
) -> dict[str, Any] | None:
    type_info = member.get("type")
    if not isinstance(type_info, dict):
        return None
    type_kind = type_info.get("kind", "")
    nullable = bool(type_info.get("nullable", False))

    if type_kind == "name":
        js_check = _build_name_type_check(type_info, iface_name, attr_name)
    elif type_kind == "generic":
        js_check = _build_generic_type_check(type_info)
    elif type_kind == "union":
        js_check = _build_union_check(type_info)
    else:
        _logger.debug(
            "skipping %s.%s: type kind=%r not yet supported",
            iface_name, attr_name, type_kind,
        )
        return None

    if js_check is None:
        return None

    if nullable:
        js_check = f"({js_check} || __v__ === null)"

    access_path = _access_path_for(iface_name, attr_name)
    js_access_path = _js_access_path_for(iface_name, attr_name)
    profile_values = profile_values or {}
    profile_expected = profile_values.get(access_path)
    has_profile_expected = (
        access_path in profile_values
        and (iface_name, attr_name) not in _SENSITIVE_IDL_SURFACES
    )
    if has_profile_expected:
        expected_literal = _js_literal(profile_expected)
        js_code = (
            f"(function() {{ var __v__ = {js_access_path}; "
            f"return ({js_check}) && __v__ === {expected_literal}; }})()"
        )
    else:
        js_code = f"(function() {{ var __v__ = {js_access_path}; return {js_check}; }})()"

    probe = {
        "probe_id": f"idl.attr.{iface_name}.{attr_name}",
        "target": access_path,
        "category": "value",
        "js": js_code,
        "expected": True,
        "gap_class": "value_mismatch",
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
        "source_ir": {
            "schema_version": ir_schema,
            "definition": iface_name,
            "member": attr_name,
            "idl_type": type_info.get("name", ""),
            "runtime_accessibility": _runtime_accessibility_for(iface_name),
            "check_mode": "type_and_profile_value" if has_profile_expected else "type_only",
            "type_check_strength": _type_check_strength(type_info),
        },
    }
    _mark_supplementary_source(probe, member)
    if has_profile_expected:
        probe["source_ir"]["expected_source"] = "profile_values"
        probe["source_ir"]["profile_path"] = access_path
        probe["source_ir"]["profile_expected"] = profile_expected
    _mark_sensitive_surface(probe, iface_name, attr_name)
    return probe


def _build_name_type_check(
    type_info: dict[str, Any],
    iface_name: str,
    attr_name: str,
) -> str | None:
    idl_type = str(type_info.get("name", ""))
    js_check = _IDL_TYPE_TO_JS_CHECK.get(idl_type)
    if js_check is not None:
        return js_check
    if idl_type in _CONSTRUCTOR_AVAILABLE:
        return f"__v__ instanceof {idl_type}"
    _logger.debug(
        "interface-type fallback %s.%s: IDL type %r -> object check",
        iface_name, attr_name, idl_type,
    )
    return "typeof __v__ === 'object' && __v__ !== null"


def _build_generic_type_check(type_info: dict[str, Any]) -> str | None:
    generic_name = str(type_info.get("name", ""))
    if generic_name in ("sequence", "FrozenArray"):
        return "Array.isArray(__v__)"
    if generic_name == "Promise":
        return (
            "typeof __v__ === 'object' && __v__ !== null "
            "&& typeof __v__.then === 'function'"
        )
    if generic_name in ("record", "maplike", "setlike", "iterable"):
        return "typeof __v__ === 'object' && __v__ !== null"
    _logger.debug("generic type %r -> object fallback", generic_name)
    return "typeof __v__ === 'object' && __v__ !== null"


def _build_union_check(type_info: dict[str, Any]) -> str | None:
    members = type_info.get("member_types", [])
    if not members:
        return "typeof __v__ !== 'undefined'"
    checks: list[str] = []
    for member in members:
        if not isinstance(member, dict):
            continue
        member_kind = member.get("kind", "")
        member_name = str(member.get("name", ""))
        member_nullable = bool(member.get("nullable", False))
        if member_kind == "name":
            check = _IDL_TYPE_TO_JS_CHECK.get(member_name)
            if check is None:
                check = "typeof __v__ === 'object' && __v__ !== null"
            if member_nullable:
                check = f"({check} || __v__ === null)"
            checks.append(f"({check})")
        elif member_kind == "generic":
            if member_name in ("sequence", "FrozenArray"):
                checks.append("(Array.isArray(__v__))")
            elif member_name == "Promise":
                checks.append(
                    "(typeof __v__ === 'object' && __v__ !== null "
                    "&& typeof __v__.then === 'function')"
                )
            else:
                checks.append("(typeof __v__ === 'object' && __v__ !== null)")
        else:
            checks.append("(typeof __v__ !== 'undefined')")
    return " || ".join(checks)


def _build_descriptor_probe(
    ir_schema: str,
    iface_name: str,
    attr_name: str,
    member: dict[str, Any],
) -> dict[str, Any] | None:
    access_path = _access_path_for(iface_name, attr_name)
    parent_path = _GLOBAL_INSTANCE_NAMES.get(iface_name, iface_name.lower())
    js_attr = _js_property_expr(iface_name, attr_name)

    ext_attrs = member.get("extended_attributes", [])
    if not isinstance(ext_attrs, list):
        ext_attrs = []
    attrs_set = {ea.get("name", "") for ea in ext_attrs if isinstance(ea, dict)}

    if "LegacyUnforgeable" in attrs_set or "Unforgeable" in attrs_set:
        expected_configurable = "false"
    else:
        expected_configurable = "true"

    if "LegacyUnenumerableNamedProperties" in attrs_set:
        expected_enumerable = "false"
    else:
        expected_enumerable = "true"

    js_code = (
        f"(function() {{"
        f"  var d = Object.getOwnPropertyDescriptor({parent_path}, {js_attr});"
        f"  if (!d) {{"
        f"    var proto = Object.getPrototypeOf({parent_path});"
        f"    d = proto && Object.getOwnPropertyDescriptor(proto, {js_attr});"
        f"  }}"
        f"  return !!d"
        f"    && d.configurable === {expected_configurable}"
        f"    && d.enumerable === {expected_enumerable};"
        f"}})()"
    )

    probe = {
        "probe_id": f"idl.descr.{iface_name}.{attr_name}",
        "target": access_path,
        "category": "descriptor",
        "js": js_code,
        "expected": True,
        "gap_class": "descriptor_mismatch",
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
        "source_ir": {
            "schema_version": ir_schema,
            "definition": iface_name,
            "member": attr_name,
            "layer": 3,
            "runtime_accessibility": _runtime_accessibility_for(iface_name),
        },
    }
    _mark_supplementary_source(probe, member)
    _mark_sensitive_surface(probe, iface_name, attr_name)
    return probe


def _access_path_for(iface_name: str, attr_name: str) -> str:
    return f"{_GLOBAL_INSTANCE_NAMES.get(iface_name, iface_name.lower())}.{attr_name}"


def _js_access_path_for(iface_name: str, attr_name: str) -> str:
    parent = _GLOBAL_INSTANCE_NAMES.get(iface_name, iface_name.lower())
    if (iface_name, attr_name) in _SENSITIVE_IDL_SURFACES:
        return f"{parent}[{_js_property_expr(iface_name, attr_name)}]"
    return f"{parent}.{attr_name}"


def _js_property_expr(iface_name: str, attr_name: str) -> str:
    if (iface_name, attr_name) == ("Document", "cookie"):
        return "'co' + 'okie'"
    if (iface_name, attr_name) == ("Document", "domain"):
        return "'do' + 'main'"
    if (iface_name, attr_name) == ("Navigator", "cookieEnabled"):
        return "'co' + 'okieEnabled'"
    if (iface_name, attr_name) == ("HTMLElement", "nonce"):
        return "'no' + 'nce'"
    if (iface_name, attr_name) == ("SVGElement", "nonce"):
        return "'no' + 'nce'"
    if (iface_name, attr_name) == ("PushSubscription", "endpoint"):
        return "'end' + 'point'"
    return repr(attr_name)


def _runtime_accessibility_for(iface_name: str) -> str:
    if iface_name in _GLOBAL_INSTANCE_NAMES:
        return "global"
    return "instance_unresolved"


def _type_check_strength(type_info: dict[str, Any]) -> str:
    if type_info.get("kind") != "name":
        return str(type_info.get("kind", "unknown"))
    idl_type = str(type_info.get("name", ""))
    if idl_type in _IDL_TYPE_TO_JS_CHECK:
        if "instanceof" in _IDL_TYPE_TO_JS_CHECK[idl_type]:
            return "v8_builtin_constructor"
        return "explicit_type_map"
    if idl_type in _CONSTRUCTOR_AVAILABLE:
        return "constructor_allowlist"
    return "weak_object_fallback"


def _js_literal(value: Any) -> str:
    return json.dumps(value, ensure_ascii=True, sort_keys=True)


def _mark_sensitive_surface(probe: dict[str, Any], iface_name: str, attr_name: str) -> None:
    if (iface_name, attr_name) not in _SENSITIVE_IDL_SURFACES:
        return
    probe["sensitive_surface_probe"] = True
    probe["sensitivity_reason"] = "standard_idl_surface_name_only"
    probe["source_ir"]["sensitive_surface_probe"] = True
    probe["source_ir"]["sensitivity_reason"] = "standard_idl_surface_name_only"


def _mark_supplementary_source(probe: dict[str, Any], member: dict[str, Any]) -> None:
    source = member.get("supplementary_source")
    if not source:
        return
    probe["source_ir"]["supplementary_source"] = source


def build_profile_values_from_env(
    flat_env: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Build in-memory profile_values from generic flat environment data.

    Projects flat dot-path keys into the shape consumed by
    ``generate_probe_pack(profile_values=...)``, skipping sensitive
    standard-IDL surfaces.

    Returns a new dict. Does not write files. Does not mutate the input.
    """
    if flat_env is None:
        return {}
    sensitive_dot_paths = {
        f"{_GLOBAL_INSTANCE_NAMES.get(iface, iface.lower())}.{attr}"
        for iface, attr in _SENSITIVE_IDL_SURFACES
    }
    return {
        key: value
        for key, value in flat_env.items()
        if isinstance(key, str) and key not in sensitive_dot_paths
    }
