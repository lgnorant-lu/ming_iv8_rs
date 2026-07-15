//! Extra V8 ObjectTemplate bindings not exposed by upstream rusty_v8.
//!
//! Compiled from [`cxx/iv8_v8_extra.cc`] via [`build.rs`].
//!
//! These provide:
//! - [`mark_as_undetectable`] — implements [[IsHTMLDDA]] semantics (typeof
//!   === 'undefined', falsy, == null) for objects created from this template.
//! - [`set_call_as_function_handler`] — enables calling instances created
//!   from this template as a function (needed e.g. for `document.all('id')`).
//!
//! See `docs/research/v0.2/mark-as-undetectable-fork.md` for the full design
//! and rationale.

use v8::ObjectTemplate;

unsafe extern "C" {
    fn v8__ObjectTemplate__MarkAsUndetectable(this: *const ObjectTemplate);
    fn v8__ObjectTemplate__SetCallAsFunctionHandler(
        this: *const ObjectTemplate,
        callback: v8::FunctionCallback,
        data_or_null: *const v8::Value,
    );
    fn iv8__V8__InitializeICU(icu_data_file: *const std::os::raw::c_char) -> i32;
    fn iv8__ICU__SetFileAccessOnlyPackages() -> i32;
}

/// Load ICU via Chromium's `V8::InitializeICU(path)` (file-based).
/// Prefer this over `set_common_data_77` when the prebuilt V8 expects
/// external icudtl.dat (same as ref iv8 / d8).
#[inline]
pub fn initialize_icu_from_file(path: &std::path::Path) -> bool {
    use std::ffi::CString;
    let Ok(c) = CString::new(path.to_string_lossy().as_bytes()) else {
        return false;
    };
    unsafe { iv8__V8__InitializeICU(c.as_ptr()) != 0 }
}

/// After `set_common_data`, restrict ICU to package data only (Chromium
/// `icu_util.cc` sequence). Returns true if ICU reported U_ZERO_ERROR.
#[inline]
pub fn icu_set_file_access_only_packages() -> bool {
    unsafe { iv8__ICU__SetFileAccessOnlyPackages() != 0 }
}

/// Marks instances created from this `ObjectTemplate` as undetectable.
///
/// In many ways, undetectable objects behave as though they are not there:
/// `typeof obj === 'undefined'`, `Boolean(obj) === false`, `obj == null`.
/// However, properties can still be accessed and called on them as on
/// normal objects.
///
/// This implements V8's `ObjectTemplate::MarkAsUndetectable` API, which
/// upstream rusty_v8 does not expose.
///
/// # V8 invariant
///
/// V8 requires that an undetectable ObjectTemplate also has a
/// CallAsFunctionHandler installed (V8 asserts this when instantiating).
/// You **must** call [`set_call_as_function_handler`] before instantiating
/// an undetectable template; the handler may be a no-op.
#[inline]
pub fn mark_as_undetectable(template: &ObjectTemplate) {
    // SAFETY: We pass a reference to a valid ObjectTemplate. The C++ side
    // reinterprets it as a v8::Local<ObjectTemplate>, which is safe because
    // v8::Local<T> is layout-compatible with T*.
    unsafe { v8__ObjectTemplate__MarkAsUndetectable(template) };
}

/// Sets the callback used when calling instances created from this template
/// as a function (e.g. `instance(args)`).
///
/// If `data` is `None`, no callback data is passed to the callback.
#[inline]
pub fn set_call_as_function_handler(
    template: &ObjectTemplate,
    callback: v8::FunctionCallback,
    data: Option<v8::Local<v8::Value>>,
) {
    let data_ptr: *const v8::Value = match data {
        Some(local) => &*local,
        None => std::ptr::null(),
    };
    // SAFETY: Same as above. `callback` is a function pointer with the V8
    // FunctionCallback ABI. `data_ptr` is either null or a valid Local handle.
    unsafe {
        v8__ObjectTemplate__SetCallAsFunctionHandler(template, callback, data_ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" fn test_cb(_: *const v8::FunctionCallbackInfo) {}

    #[test]
    fn test_mark_as_undetectable_completes_without_panic() {
        crate::v8_init::ensure_v8_initialized();
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());
        v8::scope!(let hs, &mut isolate);
        let ctx = v8::Context::new(hs, Default::default());
        let scope = &mut v8::ContextScope::new(hs, ctx);
        let tmpl = v8::ObjectTemplate::new(scope);
        set_call_as_function_handler(&tmpl, test_cb, None);
        mark_as_undetectable(&tmpl);
        let _obj = tmpl.new_instance(scope);
    }
}
