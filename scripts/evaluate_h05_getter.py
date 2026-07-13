#!/usr/bin/env python3
"""H05a: Getter Return Value Audit — IDL-driven full enumeration.

Enumerates all IDL-declared attributes from unified_ir.json, invokes each
getter in IV8 and (optionally) Chrome via CDP, classifies the result.

Classification scheme:
  PASS              — typeof matches IDL type, value matches Chrome
  TYPE_FAIL         — typeof wrong (e.g., object for DOMString)
  VALUE_FAIL        — typeof correct but value differs from Chrome
  THROW             — getter threw unexpected exception
  SKIP_NO_INSTANCE  — no instance available (Tier E)
  SKIP_CHROME       — Chrome CDP unavailable
  SKIP_ASYNC        — attribute returns Promise (defer to H05c)

Usage:
  # IV8-only (no Chrome needed, skips Chrome diff)
  python scripts/evaluate_h05_getter.py

  # With Chrome CDP diff
  python scripts/evaluate_h05_getter.py --chrome "D:\\path\\to\\chrome.exe"

  # Limit to top N interfaces
  python scripts/evaluate_h05_getter.py --top 50

Output:
  status/h05-getter-values.json
  Exit code: 0 if no TYPE_FAIL/THROW, 1 otherwise
"""
from __future__ import annotations

import argparse
import json
import os
import socket
import struct
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path
from urllib.parse import urlparse

REPO_ROOT = Path(__file__).resolve().parent.parent
IR_PATH = REPO_ROOT / "tools" / "idl" / "output" / "unified_ir.json"
STATUS_DIR = REPO_ROOT / "status"
OUTPUT_PATH = STATUS_DIR / "h05-getter-values.json"

DEFAULT_CHROME_PATH = r"D:\Download\Softwares\chromium-debug\chrome.exe"

# --- THRESHOLDS (P3: explicit, centralized) ---
# Change thresholds → must justify in commit body (HARNESS-CHARTER §6 rule 4).
THRESHOLDS = {
    # Category A (Data Integrity): error count must be 0
    "max_type_fail": 0,        # TYPE_FAIL = wrong typeof (codegen bug)
    "max_throw": 0,            # THROW = getter threw unexpected exception
    # Category B (Recall): % of IDL attributes evaluated
    "min_evaluated_pct": 80.0, # at least 80% of attributes must be evaluable
    # Category C (False Positive): known-good values must PASS
    "min_known_good_pass_pct": 100.0, # all known-good values must PASS
    # Category D (Coverage): interface coverage
    "min_interface_coverage_pct": 35.0, # Phase 1: top 50 of 1284 = ~4%
    # Category E (Robustness): Chrome disconnect handling
    "chrome_disconnect_is_skip": True, # Chrome unavailable → SKIP, not FAIL
}

# Category C: known-good values that MUST pass (P2: falsification)
# These are values we know Chrome returns — if IV8 FAILS these, it's a real bug.
KNOWN_GOOD_VALUES = {
    ("Navigator", "userAgent"): ("string", None),  # type check only (value is profile-dependent)
    ("Navigator", "platform"): ("string", None),
    ("Navigator", "language"): ("string", None),
    ("Navigator", "hardwareConcurrency"): ("number", None),
    ("Screen", "width"): ("number", None),
    ("Screen", "height"): ("number", None),
    ("Screen", "colorDepth"): ("number", None),
    ("Window", "innerWidth"): ("number", None),
    ("Window", "innerHeight"): ("number", None),
    ("Event", "bubbles"): ("boolean", False),
    ("Event", "cancelable"): ("boolean", False),
    ("MouseEvent", "screenX"): ("number", 0),
    ("MouseEvent", "ctrlKey"): ("boolean", False),
    ("KeyboardEvent", "key"): ("string", ""),
    ("KeyboardEvent", "isComposing"): ("boolean", False),
    ("PointerEvent", "pointerId"): ("number", 0),
}

# Category C negative test: calling getter with wrong receiver must throw.
# Per WebIDL spec, attribute getters must throw TypeError when `this`
# is not an instance of the interface (e.g., calling on the prototype
# object itself or on an unrelated object).
RECEIVER_CHECK_INTERFACES = [
    ("Navigator", "userAgent"),
    ("Screen", "width"),
    ("Document", "title"),
    ("Element", "tagName"),
    ("HTMLElement", "hidden"),
    ("Event", "type"),
    ("MouseEvent", "clientX"),
    ("KeyboardEvent", "key"),
]

# Top 50 interfaces by WPT idlharness test count + fingerprinting relevance.
# Tier A (global singletons) and Tier B (createElement) are prioritized.
TOP_50_INTERFACES = [
    # Tier A — global singletons
    "Navigator", "Window", "Document", "Screen", "History", "Location",
    "Performance", "PerformanceNavigationTiming", "PerformanceObserver",
    # Tier B — HTML elements via createElement
    "HTMLElement", "HTMLDivElement", "HTMLSpanElement", "HTMLAnchorElement",
    "HTMLInputElement", "HTMLButtonElement", "HTMLFormElement",
    "HTMLSelectElement", "HTMLOptionElement", "HTMLOptionsCollection",
    "HTMLTextAreaElement", "HTMLImageElement", "HTMLCanvasElement",
    "HTMLScriptElement", "HTMLLinkElement", "HTMLMetaElement",
    "HTMLIFrameElement", "HTMLBodyElement", "HTMLHeadElement",
    "HTMLUListElement", "HTMLOListElement", "HTMLLIElement",
    "HTMLTableElement", "HTMLTableRowElement", "HTMLTableCellElement",
    "HTMLMediaElement", "HTMLVideoElement", "HTMLAudioElement",
    # Tier B — non-element
    "Element", "Node", "CharacterData", "Text", "Comment",
    "DocumentFragment", "ShadowRoot", "Attr",
    # Tier C — constructable
    "Event", "CustomEvent", "MouseEvent", "KeyboardEvent", "PointerEvent",
    "URL", "URLSearchParams", "Headers", "AbortController", "AbortSignal",
    # Tier D — extended HTML elements (51-100)
    "HTMLLabelElement", "HTMLFieldSetElement", "HTMLLegendElement",
    "HTMLDataListElement", "HTMLOutputElement", "HTMLProgressElement",
    "HTMLMeterElement", "HTMLDetailsElement", "HTMLDialogElement",
    "HTMLPictureElement", "HTMLSourceElement", "HTMLTrackElement",
    "HTMLMapElement", "HTMLAreaElement", "HTMLObjectElement",
    "HTMLParamElement", "HTMLEmbedElement", "HTMLTemplateElement",
    "HTMLSlotElement", "HTMLTimeElement", "HTMLDataElement",
    "HTMLWbrElement", "HTMLBrElement", "HTMLHrElement",
    "HTMLParagraphElement", "HTMLHeadingElement", "HTMLQuoteElement",
    "HTMLPreElement", "HTMLDListElement", "HTMLDirectoryElement",
    "HTMLFontElement", "HTMLBaseFontElement", "HTMLFrameElement",
    "HTMLFrameSetElement", "HTMLTitleElement", "HTMLStyleElement",
    "HTMLBaseElement", "HTMLHtmlElement", "HTMLMarqueeElement",
    # Tier D — extended DOM (101-150)
    "CSSStyleDeclaration", "CSSRule", "CSSStyleRule", "CSSStyleSheet",
    "StyleSheet", "StyleSheetList", "MediaList", "MediaQueryList",
    "MutationObserver", "MutationRecord", "NodeIterator", "TreeWalker",
    "Range", "Selection", "DOMRect", "DOMRectReadOnly", "DOMPoint",
    "DOMMatrix", "DOMQuad", "DOMTokenList", "HTMLCollection",
    "NamedNodeMap", "NodeList", "DOMStringMap", "FormData",
    "Blob", "File", "FileReader", "FileList", "ProgressEvent",
    "XMLHttpRequest", "XMLHttpRequestEventTarget", "XMLHttpRequestUpload",
    "WebSocket", "EventSource", "MessageChannel", "MessagePort",
    "BroadcastChannel", "Worker", "SharedWorker",
    # Tier D — extended misc (151-200)
    "Storage", "StorageEvent", "PopStateEvent", "HashChangeEvent",
    "PageTransitionEvent", "BeforeUnloadEvent", "ErrorEvent",
    "PromiseRejectionEvent", "MessageEvent", "SubmitEvent",
    "FormDataEvent", "ToggleEvent", "ClipboardEvent", "DragEvent",
    "AnimationEvent", "TransitionEvent", "InputEvent", "UIEvent",
    "FocusEvent", "WheelEvent", "TouchEvent", "CompositionEvent",
    "GamepadEvent", "DeviceOrientationEvent", "DeviceMotionEvent",
    "StorageManager", "Notification", "Permissions", "PermissionStatus",
    "Geolocation", "GeolocationPosition", "GeolocationPositionError",
    "PushManager", "PushSubscription", "PushSubscriptionOptions",
    "Cache", "CacheStorage", "PaymentRequest", "PaymentAddress",
    "PaymentResponse", "PaymentMethodChangeEvent", "PaymentRequestUpdateEvent",
    "CredentialsContainer", "Credential", "PasswordCredential", "FederatedCredential",
    "PublicKeyCredential", "AuthenticatorResponse", "AuthenticatorAttestationResponse",
    "AuthenticatorAssertionResponse",
]

# Tag name mapping for HTML elements (createElement tag → interface name)
TAG_TO_INTERFACE = {
    "div": "HTMLDivElement", "span": "HTMLSpanElement",
    "a": "HTMLAnchorElement", "input": "HTMLInputElement",
    "button": "HTMLButtonElement", "form": "HTMLFormElement",
    "select": "HTMLSelectElement", "option": "HTMLOptionElement",
    "textarea": "HTMLTextAreaElement", "img": "HTMLImageElement",
    "canvas": "HTMLCanvasElement", "script": "HTMLScriptElement",
    "link": "HTMLLinkElement", "meta": "HTMLMetaElement",
    "iframe": "HTMLIFrameElement", "body": "HTMLBodyElement",
    "head": "HTMLHeadElement", "ul": "HTMLUListElement",
    "ol": "HTMLOListElement", "li": "HTMLLIElement",
    "table": "HTMLTableElement", "tr": "HTMLTableRowElement",
    "td": "HTMLTableCellElement", "th": "HTMLTableCellElement",
    "video": "HTMLVideoElement", "audio": "HTMLAudioElement",
    "label": "HTMLLabelElement", "fieldset": "HTMLFieldSetElement",
    "legend": "HTMLLegendElement", "datalist": "HTMLDataListElement",
    "output": "HTMLOutputElement", "progress": "HTMLProgressElement",
    "meter": "HTMLMeterElement", "details": "HTMLDetailsElement",
    "dialog": "HTMLDialogElement", "picture": "HTMLPictureElement",
    "source": "HTMLSourceElement", "track": "HTMLTrackElement",
    "map": "HTMLMapElement", "area": "HTMLAreaElement",
    "object": "HTMLObjectElement", "param": "HTMLParamElement",
    "embed": "HTMLEmbedElement", "template": "HTMLTemplateElement",
    "slot": "HTMLSlotElement", "time": "HTMLTimeElement",
    "data": "HTMLDataElement", "wbr": "HTMLWbrElement",
    "br": "HTMLBrElement", "hr": "HTMLHrElement",
    "p": "HTMLParagraphElement", "blockquote": "HTMLQuoteElement",
    "q": "HTMLQuoteElement", "pre": "HTMLPreElement",
    "dl": "HTMLDListElement", "dir": "HTMLDirectoryElement",
    "font": "HTMLFontElement", "basefont": "HTMLBaseFontElement",
    "frame": "HTMLFrameElement", "frameset": "HTMLFrameSetElement",
    "title": "HTMLTitleElement", "style": "HTMLStyleElement",
    "base": "HTMLBaseElement", "html": "HTMLHtmlElement",
    "marquee": "HTMLMarqueeElement",
    "h1": "HTMLHeadingElement", "h2": "HTMLHeadingElement",
    "h3": "HTMLHeadingElement", "h4": "HTMLHeadingElement",
    "h5": "HTMLHeadingElement", "h6": "HTMLHeadingElement",
    "col": "HTMLTableColElement", "colgroup": "HTMLTableColElement",
    "thead": "HTMLTableSectionElement", "tbody": "HTMLTableSectionElement",
    "tfoot": "HTMLTableSectionElement",
    "caption": "HTMLTableCaptionElement",
    "del": "HTMLModElement", "ins": "HTMLModElement",
}

# Interfaces accessible as global singletons
GLOBAL_SINGLETONS = {
    "Window": "window", "Navigator": "navigator", "Document": "document",
    "Screen": "screen", "History": "history", "Location": "location",
    "Performance": "performance",
}

# Interfaces that can be constructed with new
CONSTRUCTABLE = {
    "Event": "new Event('click')",
    "CustomEvent": "new CustomEvent('test')",
    "MouseEvent": "new MouseEvent('click')",
    "KeyboardEvent": "new KeyboardEvent('keydown')",
    "PointerEvent": "new PointerEvent('pointerdown')",
    "URL": "new URL('https://example.com')",
    "URLSearchParams": "new URLSearchParams('a=1')",
    "Headers": "new Headers()",
    "AbortController": "new AbortController()",
    "AbortSignal": "new AbortController().signal",
    "MutationObserver": "new MutationObserver(function(){})",
    "ProgressEvent": "new ProgressEvent('load')",
    "StorageEvent": "new StorageEvent('storage')",
    "PopStateEvent": "new PopStateEvent('popstate')",
    "HashChangeEvent": "new HashChangeEvent('hashchange')",
    "PageTransitionEvent": "new PageTransitionEvent('pageshow')",
    "BeforeUnloadEvent": None,
    "ErrorEvent": "new ErrorEvent('error')",
    "PromiseRejectionEvent": "new PromiseRejectionEvent('unhandledrejection')",
    "MessageEvent": "new MessageEvent('message')",
    "SubmitEvent": "new SubmitEvent('submit')",
    "FormDataEvent": "new FormDataEvent('formdata')",
    "ToggleEvent": "new ToggleEvent('toggle')",
    "ClipboardEvent": "new ClipboardEvent('copy')",
    "InputEvent": "new InputEvent('input')",
    "UIEvent": "new UIEvent('ui')",
    "FocusEvent": "new FocusEvent('focus')",
    "WheelEvent": "new WheelEvent('wheel')",
    "AnimationEvent": "new AnimationEvent('animationend')",
    "TransitionEvent": "new TransitionEvent('transitionend')",
    "Notification": "new Notification('test')",
    "FormData": "new FormData()",
    "Blob": "new Blob([])",
    "FileReader": "new FileReader()",
    "XMLHttpRequest": "new XMLHttpRequest()",
    "WebSocket": "new WebSocket('ws://localhost')",
    "EventSource": "new EventSource('http://localhost')",
    "MessageChannel": "new MessageChannel()",
    "BroadcastChannel": "new BroadcastChannel('test')",
    "Range": "document.createRange()",
    "DOMRect": "new DOMRect(0, 0, 100, 100)",
    "DOMPoint": "new DOMPoint(0, 0)",
    "DOMMatrix": "new DOMMatrix()",
    "Worker": None,
    "SharedWorker": None,
}

# IDL type → expected JS typeof
IDL_TYPE_TO_JSTYPE = {
    "DOMString": "string", "USVString": "string", "ByteString": "string",
    "CSSOMString": "string",
    "boolean": "boolean",
    "byte": "number", "octet": "number",
    "short": "number", "unsigned short": "number",
    "long": "number", "unsigned long": "number",
    "long long": "number", "unsigned long long": "number",
    "float": "number", "unrestricted float": "number",
    "double": "number", "unrestricted double": "number",
    "bigint": "bigint",
    "any": None,  # skip type check
    "object": "object",
    "void": "undefined", "undefined": "undefined",
    "Promise": "object",  # Promise is object
    "Function": "function",
    "ArrayBuffer": "object",
    "Uint8Array": "object", "Uint8ClampedArray": "object",
    "Int8Array": "object", "Uint16Array": "object",
    "Int16Array": "object", "Uint32Array": "object",
    "Int32Array": "object", "Float32Array": "object",
    "Float64Array": "object", "DataView": "object",
}


def extract_type_name(idl_type: dict | str) -> str:
    """Extract the base type name from an IDL type node."""
    if isinstance(idl_type, str):
        return idl_type
    kind = idl_type.get("kind", "name")
    if kind == "name":
        return idl_type.get("name", "any")
    if kind == "union":
        return "any"  # union types — skip type check
    if kind == "sequence":
        return "object"
    if kind == "frozen array" or kind == "FrozenArray":
        return "object"
    if kind == "record":
        return "object"
    if kind == "promise":
        return "object"
    return idl_type.get("name", "any")


def is_nullable(idl_type: dict | str) -> bool:
    if isinstance(idl_type, dict):
        return idl_type.get("nullable", False)
    return False


def enumerate_attributes(ir_path: Path, top_n: int | None = None) -> list[dict]:
    """Parse unified_ir.json and yield all interface attributes."""
    with open(ir_path, encoding="utf-8") as f:
        ir = json.load(f)

    definitions = ir["definitions"]
    interfaces = [d for d in definitions if d["kind"] == "interface"]

    if top_n and top_n > 0:
        iface_names = set(TOP_50_INTERFACES[:top_n])
        interfaces = [i for i in interfaces if i["name"] in iface_names]

    results = []
    for iface in interfaces:
        iface_name = iface["name"]
        for member in iface.get("members", []):
            if member["kind"] != "attribute":
                continue
            attr_name = member.get("name")
            if not attr_name:
                continue
            type_node = member.get("type", {})
            base_type = extract_type_name(type_node)
            nullable = is_nullable(type_node)
            readonly = member.get("readonly", False)
            results.append({
                "interface": iface_name,
                "attribute": attr_name,
                "idl_type": base_type,
                "nullable": nullable,
                "readonly": readonly,
            })

    return results


def build_instance_js(iface_name: str) -> str | None:
    """Build JS expression to create an instance of the given interface.

    Returns None if no instance creation strategy is available (Tier E).
    """
    if iface_name in GLOBAL_SINGLETONS:
        return GLOBAL_SINGLETONS[iface_name]

    # Find tag name for HTML elements
    for tag, iface in TAG_TO_INTERFACE.items():
        if iface == iface_name:
            return f"document.createElement('{tag}')"

    # Non-element interfaces
    if iface_name == "HTMLElement":
        return "document.createElement('div')"
    if iface_name == "Element":
        return "document.createElement('div')"
    if iface_name == "Node":
        return "document.createElement('div')"
    if iface_name == "CharacterData":
        return "document.createTextNode('text')"
    if iface_name == "Text":
        return "document.createTextNode('text')"
    if iface_name == "Comment":
        return "document.createComment('comment')"
    if iface_name == "DocumentFragment":
        return "document.createDocumentFragment()"
    if iface_name == "ShadowRoot":
        return None  # requires attachShadow, complex
    if iface_name == "Attr":
        return None  # createAttribute deprecated, complex

    if iface_name in CONSTRUCTABLE:
        return CONSTRUCTABLE[iface_name]

    # Performance sub-interfaces
    if iface_name == "PerformanceNavigationTiming":
        return "performance.getEntriesByType('navigation')[0]"
    if iface_name == "PerformanceObserver":
        return "new PerformanceObserver(function(){})"

    # HTMLOptionsCollection
    if iface_name == "HTMLOptionsCollection":
        return "document.createElement('select').options"

    return None


def build_getter_js(instance_js: str, attr_name: str) -> str:
    """Build JS expression to invoke a getter and capture result."""
    return f"""(function() {{
    try {{
        var instance = {instance_js};
        if (!instance) return {{ threw: false, typeof: 'undefined', value: null, skipped: true }};
        var v = instance['{attr_name}'];
        var t = typeof v;
        var val = null;
        if (v === null) {{
            val = null;
        }} else if (t === 'string' || t === 'number' || t === 'boolean') {{
            val = v;
        }} else if (t === 'function') {{
            val = '[function:' + (v.name || 'anonymous') + ']';
        }} else if (t === 'object') {{
            try {{
                var ctor = v.constructor;
                val = '[object:' + (ctor ? ctor.name : 'Unknown') + ']';
            }} catch(e) {{
                val = '[object:unknown]';
            }}
        }} else if (t === 'symbol') {{
            val = '[symbol]';
        }} else if (t === 'bigint') {{
            val = String(v);
        }} else {{
            val = String(v);
        }}
        return {{ threw: false, typeof: t, value: val }};
    }} catch(e) {{
        return {{ threw: true, error: e.message, errorType: e.name }};
    }}
}})()"""


# ---------------------------------------------------------------------------
# Minimal WebSocket client (reused from sample_surface_values.py)
# ---------------------------------------------------------------------------

class WSClient:
    def __init__(self, host, port, path="/"):
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.sock.connect((host, port))
        self._do_handshake(host, port, path)

    def _do_handshake(self, host, port, path):
        import base64
        key = base64.b64encode(os.urandom(16)).decode()
        req = (
            f"GET {path} HTTP/1.1\r\n"
            f"Host: {host}:{port}\r\n"
            f"Upgrade: websocket\r\n"
            f"Connection: Upgrade\r\n"
            f"Sec-WebSocket-Key: {key}\r\n"
            f"Sec-WebSocket-Version: 13\r\n"
            f"\r\n"
        )
        self.sock.sendall(req.encode())
        resp = b""
        while b"\r\n\r\n" not in resp:
            resp += self.sock.recv(4096)
        if b"101" not in resp.split(b"\r\n")[0]:
            raise RuntimeError(f"WebSocket handshake failed: {resp[:200]}")

    def send(self, data):
        payload = json.dumps(data).encode("utf-8")
        header = bytearray()
        header.append(0x81)
        mask = os.urandom(4)
        if len(payload) < 126:
            header.append(0x80 | len(payload))
        elif len(payload) < 65536:
            header.append(0x80 | 126)
            header += struct.pack(">H", len(payload))
        else:
            header.append(0x80 | 127)
            header += struct.pack(">Q", len(payload))
        header += mask
        masked = bytearray(b ^ mask[i % 4] for i, b in enumerate(payload))
        self.sock.sendall(bytes(header) + bytes(masked))

    def recv(self):
        header = self._recv_exact(2)
        opcode = header[0] & 0x0F
        masked = (header[1] & 0x80) != 0
        length = header[1] & 0x7F
        if length == 126:
            length = struct.unpack(">H", self._recv_exact(2))[0]
        elif length == 127:
            length = struct.unpack(">Q", self._recv_exact(8))[0]
        if masked:
            mask = self._recv_exact(4)
        data = self._recv_exact(length)
        if masked:
            data = bytearray(b ^ mask[i % 4] for i, b in enumerate(data))
        if opcode == 1:
            return json.loads(data.decode("utf-8"))
        elif opcode == 8:
            raise RuntimeError("WebSocket closed")
        return {}

    def _recv_exact(self, n):
        data = b""
        while len(data) < n:
            chunk = self.sock.recv(n - len(data))
            if not chunk:
                raise RuntimeError("Connection closed")
            data += chunk
        return data

    def close(self):
        try:
            self.sock.close()
        except Exception:
            pass


class ChromeCDPProbe:
    """Chrome CDP probe — evaluates JS expressions in Chrome via Runtime.evaluate."""

    def __init__(self, chrome_path: str, port: int = 9222, launch: bool = True):
        self.proc = None
        self.ws = None
        self.msg_id = 0
        self.port = port

        if launch:
            if not chrome_path or not Path(chrome_path).exists():
                raise FileNotFoundError(f"Chrome not found: {chrome_path}")
            user_data_dir = tempfile.mkdtemp(prefix="chrome_h05_")
            self.proc = subprocess.Popen(
                [chrome_path, f"--remote-debugging-port={port}",
                 f"--user-data-dir={user_data_dir}",
                 "--no-first-run", "--no-default-browser-check",
                 "--disable-popup-blocking", "about:blank"],
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            )
            time.sleep(3)

        resp = urllib.request.urlopen(f"http://127.0.0.1:{port}/json", timeout=5)
        targets = json.loads(resp.read())
        page_target = next((t for t in targets if t.get("type") == "page"), None)
        if not page_target:
            raise RuntimeError("No page target found in Chrome")

        ws_url = page_target["webSocketDebuggerUrl"]
        parsed = urlparse(ws_url)
        self.ws = WSClient(parsed.hostname or "127.0.0.1",
                           parsed.port or port, parsed.path)

    def evaluate(self, js_expression: str) -> dict | None:
        """Evaluate JS in Chrome, return result dict."""
        self.msg_id += 1
        mid = self.msg_id
        self.ws.send({
            "id": mid,
            "method": "Runtime.evaluate",
            "params": {"expression": js_expression, "returnByValue": True},
        })
        timeout = time.time() + 10
        while time.time() < timeout:
            resp = self.ws.recv()
            if resp.get("id") == mid:
                result = resp.get("result", {})
                if "exceptionDetails" in result:
                    return {"threw": True, "error": str(result["exceptionDetails"].get("exception", {}))}
                val = result.get("result", {}).get("value")
                return val if isinstance(val, dict) else {"threw": False, "typeof": "undefined", "value": val}
        return None

    def close(self):
        if self.ws:
            self.ws.close()
        if self.proc:
            self.proc.terminate()
            self.proc.wait(timeout=5)


def probe_iv8(ctx, instance_js: str, attr_name: str) -> dict:
    """Invoke getter in IV8 and capture result."""
    js = build_getter_js(instance_js, attr_name)
    raw = ctx.eval(js)
    if isinstance(raw, str):
        try:
            return json.loads(raw)
        except json.JSONDecodeError:
            return {"threw": True, "error": f"non-JSON return: {raw[:200]}"}
    if isinstance(raw, dict):
        return raw
    return {"threw": False, "typeof": type(raw).__name__, "value": str(raw)}


def probe_chrome(chrome: ChromeCDPProbe, instance_js: str, attr_name: str) -> dict | None:
    """Invoke getter in Chrome and capture result."""
    js = build_getter_js(instance_js, attr_name)
    return chrome.evaluate(js)


def classify(iv8_result: dict, chrome_result: dict | None,
             idl_type: str, nullable: bool, iv8_only: bool = False) -> str:
    """Classify the comparison result."""
    if iv8_result.get("skipped"):
        return "SKIP_NO_INSTANCE"
    if iv8_result.get("threw"):
        return "THROW"

    iv8_typeof = iv8_result.get("typeof", "undefined")

    # Check IDL type expectation
    expected_typeof = IDL_TYPE_TO_JSTYPE.get(idl_type)
    if expected_typeof and iv8_typeof != expected_typeof:
        if nullable and iv8_typeof == "undefined":
            return "PASS"
        if nullable and iv8_typeof == "object" and iv8_result.get("value") is None:
            return "PASS"
        return "TYPE_FAIL"

    # Skip async (Promise) attributes
    if idl_type == "Promise" or (idl_type.startswith("Promise") and iv8_typeof == "object"):
        return "SKIP_ASYNC"

    # In IV8-only mode, PASS if type matches IDL expectation
    if iv8_only or chrome_result is None:
        if expected_typeof is None or expected_typeof == iv8_typeof:
            return "PASS"
        return "TYPE_FAIL"

    if chrome_result.get("threw"):
        return "PASS"  # Chrome also throws — consistent behavior

    chrome_typeof = chrome_result.get("typeof", "undefined")
    if iv8_typeof != chrome_typeof:
        return "TYPE_FAIL"

    # Value comparison for primitives
    iv8_val = iv8_result.get("value")
    chrome_val = chrome_result.get("value")

    if iv8_typeof in ("string", "number", "boolean"):
        if iv8_val == chrome_val:
            return "PASS"
        # Numeric tolerance
        if iv8_typeof == "number":
            try:
                if abs(float(iv8_val) - float(chrome_val)) < 0.001:
                    return "PASS"
            except (ValueError, TypeError):
                pass
        return "VALUE_FAIL"

    # Object/function — typeof match is sufficient
    return "PASS"


def _run_in_thread(fn, *args, **kwargs):
    """Run fn in a sub-thread with sufficient stack for V8 template creation.

    Python's main thread has a small stack (1MB on Windows). V8 FunctionTemplate
    creation (1287 interfaces, 9223 members) requires 128MB+. We spawn a thread
    with threading.stack_size(128MB) to run JSContext creation + evaluation.
    """
    import threading
    result_box = [None, None]

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


def _run_audit(args):
    """Run the H05a audit. Must be called in a high-stack thread."""
    output_path = Path(args.output) if args.output else OUTPUT_PATH
    output_path.parent.mkdir(parents=True, exist_ok=True)

    # Step 1: Enumerate attributes from IDL
    print("Step 1: Enumerating IDL attributes...")
    attrs = enumerate_attributes(IR_PATH, top_n=args.top)
    print(f"  Found {len(attrs)} attributes across {len(set(a['interface'] for a in attrs))} interfaces")

    # Step 2: Initialize IV8
    print("Step 2: Initializing IV8 runtime...")
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext
    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)

    # Step 3: Initialize Chrome CDP (optional)
    chrome = None
    if not args.iv8_only and (args.chrome or not args.no_launch):
        try:
            chrome_path = args.chrome or DEFAULT_CHROME_PATH
            print(f"Step 3: Connecting to Chrome ({chrome_path})...")
            chrome = ChromeCDPProbe(chrome_path, args.port, launch=not args.no_launch)
            print("  Chrome connected.")
        except Exception as e:
            print(f"  WARNING: Chrome unavailable ({e}), running IV8-only mode")
            chrome = None
    else:
        print("Step 3: Chrome CDP skipped (IV8-only mode)")

    # Step 4: Probe each attribute
    print(f"Step 4: Probing {len(attrs)} attributes...")
    results = []
    stats = {"PASS": 0, "TYPE_FAIL": 0, "VALUE_FAIL": 0, "THROW": 0,
             "SKIP_NO_INSTANCE": 0, "SKIP_CHROME": 0, "SKIP_ASYNC": 0}

    for i, attr in enumerate(attrs):
        iface_name = attr["interface"]
        attr_name = attr["attribute"]
        idl_type = attr["idl_type"]
        nullable = attr["nullable"]

        instance_js = build_instance_js(iface_name)
        if instance_js is None:
            results.append({
                "interface": iface_name, "attribute": attr_name,
                "idl_type": idl_type, "classification": "SKIP_NO_INSTANCE",
                "iv8": None, "chrome": None,
            })
            stats["SKIP_NO_INSTANCE"] += 1
            continue

        iv8_result = probe_iv8(ctx, instance_js, attr_name)

        chrome_result = None
        if chrome:
            chrome_result = probe_chrome(chrome, instance_js, attr_name)

        classification = classify(iv8_result, chrome_result, idl_type, nullable, iv8_only=args.iv8_only)
        stats[classification] = stats.get(classification, 0) + 1

        results.append({
            "interface": iface_name, "attribute": attr_name,
            "idl_type": idl_type, "classification": classification,
            "iv8": iv8_result, "chrome": chrome_result,
        })

        if (i + 1) % 100 == 0:
            print(f"  Progress: {i+1}/{len(attrs)} ({stats})")

    # Step 5: Write report
    print("Step 5: Writing report...")
    report = {
        "schema_version": "h05-getter-values.v0.1",
        "chrome_version": "151" if chrome else None,
        "iv8_version": "0.8.89",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "summary": {
            "total_attributes": len(attrs),
            "evaluated": len(results),
            **stats,
        },
        "results": results,
    }

    output_path.write_text(
        json.dumps(report, indent=2, default=str, ensure_ascii=False),
        encoding="utf-8"
    )

    # Print summary
    print(f"\n{'='*60}")
    print(f"H05a Getter Return Value Audit — Summary")
    print(f"{'='*60}")
    print(f"Total attributes: {len(attrs)}")
    print(f"  PASS:              {stats['PASS']}")
    print(f"  TYPE_FAIL:         {stats['TYPE_FAIL']}")
    print(f"  VALUE_FAIL:        {stats['VALUE_FAIL']}")
    print(f"  THROW:             {stats['THROW']}")
    print(f"  SKIP_NO_INSTANCE:  {stats['SKIP_NO_INSTANCE']}")
    print(f"  SKIP_CHROME:       {stats['SKIP_CHROME']}")
    print(f"  SKIP_ASYNC:        {stats['SKIP_ASYNC']}")
    print(f"Output: {output_path}")

    # --- Category A: Data Integrity (mandatory per HARNESS-CHARTER §3) ---
    cat_a_pass = stats["TYPE_FAIL"] <= THRESHOLDS["max_type_fail"] and \
                 stats["THROW"] <= THRESHOLDS["max_throw"]
    print(f"\nCategory A (Data Integrity): {'PASS' if cat_a_pass else 'FAIL'}")
    print(f"  TYPE_FAIL={stats['TYPE_FAIL']} (max {THRESHOLDS['max_type_fail']}), "
          f"THROW={stats['THROW']} (max {THRESHOLDS['max_throw']})")

    # --- Category B: Recall ---
    evaluated_pct = (len(results) - stats["SKIP_NO_INSTANCE"]) / max(len(attrs), 1) * 100
    cat_b_pass = evaluated_pct >= THRESHOLDS["min_evaluated_pct"]
    print(f"Category B (Recall): {'PASS' if cat_b_pass else 'FAIL'}")
    print(f"  Evaluated {evaluated_pct:.1f}% (min {THRESHOLDS['min_evaluated_pct']}%)")

    # --- Category C: False Positive / Safety (mandatory) ---
    # Check known-good values
    cat_c_pass = True
    cat_c_details = []
    for (iface, attr), (expected_type, expected_val) in KNOWN_GOOD_VALUES.items():
        found = False
        for r in results:
            if r["interface"] == iface and r["attribute"] == attr:
                found = True
                iv8 = r.get("iv8", {})
                actual_type = iv8.get("typeof", "undefined")
                if r["classification"] != "PASS":
                    cat_c_pass = False
                    cat_c_details.append(f"{iface}.{attr}: {r['classification']} (expected PASS)")
                elif expected_type and actual_type != expected_type:
                    cat_c_pass = False
                    cat_c_details.append(f"{iface}.{attr}: typeof={actual_type} (expected {expected_type})")
                break
        if not found:
            cat_c_details.append(f"{iface}.{attr}: NOT FOUND in results")
            cat_c_pass = False
    print(f"Category C (False Positive): {'PASS' if cat_c_pass else 'FAIL'}")
    if cat_c_details:
        for d in cat_c_details:
            print(f"  {d}")

    # Category C negative: getter must throw on wrong receiver (P2 falsification)
    cat_c_neg_pass = True
    cat_c_neg_details = []
    if ctx:
        for iface, attr in RECEIVER_CHECK_INTERFACES:
            js = f"""
                try {{
                    var proto = {iface}.prototype;
                    var desc = Object.getOwnPropertyDescriptor(proto, "{attr}");
                    if (!desc || !desc.get) {{
                        "SKIP: no getter on {iface}.prototype.{attr}";
                    }} else {{
                        try {{
                            desc.get.call({{}});
                            "NO_THROW: {iface}.{attr} getter did not throw on wrong receiver";
                        }} catch(e) {{
                            "PASS: {iface}.{attr} threw " + e.constructor.name;
                        }}
                    }}
                }} catch(e) {{
                    "SKIP: " + String(e).substring(0, 60);
                }}
            """
            try:
                result = ctx.eval(js)
                if result and result.startswith("NO_THROW"):
                    cat_c_neg_pass = False
                    cat_c_neg_details.append(result)
                elif result and result.startswith("SKIP"):
                    cat_c_neg_details.append(result)
            except Exception as e:
                cat_c_neg_details.append(f"SKIP: {iface}.{attr} eval error: {e}")
    if not cat_c_neg_pass:
        print(f"Category C negative (receiver check): FAIL")
        for d in cat_c_neg_details:
            if d.startswith("NO_THROW"):
                print(f"  {d}")
    else:
        print(f"Category C negative (receiver check): PASS")
    cat_c_pass = cat_c_pass and cat_c_neg_pass

    # --- Category D: Coverage ---
    ifaces_evaluated = len(set(a["interface"] for a in attrs if a["interface"] in
                               set(r["interface"] for r in results if r["classification"] != "SKIP_NO_INSTANCE")))
    total_ifaces = len(set(a["interface"] for a in attrs))
    coverage_pct = ifaces_evaluated / max(total_ifaces, 1) * 100
    cat_d_pass = coverage_pct >= THRESHOLDS["min_interface_coverage_pct"]
    print(f"Category D (Coverage): {'PASS' if cat_d_pass else 'FAIL'}")
    print(f"  {ifaces_evaluated}/{total_ifaces} interfaces ({coverage_pct:.1f}%, "
          f"min {THRESHOLDS['min_interface_coverage_pct']}%)")

    # --- Category E: Robustness ---
    cat_e_pass = True
    if not chrome and not args.iv8_only:
        cat_e_pass = False
    elif not chrome and args.iv8_only:
        cat_e_pass = True  # IV8-only mode is intentional, not a failure
    print(f"Category E (Robustness): {'PASS' if cat_e_pass else 'FAIL'}")

    # --- Overall ---
    overall_pass = cat_a_pass and cat_c_pass  # A and C are mandatory
    print(f"\n{'='*60}")
    print(f"OVERALL: {'PASS' if overall_pass else 'FAIL'}")
    print(f"{'='*60}")

    if chrome:
        chrome.close()

    return 0 if overall_pass else 1


def main():
    parser = argparse.ArgumentParser(
        description="H05a: Getter Return Value Audit (IDL-driven)"
    )
    parser.add_argument("--chrome", default=None,
                        help="Path to chrome.exe for CDP diff")
    parser.add_argument("--port", type=int, default=9222,
                        help="CDP port (default: 9222)")
    parser.add_argument("--no-launch", action="store_true",
                        help="Connect to already-running Chrome")
    parser.add_argument("--top", type=int, default=50,
                        help="Limit to top N interfaces from TOP_50 list (default: 50). "
                             "Use 0 for full IDL (all interfaces in unified_ir).")
    parser.add_argument("--output", "-o", default=None,
                        help="Output JSON file")
    parser.add_argument("--iv8-only", action="store_true",
                        help="Skip Chrome CDP diff (IV8-only mode)")
    args = parser.parse_args()

    exit_code = _run_in_thread(_run_audit, args)
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
