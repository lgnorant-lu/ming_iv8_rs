//! hookNative: register hooks for native API paths.
//!
//! In iv8 0.1.2, hookNative is implemented but the callback never fires
//! (path format bug). We replicate this behavior in strict_compat mode (v0.1).
//!
//! The JS shim is in `shims/hook_native.js`.

#[cfg(test)]
mod tests {
    use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};

    fn make_kernel() -> EmbeddedV8Kernel {
        EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
    }

    #[test]
    fn hook_native_exists_on_iv8() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value("typeof __iv8__.hookNative");
        assert_eq!(result, RustValue::String("function".into()));
    }

    #[test]
    fn hook_native_looks_native() {
        let mut kernel = make_kernel();
        let result = kernel.eval_to_rust_value("__iv8__.hookNative.toString()");
        assert_eq!(
            result,
            RustValue::String("function hookNative() { [native code] }".into())
        );
    }

    #[test]
    fn hook_native_accepts_valid_call() {
        let mut kernel = make_kernel();
        // Should not throw — just silently stores the hook
        let result = kernel.eval(
            "__iv8__.hookNative('Navigator.prototype.userAgent', function() { return 'spoofed'; })",
            EvalOpts::default(),
        );
        assert!(result.is_ok(), "hookNative should not throw on valid call");
    }

    #[test]
    fn hook_native_does_not_apply_in_strict_compat() {
        let mut kernel = make_kernel();
        // Register a hook for navigator.userAgent
        kernel
            .eval(
                "__iv8__.hookNative('Navigator.prototype.userAgent', function() { return 'HOOKED'; })",
                EvalOpts::default(),
            )
            .unwrap();

        // The hook should NOT fire (matching iv8 0.1.2 bug)
        let result = kernel.eval_to_rust_value("navigator.userAgent");
        match result {
            RustValue::String(s) => {
                assert!(
                    !s.contains("HOOKED"),
                    "hookNative should not apply in strict_compat mode, got: {}",
                    s
                );
                assert!(s.contains("Chrome"), "should still be original UA: {}", s);
            }
            other => panic!("expected String, got: {:?}", other),
        }
    }

    #[test]
    fn hook_native_throws_on_no_args() {
        let mut kernel = make_kernel();
        let err = kernel
            .eval("__iv8__.hookNative()", EvalOpts::default())
            .unwrap_err();
        match err {
            iv8_core::IV8Error::Js { message, .. } => {
                assert!(
                    message.contains("requires at least 1 argument"),
                    "msg: {}",
                    message
                );
            }
            other => panic!("expected Js error, got: {:?}", other),
        }
    }

    #[test]
    fn hook_native_throws_on_non_string_path() {
        let mut kernel = make_kernel();
        let err = kernel
            .eval("__iv8__.hookNative(123, function() {})", EvalOpts::default())
            .unwrap_err();
        match err {
            iv8_core::IV8Error::Js { name, message, .. } => {
                assert_eq!(name, "TypeError");
                assert!(message.contains("must be a string"), "msg: {}", message);
            }
            other => panic!("expected TypeError, got: {:?}", other),
        }
    }

    #[test]
    fn hook_native_throws_on_empty_path() {
        let mut kernel = make_kernel();
        let err = kernel
            .eval("__iv8__.hookNative('', function() {})", EvalOpts::default())
            .unwrap_err();
        match err {
            iv8_core::IV8Error::Js { message, .. } => {
                assert!(
                    message.contains("api name is empty"),
                    "msg: {}",
                    message
                );
            }
            other => panic!("expected Js error, got: {:?}", other),
        }
    }

    #[test]
    fn hook_native_accepts_path_only() {
        let mut kernel = make_kernel();
        // hookNative with only path (no function) should work — clears hook
        let result = kernel.eval(
            "__iv8__.hookNative('Navigator.prototype.userAgent')",
            EvalOpts::default(),
        );
        assert!(result.is_ok(), "hookNative with path only should not throw");
    }
}
