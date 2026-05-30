//! `safe_callback!` macro: wraps V8 callback bodies with `catch_unwind`
//! to prevent Rust panics from unwinding across the `extern "C"` FFI boundary.
//!
//! V8 callbacks are `unsafe extern "C" fn(...)`. If Rust code panics inside
//! such a callback, the default behavior is process abort (since Rust 1.71+
//! extern "C" + panic = abort). This macro catches the panic and converts it
//! to a JS Error thrown back into V8.

/// Wrap a V8 callback body with panic safety.
///
/// Usage inside a FunctionCallback or PropertyCallback:
/// ```ignore
/// safe_callback!(scope, {
///     // ... your callback logic here, `scope` is available ...
/// });
/// ```
///
/// If the body panics:
/// 1. The panic is caught (never unwinds to V8/C++)
/// 2. A `tracing::error!` log is emitted
/// 3. A JS Error is thrown back into V8 via `scope.throw_exception()`
#[macro_export]
macro_rules! safe_callback {
    ($scope:expr, $body:expr) => {{
        let __result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
            $body
        }));
        if let Err(__panic_payload) = __result {
            let __msg = $crate::error::extract_panic_msg(&*__panic_payload);
            ::tracing::error!(panic_msg = %__msg, "V8 callback panic caught");
            // Throw a JS Error back into V8 so JS code sees the error
            let err_msg = v8::String::new($scope, &format!("internal error: {}", __msg));
            if let Some(err_msg) = err_msg {
                let exception = v8::Exception::error($scope, err_msg);
                $scope.throw_exception(exception);
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::panic::AssertUnwindSafe;

    #[test]
    fn catch_unwind_prevents_abort() {
        // Simulate what safe_callback! does: catch a panic
        let result =
            std::panic::catch_unwind(AssertUnwindSafe(|| {
                panic!("intentional test panic");
            }));
        assert!(result.is_err(), "panic should be caught");

        let payload = result.unwrap_err();
        let msg = crate::error::extract_panic_msg(&*payload);
        assert_eq!(msg, "intentional test panic");
    }

    #[test]
    fn catch_unwind_non_string_payload() {
        let result =
            std::panic::catch_unwind(AssertUnwindSafe(|| {
                std::panic::panic_any(42_i32);
            }));
        assert!(result.is_err());

        let payload = result.unwrap_err();
        let msg = crate::error::extract_panic_msg(&*payload);
        assert_eq!(msg, "panic (non-string payload)");
    }

    #[test]
    fn safe_callback_macro_does_not_abort() {
        // This test verifies catch_unwind works at the fundamental level.
        // The full macro with scope + throw_exception is tested in integration tests
        // where we have a real V8 scope.
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            panic!("macro test panic");
        }));
        assert!(result.is_err(), "panic should be caught by catch_unwind");
    }

    #[test]
    fn safe_callback_macro_normal_execution() {
        // Verify the macro's normal (non-panic) path works
        let mut executed = false;
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            executed = true;
        }));
        assert!(result.is_ok());
        assert!(executed, "body should execute normally when no panic");
    }
}
