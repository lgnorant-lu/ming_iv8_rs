#!/usr/bin/env python3
"""IDL coverage audit: compare bcd-collector (Chrome 148) interface/member
existence against the IV8 runtime surface.

Produces data/idl_coverage_report.json with three interface-level categories:
  - MISSING : bcd-collector reports the interface present (Window, result=true)
              but IV8 does not expose it as a global function.
  - EXTRA   : IV8 exposes the interface but bcd-collector does not list it
              (includes JS builtins like Object/Array which are expected extras).
  - MATCH   : Both bcd-collector and IV8 expose the interface.

For MATCH interfaces, member-level coverage is computed:
  bcd depth-2 members (api.Interface.Member) vs
  IV8 Object.getOwnPropertyNames(Interface.prototype).

Usage:
  .venv\\Scripts\\python.exe scripts/idl_coverage_audit.py

Output:
  data/idl_coverage_report.json
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = REPO_ROOT / "data"

BCD_FULL = DATA_DIR / "bcd-chrome148.json"
BCD_IFACES = DATA_DIR / "bcd-chrome148-interfaces.json"
OUT_PATH = DATA_DIR / "idl_coverage_report.json"

# P0 interfaces for detailed member-level reporting.
P0_INTERFACES = {"Navigator", "Window", "Document", "Screen"}

# JS builtins and non-API globals that IV8 legitimately exposes but bcd
# does not track (these are expected EXTRA entries, not coverage gaps).
JS_BUILTIN_GLOBALS = frozenset({
    "Object", "Function", "Array", "Number", "Boolean", "String", "Symbol",
    "Date", "Promise", "RegExp", "Error", "AggregateError", "EvalError",
    "RangeError", "ReferenceError", "SyntaxError", "TypeError", "URIError",
    "ArrayBuffer", "Uint8Array", "Uint8ClampedArray", "Int8Array",
    "Uint16Array", "Int16Array", "Uint32Array", "Int32Array",
    "Float16Array", "Float32Array", "Float64Array", "BigInt64Array",
    "BigUint64Array", "DataView", "Map", "Set", "WeakMap", "WeakSet",
    "WeakRef", "FinalizationRegistry", "Iterator", "AsyncIterator",
    "Generator", "AsyncGenerator", "GeneratorFunction", "AsyncGeneratorFunction",
    "BigInt", "Math", "JSON", "Reflect", "Proxy", "Intl", "Atomics",
    "globalThis", "Infinity", "NaN", "undefined", "eval", "isFinite",
    "isNaN", "parseFloat", "parseInt", "decodeURI", "decodeURIComponent",
    "encodeURI", "encodeURIComponent", "escape", "unescape",
    "Boolean", "Number", "String", "Array", "Object",
    "SharedArrayBuffer", "WebAssembly", "queueMicrotask",
    "structuredClone", "setTimeout", "clearTimeout", "setInterval",
    "clearInterval", "console", "reportError", "createImageBitmap",
    "crossOriginIsolated", "isSecureContext", "origin",
    "name", "length", "top", "parent", "self", "window", "frames",
    "closed", "opener", "status", "frameElement",
    "crypto",
})

# JS engine builtins that appear on window but are not Web API interfaces.
JS_BUILTIN_PREFIXES = frozenset({
    "WebKit",  # WebKit-specific testing stubs
})


def load_bcd() -> tuple[set[str], dict[str, set[str]]]:
    """Load bcd data. Returns (interface_names, interface->members).

    Only Window exposure with result=True is counted.
    Interface names have the 'api.' prefix stripped.
    Members are depth-2 (api.Interface.Member), depth-3+ sub-features excluded.
    """
    with open(BCD_FULL, encoding="utf-8") as f:
        data = json.load(f)
    entries = data["entries"]

    interfaces: set[str] = set()
    members: dict[str, set[str]] = {}

    for entry in entries:
        if entry.get("exposure") != "Window":
            continue
        if not entry.get("result"):
            continue
        name: str = entry["name"]
        if not name.startswith("api."):
            continue
        rest = name[4:]
        parts = rest.split(".")
        if len(parts) == 1:
            interfaces.add(parts[0])
        elif len(parts) == 2:
            iface, member = parts[0], parts[1]
            interfaces.add(iface)  # ensure interface is registered
            members.setdefault(iface, set()).add(member)
        # depth 3+ are sub-features (params, options) — skipped for L0

    return interfaces, members


def load_bcd_interface_list() -> set[str]:
    """Load the interface-only bcd file for the total count."""
    with open(BCD_IFACES, encoding="utf-8") as f:
        data = json.load(f)
    entries = data["entries"]
    return {
        e["name"][4:]
        for e in entries
        if e.get("exposure") == "Window" and e.get("result") and e["name"].startswith("api.")
    }


# JS probe: enumerate window globals, collect function-typed properties
# (interface constructors) and their prototype members.
PROBE_JS = r"""
(function() {
    var names = Object.getOwnPropertyNames(window);
    var interfaces = {};
    var nonFunction = [];
    for (var i = 0; i < names.length; i++) {
        var n = names[i];
        var val;
        try { val = window[n]; } catch(e) { nonFunction.push(n); continue; }
        if (typeof val !== 'function') {
            nonFunction.push(n);
            continue;
        }
        var members = [];
        var proto = null;
        try { proto = val.prototype; } catch(e) {}
        if (proto) {
            try { members = Object.getOwnPropertyNames(proto); } catch(e) {}
        }
        // Filter out 'constructor' from members (it's the function itself)
        members = members.filter(function(m){ return m !== 'constructor'; });
        interfaces[n] = members;
    }
    return JSON.stringify({interfaces: interfaces, nonFunction: nonFunction});
})()
"""


def probe_iv8() -> tuple[dict[str, list[str]], list[str]]:
    """Probe the IV8 runtime for global interfaces and their prototype members."""
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext

    print("Initializing IV8 runtime...")
    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)
    print("Probing global interfaces...")

    raw = ctx.eval(PROBE_JS)
    data = json.loads(raw)
    return data["interfaces"], data["nonFunction"]


def is_expected_extra(name: str) -> bool:
    """Check if an EXTRA interface is a known JS builtin (expected, not a gap)."""
    if name in JS_BUILTIN_GLOBALS:
        return True
    for prefix in JS_BUILTIN_PREFIXES:
        if name.startswith(prefix):
            return True
    return False


def main() -> int:
    # ── Load bcd ──
    print("Loading bcd-collector data...")
    bcd_ifaces_all, bcd_members = load_bcd()
    bcd_iface_list = load_bcd_interface_list()
    print(f"  bcd interfaces (Window, result=true): {len(bcd_ifaces_all)}")
    print(f"  bcd interface-file total (Window): {len(bcd_iface_list)}")

    # ── Probe IV8 ──
    iv8_interfaces_raw, iv8_nonfunc = probe_iv8()
    iv8_ifaces = set(iv8_interfaces_raw.keys())
    print(f"  IV8 global functions (interface candidates): {len(iv8_ifaces)}")
    print(f"  IV8 non-function globals: {len(iv8_nonfunc)}")

    # ── Interface-level diff ──
    missing_in_iv8 = sorted(bcd_ifaces_all - iv8_ifaces)
    extra_in_iv8 = sorted(iv8_ifaces - bcd_ifaces_all)
    matched = sorted(bcd_ifaces_all & iv8_ifaces)

    extra_expected = [n for n in extra_in_iv8 if is_expected_extra(n)]
    extra_unexpected = [n for n in extra_in_iv8 if not is_expected_extra(n)]

    # ── Member-level coverage for matched interfaces ──
    member_coverage: dict[str, dict] = {}
    coverage_pcts: list[float] = []

    for iface in matched:
        bcd_mem = bcd_members.get(iface, set())
        iv8_mem = set(iv8_interfaces_raw.get(iface, []))
        # Remove constructor from IV8 members (already filtered in JS, but be safe)
        iv8_mem.discard("constructor")

        mem_missing = sorted(bcd_mem - iv8_mem)
        mem_extra = sorted(iv8_mem - bcd_mem)
        mem_matched = sorted(bcd_mem & iv8_mem)

        if bcd_mem:
            pct = round(len(mem_matched) / len(bcd_mem) * 100, 1)
        else:
            pct = 100.0 if not iv8_mem else 100.0  # no bcd members to compare

        coverage_pcts.append(pct)
        member_coverage[iface] = {
            "bcd_members": len(bcd_mem),
            "iv8_members": len(iv8_mem),
            "matched": len(mem_matched),
            "missing": mem_missing,
            "extra": mem_extra,
            "coverage_pct": pct,
        }

    avg_coverage = (
        round(sum(coverage_pcts) / len(coverage_pcts), 1) if coverage_pcts else 0.0
    )

    # ── P0 interface details ──
    p0_details = {}
    for iface in sorted(P0_INTERFACES):
        if iface in member_coverage:
            p0_details[iface] = member_coverage[iface]

    # ── Build report ──
    report = {
        "metadata": {
            "generated_at": __import__("datetime").datetime.now().isoformat(),
            "bcd_source": str(BCD_FULL.name),
            "bcd_chrome_version": json.load(open(BCD_FULL, encoding="utf-8")).get("chrome_version"),
            "bcd_interfaces_total": len(bcd_iface_list),
            "bcd_interfaces_present": len(bcd_ifaces_all),
            "iv8_interfaces_total": len(iv8_ifaces),
            "iv8_non_function_globals": len(iv8_nonfunc),
        },
        "summary": {
            "missing_in_iv8": len(missing_in_iv8),
            "extra_in_iv8": len(extra_in_iv8),
            "extra_expected_js_builtins": len(extra_expected),
            "extra_unexpected": len(extra_unexpected),
            "matched": len(matched),
            "avg_member_coverage_pct": avg_coverage,
        },
        "missing_in_iv8": missing_in_iv8,
        "extra_in_iv8": {
            "expected_js_builtins": extra_expected,
            "unexpected": extra_unexpected,
        },
        "matched": matched,
        "p0_interface_details": p0_details,
        "member_coverage": member_coverage,
    }

    DATA_DIR.mkdir(parents=True, exist_ok=True)
    OUT_PATH.write_text(
        json.dumps(report, indent=2, ensure_ascii=False),
        encoding="utf-8",
    )

    # ── Console summary ──
    print()
    print("=" * 72)
    print(" IDL COVERAGE AUDIT (L0)")
    print("=" * 72)
    print(f"  bcd interfaces (Window, present) : {len(bcd_ifaces_all)}")
    print(f"  IV8 interfaces (global functions): {len(iv8_ifaces)}")
    print()
    print(f"  MISSING in IV8  : {len(missing_in_iv8)}")
    print(f"  EXTRA in IV8    : {len(extra_in_iv8)}  "
          f"(expected JS builtins: {len(extra_expected)}, "
          f"unexpected: {len(extra_unexpected)})")
    print(f"  MATCHED         : {len(matched)}")
    print(f"  Avg member coverage : {avg_coverage}%")
    print()

    print("-" * 72)
    print(" P0 Interface Member Coverage")
    print("-" * 72)
    for iface in sorted(P0_INTERFACES):
        if iface in member_coverage:
            mc = member_coverage[iface]
            print(f"  {iface:20s}  bcd={mc['bcd_members']:3d}  "
                  f"iv8={mc['iv8_members']:3d}  "
                  f"missing={len(mc['missing']):3d}  "
                  f"extra={len(mc['extra']):3d}  "
                  f"coverage={mc['coverage_pct']}%")
            if mc["missing"]:
                print(f"    missing members: {', '.join(mc['missing'][:20])}"
                      + (f" ... (+{len(mc['missing'])-20})" if len(mc['missing'])>20 else ""))
        else:
            status = "MISSING from IV8" if iface in missing_in_iv8 else "not in bcd"
            print(f"  {iface:20s}  [{status}]")
    print()

    if missing_in_iv8:
        print("-" * 72)
        print(f" Top MISSING interfaces (bcd has, IV8 doesn't) — {len(missing_in_iv8)} total:")
        print("-" * 72)
        for name in missing_in_iv8[:40]:
            print(f"  {name}")
        if len(missing_in_iv8) > 40:
            print(f"  ... and {len(missing_in_iv8) - 40} more")
        print()

    if extra_unexpected:
        print("-" * 72)
        print(f" Unexpected EXTRA interfaces (IV8 has, bcd doesn't) — {len(extra_unexpected)} total:")
        print("-" * 72)
        for name in extra_unexpected[:40]:
            print(f"  {name}")
        if len(extra_unexpected) > 40:
            print(f"  ... and {len(extra_unexpected) - 40} more")
        print()

    print(f"Report written to: {OUT_PATH}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
