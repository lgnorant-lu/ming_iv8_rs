//! MarkAsUndetectable: implement [[IsHTMLDDA]] for `__iv8__` tool object.
//!
//! v0.2: Uses real V8 `MarkAsUndetectable` via [`iv8_core::v8_extra`] (compiled
//! through cc crate; see crates/iv8-core/cxx/iv8_v8_extra.cc).
//!
//! With real MarkAsUndetectable installed:
//! - `typeof __iv8__` returns `"undefined"` (was "object" in v0.1)
//! - `__iv8__ == null` returns `true` (was false in v0.1)
//! - `Boolean(__iv8__)` returns `false` (was true in v0.1)
//! - `if (__iv8__) { ... }` does not enter (was true in v0.1)
//! - `'__iv8__' in window` still returns `true` (unchanged)
//! - Properties on `__iv8__` remain accessible (`__iv8__.page.load`, etc.)
//!
//! V8 invariant: MarkAsUndetectable requires CallAsFunctionHandler. We install
//! a no-op handler (the tool object is not meant to be called as a function;
//! the handler exists only to satisfy V8's runtime check).

use iv8_core::v8_extra;

/// No-op CallAsFunctionHandler. Returns undefined.
///
/// Required because V8 asserts that an undetectable ObjectTemplate has a
/// call handler when an instance is created. The `__iv8__` tool object is
/// not designed to be invoked as a function (`__iv8__()`), so this handler
/// just returns undefined silently.
fn iv8_tool_call_handler(
    _scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
}

/// Install the `__iv8__` tool object on the global with [[IsHTMLDDA]] semantics.
///
/// The returned object is the `__iv8__` instance. Subsequent calls (like
/// `wrap_native::install`) will populate it with methods.
///
/// Properties:
/// - `typeof __iv8__ === 'undefined'` (via MarkAsUndetectable)
/// - Not enumerable on `window` (via DontEnum)
/// - Properties accessible (e.g. `__iv8__.wrapNative` works after install)
pub fn install_iv8_tool_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    global: v8::Local<'s, v8::Object>,
    name: &str,
) -> v8::Local<'s, v8::Object> {
    // Build an ObjectTemplate with both undetectable + call handler set
    // (V8 requires both together).
    let templ = v8::ObjectTemplate::new(scope);
    v8_extra::mark_as_undetectable(&templ);
    v8_extra::set_call_as_function_handler(
        &templ,
        v8::MapFnTo::map_fn_to(iv8_tool_call_handler),
        None,
    );
    let tool_obj = templ
        .new_instance(scope)
        .expect("failed to create __iv8__ undetectable instance");

    // Install on global with DontEnum so Object.keys does not show it
    let key = v8::String::new(scope, name).expect("key");
    global.define_own_property(
        scope,
        key.into(),
        tool_obj.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    tool_obj
}

/// Install `document.all` on the document object as a callable HTMLDDA object.
///
/// Mirrors HTMLAllCollection semantics:
/// - `typeof document.all === 'undefined'`
/// - `document.all == null` is true
/// - `document.all('id')` calls the call handler (currently a stub returning
///   undefined; full getElementById integration is a v0.2 follow-up once DOM
///   is wired into the call path).
pub fn install_document_all(scope: &v8::PinScope<'_, '_>, document: v8::Local<v8::Object>) {
    let templ = v8::ObjectTemplate::new(scope);
    v8_extra::mark_as_undetectable(&templ);
    // For now, document.all('id') returns undefined. Full integration with
    // the DOM tree (getElementById) is tracked as a v0.2 follow-up.
    v8_extra::set_call_as_function_handler(
        &templ,
        v8::MapFnTo::map_fn_to(iv8_tool_call_handler),
        None,
    );
    let all_obj = templ
        .new_instance(scope)
        .expect("failed to create document.all undetectable instance");

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

    fn eval_with_setup(
        setup: impl FnOnce(&v8::PinScope<'_, '_>, v8::Local<v8::Object>),
        source: &str,
    ) -> String {
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
            |scope, global| {
                install_iv8_tool_object(scope, global, "__iv8__");
            },
            "'__iv8__' in this",
        );
        assert_eq!(result, "true");
    }

    #[test]
    fn iv8_tool_not_in_object_keys() {
        let result = eval_with_setup(
            |scope, global| {
                install_iv8_tool_object(scope, global, "__iv8__");
            },
            "Object.keys(this).includes('__iv8__')",
        );
        assert_eq!(result, "false");
    }

    #[test]
    fn iv8_tool_typeof_is_undefined() {
        // v0.2: with real MarkAsUndetectable, typeof returns "undefined".
        let result = eval_with_setup(
            |scope, global| {
                install_iv8_tool_object(scope, global, "__iv8__");
            },
            "typeof __iv8__",
        );
        assert_eq!(result, "undefined");
    }

    #[test]
    fn iv8_tool_loose_equals_null() {
        let result = eval_with_setup(
            |scope, global| {
                install_iv8_tool_object(scope, global, "__iv8__");
            },
            "__iv8__ == null",
        );
        assert_eq!(result, "true");
    }

    #[test]
    fn iv8_tool_boolean_is_false() {
        let result = eval_with_setup(
            |scope, global| {
                install_iv8_tool_object(scope, global, "__iv8__");
            },
            "Boolean(__iv8__)",
        );
        assert_eq!(result, "false");
    }

    #[test]
    fn iv8_tool_if_does_not_enter() {
        let result = eval_with_setup(
            |scope, global| {
                install_iv8_tool_object(scope, global, "__iv8__");
            },
            "let x = 0; if (__iv8__) { x = 1; } x",
        );
        assert_eq!(result, "0");
    }

    #[test]
    fn iv8_tool_properties_still_accessible() {
        let result = eval_with_setup(
            |scope, global| {
                let obj = install_iv8_tool_object(scope, global, "__iv8__");
                let key = v8::String::new(scope, "marker").unwrap();
                let val = v8::Number::new(scope, 42.0);
                obj.set(scope, key.into(), val.into()).unwrap();
            },
            "__iv8__.marker",
        );
        assert_eq!(result, "42");
    }

    #[test]
    fn document_all_typeof_is_undefined() {
        let result = eval_with_setup(
            |scope, global| {
                let doc = v8::Object::new(scope);
                let key = v8::String::new(scope, "document").unwrap();
                global.set(scope, key.into(), doc.into());
                install_document_all(scope, doc);
            },
            "typeof document.all",
        );
        assert_eq!(result, "undefined");
    }

    #[test]
    fn document_all_loose_equals_null() {
        let result = eval_with_setup(
            |scope, global| {
                let doc = v8::Object::new(scope);
                let key = v8::String::new(scope, "document").unwrap();
                global.set(scope, key.into(), doc.into());
                install_document_all(scope, doc);
            },
            "document.all == null",
        );
        assert_eq!(result, "true");
    }

    #[test]
    fn document_all_callable_returns_undefined() {
        let result = eval_with_setup(
            |scope, global| {
                let doc = v8::Object::new(scope);
                let key = v8::String::new(scope, "document").unwrap();
                global.set(scope, key.into(), doc.into());
                install_document_all(scope, doc);
            },
            "typeof document.all('myId')",
        );
        // Without DOM integration, the call returns undefined (stub handler).
        assert_eq!(result, "undefined");
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
