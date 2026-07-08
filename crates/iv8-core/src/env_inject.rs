//! Environment injection: expose environment dot-path values to JS global.
//!
//! Creates sub-objects (navigator, screen, window, document, etc.) on the global
//! and installs NamedPropertyHandler to serve values from EnvironmentMap.

use crate::state::RuntimeState;
use std::collections::HashMap;

/// Keys owned by native navigator getters (installed via native_env.rs
/// on Navigator.prototype). env_inject must skip these to avoid
/// READ_ONLY own data property shadowing the native accessor.
///
/// Includes keys that may not yet exist as direct entries in
/// iv8-defaults.json (e.g. languages, plugins). Those entries are
/// harmless: the skip check simply never matches them.
const NATIVE_NAVIGATOR_KEYS: &[&str] = &[
    "userAgent",
    "appVersion",
    "platform",
    "vendor",
    "vendorSub",
    "product",
    "productSub",
    "language",
    "languages",
    "hardwareConcurrency",
    "deviceMemory",
    "maxTouchPoints",
    "cookieEnabled",
    "onLine",
    "doNotTrack",
    "webdriver",
    "appName",
    "appCodeName",
    "permissions",
    "mediaDevices",
    "serviceWorker",
    "pdfViewerEnabled",
    "plugins",
    "mimeTypes",
    "connection",
    "geolocation",
    "clipboard",
    "credentials",
];

/// Keys owned by native screen getters (installed via native_env.rs
/// on Screen.prototype). env_inject must skip these to avoid
/// READ_ONLY own data property shadowing the native accessor.
const NATIVE_SCREEN_KEYS: &[&str] = &[
    "width",
    "height",
    "availWidth",
    "availHeight",
    "colorDepth",
    "pixelDepth",
    "availLeft",
    "availTop",
];

/// Keys owned by native window getters (installed via global_template).
/// env_inject must skip these to avoid READ_ONLY own data property
/// shadowing the native accessor installed in embedded_v8.rs.
const NATIVE_WINDOW_KEYS: &[&str] = &[
    "innerWidth",
    "innerHeight",
    "outerWidth",
    "outerHeight",
    "devicePixelRatio",
];

/// Install all environment fields into the V8 global object.
/// Called once during JSContext creation, after RuntimeState is installed.
pub fn install_environment(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let state = RuntimeState::get(scope);
    let env = &state.environment;

    // Group dot-paths by their top-level prefix
    // e.g. "navigator.userAgent" → prefix "navigator", field "userAgent"
    let mut groups: HashMap<String, Vec<(String, serde_json::Value)>> = HashMap::new();

    for (path, value) in env.iter() {
        if let Some(dot_pos) = path.find('.') {
            let prefix = &path[..dot_pos];
            let field = &path[dot_pos + 1..];
            groups
                .entry(prefix.to_string())
                .or_default()
                .push((field.to_string(), value.clone()));
        }
        // Top-level keys without dots are handled separately if needed
    }

    // For each prefix group, create a JS object and set properties
    for (prefix, fields) in &groups {
        // Special case: "window" fields should be installed on globalThis itself
        // (window IS globalThis in browsers)
        let obj = if prefix == "window" {
            global
        } else {
            get_or_create_object(scope, global, prefix)
        };
        install_fields_on_object(scope, obj, fields, prefix);
    }

    // Set window = globalThis (self-reference)
    let window_key = crate::v8_utils::v8_string(scope, "window");
    global.set(scope, window_key.into(), global.into());

    // Set self = globalThis
    let self_key = crate::v8_utils::v8_string(scope, "self");
    global.set(scope, self_key.into(), global.into());
}

/// Get an existing property as object, or create a new empty object.
fn get_or_create_object<'s>(
    scope: &'s v8::PinScope<'s, '_>,
    parent: v8::Local<'s, v8::Object>,
    name: &str,
) -> v8::Local<'s, v8::Object> {
    let key = crate::v8_utils::v8_string(scope, name);

    // Check if it already exists
    if let Some(existing) = parent.get(scope, key.into()) {
        if existing.is_object() && !existing.is_null_or_undefined() {
            // SAFETY: we just checked is_object
            return unsafe { v8::Local::cast_unchecked(existing) };
        }
    }

    // Create new object and set it on parent
    let obj = v8::Object::new(scope);
    parent.set(scope, key.into(), obj.into());
    obj
}

/// Install fields on a JS object using lazy data properties.
/// For nested paths (e.g. "userAgentData.brands"), creates sub-objects recursively.
fn install_fields_on_object(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    fields: &[(String, serde_json::Value)],
    prefix: &str,
) {
    // Separate direct fields from nested ones
    let mut direct: Vec<(&str, &serde_json::Value)> = Vec::new();
    let mut nested: HashMap<String, Vec<(String, serde_json::Value)>> = HashMap::new();

    for (field, value) in fields {
        if let Some(dot_pos) = field.find('.') {
            let sub_prefix = &field[..dot_pos];
            let sub_field = &field[dot_pos + 1..];
            nested
                .entry(sub_prefix.to_string())
                .or_default()
                .push((sub_field.to_string(), value.clone()));
        } else {
            direct.push((field.as_str(), value));
        }
    }

    // Set direct fields with ReadOnly + DontDelete
    // (prevents JS from modifying/deleting)
    for (field_name, value) in &direct {
        // Skip native-owned navigator keys — they get native
        // accessors via native_env.rs Navigator.prototype.
        if prefix == "navigator"
            && NATIVE_NAVIGATOR_KEYS.contains(field_name)
        {
            continue;
        }
        // Skip native-owned screen keys — they get native
        // accessors via native_env.rs Screen.prototype.
        if prefix == "screen"
            && NATIVE_SCREEN_KEYS.contains(field_name)
        {
            continue;
        }
        // Skip native-owned window keys — they get native
        // accessors via global_template in embedded_v8.rs.
        if prefix == "window"
            && NATIVE_WINDOW_KEYS.contains(field_name)
        {
            continue;
        }
        let key = crate::v8_utils::v8_string(scope, field_name);
        let v8_value = json_to_v8(scope, value);
        obj.define_own_property(
            scope,
            key.into(),
            v8_value,
            v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE,
        );
    }

    // Recursively handle nested fields
    for (sub_prefix, sub_fields) in &nested {
        let sub_obj = get_or_create_object(scope, obj, sub_prefix);
        install_fields_on_object(
            scope,
            sub_obj,
            sub_fields,
            &format!("{prefix}.{sub_prefix}"),
        );
    }
}

/// Convert a serde_json::Value to a v8::Local<Value>.
fn json_to_v8<'s>(
    scope: &'s v8::PinScope<'s, '_>,
    value: &serde_json::Value,
) -> v8::Local<'s, v8::Value> {
    match value {
        serde_json::Value::Null => v8::null(scope).into(),
        serde_json::Value::Bool(b) => v8::Boolean::new(scope, *b).into(),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                v8::Integer::new(scope, i as i32).into()
            } else if let Some(f) = n.as_f64() {
                v8::Number::new(scope, f).into()
            } else {
                v8::undefined(scope).into()
            }
        }
        serde_json::Value::String(s) => v8::String::new(scope, s)
            .map(|s| s.into())
            .unwrap_or_else(|| v8::undefined(scope).into()),
        serde_json::Value::Array(arr) => {
            let v8_arr = v8::Array::new(scope, arr.len() as i32);
            for (i, item) in arr.iter().enumerate() {
                let v8_item = json_to_v8(scope, item);
                v8_arr.set_index(scope, i as u32, v8_item);
            }
            v8_arr.into()
        }
        serde_json::Value::Object(map) => {
            let obj = v8::Object::new(scope);
            for (k, v) in map {
                let key = crate::v8_utils::v8_string(scope, k);
                let val = json_to_v8(scope, v);
                obj.set(scope, key.into(), val);
            }
            obj.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::embedded_v8::EmbeddedV8Kernel;
    use crate::kernel::{EvalOpts, KernelConfig};
    use crate::RustValue;

    fn make_kernel_with_env() -> EmbeddedV8Kernel {
        EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
    }

    #[test]
    fn navigator_user_agent_accessible() {
        let mut kernel = make_kernel_with_env();
        // Install environment into the context
        kernel.install_environment();
        let result = kernel.eval_to_rust_value("navigator.userAgent");
        match result {
            RustValue::String(ua) => {
                assert!(ua.contains("Chrome"), "UA should contain Chrome: {}", ua);
            }
            other => panic!("expected String, got {:?}", other),
        }
    }

    #[test]
    fn screen_width_accessible() {
        let mut kernel = make_kernel_with_env();
        kernel.install_environment();
        let result = kernel.eval_to_rust_value("screen.width");
        match result {
            RustValue::Int(w) => assert!(w > 0, "screen.width should be positive: {}", w),
            RustValue::Float(w) => assert!(w > 0.0, "screen.width should be positive: {}", w),
            other => panic!("expected number, got {:?}", other),
        }
    }

    #[test]
    fn nested_field_accessible() {
        let mut kernel = make_kernel_with_env();
        kernel.install_environment();
        // navigator.connection.effectiveType should exist
        let result =
            kernel.eval_to_rust_value("navigator.connection && navigator.connection.effectiveType");
        // May be null if not in defaults, but shouldn't throw
        assert!(
            !matches!(result, RustValue::JsObject(_)),
            "should not be an error object: {:?}",
            result
        );
    }

    #[test]
    fn document_ready_state() {
        let mut kernel = make_kernel_with_env();
        kernel.install_environment();
        let result = kernel.eval_to_rust_value("document.readyState");
        // Should be a string from defaults
        match result {
            RustValue::String(_) | RustValue::Null => {} // both acceptable
            other => panic!("expected String or Null, got {:?}", other),
        }
    }

    #[test]
    fn environment_override_works() {
        let mut overrides = std::collections::HashMap::new();
        overrides.insert(
            "navigator.userAgent".to_string(),
            serde_json::Value::String("TestBot/1.0".to_string()),
        );
        let config = KernelConfig {
            environment_overrides: Some(overrides),
            ..KernelConfig::default()
        };
        let mut kernel = EmbeddedV8Kernel::new(config).unwrap();
        kernel.install_environment();
        let result = kernel.eval_to_rust_value("navigator.userAgent");
        assert_eq!(result, RustValue::String("TestBot/1.0".into()));
    }

    #[test]
    fn navigator_user_agent_not_deletable() {
        let mut kernel = make_kernel_with_env();
        kernel.install_environment();
        // navigator.userAgent lives on Navigator.prototype (not own property),
        // so delete on the instance is a no-op that returns true — matching Chrome.
        let result = kernel.eval_to_rust_value("'use strict'; delete navigator.userAgent");
        assert_eq!(result, RustValue::Bool(true));
    }

    #[test]
    fn navigator_user_agent_not_writable() {
        let mut kernel = make_kernel_with_env();
        kernel.install_environment();
        let result = kernel.eval_to_rust_value(
            "'use strict'; try { navigator.userAgent = 'hacked'; 'written' } catch(e) { 'protected' }"
        );
        assert_eq!(result, RustValue::String("protected".into()));
    }

    #[test]
    fn navigator_user_agent_not_own_property() {
        let mut kernel = make_kernel_with_env();
        kernel.install_environment();
        // After v0.8.69, navigator.userAgent must NOT be an own
        // data property. It should be a prototype accessor from
        // native_env.rs. hasOwnProperty must return false.
        let result = kernel.eval_to_rust_value(
            "Object.prototype.hasOwnProperty\
             .call(navigator, 'userAgent')",
        );
        assert_eq!(
            result,
            RustValue::Bool(false),
            "navigator.userAgent must not be own property"
        );
    }

    #[test]
    fn screen_width_not_own_property() {
        let mut kernel = make_kernel_with_env();
        kernel.install_environment();
        // After v0.8.69, screen.width must NOT be an own data
        // property. It should be a prototype accessor from
        // native_env.rs. hasOwnProperty must return false.
        let result = kernel.eval_to_rust_value(
            "Object.prototype.hasOwnProperty\
             .call(screen, 'width')",
        );
        assert_eq!(
            result,
            RustValue::Bool(false),
            "screen.width must not be own property"
        );
    }

    #[test]
    fn document_ready_state_is_accessor_not_data() {
        let mut kernel = make_kernel_with_env();
        // P1-DESC: document.readyState is now an accessor property (not data),
        // matching Chrome's WebIDL descriptor shape.
        // Kernel::new() runs full init including install_undetect_shims
        // which evals DOCUMENT_PROPS_JS that installs the accessor.
        let result = kernel.eval_to_rust_value(
            "var d = Object.getOwnPropertyDescriptor(document, 'readyState');\
             d && typeof d.get === 'function' && d.set === undefined",
        );
        assert_eq!(
            result,
            RustValue::Bool(true),
            "document.readyState must be accessor with getter and no setter"
        );
    }
}
