//! expose: register Rust callbacks as JS global functions via FunctionTemplate.
//!
//! Uses v8 147's FunctionTemplate::builder with data slot to store the callback.

use std::ffi::c_void;

type ExposedCallback = Box<dyn Fn(&[String]) -> Result<String, String> + Send + 'static>;

/// Data stored in V8 External for each exposed function.
struct ExposedFnData {
    callback: ExposedCallback,
}

/// Register a named function on the V8 global object.
/// When called from JS, invokes the Rust closure with string args.
///
/// NOTE: ExposedFnData is leaked (Box::into_raw) into a V8 External.
/// It lives for the JSContext lifetime. Bounded leak (~64 bytes per call).
/// v0.2+ should use weak callbacks to free on GC.
pub fn expose_function(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    name: &str,
    callback: ExposedCallback,
) {
    let data = Box::new(ExposedFnData { callback });
    let data_ptr = Box::into_raw(data) as *mut c_void;
    let external = v8::External::new(scope, data_ptr);

    // Use FunctionTemplate::builder_raw with our extern "C" callback
    let tmpl = v8::FunctionTemplate::builder_raw(exposed_fn_trampoline)
        .data(external.into())
        .build(scope);

    let func = crate::v8_utils::v8_fn(scope, &tmpl);
    let name_str = crate::v8_utils::v8_string(scope, name);
    func.set_name(name_str);
    global.set(scope, name_str.into(), func.into());
}

/// The raw extern "C" trampoline that V8 calls.
/// Extracts the ExposedFnData from the External, collects args, calls the closure.
unsafe extern "C" fn exposed_fn_trampoline(info: *const v8::FunctionCallbackInfo) {
    // Use catch_unwind to prevent panics from crossing FFI boundary
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };

        // Open a callback scope
        v8::callback_scope!(unsafe scope, info_ref);

        // Get args and data
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let data = args.data();
        if !data.is_external() {
            return;
        }
        let external: v8::Local<v8::External> = unsafe { v8::Local::cast_unchecked(data) };
        let fn_data = unsafe { &*(external.value() as *const ExposedFnData) };

        // Collect arguments as strings
        let argc = args.length();
        let mut string_args: Vec<String> = Vec::with_capacity(argc as usize);
        for i in 0..argc {
            let arg = args.get(i);
            string_args.push(arg.to_rust_string_lossy(scope));
        }

        // Call the Rust closure
        match (fn_data.callback)(&string_args) {
            Ok(result_str) => {
                if let Some(s) = v8::String::new(scope, &result_str) {
                    rv.set(s.into());
                }
            }
            Err(err_msg) => {
                if let Some(msg) = v8::String::new(scope, &err_msg) {
                    let exc = v8::Exception::error(scope, msg);
                    scope.throw_exception(exc);
                }
            }
        }
    }));

    if result.is_err() {
        tracing::error!("panic in exposed function callback");
    }
}

#[cfg(test)]
mod tests {
    use crate::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};

    #[test]
    fn expose_simple_function() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.expose_fn(
            "greet",
            Box::new(|args| {
                let name = args.first().map(|s| s.as_str()).unwrap_or("world");
                Ok(format!("hello {}", name))
            }),
        );
        let result = kernel.eval_to_rust_value("greet('iv8')");
        assert_eq!(result, RustValue::String("hello iv8".into()));
    }

    #[test]
    fn expose_function_no_args() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.expose_fn("getVersion", Box::new(|_| Ok("0.1.0".to_string())));
        let result = kernel.eval_to_rust_value("getVersion()");
        assert_eq!(result, RustValue::String("0.1.0".into()));
    }

    #[test]
    fn expose_function_throws_error() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.expose_fn(
            "failMe",
            Box::new(|_| Err("something went wrong".to_string())),
        );
        let err = kernel.eval("failMe()", EvalOpts::default()).unwrap_err();
        match err {
            crate::IV8Error::Js { message, .. } => {
                assert!(message.contains("something went wrong"), "msg: {}", message);
            }
            other => panic!("expected Js error, got: {:?}", other),
        }
    }

    #[test]
    fn expose_function_multiple_args() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.expose_fn(
            "add",
            Box::new(|args| {
                let a: f64 = args.first().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let b: f64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                Ok((a + b).to_string())
            }),
        );
        let result = kernel.eval_to_rust_value("add(3, 4)");
        assert_eq!(result, RustValue::String("7".into()));
    }

    #[test]
    fn expose_function_stateful() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let counter_clone = counter.clone();
        kernel.expose_fn(
            "increment",
            Box::new(move |_| {
                let val = counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                Ok(val.to_string())
            }),
        );
        assert_eq!(
            kernel.eval_to_rust_value("increment()"),
            RustValue::String("1".into())
        );
        assert_eq!(
            kernel.eval_to_rust_value("increment()"),
            RustValue::String("2".into())
        );
        assert_eq!(
            kernel.eval_to_rust_value("increment()"),
            RustValue::String("3".into())
        );
    }
}
