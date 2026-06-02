//! V8 infallible helper wrappers.
//!
//! These wrap V8 API calls that only fail on OOM, which V8's OOM handler
//! handles by terminating the process. Callers can assume these never
//! return `None` or `Err` in practice.
//!
//! Centralizing `.expect()` here lets the rest of the codebase use clean
//! calls without per-site `// SAFETY:` annotations.
#![expect(clippy::expect_used, reason = "OOM-only: v8_string/v8_fn/v8_resolver")]

/// Create a V8 string. Only fails on OOM (process-terminating).
pub fn v8_string<'s>(
    scope: &v8::PinScope<'s, '_>,
    s: &str,
) -> v8::Local<'s, v8::String> {
    v8::String::new(scope, s).expect("v8_string: OOM")
}

/// Create a V8 function from a FunctionTemplate. Only fails on OOM.
pub fn v8_fn<'s>(
    scope: &v8::PinScope<'s, '_>,
    tmpl: &v8::FunctionTemplate,
) -> v8::Local<'s, v8::Function> {
    tmpl.get_function(scope).expect("v8_fn: OOM")
}

/// Create a V8 PromiseResolver. Only fails on OOM.
pub fn v8_resolver<'s>(
    scope: &v8::PinScope<'s, '_>,
) -> v8::Local<'s, v8::PromiseResolver> {
    v8::PromiseResolver::new(scope).expect("v8_resolver: OOM")
}
