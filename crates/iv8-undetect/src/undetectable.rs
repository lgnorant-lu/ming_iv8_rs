//! MarkAsUndetectable: implement [[IsHTMLDDA]] for document.all and __iv8__ tool object.
//!
//! NOTE: v8 crate 147 does NOT expose `ObjectTemplate::MarkAsUndetectable()`.
//! This is a known gap (the C++ API exists but the Rust binding doesn't wrap it).
//!
//! For v0.1, we use a JS-level workaround:
//! - `__iv8__` is installed as DontEnum (not in Object.keys) but typeof will be "object"
//! - `document.all` uses the same approach
//!
//! TODO (M1 follow-up): Either:
//! 1. Fork v8 crate and add MarkAsUndetectable binding
//! 2. Use raw FFI to call the C++ API directly
//! 3. Upstream a PR to denoland/rusty_v8
//!
//! Without the real MarkAsUndetectable:
//! - `typeof __iv8__` will be "object" (not "undefined") [FAIL]
//! - `__iv8__ == null` will be false (not true) [FAIL]
//! - `'__iv8__' in window` will be true [OK]
//! - `Object.keys(window).includes('__iv8__')` will be false [OK]

/// Install the `__iv8__` tool object on the global with DontEnum attribute.
/// Without MarkAsUndetectable, typeof will be "object" (not "undefined").
/// This is a v0.1 limitation — see module doc for follow-up plan.
pub fn install_iv8_tool_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    global: v8::Local<'s, v8::Object>,
    name: &str,
) -> v8::Local<'s, v8::Object> {
    let tool_obj = v8::Object::new(scope);

    // Install on global with DontEnum (Object.keys won't show it)
    let key = v8::String::new(scope, name).expect("key");
    global.define_own_property(
        scope,
        key.into(),
        tool_obj.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    tool_obj
}

/// Install `document.all` on the document object with DontEnum.
/// Without MarkAsUndetectable, typeof will be "object" (not "undefined").
/// This is a v0.1 limitation.
pub fn install_document_all(scope: &v8::PinScope<'_, '_>, document: v8::Local<v8::Object>) {
    let all_obj = v8::Object::new(scope);

    let key = v8::String::new(scope, "all").expect("key");
    document.define_own_property(
        scope,
        key.into(),
        all_obj.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_with_setup(setup: impl FnOnce(&v8::PinScope<'_, '_>, v8::Local<v8::Object>), source: &str) -> String {
        iv8_core::v8_init::ensure_v8_initialized();
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());

        v8::scope!(handle_scope, &mut isolate);
        let context = v8::Context::new(handle_scope, Default::default());
        v8::scope_with_context!(scope, handle_scope, context);

        let global = context.global(scope);
        setup(scope, global);

        let source_str = v8::String::new(scope, source).unwrap();
        v8::tc_scope!(tc, scope);
        let script = v8::Script::compile(tc, source_str, None).unwrap();
        let result = script.run(tc).unwrap();
        result.to_rust_string_lossy(tc)
    }

    #[test]
    fn iv8_tool_in_operator_true() {
        let result = eval_with_setup(
            |scope, global| { install_iv8_tool_object(scope, global, "__iv8__"); },
            "'__iv8__' in this",
        );
        assert_eq!(result, "true");
    }

    #[test]
    fn iv8_tool_not_in_object_keys() {
        let result = eval_with_setup(
            |scope, global| { install_iv8_tool_object(scope, global, "__iv8__"); },
            "Object.keys(this).includes('__iv8__')",
        );
        assert_eq!(result, "false");
    }

    #[test]
    fn iv8_tool_is_object() {
        // Without MarkAsUndetectable, typeof is "object" (known v0.1 limitation)
        let result = eval_with_setup(
            |scope, global| { install_iv8_tool_object(scope, global, "__iv8__"); },
            "typeof __iv8__",
        );
        // v0.1: "object" (not "undefined" — that requires MarkAsUndetectable)
        assert_eq!(result, "object");
    }

    #[test]
    fn iv8_tool_accessible() {
        let result = eval_with_setup(
            |scope, global| { install_iv8_tool_object(scope, global, "__iv8__"); },
            "__iv8__ !== undefined",
        );
        assert_eq!(result, "true");
    }

    #[test]
    fn document_all_accessible() {
        let result = eval_with_setup(
            |scope, global| {
                let doc = v8::Object::new(scope);
                let key = v8::String::new(scope, "document").unwrap();
                global.set(scope, key.into(), doc.into());
                install_document_all(scope, doc);
            },
            "document.all !== undefined",
        );
        assert_eq!(result, "true");
    }

    #[test]
    fn document_all_not_in_keys() {
        let result = eval_with_setup(
            |scope, global| {
                let doc = v8::Object::new(scope);
                let key = v8::String::new(scope, "document").unwrap();
                global.set(scope, key.into(), doc.into());
                install_document_all(scope, doc);
            },
            "Object.keys(document).includes('all')",
        );
        assert_eq!(result, "false");
    }
}
