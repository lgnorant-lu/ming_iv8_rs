#!/usr/bin/env python3
"""Regenerate probe coverage matrix vs unified_ir.json (v0.8.98 S6).

Usage:
  uv run python tools/idl_probe/coverage_report.py
  uv run python tools/idl_probe/coverage_report.py --out path/to/matrix.json

Does not implement full-1284 default pack; reports tiers only.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import sys

_ROOT = Path(__file__).resolve().parents[2]
if str(_ROOT) not in sys.path:
    sys.path.insert(0, str(_ROOT))

from tools.idl_probe.generate_probe_pack import generate_probe_pack  # noqa: E402
_IR = _ROOT / "tools" / "idl" / "output" / "unified_ir.json"
_DEFAULT_OUT = (
    _ROOT
    / "docs"
    / "roadmap"
    / "v0.8"
    / "native-substrate"
    / "v0.8.98-probe-coverage-matrix.json"
)

P0 = [
    "Navigator",
    "Screen",
    "Window",
    "Document",
    "Location",
    "Performance",
    "Storage",
    "HTMLCanvasElement",
    "CanvasRenderingContext2D",
    "WebGLRenderingContext",
    "WebGL2RenderingContext",
    "AudioContext",
    "OfflineAudioContext",
    "BaseAudioContext",
    "Crypto",
    "SubtleCrypto",
    "NavigatorUAData",
    "PluginArray",
    "MimeTypeArray",
    "XMLHttpRequest",
    "WebSocket",
    "Worker",
    "WorkerNavigator",
    "Permissions",
    "Geolocation",
    "BatteryManager",
    "NetworkInformation",
    "OffscreenCanvas",
    "GPU",
]

P1 = [
    "Notification",
    "SpeechSynthesis",
    "Bluetooth",
    "USB",
    "HID",
    "Serial",
    "XRSystem",
    "MediaDevices",
    "MediaStream",
    "RTCPeerConnection",
    "AnalyserNode",
    "OscillatorNode",
    "GainNode",
    "DynamicsCompressorNode",
    "AudioBuffer",
    "FontFace",
    "CSSStyleDeclaration",
    "DOMRect",
    "IntersectionObserver",
    "MutationObserver",
    "ResizeObserver",
    "PerformanceObserver",
    "Intl",
    "TextEncoder",
    "TextDecoder",
    "URL",
    "URLSearchParams",
    "Headers",
    "Request",
    "Response",
    "AbortController",
    "ReadableStream",
    "WritableStream",
    "TransformStream",
]


def build_report() -> dict:
    ir = json.loads(_IR.read_text(encoding="utf-8"))
    ir_ifaces = sorted(
        {
            d["name"]
            for d in ir.get("definitions", [])
            if d.get("kind") == "interface" and d.get("name")
        }
    )
    pack = generate_probe_pack()
    exists = sorted(
        {
            p["probe_id"].split(".")[-1]
            for p in pack["probes"]
            if p["probe_id"].startswith("idl.exists.")
        }
    )
    exists_set = set(exists)
    missing = [n for n in ir_ifaces if n not in exists_set]

    def tier_map(names: list[str]) -> dict[str, str]:
        out: dict[str, str] = {}
        for n in names:
            if n in exists_set:
                out[n] = "IN_PACK"
            elif n in ir_ifaces:
                out[n] = "IN_IR_MISSING_PACK"
            else:
                out[n] = "NOT_IN_IR"
        return out

    return {
        "schema": "iv8-probe-coverage-matrix.v0.1",
        "ir_interfaces": len(ir_ifaces),
        "default_pack_interfaces": len(exists),
        "default_pack_probes": len(pack["probes"]),
        "missing_from_default_pack": len(missing),
        "coverage_ratio": round(len(exists) / max(len(ir_ifaces), 1), 4),
        "p0_high_signal": tier_map(P0),
        "p1_medium_signal": tier_map(P1),
        "missing_sample_first_80": missing[:80],
        "codegen_note": "L-codegen ~1284 IR templates via install_all; pack is tiered",
        "policy": "default pack is NOT required to equal IR 1284; use P0/P1 + on-demand generate",
    }


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, default=_DEFAULT_OUT)
    args = ap.parse_args()
    report = build_report()
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(report, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
    print(
        f"wrote {args.out} ifaces={report['default_pack_interfaces']} "
        f"probes={report['default_pack_probes']} ratio={report['coverage_ratio']}"
    )


if __name__ == "__main__":
    main()
