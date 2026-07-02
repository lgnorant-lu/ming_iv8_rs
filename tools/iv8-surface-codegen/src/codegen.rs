//! Code generation engine — generates Rust source for FunctionTemplate stubs.

use crate::ea_handler::{process_interface_ea, EaResult};
use crate::ir::Definition;
use crate::type_mapper;
use std::collections::BTreeMap;

/// Attributes excluded from codegen — handled by JS shims instead.
/// (interface_name, attribute_name)
///
/// These properties are installed at runtime by `DOCUMENT_PROPS_JS`
/// (see `crates/iv8-core/src/shims/document_props.rs`). If codegen also
/// installs a native accessor on the prototype, the shim's
/// `Object.defineProperty` on the instance is silently shadowed
/// (prototype accessor wins over instance data property), or the `in`
/// operator finds the prototype accessor and the shim guard skips
/// installation — leaving the native stub (returns undefined) as the
/// effective value.
///
/// Excluding them here ensures the shim is the single source of truth.
const EXCLUDED_ATTRIBUTES: &[(&str, &str)] = &[
    // --- Document properties (installed by DOCUMENT_PROPS_JS) ---
    ("Document", "cookie"),
    ("Document", "referrer"),
    ("Document", "hidden"),
    ("Document", "visibilityState"),
    ("Document", "readyState"),
    ("Document", "domain"),
    ("Document", "URL"),
    ("Document", "title"),
    ("Document", "documentURI"),
    ("Document", "scrollingElement"),
    ("Document", "currentScript"),
    ("Document", "defaultView"),
    ("Document", "characterSet"),
    ("Document", "contentType"),
    ("Document", "compatMode"),
    ("Document", "lastModified"),
    ("Document", "fullscreenEnabled"),
    ("Document", "pictureInPictureEnabled"),
    // --- Node properties (inherited by Document, installed by DOCUMENT_PROPS_JS) ---
    ("Node", "baseURI"),
    ("Node", "ownerDocument"),
];

fn should_skip_attribute(interface_name: &str, attr_name: &str) -> bool {
    EXCLUDED_ATTRIBUTES
        .iter()
        .any(|(iface, attr)| *iface == interface_name && *attr == attr_name)
}

/// Interfaces that are constructable via JS alias constructors not
/// represented as [Constructor] members in the IDL. These have
/// [LegacyFactoryFunction] or [NamedConstructor] ext_attrs, or are
/// historically constructable in browsers without IDL annotation.
const KNOWN_CONSTRUCTABLE: &[&str] = &[
    "HTMLImageElement",
    "HTMLAudioElement",
    "HTMLOptionElement",
];

fn is_constructable(def: &Definition) -> bool {
    if def.kind == "callback_interface" {
        return false;
    }
    if let Some(ref name) = def.name {
        if KNOWN_CONSTRUCTABLE.iter().any(|n| *n == name) {
            return true;
        }
    }
    if def
        .ext_attrs
        .iter()
        .any(|ea| ea.starts_with("Constructor"))
    {
        return true;
    }
    if def
        .ext_attrs
        .iter()
        .any(|ea| ea.starts_with("NamedConstructor") || ea.starts_with("LegacyFactoryFunction"))
    {
        return true;
    }
    def.members.iter().any(|m| m.kind == "constructor")
}

/// Get the required argument count for the interface constructor.
/// Returns 0 if no constructor or all args are optional/variadic.
fn constructor_arg_count(def: &Definition) -> usize {
    for m in &def.members {
        if m.kind == "constructor" {
            return m.required_arg_count;
        }
    }
    0
}

fn parse_const_value(raw: &str) -> Option<f64> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(hex) = trimmed.strip_prefix("0x").or_else(|| trimmed.strip_prefix("0X")) {
        let neg = trimmed.starts_with('-');
        let clean = hex.trim_start_matches('-');
        return i64::from_str_radix(clean, 16)
            .ok()
            .map(|v| if neg { -(v as f64) } else { v as f64 });
    }
    trimmed.parse::<f64>().ok()
}

fn format_const_v8_value(raw: &str) -> String {
    match parse_const_value(raw) {
        Some(v) => format!("v8::Number::new(scope, {}.0).into()", v),
        None => format!("v8::String::new(scope, \"{}\").unwrap().into()", raw.replace('"', "\\\"")),
    }
}

pub struct GeneratedFile {
    pub domain: String,
    pub content: String,
    pub interface_count: usize,
}

pub struct InstallInfo {
    /// Topologically sorted interface names
    pub sorted: Vec<String>,
    /// name → domain
    pub domain_of: BTreeMap<String, String>,
}

/// Generate all domain files + install info from merged definitions.
pub fn generate_all(
    definitions: &[Definition],
    sorted: &[String],
) -> (Vec<GeneratedFile>, InstallInfo) {
    let mut by_name: BTreeMap<String, &Definition> = BTreeMap::new();
    for def in definitions {
        if let Some(name) = &def.name {
            if def.kind != "interface" && def.kind != "callback_interface" {
                continue;
            }
            by_name.insert(name.clone(), def);
        }
    }

    let mut domains: BTreeMap<String, Vec<&Definition>> = BTreeMap::new();
    let mut domain_of = BTreeMap::new();
    for name in sorted {
        if let Some(def) = by_name.get(name) {
            let domain = crate::topo::classify_domain(name).to_string();
            domains.entry(domain.clone()).or_default().push(def);
            domain_of.insert(name.clone(), domain);
        }
    }

    let mut files = Vec::new();
    for (domain, defs) in &domains {
        let content = generate_domain_file(domain, defs, &by_name);
        files.push(GeneratedFile {
            domain: domain.clone(),
            content,
            interface_count: defs.len(),
        });
    }
    files.sort_by(|a, b| a.domain.cmp(&b.domain));

    let sorted_interfaces: Vec<String> = sorted
        .iter()
        .filter(|n| by_name.contains_key(*n))
        .cloned()
        .collect();

    (
        files,
        InstallInfo {
            sorted: sorted_interfaces,
            domain_of,
        },
    )
}

fn generate_domain_file(
    domain: &str,
    defs: &[&Definition],
    _all: &BTreeMap<String, &Definition>,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("//! Generated stubs: {}\n", domain));
    out.push_str("//! Auto-generated by iv8-surface-codegen.\n");
    out.push_str("#![allow(unused_imports)]\n\n");
    let needs_illegal = defs.iter().any(|d| !is_constructable(d));
    if needs_illegal {
        out.push_str("use super::{construct_only, illegal_constructor};\n");
    } else {
        out.push_str("use super::construct_only;\n");
    }
    out.push_str("use v8::Local;\n");
    out.push_str("use v8::FunctionTemplate;\n\n");

    for def in defs {
        let name = match &def.name {
            Some(n) => n,
            None => continue,
        };
        let fn_name = type_mapper::idl_name_to_rust(name);
        let ea = process_interface_ea(def);

        let callbacks = generate_callbacks(def, &fn_name);
        if !callbacks.is_empty() {
            out.push_str(&callbacks);
        }
        out.push_str(&generate_template_function(def, &ea, &fn_name));
        out.push('\n');
    }

    let module_name = domain.replace('-', "_");
    out.push_str(&format!("pub fn fix_accessors_{}(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {{\n", module_name));
    let mut fix_count = 0;
    for def in defs {
        let name = match &def.name { Some(n) => n, None => continue };
        let fn_name = type_mapper::idl_name_to_rust(name);
        let ea = process_interface_ea(def);
        if ea.is_global || ea.no_interface_object { continue; }
        let mut idx = 0;
        for m in &def.members {
            if m.kind != "attribute" {
                if m.kind == "operation" { idx += 1; }
                continue;
            }
            let attr_name = match &m.name { Some(n) => n, None => continue };
            if should_skip_attribute(name, attr_name) { continue; }
            idx += 1;
            let has_setter = !m.readonly || m.has_put_forwards || m.has_replaceable;
            if !has_setter { continue; }
            out.push_str("    {\n");
            out.push_str(&format!("        let ctor_key = v8::String::new(scope, \"{}\").unwrap();\n", name));
            out.push_str("        if let Some(ctor_val) = global.get(scope, ctor_key.into()) {\n");
            out.push_str("            if ctor_val.is_function() {\n");
            out.push_str("                let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };\n");
            out.push_str("                let proto_key = v8::String::new(scope, \"prototype\").unwrap();\n");
            out.push_str("                if let Some(proto_val) = ctor.get(scope, proto_key.into()) {\n");
            out.push_str("                    if let Some(proto_obj) = proto_val.to_object(scope) {\n");
            out.push_str(&format!("                        let attr_key = v8::String::new(scope, \"{}\").unwrap();\n", attr_name));
            out.push_str(&format!("                        let g = v8::FunctionTemplate::builder_raw({}_get_{}).length(0).build(scope);\n", fn_name, idx));
            out.push_str(&format!("                        g.set_class_name(v8::String::new(scope, \"get {}\").unwrap());\n", attr_name));
            out.push_str(&format!("                        let s = v8::FunctionTemplate::builder_raw({}_set_{}).length(1).build(scope);\n", fn_name, idx));
            out.push_str(&format!("                        s.set_class_name(v8::String::new(scope, \"set {}\").unwrap());\n", attr_name));
            out.push_str("                        let gf = g.get_function(scope).unwrap();\n");
            out.push_str("                        let sf = s.get_function(scope).unwrap();\n");
            out.push_str("                        let mut d = v8::PropertyDescriptor::new_from_get_set(gf.into(), sf.into());\n");
            out.push_str("                        d.set_enumerable(true);\n");
            out.push_str("                        d.set_configurable(true);\n");
            out.push_str("                        let _ = proto_obj.define_property(scope, attr_key.into(), &d);\n");
            out.push_str("                    }\n");
            out.push_str("                }\n");
            out.push_str("            }\n");
            out.push_str("        }\n");
            out.push_str("    }\n");
            fix_count += 1;
        }
    }
    out.push_str(&format!("    // fixed {} accessors\n", fix_count));
    out.push_str("}\n\n");

    out
}

fn generate_callbacks(def: &Definition, fn_name: &str) -> String {
    let mut out = String::new();
    let mut idx = 0;

    let iface_name = def.name.as_deref().unwrap_or("Unknown");

    // Prototype chain traversal check: verifies that `this` is an instance of
    // the interface by walking the prototype chain. This is semantically
    // equivalent to `instanceof` and matches Chrome's "Illegal invocation"
    // behavior for WebIDL receiver checks.
    //
    // Algorithm:
    // 1. Look up global[iface_name] → constructor Function
    // 2. Get constructor["prototype"] → prototype Object
    // 3. If this === prototype → throw TypeError (prototype is not its own instance)
    // 4. Walk this.__proto__ chain; if prototype found → PASS; else → throw TypeError
    //
    // This approach is required because rusty-v8 does not expose:
    // - FunctionCallbackInfo::Holder()
    // - FunctionTemplate::HasInstance()
    // - Object::instance_of()
    // - Signature on AccessorConfiguration (FFI omits signature param)
    //
    // See: docs/roadmap/v0.8/analysis/codegen-null-this-design.md Section 12
    let prototype_chain_check = format!(
        "        let __args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);\n\
         \x20       let __this = __args.this();\n\
         \x20       let __ctx = scope.get_current_context();\n\
         \x20       let __global = __ctx.global(scope);\n\
         \x20       let __iface_name = v8::String::new(scope, \"{iface}\").unwrap();\n\
         \x20       if let Some(__ctor_val) = __global.get(scope, __iface_name.into()) {{\n\
         \x20           if __ctor_val.is_function() {{\n\
         \x20               let __ctor = unsafe {{ v8::Local::<v8::Function>::cast_unchecked(__ctor_val) }};\n\
         \x20               let __proto_key = v8::String::new(scope, \"prototype\").unwrap();\n\
         \x20               if let Some(__proto_val) = __ctor.get(scope, __proto_key.into()) {{\n\
         \x20                   if __proto_val.is_object() && !__proto_val.is_null_or_undefined() {{\n\
         \x20                       let __proto = unsafe {{ v8::Local::<v8::Object>::cast_unchecked(__proto_val) }};\n\
         \x20                       if __this.strict_equals(__proto.into()) {{\n\
         \x20                           let __msg = v8::String::new(scope, \"Illegal invocation\").unwrap();\n\
         \x20                           let __exc = v8::Exception::type_error(scope, __msg);\n\
         \x20                           scope.throw_exception(__exc);\n\
         \x20                           return;\n\
         \x20                       }}\n\
         \x20                       let mut __current: v8::Local<v8::Value> = __this.into();\n\
         \x20                       let mut __found = false;\n\
         \x20                       for _ in 0..20usize {{\n\
         \x20                           let Some(__cur_obj) = __current.to_object(scope) else {{ break; }};\n\
         \x20                           let Some(__parent) = __cur_obj.get_prototype(scope) else {{ break; }};\n\
         \x20                           if __parent.is_null_or_undefined() || !__parent.is_object() {{ break; }}\n\
         \x20                           if __parent.strict_equals(__proto.into()) {{ __found = true; break; }}\n\
         \x20                           __current = __parent;\n\
         \x20                       }}\n\
         \x20                       if !__found {{\n\
         \x20                           let __msg = v8::String::new(scope, \"Illegal invocation\").unwrap();\n\
         \x20                           let __exc = v8::Exception::type_error(scope, __msg);\n\
         \x20                           scope.throw_exception(__exc);\n\
         \x20                           return;\n\
         \x20                       }}\n\
         \x20                   }}\n\
         \x20               }}\n\
         \x20           }}\n\
         \x20       }}\n",
        iface = iface_name,
    );

    let receiver_check = &prototype_chain_check;

    // Promise variant: for operations returning Promise<T>, receiver check
    // failure must return Promise.reject(TypeError) instead of throwing.
    // idlharness throwOrReject expects Promise-returning ops to reject, not throw.
    let prototype_chain_check_promise = format!(
        "        let __args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);\n\
         \x20       let __this = __args.this();\n\
         \x20       let __ctx = scope.get_current_context();\n\
         \x20       let __global = __ctx.global(scope);\n\
         \x20       let __iface_name = v8::String::new(scope, \"{iface}\").unwrap();\n\
         \x20       if let Some(__ctor_val) = __global.get(scope, __iface_name.into()) {{\n\
         \x20           if __ctor_val.is_function() {{\n\
         \x20               let __ctor = unsafe {{ v8::Local::<v8::Function>::cast_unchecked(__ctor_val) }};\n\
         \x20               let __proto_key = v8::String::new(scope, \"prototype\").unwrap();\n\
         \x20               if let Some(__proto_val) = __ctor.get(scope, __proto_key.into()) {{\n\
         \x20                   if __proto_val.is_object() && !__proto_val.is_null_or_undefined() {{\n\
         \x20                       let __proto = unsafe {{ v8::Local::<v8::Object>::cast_unchecked(__proto_val) }};\n\
         \x20                       if __this.strict_equals(__proto.into()) {{\n\
         \x20                           let __msg = v8::String::new(scope, \"Illegal invocation\").unwrap();\n\
         \x20                           let __exc = v8::Exception::type_error(scope, __msg);\n\
         \x20                           if let Some(__resolver) = v8::PromiseResolver::new(scope) {{\n\
         \x20                               let _ = __resolver.reject(scope, __exc.into());\n\
         \x20                               let mut __rv = v8::ReturnValue::from_function_callback_info(info_ref);\n\
         \x20                               __rv.set(__resolver.get_promise(scope).into());\n\
         \x20                           }}\n\
         \x20                           return;\n\
         \x20                       }}\n\
         \x20                       let mut __current: v8::Local<v8::Value> = __this.into();\n\
         \x20                       let mut __found = false;\n\
         \x20                       for _ in 0..20usize {{\n\
         \x20                           let Some(__cur_obj) = __current.to_object(scope) else {{ break; }};\n\
         \x20                           let Some(__parent) = __cur_obj.get_prototype(scope) else {{ break; }};\n\
         \x20                           if __parent.is_null_or_undefined() || !__parent.is_object() {{ break; }}\n\
         \x20                           if __parent.strict_equals(__proto.into()) {{ __found = true; break; }}\n\
         \x20                           __current = __parent;\n\
         \x20                       }}\n\
         \x20                       if !__found {{\n\
         \x20                           let __msg = v8::String::new(scope, \"Illegal invocation\").unwrap();\n\
         \x20                           let __exc = v8::Exception::type_error(scope, __msg);\n\
         \x20                           if let Some(__resolver) = v8::PromiseResolver::new(scope) {{\n\
         \x20                               let _ = __resolver.reject(scope, __exc.into());\n\
         \x20                               let mut __rv = v8::ReturnValue::from_function_callback_info(info_ref);\n\
         \x20                               __rv.set(__resolver.get_promise(scope).into());\n\
         \x20                           }}\n\
         \x20                           return;\n\
         \x20                       }}\n\
         \x20                   }}\n\
         \x20               }}\n\
         \x20           }}\n\
         \x20       }}\n",
        iface = iface_name,
    );

    let op_receiver_check = &prototype_chain_check;

    for m in &def.members {
        if m.kind == "attribute" {
            let attr_name = m.name.as_deref().unwrap_or("");
            if should_skip_attribute(def.name.as_deref().unwrap_or(""), attr_name) {
                continue;
            }
            idx += 1;
            let type_name = m.idl_type.as_deref().unwrap_or("any");
            let tm = type_mapper::map_idl_type(type_name);

            // Getter — receiver check for null-this (accessor properties pass raw receiver)
            out.push_str(&format!(
                "pub(crate) unsafe extern \"C\" fn {}_get_{}(_info: *const v8::FunctionCallbackInfo) {{\n",
                fn_name, idx
            ));
            out.push_str(
                "    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {\n",
            );
            out.push_str("        let info_ref = unsafe { &*_info };\n");
            out.push_str("        v8::callback_scope!(unsafe scope, info_ref);\n");
            out.push_str(receiver_check);
            out.push_str(
                "        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);\n",
            );
            out.push_str(&format!("        rv.set({});\n", tm.default_value));
            out.push_str("    }));\n");
            out.push_str("}\n\n");

            // Setter — generated for non-readonly OR readonly-with-PutForwards/Replaceable
            if !m.readonly || m.has_put_forwards || m.has_replaceable {
                out.push_str(&format!(
                    "pub(crate) unsafe extern \"C\" fn {}_set_{}(_info: *const v8::FunctionCallbackInfo) {{\n",
                    fn_name, idx
                ));
                out.push_str(
                    "    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {\n",
                );
                out.push_str("        let info_ref = unsafe { &*_info };\n");
                out.push_str("        v8::callback_scope!(unsafe scope, info_ref);\n");
                out.push_str(receiver_check);
                out.push_str("    }));\n");
                out.push_str("}\n\n");
            }
        }

        if m.kind == "operation" {
            idx += 1;
            let ret_name = m.return_type.as_deref().unwrap_or("undefined");
            let tm = type_mapper::map_idl_type(ret_name);
            let is_promise_ret = type_mapper::is_promise_public(ret_name);
            out.push_str(&format!(
                "unsafe extern \"C\" fn {}_op_{}(_info: *const v8::FunctionCallbackInfo) {{\n",
                fn_name, idx
            ));
            out.push_str(
                "    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {\n",
            );
            out.push_str("        let info_ref = unsafe { &*_info };\n");
            out.push_str("        v8::callback_scope!(unsafe scope, info_ref);\n");
            if is_promise_ret {
                out.push_str(&format!(
                    "        if !crate::promise_check::check_receiver_promise(scope, _info, \"{iface}\") {{ return; }}\n",
                    iface = iface_name,
                ));
            } else {
                out.push_str(op_receiver_check);
            }
            if m.required_arg_count > 0 {
                out.push_str("        let __args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);\n");
                out.push_str(&format!(
                    "        if __args.length() < {} {{\n",
                    m.required_arg_count
                ));
                out.push_str(&format!(
                    "            let msg = v8::String::new(scope, &format!(\"{{}} argument(s) required, but only {{}} present\", {}, __args.length())).unwrap();\n",
                    m.required_arg_count
                ));
                out.push_str("            let exc = v8::Exception::type_error(scope, msg);\n");
                out.push_str("            scope.throw_exception(exc);\n");
                out.push_str("            return;\n");
                out.push_str("        }\n");
            }
            out.push_str(
                "        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);\n",
            );
            out.push_str(&format!("        rv.set({});\n", tm.default_value));
            out.push_str("    }));\n");
            out.push_str("}\n\n");
        }
    }
    out
}

fn generate_template_function(def: &Definition, ea: &EaResult, fn_name: &str) -> String {
    let name = def.name.as_deref().unwrap_or("Unknown");
    let mut out = String::new();

    // Check both IR inheritance and overrides for known mixin interfaces
    const INHERITANCE_OVERRIDES: &[(&str, &str)] = &[
        ("Navigator", "EventTarget"),
        ("WorkerNavigator", "EventTarget"),
        ("Storage", "EventTarget"),
        ("XMLHttpRequestEventTarget", "EventTarget"),
        ("XMLHttpRequest", "XMLHttpRequestEventTarget"),
        ("XMLHttpRequestUpload", "XMLHttpRequestEventTarget"),
    ];
    let effective_parent = def.inheritance.as_ref().map(|s| s.as_str()).or_else(|| {
        INHERITANCE_OVERRIDES
            .iter()
            .find(|(iface, _)| *iface == name)
            .map(|(_, parent)| *parent)
    });

    // Determine parent create function name for cross-referencing
    let _parent_fn = effective_parent.map(|p| {
        format!(
            "{}::create_{}_template",
            crate::topo::classify_domain(p).replace('-', "_"),
            type_mapper::idl_name_to_rust(p)
        )
    });

    out.push_str(&format!("/// Create FunctionTemplate for {}.\n", name));
    out.push_str(&format!("pub fn create_{}_template<'s>(\n", fn_name));
    out.push_str("    scope: &v8::PinScope<'s, '_>,\n");
    out.push_str("    _parent: Option<v8::Local<'s, v8::FunctionTemplate>>,\n");
    out.push_str(") -> v8::Local<'s, v8::FunctionTemplate> {\n");
    let ctor_cb = if is_constructable(def) {
        "construct_only"
    } else {
        "illegal_constructor"
    };
    let ctor_length = constructor_arg_count(def);
    if ctor_length > 0 {
        out.push_str(&format!(
            "    let tmpl = v8::FunctionTemplate::builder_raw({}).length({}).build(scope);\n",
            ctor_cb, ctor_length
        ));
    } else {
        out.push_str(&format!(
            "    let tmpl = v8::FunctionTemplate::builder_raw({}).build(scope);\n",
            ctor_cb
        ));
    }
    out.push_str("    tmpl.read_only_prototype();\n");
    out.push_str(&format!(
        "    tmpl.set_class_name(v8::String::new(scope, \"{}\").unwrap());\n",
        name
    ));

    // Inheritance
    if effective_parent.is_some() {
        out.push_str("    if let Some(p) = _parent {\n");
        out.push_str("        tmpl.inherit(p);\n");
        out.push_str("    }\n");
    }

    // Prototype setup
    let _has_members = !def.members.is_empty();
    out.push_str("\n    let proto = tmpl.prototype_template(scope);\n");
    
    // For [Global] interfaces, attributes go on the instance (tmpl) not prototype
    let target_var = "proto";

    // Symbol.toStringTag
    out.push_str("    {\n");
    out.push_str("        let tag_sym = v8::Symbol::get_to_string_tag(scope);\n");
    out.push_str(&format!(
        "        let tag_val = v8::String::new(scope, \"{}\").unwrap();\n",
        name
    ));
    out.push_str("        proto.set(tag_sym.into(), tag_val.into());\n");
    out.push_str("    }\n");

    // Members — generate for attribute, operation, and const
    // Large interfaces are split into helper functions to avoid stack overflow.
    const MEMBER_BATCH_SIZE: usize = 10;
    let mut member_blocks: Vec<String> = Vec::new();
    let mut const_blocks: Vec<String> = Vec::new();
    let mut idx = 0;
    for m in &def.members {
        if m.kind == "const" {
            if let (Some(cname), Some(cval)) = (&m.name, &m.const_value) {
                let v8_val = format_const_v8_value(cval);
                let mut block = String::new();
                block.push_str(&format!("    // const: {}\n", cname));
                block.push_str("    {\n");
                block.push_str(&format!(
                    "        let name = v8::String::new(scope, \"{}\").unwrap();\n",
                    cname
                ));
                block.push_str(&format!("        let val = {};\n", v8_val));
                block.push_str(&format!("        {}.set_with_attr(name.into(), val, v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE);\n", target_var));
                block.push_str("    }\n");
                member_blocks.push(block);

                let mut ctor_block = String::new();
                ctor_block.push_str(&format!(
                    "    {{ let name = v8::String::new(scope, \"{}\").unwrap(); let val = {}; ctor.define_own_property(scope, name.into(), val, v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE); }}\n",
                    cname, v8_val
                ));
                const_blocks.push(ctor_block);
            }
            continue;
        }
        if m.kind != "attribute" && m.kind != "operation" {
            continue;
        }
        if m.kind == "attribute" {
            let attr_name = m.name.as_deref().unwrap_or("");
            if should_skip_attribute(def.name.as_deref().unwrap_or(""), attr_name) {
                continue;
            }
        }
        idx += 1;
        if m.kind == "attribute" {
            let attr_name = m.name.as_deref().unwrap_or("unknown");
            let mut block = String::new();
            block.push_str(&format!("    // attribute: {}\n", attr_name));
            block.push_str("    {\n");
            block.push_str(&format!(
                "        let name = v8::String::new(scope, \"{}\").unwrap();\n",
                attr_name
            ));
            block.push_str(&format!(
                "        let getter = v8::FunctionTemplate::builder_raw({}_get_{}).length(0).build(scope);\n",
                fn_name, idx
            ));
            block.push_str(&format!(
                "        getter.set_class_name(v8::String::new(scope, \"get {}\").unwrap());\n",
                attr_name
            ));
            if m.readonly && !m.has_put_forwards && !m.has_replaceable {
                block.push_str(&format!("        {}.set_accessor_property(name.into(), Some(getter), None, v8::PropertyAttribute::NONE);\n", target_var));
            } else {
                block.push_str(&format!(
                    "        let setter = v8::FunctionTemplate::builder_raw({}_set_{}).length(1).build(scope);\n", fn_name, idx));
                block.push_str(&format!(
                    "        setter.set_class_name(v8::String::new(scope, \"set {}\").unwrap());\n",
                    attr_name
                ));
                block.push_str(&format!("        {}.set_accessor_property(name.into(), Some(getter), Some(setter), v8::PropertyAttribute::NONE);\n", target_var));
            }
            block.push_str("    }\n");
            member_blocks.push(block);
        }

        if m.kind == "operation" {
            let op_name = m.name.as_deref().unwrap_or("unknown");
            // Calculate minOverloadLength: min required_arg_count across all
            // overloads with the same name. WebIDL spec: interface object
            // .length = minOverloadLength for overloaded operations.
            let min_arg_count = def
                .members
                .iter()
                .filter(|m2| m2.kind == "operation" && m2.name.as_deref() == Some(op_name))
                .map(|m2| m2.required_arg_count)
                .min()
                .unwrap_or(m.required_arg_count);
            let arg_count = min_arg_count;
            let mut block = String::new();
            block.push_str(&format!("    // method: {}()\n", op_name));
            block.push_str("    {\n");
            block.push_str(&format!(
                "        let name = v8::String::new(scope, \"{}\").unwrap();\n",
                op_name
            ));
            if arg_count > 0 {
                block.push_str(&format!(
                    "        let func_tmpl = v8::FunctionTemplate::builder_raw({}_op_{}).length({}).build(scope);\n",
                    fn_name, idx, arg_count));
            } else {
                block.push_str(&format!(
                    "        let func_tmpl = v8::FunctionTemplate::builder_raw({}_op_{}).build(scope);\n", fn_name, idx));
            }
            block.push_str("        func_tmpl.set_class_name(name);\n");
            block.push_str(&format!("        {}.set(name.into(), func_tmpl.into());\n", target_var));
            block.push_str("    }\n");
            member_blocks.push(block);
        }
    }

    if member_blocks.len() <= MEMBER_BATCH_SIZE {
        for block in &member_blocks {
            out.push_str(block);
        }
    } else {
        let mut helper_fns = String::new();
        let (helper_param_type, helper_param_name) = if ea.is_global {
            ("FunctionTemplate", "tmpl")
        } else {
            ("ObjectTemplate", "proto")
        };
        for (batch_i, chunk) in member_blocks.chunks(MEMBER_BATCH_SIZE).enumerate() {
            let helper_name = format!("install_{}_members_{}", fn_name, batch_i + 1);
            out.push_str(&format!("    {}(scope, {});\n", helper_name, helper_param_name));
            helper_fns.push_str(&format!(
                "fn {}<'s>(scope: &v8::PinScope<'s, '_>, proto: v8::Local<'s, v8::{}>) {{\n",
                helper_name, helper_param_type
            ));
            for block in chunk {
                helper_fns.push_str(block);
            }
            helper_fns.push_str("}\n\n");
        }
        if !const_blocks.is_empty() {
            out.push_str("    if let Some(ctor) = tmpl.get_function(scope) {\n");
            for block in &const_blocks {
                out.push_str(block);
            }
            out.push_str("    }\n");
        }
        out.push_str("\n    tmpl\n");
        out.push_str("}\n\n");
        out.push_str(&helper_fns);
        return out;
    }

    if !const_blocks.is_empty() {
        out.push_str("    if let Some(ctor) = tmpl.get_function(scope) {\n");
        for block in &const_blocks {
            out.push_str(block);
        }
        out.push_str("    }\n");
    }
    out.push_str("\n    tmpl\n");
    out.push_str("}\n");
    out
}

/// Generate the install_all function for generated/mod.rs.
/// Creates templates in topological order, handles parent wiring,
/// registers constructors on global with DONT_ENUM.
pub fn generate_install_all(
    definitions: &[Definition],
    sorted: &[String],
    domain_of: &BTreeMap<String, String>,
) -> String {
    let mut by_name: BTreeMap<String, &Definition> = BTreeMap::new();
    for def in definitions {
        if let Some(name) = &def.name {
            if def.kind == "interface" || def.kind == "callback_interface" {
                by_name.insert(name.clone(), def);
            }
        }
    }

    let mut out = String::new();
    out.push_str("//! Generated install_all — creates all templates in topological order.\n");
    out.push_str("//! v0.8.26: Global-handle HashMap + v8::scope! batch blocks.\n\n");
    out.push_str("use v8::Local;\nuse v8::Object;\nuse v8::Global;\nuse v8::FunctionTemplate;\n\n");

    out.push_str("pub fn install_all(scope: &mut v8::PinScope<'_, '_>, global: Local<Object>) {\n");
    out.push_str("    let mut templates: std::collections::HashMap<&str, v8::Global<FunctionTemplate>> = std::collections::HashMap::new();\n\n");

    const BATCH_SIZE: usize = 5;

    // Phase 1: Template creation with scope-break batches
    // BATCH_SIZE counts ACTUAL templates created, not sorted array indices
    let mut created = 0usize;
    let mut batch_num = 0usize;
    for name in sorted {
        let def = match by_name.get(name.as_str()) {
            Some(d) => d,
            None => continue,
        };

        if created % BATCH_SIZE == 0 {
            if created > 0 {
                out.push_str("    } // end batch\n");
            }
            batch_num += 1;
            out.push_str(&format!(
                "    // Batch {}: {} templates\n",
                batch_num, BATCH_SIZE
            ));
            out.push_str("    {\n");
            out.push_str("        v8::scope!(let scope, scope);\n");
        }

        let fn_name = type_mapper::idl_name_to_rust(name);
        let domain = domain_of
            .get(name.as_str())
            .map(|d| d.as_str())
            .unwrap_or("web_apis");
        let domain_mod = domain.replace('-', "_");

        // Some interfaces use WebIDL `implements` (mixin) instead of `:`
        // (inheritance), so their `inheritance` field is None in the IR
        // even though they should inherit EventTarget. Add known overrides.
        const INHERITANCE_OVERRIDES: &[(&str, &str)] = &[
            ("Navigator", "EventTarget"),
            ("WorkerNavigator", "EventTarget"),
            ("Storage", "EventTarget"),
            ("XMLHttpRequestEventTarget", "EventTarget"),
            ("XMLHttpRequest", "XMLHttpRequestEventTarget"),
            ("XMLHttpRequestUpload", "XMLHttpRequestEventTarget"),
        ];
        let effective_parent = def.inheritance.as_ref().map(|s| s.as_str()).or_else(|| {
            INHERITANCE_OVERRIDES
                .iter()
                .find(|(iface, _)| *iface == name.as_str())
                .map(|(_, parent)| *parent)
        });

        let parent_code = match effective_parent {
            Some(p) => format!("templates.get(\"{}\").map(|g| v8::Local::new(scope, g))", p),
            None => "None".to_string(),
        };

        out.push_str(&format!(
            "        let tmpl_{0} = super::{dom}::create_{0}_template(scope, {parent});\n",
            fn_name,
            dom = domain_mod,
            parent = parent_code,
        ));
        out.push_str(&format!(
            "        templates.insert(\"{}\", v8::Global::new(scope, tmpl_{}));\n",
            name, fn_name,
        ));
        created += 1;
    }
    out.push_str("    } // end last batch\n\n");

    // Phase 2: Global registration with scope-break batches
    out.push_str("    // Register constructors on global (non-enumerable)\n");

    let reg_batch_size: usize = 100;
    let mut reg_count = 0;
    for name in sorted {
        let def = match by_name.get(name.as_str()) {
            Some(d) => d,
            None => continue,
        };
        let ea = process_interface_ea(def);
        if ea.no_interface_object {
            out.push_str(&format!(
                "    // {}: NoInterfaceObject — skip global registration\n",
                name
            ));
            continue;
        }

        if reg_count % reg_batch_size == 0 {
            if reg_count > 0 {
                out.push_str("    } // end registration batch\n");
            }
            out.push_str("    {\n");
            out.push_str("        v8::scope!(let scope, scope);\n");
        }

        let fn_name = type_mapper::idl_name_to_rust(name);
        out.push_str(&format!(
            "        if let Some(ctor_{0}) = templates.get(\"{1}\").map(|g| v8::Local::new(scope, g)).and_then(|t| t.get_function(scope)) {{\n",
            fn_name, name,
        ));
        out.push_str(&format!(
            "            let name_{0} = v8::String::new(scope, \"{1}\").unwrap();\n",
            fn_name, name,
        ));
        out.push_str(&format!(
            "            global.define_own_property(scope, name_{0}.into(), ctor_{0}.into(), v8::PropertyAttribute::DONT_ENUM);\n",
            fn_name,
        ));

        // Fix constructor __proto__ chain. V8 FunctionTemplate::inherit()
        // sets prototype.__proto__ but NOT constructor.__proto__.
        const INHERITANCE_OVERRIDES_REG: &[(&str, &str)] = &[
            ("Navigator", "EventTarget"),
            ("WorkerNavigator", "EventTarget"),
            ("Storage", "EventTarget"),
            ("XMLHttpRequestEventTarget", "EventTarget"),
            ("XMLHttpRequest", "XMLHttpRequestEventTarget"),
            ("XMLHttpRequestUpload", "XMLHttpRequestEventTarget"),
        ];
        let effective_parent_reg = def.inheritance.as_ref().map(|s| s.as_str()).or_else(|| {
            INHERITANCE_OVERRIDES_REG
                .iter()
                .find(|(iface, _)| *iface == name.as_str())
                .map(|(_, parent)| *parent)
        });
        if effective_parent_reg.is_some() {
            let parent = effective_parent_reg.unwrap();
            out.push_str(&format!(
                "            if let Some(pctor) = templates.get(\"{0}\").map(|g| v8::Local::new(scope, g)).and_then(|t| t.get_function(scope)) {{\n",
                parent,
            ));
            out.push_str(&format!(
                "                let cobj: v8::Local<v8::Object> = ctor_{}.into();\n",
                fn_name,
            ));
            out.push_str("                let _ = cobj.set_prototype(scope, pctor.into());\n");
            out.push_str("            }\n");
        }

        let const_members: Vec<(&String, &String)> = def
            .members
            .iter()
            .filter_map(|m| {
                if m.kind == "const" {
                    m.name.as_ref().and_then(|n| m.const_value.as_ref().map(|v| (n, v)))
                } else {
                    None
                }
            })
            .collect();
        for (cname, cval) in &const_members {
            let v8_val = format_const_v8_value(cval);
            out.push_str(&format!(
                "            {{ let ck = v8::String::new(scope, \"{}\").unwrap(); let cv = {}; ctor_{}.define_own_property(scope, ck.into(), cv, v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE); }}\n",
                cname, v8_val, fn_name,
            ));
        }

        out.push_str("        }\n");

        if let Some(ref alias) = ea.named_constructor {
            let alias_ident = alias.to_lowercase().replace('-', "_");
            out.push_str(&format!(
                "        // NamedConstructor alias: {}\n",
                alias
            ));
            out.push_str(&format!(
                "        if let Some(ctor_{0}) = templates.get(\"{1}\").map(|g| v8::Local::new(scope, g)).and_then(|t| t.get_function(scope)) {{\n",
                fn_name, name,
            ));
            out.push_str(&format!(
                "            let name_{0} = v8::String::new(scope, \"{1}\").unwrap();\n",
                alias_ident, alias,
            ));
            out.push_str(&format!(
                "            global.define_own_property(scope, name_{0}.into(), ctor_{1}.into(), v8::PropertyAttribute::DONT_ENUM);\n",
                alias_ident, fn_name,
            ));
            out.push_str("        }\n");
        }

        reg_count += 1;
    }
    out.push_str("    } // end last registration batch\n");

    out.push_str("}\n");

    // Phase 3 JS: For [Global] interfaces, move attributes from prototype to globalThis.
    // This is eval'd AFTER all installations (in freeze_all_prototypes) to avoid GC crash.
    let mut global_move_js = String::from("(function(){");
    for name in sorted {
        let def = match by_name.get(name.as_str()) {
            Some(d) => d,
            None => continue,
        };
        let ea = process_interface_ea(def);
        if !ea.is_global { continue; }
        let attrs: Vec<&str> = def.members.iter()
            .filter(|m| m.kind == "attribute" || m.kind == "operation")
            .filter_map(|m| m.name.as_deref())
            .collect();
        if attrs.is_empty() { continue; }
        let attrs_js: Vec<String> = attrs.iter().map(|a| format!("'{}'", a)).collect();
        global_move_js.push_str(&format!(
            "try{{var p={}.prototype;var names=[{}];for(var i=0;i<names.length;i++){{try{{var d=Object.getOwnPropertyDescriptor(p,names[i]);if(d){{Object.defineProperty(globalThis,names[i],d);delete p[names[i]];}}}}catch(e){{}}}}}}catch(e){{}}",
            name, attrs_js.join(","),
        ));
    }
    global_move_js.push_str("})();");

    out.push_str("\npub fn fix_accessor_properties(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {\n");
    let mut domains_set: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for d in domain_of.values() { domains_set.insert(d.as_str()); }
    for domain in &domains_set {
        let module = domain.replace('-', "_");
        out.push_str(&format!("    super::{}::fix_accessors_{}(scope, global);\n", module, module));
    }
    out.push_str("}\n");

    // Generate fix_global_accessor_properties: for [Global] interfaces,
    // define accessor properties directly on globalThis using Object::define_property.
    // This replaces GLOBAL_MOVE_JS and avoids V8 FunctionTemplate accessor limitation.
    out.push_str("\npub fn fix_global_accessor_properties(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {\n");
    let mut global_fix_count = 0;
    for name in sorted {
        let def = match by_name.get(name.as_str()) {
            Some(d) => d,
            None => continue,
        };
        let ea = process_interface_ea(def);
        if !ea.is_global { continue; }
        let fn_name = type_mapper::idl_name_to_rust(name);
        let domain = domain_of.get(name.as_str()).map(|s| s.as_str()).unwrap_or("web_apis");
        let module = domain.replace('-', "_");
        let mut idx = 0;
        for m in &def.members {
            if m.kind != "attribute" {
                if m.kind == "operation" { idx += 1; }
                continue;
            }
            let attr_name = match &m.name { Some(n) => n, None => continue };
            if should_skip_attribute(name, attr_name) { continue; }
            idx += 1;
            let has_setter = !m.readonly || m.has_put_forwards || m.has_replaceable;

            // For global attributes, define on globalThis directly.
            // Skip if already an accessor property (has get/set).
            // Data properties will be converted to accessor by JS post-fix
            // in freeze_all_prototypes (preserving their values).
            out.push_str("    {\n");
            out.push_str(&format!("        let attr_key = v8::String::new(scope, \"{}\").unwrap();\n", attr_name));
            out.push_str("        let should_skip = {\n");
            out.push_str("            let desc = global.get_own_property_descriptor(scope, attr_key.into());\n");
            out.push_str("            desc.is_some()\n");
            out.push_str("        };\n");
            out.push_str("        if should_skip { /* skip shim-set property */ } else {\n");
            out.push_str(&format!("        let getter_tmpl = v8::FunctionTemplate::builder_raw(super::{}::{}_get_{}).length(0).build(scope);\n", module, fn_name, idx));
            out.push_str(&format!("        getter_tmpl.set_class_name(v8::String::new(scope, \"get {}\").unwrap());\n", attr_name));
            out.push_str("        let getter_fn = getter_tmpl.get_function(scope).unwrap();\n");
            if has_setter {
                out.push_str(&format!("        let setter_tmpl = v8::FunctionTemplate::builder_raw(super::{}::{}_set_{}).length(1).build(scope);\n", module, fn_name, idx));
                out.push_str(&format!("        setter_tmpl.set_class_name(v8::String::new(scope, \"set {}\").unwrap());\n", attr_name));
                out.push_str("        let setter_fn = setter_tmpl.get_function(scope).unwrap();\n");
                out.push_str("        let mut desc = v8::PropertyDescriptor::new_from_get_set(getter_fn.into(), setter_fn.into());\n");
            } else {
                out.push_str("        let mut desc = v8::PropertyDescriptor::new_from_get_set(getter_fn.into(), v8::undefined(scope).into());\n");
            }
            out.push_str("        desc.set_enumerable(true);\n");
            out.push_str("        desc.set_configurable(true);\n");
            out.push_str("        let _ = global.define_property(scope, attr_key.into(), &desc);\n");
            out.push_str("        }\n");
            out.push_str("    }\n");
            global_fix_count += 1;
        }
    }
    out.push_str(&format!("    // Fixed {} global accessor properties\n", global_fix_count));
    out.push_str("}\n");

    // Generate fix_global_operation_lengths: fix .length of [Global] operations
    // on globalThis to match WebIDL minOverloadLength.
    // Shim-installed operations (window_extras.rs) have wrong .length.
    // This function uses Object.defineProperty to set correct .length.
    out.push_str("\npub fn fix_global_operation_lengths(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {\n");
    {
        let mut seen = std::collections::BTreeSet::new();
        for def in definitions {
            if def.kind != "interface" { continue; }
            let ea = process_interface_ea(def);
            if !ea.is_global { continue; }
            for m in &def.members {
                if m.kind != "operation" { continue; }
                let name = m.name.as_deref().unwrap_or("");
                if name.is_empty() || seen.contains(name) { continue; }
                seen.insert(name.to_string());
                // Calculate minOverloadLength across all overloads with same name
                let min_args = def
                    .members
                    .iter()
                    .filter(|m2| m2.kind == "operation" && m2.name.as_deref() == Some(name))
                    .map(|m2| m2.required_arg_count)
                    .min()
                    .unwrap_or(0);
                out.push_str(&format!("    {{\n"));
                out.push_str(&format!("        let key = v8::String::new(scope, \"{}\").unwrap();\n", name));
                out.push_str(&format!("        if let Some(fn_val) = global.get(scope, key.into()) {{\n"));
                out.push_str(&format!("            if fn_val.is_function() {{\n"));
                out.push_str(&format!("                let len_key = v8::String::new(scope, \"length\").unwrap();\n"));
                out.push_str(&format!("                let len_val = v8::Number::new(scope, {} as f64);\n", min_args));
                out.push_str(&format!("                let fn_obj: v8::Local<v8::Object> = unsafe {{ v8::Local::cast_unchecked(fn_val) }};\n"));
                out.push_str(&format!("                let _ = fn_obj.create_data_property(scope, len_key.into(), len_val.into());\n"));
                out.push_str(&format!("            }}\n"));
                out.push_str(&format!("        }}\n"));
                out.push_str(&format!("    }}\n"));
            }
        }
    }
    out.push_str("}\n");

    // Generate GLOBAL_ATTR_NAMES — list of all [Global] attribute names.
    // Used by freeze_all_prototypes JS post-fix to convert data properties
    // to accessor properties (preserving values) for idlharness compliance.
    out.push_str("\npub const GLOBAL_ATTR_NAMES: &[&str] = &[\n");
    {
        let mut seen = std::collections::BTreeSet::new();
        for def in definitions {
            if def.kind != "interface" { continue; }
            let ea = process_interface_ea(def);
            if !ea.is_global { continue; }
            for m in &def.members {
                if m.kind == "attribute" {
                    let name = m.name.as_deref().unwrap_or("");
                    if name.is_empty() || seen.contains(name) { continue; }
                    seen.insert(name.to_string());
                    out.push_str(&format!("    \"{}\",\n", name));
                }
            }
        }
    }
    out.push_str("];\n");

    // GLOBAL_MOVE_JS is now empty — replaced by fix_global_accessor_properties
    out.push_str("\npub const GLOBAL_MOVE_JS: &str = \"(function(){});\";\n");
    out
}

/// Generate mod.rs that aggregates all domain modules.
pub fn generate_mod_rs(domains: &[String]) -> String {
    let mut out = String::new();
    out.push_str("//! Generated FunctionTemplate stubs.\n\n");
    out.push_str("/// Empty constructor shared by all generated templates.\n");
    out.push_str("pub(crate) unsafe extern \"C\" fn empty_constructor(_info: *const v8::FunctionCallbackInfo) {}\n\n");
    out.push_str("/// Construct-only constructor — creates an empty object via `new`.\n");
    out.push_str("/// Used for constructable interfaces (EventTarget, etc.) so that\n");
    out.push_str("/// `new EventTarget()` does not throw.\n");
    out.push_str("pub(crate) unsafe extern \"C\" fn construct_only(info: *const v8::FunctionCallbackInfo) {\n");
    out.push_str("    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {\n");
    out.push_str("        let info_ref = unsafe { &*info };\n");
    out.push_str("        v8::callback_scope!(unsafe scope, info_ref);\n");
    out.push_str("        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);\n");
    out.push_str("        if !args.is_construct_call() {\n");
    out.push_str("            let msg = v8::String::new(scope, \"Failed to construct: Please use the 'new' operator\").unwrap();\n");
    out.push_str("            let exc = v8::Exception::type_error(scope, msg);\n");
    out.push_str("            scope.throw_exception(exc);\n");
    out.push_str("        }\n");
    out.push_str("    }));\n");
    out.push_str("}\n\n");
    out.push_str("/// Illegal constructor — throws TypeError, matching real browser behavior for\n");
    out.push_str("/// non-constructable Web IDL interfaces.\n");
    out.push_str("pub(crate) unsafe extern \"C\" fn illegal_constructor(info: *const v8::FunctionCallbackInfo) {\n");
    out.push_str("    let info_ref = unsafe { &*info };\n");
    out.push_str("    v8::callback_scope!(unsafe scope, info_ref);\n");
    out.push_str("    let msg = v8::String::new(scope, \"Illegal constructor\").unwrap();\n");
    out.push_str("    let exc = v8::Exception::type_error(scope, msg);\n");
    out.push_str("    scope.throw_exception(exc);\n");
    out.push_str("}\n\n");
    for domain in domains {
        out.push_str(&format!("pub mod {};\n", domain.replace('-', "_")));
    }
    out.push_str("\npub mod install_all;\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Definition, MemberData};

    #[test]
    fn test_simple_interface_has_tostringtag() {
        let def = Definition {
            kind: "interface".into(),
            name: Some("Foo".into()),
            source: Some("w3c".into()),
            inheritance: None,
            ext_attrs: vec![],
            partial: false,
            values: vec![],
            target: None,
            includes: None,
            members: vec![MemberData {
                kind: "attribute".into(),
                name: Some("bar".into()),
                idl_type: Some("DOMString".into()),
                readonly: false,
                has_put_forwards: false,
                has_replaceable: false,
                return_type: None,
                arguments: vec![],
                const_value: None,
                required_arg_count: 0,
            }],
        };
        let fn_name = type_mapper::idl_name_to_rust("Foo");
        let ea = process_interface_ea(&def);
        let code = generate_template_function(&def, &ea, &fn_name);
        assert!(code.contains("get_to_string_tag"), "toStringTag missing");
        assert!(code.contains("\"Foo\""), "class name missing");
    }

    #[test]
    fn test_operation_generates_callback() {
        let def = Definition {
            kind: "interface".into(),
            name: Some("Bar".into()),
            source: Some("w3c".into()),
            inheritance: None,
            ext_attrs: vec![],
            partial: false,
            values: vec![],
            target: None,
            includes: None,
            members: vec![MemberData {
                kind: "operation".into(),
                name: Some("doThing".into()),
                idl_type: None,
                readonly: false,
                has_put_forwards: false,
                has_replaceable: false,
                return_type: Some("undefined".into()),
                arguments: vec![],
                const_value: None,
                required_arg_count: 0,
            }],
        };
        let cb = generate_callbacks(&def, "bar");
        assert!(cb.contains("bar_op_1"), "operation callback missing");
        assert!(
            cb.contains("doThing") == false,
            "op name should not be in callback code"
        );
    }

    fn make_empty_def(name: &str) -> Definition {
        Definition {
            kind: "interface".into(),
            name: Some(name.into()),
            source: Some("w3c".into()),
            inheritance: None,
            ext_attrs: vec![],
            members: vec![],
            partial: false,
            values: vec![],
            target: None,
            includes: None,
        }
    }

    #[test]
    fn test_non_constructable_uses_illegal_constructor() {
        for name in &["Node", "Element", "HTMLElement", "Window", "Crypto"] {
            let def = make_empty_def(name);
            let fn_name = type_mapper::idl_name_to_rust(name);
            let ea = process_interface_ea(&def);
            let code = generate_template_function(&def, &ea, &fn_name);
            assert!(
                code.contains("builder_raw(illegal_constructor)"),
                "{} should use illegal_constructor",
                name
            );
            assert!(
                !code.contains("builder_raw(construct_only)"),
                "{} must not use construct_only",
                name
            );
        }
    }

    fn make_constructable_def(name: &str) -> Definition {
        let mut def = make_empty_def(name);
        def.members.push(MemberData {
            kind: "constructor".into(),
            name: None,
            idl_type: None,
            readonly: false,
            has_put_forwards: false,
            has_replaceable: false,
            return_type: None,
            arguments: vec![],
            const_value: None,
            required_arg_count: 0,
        });
        def
    }

    #[test]
    fn test_constructable_uses_construct_only() {
        let def = make_constructable_def("AbortController");
        let fn_name = type_mapper::idl_name_to_rust("AbortController");
        let ea = process_interface_ea(&def);
        let code = generate_template_function(&def, &ea, &fn_name);
        assert!(
            code.contains("builder_raw(construct_only)"),
            "interface with constructor member should use construct_only"
        );

        let mut def2 = make_empty_def("HTMLImageElement");
        def2.ext_attrs.push("LegacyFactoryFunction=Image".into());
        let fn_name2 = type_mapper::idl_name_to_rust("HTMLImageElement");
        let ea2 = process_interface_ea(&def2);
        let code2 = generate_template_function(&def2, &ea2, &fn_name2);
        assert!(
            code2.contains("builder_raw(construct_only)"),
            "interface with LegacyFactoryFunction should use construct_only"
        );
    }

    #[test]
    fn test_callback_interface_is_non_constructable() {
        let mut def = make_empty_def("NodeFilter");
        def.kind = "callback_interface".into();
        def.members.push(MemberData {
            kind: "const".into(),
            name: Some("FILTER_ACCEPT".into()),
            idl_type: Some("unsigned short".into()),
            readonly: false,
            has_put_forwards: false,
            has_replaceable: false,
            return_type: None,
            arguments: vec![],
            const_value: Some("1".into()),
            required_arg_count: 0,
        });
        let fn_name = type_mapper::idl_name_to_rust("NodeFilter");
        let ea = process_interface_ea(&def);
        let code = generate_template_function(&def, &ea, &fn_name);
        assert!(
            code.contains("builder_raw(illegal_constructor)"),
            "callback_interface must use illegal_constructor even with consts"
        );
    }

    #[test]
    fn test_mod_rs_defines_illegal_constructor() {
        let mod_content = generate_mod_rs(&["web_apis".to_string()]);
        assert!(
            mod_content.contains("fn illegal_constructor"),
            "mod.rs must define illegal_constructor"
        );
        assert!(
            mod_content.contains("Illegal constructor"),
            "mod.rs illegal_constructor must throw"
        );
        assert!(
            mod_content.contains("fn construct_only"),
            "mod.rs must define construct_only"
        );
    }

    #[test]
    fn test_domain_file_imports_illegal_when_needed() {
        let defs = vec![make_empty_def("Node")];
        let def_refs: Vec<&Definition> = defs.iter().collect();
        let by_name: BTreeMap<String, &Definition> = defs
            .iter()
            .map(|d| (d.name.clone().unwrap(), d))
            .collect();
        let content = generate_domain_file("dom_core", &def_refs, &by_name);
        assert!(
            content.contains("use super::{construct_only, illegal_constructor};"),
            "domain with non-constructable interface must import illegal_constructor"
        );
    }

    #[test]
    fn test_domain_file_omits_illegal_when_not_needed() {
        let defs = vec![make_constructable_def("AbortController")];
        let def_refs: Vec<&Definition> = defs.iter().collect();
        let by_name: BTreeMap<String, &Definition> = defs
            .iter()
            .map(|d| (d.name.clone().unwrap(), d))
            .collect();
        let content = generate_domain_file("dom_core", &def_refs, &by_name);
        assert!(
            content.contains("use super::construct_only;")
                && !content.contains("illegal_constructor"),
            "domain without non-constructable interface must not import illegal_constructor"
        );
    }
}
