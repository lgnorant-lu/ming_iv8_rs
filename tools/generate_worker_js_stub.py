#!/usr/bin/env python3
"""Generate JS stub script for Worker isolate from unified IR.

Creates pure-JS interface constructors, prototype chains, and property
descriptors without using V8 FunctionTemplate. This avoids triggering
V8 GC on Worker thread (IsOnCentralStack crash).

Output: a JS string that can be eval'd in Worker context.
"""

import json
import sys


def generate_js_stub(ir_path: str) -> str:
    with open(ir_path, "r", encoding="utf-8") as f:
        ir = json.load(f)

    lines = []
    lines.append("// Auto-generated Worker interface stubs (JS Stub Injection)")
    lines.append("// Avoids V8 FunctionTemplate GC crash on Worker thread")
    lines.append("(function() {")
    lines.append("  'use strict';")

    # Phase 1: Collect Worker-visible interfaces in topological order
    interfaces = []
    for d in ir.get("definitions", []):
        if d.get("kind") != "interface":
            continue
        name = d.get("name", "")
        ext_attrs = d.get("ext_attrs", [])
        exposed_raw = None
        for ea in ext_attrs:
            if ea.get("name") == "Exposed":
                exposed_raw = ea.get("value", "")
                break
        if exposed_raw is None:
            continue
        if isinstance(exposed_raw, list):
            exposed_str = ",".join(
                [x.get("value", "") if isinstance(x, dict) else str(x) for x in exposed_raw]
            )
        else:
            exposed_str = str(exposed_raw)
        # Check if Worker-visible (includes Exposed=* since those are
        # available in all contexts including Worker)
        if "Worker" in exposed_str or exposed_str == "*":
            inheritance = d.get("inheritance", "") or ""
            members = d.get("members", [])
            interfaces.append((name, inheritance, members))

    # Phase 2: Generate constructors (sorted by inheritance depth)
    # Simple topological sort: interfaces without inheritance first
    by_name = {name: (inh, members) for name, inh, members in interfaces}
    generated = set()

    # Type-to-default-value mapping for IDL attribute types
    def idl_default_value(type_info):
        if not isinstance(type_info, dict):
            return "undefined"
        tn = type_info.get("name", "")
        if type_info.get("nullable"):
            return "null"
        string_types = {"DOMString", "USVString", "CSSOMString", "ByteString", "SVGBString"}
        if tn in string_types or "String" in tn:
            return '""'
        if tn in ("boolean",):
            return "false"
        if tn in ("byte", "octet", "short", "unsigned short", "long",
                   "unsigned long", "long long", "unsigned long long",
                   "float", "double", "unrestricted float", "unrestricted double",
                   "DOMHighResTimeStamp", "EpochTimeStamp"):
            return "0"
        # Interface types, object, any, etc.
        return "null"

    # Collect all interfaces for type lookup
    all_interface_names = set()
    for d in ir.get("definitions", []):
        if d.get("kind") == "interface":
            all_interface_names.add(d.get("name", ""))

    def gen_interface(name, inh, members):
        if name in generated:
            return
        if inh and inh in by_name and inh not in generated:
            gen_interface(inh, *by_name[inh])
        generated.add(name)

        # Constructor — compute length from IDL constructor members
        # length = number of required (non-optional) arguments
        ctor_len = 0
        for m in members:
            if m.get("kind") == "constructor":
                ctor_args = m.get("arguments", [])
                ctor_len = sum(1 for a in ctor_args if not a.get("optional", False) and not a.get("variadic", False))
                break
        # Constructor function with proper length and new-operator check
        lines.append(f"  var {name} = function {name}() {{")
        lines.append(f"    if (!(this instanceof {name})) throw new TypeError(\"Failed to construct '{name}': Please use the 'new' operator\");")
        # Initialize instance properties with type-correct default values
        for m in members:
            if m.get("kind") == "attribute":
                mname = m.get("name", "")
                if not mname:
                    continue
                ext_attrs_m = m.get("ext_attrs", [])
                is_unforgeable_m = any(ea.get("name") == "LegacyUnforgeable" for ea in ext_attrs_m)
                if is_unforgeable_m:
                    dv = idl_default_value(m.get("type", {}))
                    lines.append(f"    Object.defineProperty(this, '{mname}', {{value: {dv}, writable: false, enumerable: true, configurable: false}});")
        lines.append(f"  }};")
        if ctor_len > 0:
            lines.append(f"  Object.defineProperty({name}, 'length', {{value: {ctor_len}, writable: false, enumerable: false, configurable: true}});")
        # Prototype chain — non-writable, non-configurable, non-enumerable
        if inh and inh in by_name:
            lines.append(f"  var _proto_{name} = Object.create({inh}.prototype);")
            lines.append(f"  Object.defineProperty({name}, 'prototype', {{value: _proto_{name}, writable: false, enumerable: false, configurable: false}});")
            lines.append(f"  Object.defineProperty(_proto_{name}, 'constructor', {{value: {name}, writable: true, configurable: true, enumerable: false}});")
            # Constructor inheritance: Object.getPrototypeOf({name}) === {inh}
            lines.append(f"  Object.setPrototypeOf({name}, {inh});")
        else:
            lines.append(f"  Object.defineProperty({name}, 'prototype', {{writable: false, enumerable: false, configurable: false}});")
            lines.append(f"  Object.defineProperty({name}.prototype, 'constructor', {{value: {name}, writable: true, configurable: true, enumerable: false}});")
        # toStringTag
        lines.append(f"  Object.defineProperty({name}.prototype, Symbol.toStringTag, {{value: '{name}', configurable: true}});")
        # Register on global — non-enumerable
        lines.append(f"  Object.defineProperty(self, '{name}', {{value: {name}, writable: true, configurable: true, enumerable: false}});")

    for name, inh, members in interfaces:
        gen_interface(name, inh, members)

    # Phase 3: Generate members
    for name, inh, members in interfaces:
        for m in members:
            kind = m.get("kind", "")
            mname = m.get("name", "")
            if not mname:
                continue
            readonly = m.get("readonly", False)
            is_static = m.get("static", False)
            target = name if is_static else f"{name}.prototype"

            if kind == "const":
                cval_raw = m.get("const_value") or m.get("value", "")
                if isinstance(cval_raw, dict):
                    cv = cval_raw.get("value", "")
                    ct = cval_raw.get("type", "")
                    if ct == "Infinity":
                        cval_js = "-Infinity" if cval_raw.get("negative") else "Infinity"
                    elif ct == "NaN":
                        cval_js = "NaN"
                    elif ct == "null":
                        cval_js = "null"
                    elif ct == "number":
                        cval_js = str(cv)
                    else:
                        cval_js = str(cv) if cv else "undefined"
                elif isinstance(cval_raw, str):
                    cval_js = cval_raw if cval_raw else "undefined"
                elif isinstance(cval_raw, bool):
                    cval_js = "true" if cval_raw else "false"
                elif isinstance(cval_raw, (int, float)):
                    cval_js = str(cval_raw)
                else:
                    cval_js = "undefined"
                # On constructor
                lines.append(f"  Object.defineProperty({name}, '{mname}', {{value: {cval_js}, writable: false, enumerable: true, configurable: false}});")
                # On prototype
                lines.append(f"  Object.defineProperty({name}.prototype, '{mname}', {{value: {cval_js}, writable: false, enumerable: true, configurable: false}});")

            elif kind == "attribute":
                # Check for LegacyUnforgeable — these are own data properties
                ext_attrs = m.get("ext_attrs", [])
                is_unforgeable = any(ea.get("name") == "LegacyUnforgeable" for ea in ext_attrs)
                if is_unforgeable:
                    # Already set as own data property in constructor
                    continue
                getter_name = f"get {mname}"
                setter_name = f"set {mname}"
                dv = idl_default_value(m.get("type", {}))
                lines.append(f"  var _g_{name}_{mname} = function() {{")
                lines.append(f"    if (!(this instanceof {name}) && (this === null || this === undefined || Object.getPrototypeOf(this) !== {name}.prototype))")
                lines.append(f"      throw new TypeError('Illegal invocation');")
                lines.append(f"    return {dv};")
                lines.append(f"  }};")
                lines.append(f"  Object.defineProperty(_g_{name}_{mname}, 'name', {{value: '{getter_name}'}});")
                if readonly:
                    lines.append(f"  Object.defineProperty({target}, '{mname}', {{get: _g_{name}_{mname}, set: undefined, enumerable: true, configurable: true}});")
                else:
                    lines.append(f"  var _s_{name}_{mname} = function(v) {{")
                    lines.append(f"    if (!(this instanceof {name}) && (this === null || this === undefined || Object.getPrototypeOf(this) !== {name}.prototype))")
                    lines.append(f"      throw new TypeError('Illegal invocation');")
                    lines.append(f"  }};")
                    lines.append(f"  Object.defineProperty(_s_{name}_{mname}, 'name', {{value: '{setter_name}'}});")
                    lines.append(f"  Object.defineProperty({target}, '{mname}', {{get: _g_{name}_{mname}, set: _s_{name}_{mname}, enumerable: true, configurable: true}});")

            elif kind == "operation":
                all_args = m.get("arguments", [])
                arg_count = len(all_args)
                required_count = sum(1 for a in all_args if not a.get("optional", False) and not a.get("variadic", False))
                args = ", ".join(["a" + str(i) for i in range(arg_count)])
                lines.append(f"  var _op_{name}_{mname} = function({args}) {{")
                lines.append(f"    if (!(this instanceof {name}) && (this === null || this === undefined || Object.getPrototypeOf(this) !== {name}.prototype))")
                lines.append(f"      throw new TypeError('Illegal invocation');")
                if required_count > 0:
                    lines.append(f"    if (arguments.length < {required_count}) throw new TypeError(\"{required_count} argument(s) required, but only \" + arguments.length + \" present.\");")
                lines.append(f"  }};")
                lines.append(f"  Object.defineProperty(_op_{name}_{mname}, 'name', {{value: '{mname}'}});")
                if required_count != arg_count:
                    lines.append(f"  Object.defineProperty(_op_{name}_{mname}, 'length', {{value: {required_count}, writable: false, enumerable: false, configurable: true}});")
                lines.append(f"  Object.defineProperty({target}, '{mname}', {{value: _op_{name}_{mname}, writable: true, enumerable: true, configurable: true}});")

    lines.append("})();")
    return "\n".join(lines)


if __name__ == "__main__":
    ir_path = sys.argv[1] if len(sys.argv) > 1 else "tools/idl/output/unified_ir.json"
    js = generate_js_stub(ir_path)
    print(f"// Generated {js.count(chr(10))} lines, ~{len(js)} bytes")
    print(js)
