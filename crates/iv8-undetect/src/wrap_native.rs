//! wrapNative: make JS functions appear as native code in toString().
//!
//! Implementation strategy for v0.1:
//! Use a JS-level shim that overrides Function.prototype.toString for wrapped functions.
//! This is simpler than the FunctionTemplate approach and works with v8 147's API.
//!
//! The shim:
//! 1. Creates a wrapper function that delegates to the original
//! 2. Stores the desired name
//! 3. Patches toString to return `function name() { [native code] }`

/// JS code that installs the wrapNative helper on a given object.
/// Call this with `__iv8__` as the target.
pub const WRAP_NATIVE_SHIM: &str = r#"
(function(target) {
    const _nativeToString = Function.prototype.toString;
    const _wrappedFunctions = new WeakSet();
    const _wrappedNames = new WeakMap();

    // Override Function.prototype.toString
    Function.prototype.toString = function() {
        if (_wrappedFunctions.has(this)) {
            const name = _wrappedNames.get(this) || '';
            return 'function ' + name + '() { [native code] }';
        }
        return _nativeToString.call(this);
    };
    // Make toString itself look native
    _wrappedFunctions.add(Function.prototype.toString);
    _wrappedNames.set(Function.prototype.toString, 'toString');

    target.wrapNative = function wrapNative(fn, name) {
        if (typeof fn !== 'function') {
            throw new TypeError('wrapNative: first argument must be a function');
        }
        name = name || fn.name || '';

        // Create wrapper that delegates to original
        const wrapper = function() {
            return fn.apply(this, arguments);
        };

        // Set name and length
        Object.defineProperty(wrapper, 'name', { value: name, configurable: true });
        Object.defineProperty(wrapper, 'length', { value: fn.length, configurable: true });

        // Mark as native
        _wrappedFunctions.add(wrapper);
        _wrappedNames.set(wrapper, name);

        return wrapper;
    };

    // Make wrapNative itself look native
    _wrappedFunctions.add(target.wrapNative);
    _wrappedNames.set(target.wrapNative, 'wrapNative');
})
"#;

/// Install wrapNative on the __iv8__ tool object by evaluating the JS shim.
/// `iv8_obj_name` is the global name of the tool object (default "__iv8__").
pub fn get_install_script(iv8_obj_name: &str) -> String {
    format!("{}({})", WRAP_NATIVE_SHIM, iv8_obj_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};

    fn make_kernel() -> EmbeddedV8Kernel {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        // Create __iv8__ object and install wrapNative
        kernel
            .eval("var __iv8__ = {}", EvalOpts::default())
            .unwrap();
        let script = get_install_script("__iv8__");
        kernel.eval(&script, EvalOpts::default()).unwrap();
        kernel
    }

    #[test]
    fn wrap_native_tostring_native_code() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value(
            r#"
            function original() {}
            var wrapped = __iv8__.wrapNative(original, 'myFunc');
            wrapped.toString()
        "#,
        );
        assert_eq!(
            result,
            RustValue::String("function myFunc() { [native code] }".into())
        );
    }

    #[test]
    fn wrap_native_function_prototype_tostring_call() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value(
            r#"
            function original() {}
            var wrapped = __iv8__.wrapNative(original, 'test');
            Function.prototype.toString.call(wrapped)
        "#,
        );
        assert_eq!(
            result,
            RustValue::String("function test() { [native code] }".into())
        );
    }

    #[test]
    fn wrap_native_preserves_behavior() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value(
            r#"
            function add(a, b) { return a + b; }
            var wrapped = __iv8__.wrapNative(add, 'add');
            wrapped(3, 4)
        "#,
        );
        assert_eq!(result, RustValue::Int(7));
    }

    #[test]
    fn wrap_native_preserves_name() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value(
            r#"
            function original() {}
            var wrapped = __iv8__.wrapNative(original, 'customName');
            wrapped.name
        "#,
        );
        assert_eq!(result, RustValue::String("customName".into()));
    }

    #[test]
    fn wrap_native_preserves_length() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value(
            r#"
            function original(a, b, c) {}
            var wrapped = __iv8__.wrapNative(original, 'test');
            wrapped.length
        "#,
        );
        assert_eq!(result, RustValue::Int(3));
    }

    #[test]
    fn wrap_native_itself_looks_native() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value(
            r#"
            __iv8__.wrapNative.toString()
        "#,
        );
        assert_eq!(
            result,
            RustValue::String("function wrapNative() { [native code] }".into())
        );
    }

    #[test]
    fn wrap_native_tostring_looks_native() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value(
            r#"
            Function.prototype.toString.toString()
        "#,
        );
        assert_eq!(
            result,
            RustValue::String("function toString() { [native code] }".into())
        );
    }

    #[test]
    fn wrap_native_throws_on_non_function() {
        let mut kernel = make_kernel();
        let err = kernel
            .eval("__iv8__.wrapNative(42, 'test')", EvalOpts::default())
            .unwrap_err();
        match err {
            iv8_core::IV8Error::Js { name, message, .. } => {
                assert_eq!(name, "TypeError");
                assert!(message.contains("function"), "msg: {}", message);
            }
            other => panic!("expected TypeError, got: {:?}", other),
        }
    }
}
