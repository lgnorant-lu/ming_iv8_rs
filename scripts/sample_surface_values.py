#!/usr/bin/env python3
"""L1: CDP value + descriptor sampler for Chrome and IV8.

Samples property values AND property descriptors for key browser interfaces
from either real Chrome (via CDP) or the IV8 runtime, producing a JSON file
suitable for descriptor-level diffing.

Usage:
  # IV8 mode (no Chrome required)
  .venv\\Scripts\\python.exe scripts/sample_surface_values.py --source iv8

  # Chrome mode (launches Chrome, connects via CDP)
  .venv\\Scripts\\python.exe scripts/sample_surface_values.py --source chrome \\
      --chrome "D:\\Download\\Softwares\\chromium-debug\\chrome.exe"

  # Chrome mode (connect to already-running Chrome on port 9222)
  .venv\\Scripts\\python.exe scripts/sample_surface_values.py --source chrome \\
      --port 9222 --no-launch

Output:
  data/surface_values_{source}.json
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
DATA_DIR = REPO_ROOT / "data"

DEFAULT_CHROME_PATH = r"D:\Download\Softwares\chromium-debug\chrome.exe"


# ---------------------------------------------------------------------------
# JS collector — runs identically in Chrome and IV8
# ---------------------------------------------------------------------------

COLLECTOR_JS = r"""
(function() {
    var MAX_OBJECT_KEYS = 30;

    function findDescriptor(obj, prop) {
        var cur = obj;
        var depth = 0;
        while (cur !== null && cur !== undefined && depth < 10) {
            var desc = Object.getOwnPropertyDescriptor(cur, prop);
            if (desc) {
                return {
                    location: depth === 0 ? 'own' : 'prototype',
                    depth: depth,
                    hasGet: 'get' in desc,
                    hasSet: 'set' in desc,
                    hasValue: 'value' in desc,
                    hasWritable: 'writable' in desc,
                    writable: 'writable' in desc ? desc.writable : null,
                    enumerable: desc.enumerable,
                    configurable: desc.configurable
                };
            }
            cur = Object.getPrototypeOf(cur);
            depth++;
        }
        return null;
    }

    function sampleValue(obj, prop) {
        var info = { typeof: 'undefined', value: null, isObject: false, objectKeys: null, isError: false, error: null };
        try {
            var v = obj[prop];
            var t = typeof v;
            info.typeof = t;
            if (v === null) {
                info.value = null;
            } else if (t === 'string' || t === 'number' || t === 'boolean') {
                info.value = v;
            } else if (t === 'function') {
                info.value = '[function]';
            } else if (t === 'object') {
                info.isObject = true;
                try {
                    var keys = Object.keys(v);
                    info.objectKeys = keys.slice(0, MAX_OBJECT_KEYS);
                    info.value = '[object:' + keys.length + ' keys]';
                } catch(e) {
                    info.value = '[object:keys-throws]';
                }
            } else if (t === 'symbol') {
                info.value = '[symbol]';
            } else if (t === 'bigint') {
                info.value = String(v);
            } else {
                info.value = String(v);
            }
        } catch(e) {
            info.isError = true;
            info.error = e.message;
        }
        return info;
    }

    function sampleObject(obj, propNames) {
        var result = {};
        for (var i = 0; i < propNames.length; i++) {
            var prop = propNames[i];
            if (prop === 'constructor' || prop === '__proto__') continue;
            var entry = {
                value: sampleValue(obj, prop),
                descriptor: findDescriptor(obj, prop)
            };
            result[prop] = entry;
        }
        return result;
    }

    function getPropNames(obj) {
        // Walk instance + interface prototype only. Stop before EventTarget /
        // Object so inherited addEventListener is not attributed as Screen EXTRA
        // (Chrome Screen extends EventTarget; methods live on EventTarget.prototype).
        var seen = {};
        var names = [];
        var cur = obj;
        var depth = 0;
        var stopAt = [];
        try {
            if (typeof EventTarget !== 'undefined' && EventTarget.prototype) {
                stopAt.push(EventTarget.prototype);
            }
            stopAt.push(Object.prototype);
            if (typeof Function !== 'undefined' && Function.prototype) {
                stopAt.push(Function.prototype);
            }
        } catch (eStop) {}
        while (cur !== null && cur !== undefined && depth < 4) {
            var hitStop = false;
            for (var si = 0; si < stopAt.length; si++) {
                if (cur === stopAt[si]) { hitStop = true; break; }
            }
            if (hitStop) break;
            try {
                var own = Object.getOwnPropertyNames(cur);
                for (var i = 0; i < own.length; i++) {
                    if (!seen[own[i]]) {
                        seen[own[i]] = true;
                        names.push(own[i]);
                    }
                }
            } catch(e) {}
            cur = Object.getPrototypeOf(cur);
            depth++;
        }
        return names;
    }

    var output = {};

    // ---- Navigator ----
    try {
        var navNames = getPropNames(navigator);
        output['Navigator'] = sampleObject(navigator, navNames);
    } catch(e) {
        output['Navigator'] = { __error: e.message };
    }

    // ---- Screen ----
    try {
        var scrNames = getPropNames(screen);
        output['Screen'] = sampleObject(screen, scrNames);
    } catch(e) {
        output['Screen'] = { __error: e.message };
    }

    // ---- Window ----
    try {
        var winNames = getPropNames(window);
        winNames = winNames.filter(function(n) {
            return n !== 'window' && n !== 'self' && n !== 'top' && n !== 'parent' && n !== 'frames' && n !== 'navigator' && n !== 'screen' && n !== 'document' && n !== 'location' && n !== 'history';
        });
        output['Window'] = sampleObject(window, winNames);
    } catch(e) {
        output['Window'] = { __error: e.message };
    }

    // ---- Document ----
    try {
        var docNames = getPropNames(document);
        output['Document'] = sampleObject(document, docNames);
    } catch(e) {
        output['Document'] = { __error: e.message };
    }

    // ---- WebGLRenderingContext ----
    try {
        var canvas = document.createElement('canvas');
        var gl = canvas.getContext('webgl');
        if (gl) {
            var glNames = getPropNames(gl);
            output['WebGLRenderingContext'] = sampleObject(gl, glNames);
        } else {
            output['WebGLRenderingContext'] = { __error: 'getContext returned null' };
        }
    } catch(e) {
        output['WebGLRenderingContext'] = { __error: e.message };
    }

    // ---- AudioContext ----
    try {
        var actx = new AudioContext();
        var aNames = getPropNames(actx);
        output['AudioContext'] = sampleObject(actx, aNames);
        try { actx.close(); } catch(e) {}
    } catch(e) {
        output['AudioContext'] = { __error: e.message };
    }

    // ---- Permissions ----
    try {
        var perms = navigator.permissions;
        if (perms) {
            var permNames = getPropNames(perms);
            output['Permissions'] = sampleObject(perms, permNames);
        } else {
            output['Permissions'] = { __error: 'navigator.permissions is null' };
        }
    } catch(e) {
        output['Permissions'] = { __error: e.message };
    }

    return JSON.stringify(output);
})()
"""


# ---------------------------------------------------------------------------
# Minimal WebSocket client (RFC 6455, no deps)
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


# ---------------------------------------------------------------------------
# Chrome sampling via CDP
# ---------------------------------------------------------------------------

def sample_chrome(chrome_path: str | None, port: int, launch: bool) -> dict | None:
    """Launch Chrome (if requested), connect via CDP, collect surface values."""
    proc = None
    ws = None

    if launch:
        if not chrome_path or not Path(chrome_path).exists():
            print(f"ERROR: Chrome not found: {chrome_path}")
            return None

        user_data_dir = tempfile.mkdtemp(prefix="chrome_surface_")
        print(f"Launching Chrome: {chrome_path}")
        proc = subprocess.Popen(
            [
                chrome_path,
                f"--remote-debugging-port={port}",
                f"--user-data-dir={user_data_dir}",
                "--no-first-run",
                "--no-default-browser-check",
                "--disable-popup-blocking",
                "about:blank",
            ],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        time.sleep(3)

    try:
        resp = urllib.request.urlopen(f"http://127.0.0.1:{port}/json", timeout=5)
        targets = json.loads(resp.read())
        page_target = None
        for t in targets:
            if t.get("type") == "page":
                page_target = t
                break
        if not page_target:
            print("ERROR: No page target found")
            return None
        ws_url = page_target["webSocketDebuggerUrl"]
    except Exception as e:
        print(f"ERROR: Failed to connect to Chrome on port {port}: {e}")
        return None

    parsed = urlparse(ws_url)
    ws_host = parsed.hostname or "127.0.0.1"
    ws_port = parsed.port or port
    ws_path = parsed.path

    ws = WSClient(ws_host, ws_port, ws_path)

    msg_id = 1
    ws.send({
        "id": msg_id,
        "method": "Runtime.evaluate",
        "params": {"expression": COLLECTOR_JS, "returnByValue": True},
    })

    timeout = time.time() + 15
    while time.time() < timeout:
        resp = ws.recv()
        if resp.get("id") == msg_id:
            result = resp.get("result", {})
            if "exceptionDetails" in result:
                print(f"ERROR: JS evaluation failed: {result['exceptionDetails']}")
                return None
            value = result.get("result", {}).get("value")
            if value:
                data = json.loads(value) if isinstance(value, str) else value
                return data
            print(f"ERROR: No value in response: {resp}")
            return None

    print("ERROR: Timeout waiting for CDP response")
    return None


# ---------------------------------------------------------------------------
# IV8 sampling
# ---------------------------------------------------------------------------

def sample_iv8() -> dict:
    """Start IV8 runtime, collect surface values.

    JSContext + large DOM/codegen templates need a 128MB stack (K-010).
    Run construction/eval on a dedicated thread so Windows default stacks
    do not STATUS_STACK_OVERFLOW mid-sample.
    """
    import threading

    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext

    result: dict = {}
    error: list[BaseException] = []

    def worker():
        try:
            print("Initializing IV8 runtime...")
            ctx = JSContext()
            ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)
            print("Sampling surface values + descriptors...")
            raw = ctx.eval(COLLECTOR_JS)
            if isinstance(raw, str):
                result.update(json.loads(raw))
            elif isinstance(raw, dict):
                result.update(raw)
            else:
                raise TypeError(f"unexpected sample type: {type(raw)}")
        except BaseException as e:
            error.append(e)

    t = threading.Thread(target=worker)
    t.start()
    t.join()
    if error:
        raise error[0]
    return result


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(
        description="L1: CDP value + descriptor sampler for Chrome and IV8"
    )
    parser.add_argument(
        "--source",
        choices=["chrome", "iv8"],
        required=True,
        help="Data source to sample from",
    )
    parser.add_argument(
        "--output",
        "-o",
        default=None,
        help="Output JSON file (default: data/surface_values_{source}.json)",
    )
    parser.add_argument(
        "--chrome",
        default=DEFAULT_CHROME_PATH,
        help=f"Path to chrome.exe (default: {DEFAULT_CHROME_PATH})",
    )
    parser.add_argument(
        "--port",
        type=int,
        default=9222,
        help="CDP remote debugging port (default: 9222)",
    )
    parser.add_argument(
        "--no-launch",
        action="store_true",
        help="Do not launch Chrome; connect to an already-running instance",
    )
    args = parser.parse_args()

    output_path = Path(args.output) if args.output else DATA_DIR / f"surface_values_{args.source}.json"

    if args.source == "chrome":
        data = sample_chrome(args.chrome, args.port, launch=not args.no_launch)
        if data is None:
            sys.exit(1)
    else:
        data = sample_iv8()

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(data, indent=2, default=str, ensure_ascii=False), encoding="utf-8")

    interfaces = list(data.keys())
    total_props = 0
    for iface_name, iface_data in data.items():
        if isinstance(iface_data, dict) and "__error" not in iface_data:
            total_props += len(iface_data)

    print(f"Written to {output_path}")
    print(f"Interfaces: {interfaces}")
    print(f"Total properties sampled: {total_props}")


if __name__ == "__main__":
    main()
