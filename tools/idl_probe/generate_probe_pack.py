"""IDL probe pack generator -- reads unified_ir.json, emits ProbePack JSON.

v0.8.33 Slice 1: minimal generated probe pack for Window, Navigator,
Screen, and Location existence probes.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

_UNIFIED_IR_PATH = Path(__file__).parent.parent / "idl" / "output" / "unified_ir.json"

def generate_probe_pack(
    ir_path: str | Path | None = None,
    interfaces: list[str] | None = None,
    version: int = 1,
) -> dict[str, Any]:
    """Generate a ProbePack dict from unified_ir.json for the given interfaces.

    Args:
        ir_path: Path to unified_ir.json. Defaults to tools/idl/output/unified_ir.json.
        interfaces: Interface names to generate probes for.
                    Defaults to [Window, Navigator, Screen, Location].
        version: ProbePack version number.

    Returns:
        A dict matching the ProbePack schema recognized by ProbePack.from_dict.
    """
    if interfaces is None:
        interfaces = ["Window", "Navigator", "Screen", "Location"]

    source_path = Path(ir_path) if ir_path else _UNIFIED_IR_PATH
    ir_data = _load_ir(source_path)
    definitions = ir_data.get("definitions", [])
    ir_meta = ir_data.get("metadata", {})

    interface_map: dict[str, dict[str, Any]] = {}
    for defn in definitions:
        name = defn.get("name")
        if name and defn.get("kind") == "interface" and name in set(interfaces):
            interface_map[name] = defn

    probes: list[dict[str, Any]] = []

    probes.append({
        "probe_id": "idl.meta.ir_version",
        "target": "__idl_ir__",
        "category": "presence",
        "js": "return true;",
        "expected": True,
        "gap_class": "missing_interface",
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
    })

    for iface_name in interfaces:
        iface = interface_map.get(iface_name)
        if iface is None:
            probes.append({
                "probe_id": f"idl.exists.{iface_name}",
                "target": iface_name,
                "category": "presence",
                "js": f"return typeof {iface_name} !== 'undefined';",
                "expected": False,
                "gap_class": "missing_interface",
                "side_effects": [],
                "cleanup": "none",
                "evidence_ceiling": "diagnostic_only",
            })
            continue

        probes.append({
            "probe_id": f"idl.exists.{iface_name}",
            "target": iface_name,
            "category": "presence",
            "js": f"return typeof {iface_name} !== 'undefined';",
            "expected": True,
            "gap_class": "missing_interface",
            "side_effects": [],
            "cleanup": "none",
            "evidence_ceiling": "diagnostic_only",
        })

        if isinstance(iface.get("inheritance"), str) and iface["inheritance"]:
            probes.append({
                "probe_id": f"idl.inherits.{iface_name}",
                "target": iface_name,
                "category": "presence",
                "js": (
                    f"return {iface_name}.prototype instanceof "
                    f"{iface['inheritance']};"
                ),
                "expected": True,
                "gap_class": "prototype_chain_broken",
                "side_effects": [],
                "cleanup": "none",
                "evidence_ceiling": "diagnostic_only",
            })

        member: dict[str, Any]
        for member in iface.get("members", []):
            if member.get("kind") != "attribute":
                continue
            attr_name = member["name"]
            probe = _build_attribute_probe(iface_name, attr_name, member)
            if probe:
                probes.append(probe)

    pack_name = "idl-core-window.m1"
    return {
        "probe_pack": pack_name,
        "version": version,
        "description": (
            f"v{version} IDL-generated presence probes for "
            f"{', '.join(interfaces)} — "
            f"source: unified_ir.json "
            f"({ir_meta.get('total_interfaces', '?')} interfaces)"
        ),
        "evidence_ceiling": "diagnostic_only",
        "probes": probes,
    }


def _load_ir(path: Path) -> dict[str, Any]:
    with open(path, encoding="utf-8") as f:
        return json.load(f)


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
}


def _build_attribute_probe(
    iface_name: str,
    attr_name: str,
    member: dict[str, Any],
) -> dict[str, Any] | None:
    type_info = member.get("type")
    if not isinstance(type_info, dict) or type_info.get("kind") != "name":
        return None

    idl_type = str(type_info.get("name", ""))
    nullable = bool(type_info.get("nullable", False))
    readonly = bool(member.get("readonly", False))

    js_check = _IDL_TYPE_TO_JS_CHECK.get(idl_type)
    if js_check is None:
        return None

    access_path = f"{iface_name.lower()}.{attr_name}"

    if nullable:
        js_check = f"({js_check} || __v__ === null)"

    js_code = f"(function() {{ var __v__ = {access_path}; return {js_check}; }})()"

    return {
        "probe_id": f"idl.attr.{iface_name}.{attr_name}",
        "target": access_path,
        "category": "value",
        "js": js_code,
        "expected": True,
        "gap_class": "wrong_type",
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
    }
