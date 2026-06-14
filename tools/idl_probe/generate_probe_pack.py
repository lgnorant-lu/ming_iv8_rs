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
        },
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
            probe = _build_attribute_probe(ir_schema, iface_name, attr_name, member)
            if probe:
                probes.append(probe)

    pack_name = "idl-core-window.m1"
    return {
        "schema_version": "iv8-generated-probepack.v0.1",
        "probe_pack": pack_name,
        "version": version,
        "source": str(source_path),
        "generator": "iv8-idl-probe",
        "interfaces": list(interfaces),
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
) -> dict[str, Any] | None:
    type_info = member.get("type")
    if not isinstance(type_info, dict):
        return None
    type_kind = type_info.get("kind", "")
    if type_kind not in ("name",):
        _logger.debug(
            "skipping %s.%s: type kind=%r not yet supported",
            iface_name, attr_name, type_kind,
        )
        return None

    idl_type = str(type_info.get("name", ""))
    nullable = bool(type_info.get("nullable", False))

    js_check = _IDL_TYPE_TO_JS_CHECK.get(idl_type)
    if js_check is None:
        _logger.debug(
            "skipping %s.%s: IDL type %r not in js-check mapping",
            iface_name, attr_name, idl_type,
        )
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
        "gap_class": "value_mismatch",
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
        "source_ir": {
            "schema_version": ir_schema,
            "definition": iface_name,
            "member": attr_name,
            "idl_type": idl_type,
        },
    }
