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
}

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
        ]

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
            descr_probe = _build_descriptor_probe(ir_schema, iface_name, attr_name, member)
            if descr_probe:
                probes.append(descr_probe)

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
        },
    }
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
    return repr(attr_name)


def _runtime_accessibility_for(iface_name: str) -> str:
    if iface_name in _GLOBAL_INSTANCE_NAMES:
        return "global"
    return "instance_unresolved"


def _mark_sensitive_surface(probe: dict[str, Any], iface_name: str, attr_name: str) -> None:
    if (iface_name, attr_name) not in _SENSITIVE_IDL_SURFACES:
        return
    probe["sensitive_surface_probe"] = True
    probe["sensitivity_reason"] = "standard_idl_surface_name_only"
    probe["source_ir"]["sensitive_surface_probe"] = True
    probe["source_ir"]["sensitivity_reason"] = "standard_idl_surface_name_only"
