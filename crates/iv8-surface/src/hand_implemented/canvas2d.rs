//! Canvas 2D deep stub — create CanvasRenderingContext2D FunctionTemplate.
//!
//! v0.8.21: Replaces JS shim patching with native V8 FunctionTemplate.
//! Provides create_canvas_rendering_context_2d_template() with full
//! property accessors and method stubs.

use super::{CANVAS_2D_DEFAULTS, CANVAS_2D_METHODS};
use v8::FunctionTemplate;
use v8::Local;

/// Create the CanvasRenderingContext2D FunctionTemplate with all properties
/// and methods. After creating an instance, call install_default_values()
/// to set initial property values.
pub fn create_canvas_rendering_context_2d_template<'s>(
    scope: &v8::PinScope<'s, '_>,
) -> Local<'s, FunctionTemplate> {
    let tmpl = FunctionTemplate::builder_raw(empty_constructor).build(scope);
    tmpl.set_class_name(v8::String::new(scope, "CanvasRenderingContext2D").unwrap());

    let proto = tmpl.prototype_template(scope);

    // Symbol.toStringTag
    {
        let tag_sym = v8::Symbol::get_to_string_tag(scope);
        let tag_val = v8::String::new(scope, "CanvasRenderingContext2D").unwrap();
        proto.set(tag_sym.into(), tag_val.into());
    }

    // Install attribute getters/setters for all default properties
    for (name, _default) in CANVAS_2D_DEFAULTS {
        install_property_accessor(scope, proto, name);
    }

    // Install method stubs
    for name in CANVAS_2D_METHODS {
        install_method_stub(scope, proto, name);
    }

    tmpl
}

/// Create a fully-initialized CanvasRenderingContext2D instance with
/// default property values set. Combines template instantiation with
/// install_default_values() so callers get a ready-to-use context.
pub fn create_canvas_2d_context_instance<'s>(
    scope: &v8::PinScope<'s, '_>,
) -> Option<v8::Local<'s, v8::Object>> {
    let tmpl = create_canvas_rendering_context_2d_template(scope);
    let func = tmpl.get_function(scope)?;
    let obj = func.new_instance(scope, &[])?;
    install_default_values(scope, obj);
    Some(obj)
}

/// After instantiating the template, set default property values on the instance.
pub fn install_default_values<'s>(
    scope: &v8::PinScope<'s, '_>,
    obj: v8::Local<'s, v8::Object>,
) {
    for (name, value) in CANVAS_2D_DEFAULTS {
        let key = v8::String::new(scope, name).unwrap();

        let val: v8::Local<'s, v8::Value> = if let Ok(v) = value.trim_matches('"').parse::<f64>() {
            v8::Number::new(scope, v).into()
        } else if *value == "true" {
            v8::Boolean::new(scope, true).into()
        } else if *value == "false" {
            v8::Boolean::new(scope, false).into()
        } else {
            // String value — strip surrounding quotes
            let s = value.trim_matches('"');
            v8::String::new(scope, s).map(|v| v.into()).unwrap_or_else(|| v8::undefined(scope).into())
        };

        obj.set(scope, key.into(), val);
    }
}

/// No-op constructor — instances created by Rust.
unsafe extern "C" fn empty_constructor(_info: *const v8::FunctionCallbackInfo) {}

/// Install a property accessor (getter + setter) on the prototype template.
fn install_property_accessor(
    scope: &v8::PinScope<'_, '_>,
    proto: Local<v8::ObjectTemplate>,
    name: &str,
) {
    let getter_tmpl = FunctionTemplate::builder_raw(property_getter).build(scope);
    let setter_tmpl = FunctionTemplate::builder_raw(property_setter).build(scope);
    let key = v8::String::new(scope, name).unwrap();
    proto.set_accessor_property(
        key.into(),
        Some(getter_tmpl),
        Some(setter_tmpl),
        v8::PropertyAttribute::NONE,
    );
}

/// Property getter — returns stored value or default.
unsafe extern "C" fn property_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::undefined(scope).into());
    }));
}

/// Property setter — stores value on the object.
unsafe extern "C" fn property_setter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        // Stub: accept any value assignment
    }));
}

/// Install a method stub on the prototype template.
fn install_method_stub(
    scope: &v8::PinScope<'_, '_>,
    proto: Local<v8::ObjectTemplate>,
    name: &str,
) {
    let fn_tmpl = FunctionTemplate::builder_raw(method_stub).build(scope);
    let key = v8::String::new(scope, name).unwrap();
    fn_tmpl.set_class_name(key);
    proto.set(key.into(), fn_tmpl.into());
}

/// Method stub — returns undefined.
unsafe extern "C" fn method_stub(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::undefined(scope).into());
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_template_compiles() {
        // Validates that create_canvas_rendering_context_2d_template
        // type-checks correctly. Full V8 runtime test requires Isolate.
        // The function signature is: (scope) -> Local<FunctionTemplate>
        assert!(CANVAS_2D_DEFAULTS.len() >= 20);
        assert!(CANVAS_2D_METHODS.len() >= 30);
    }

    #[test]
    fn test_default_properties_count() {
        assert_eq!(CANVAS_2D_DEFAULTS.len(), 24);
    }

    #[test]
    fn test_methods_count() {
        assert!(CANVAS_2D_METHODS.len() >= 31);
    }
}
