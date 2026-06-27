//! Generated FunctionTemplate stubs.

/// Empty constructor shared by all generated templates.
pub(crate) unsafe extern "C" fn empty_constructor(_info: *const v8::FunctionCallbackInfo) {}

/// Construct-only constructor — creates an empty object via `new`.
/// Used for constructable interfaces (EventTarget, etc.) so that
/// `new EventTarget()` does not throw.
pub(crate) unsafe extern "C" fn construct_only(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if !args.is_construct_call() {
            let msg = v8::String::new(scope, "Failed to construct: Please use the 'new' operator").unwrap();
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
        }
    }));
}

/// Illegal constructor — throws TypeError, matching real browser behavior for
/// non-constructable Web IDL interfaces.
pub(crate) unsafe extern "C" fn illegal_constructor(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    v8::callback_scope!(unsafe scope, info_ref);
    let msg = v8::String::new(scope, "Illegal constructor").unwrap();
    let exc = v8::Exception::type_error(scope, msg);
    scope.throw_exception(exc);
}

pub mod bluetooth;
pub mod cache_api;
pub mod chrome_extensions;
pub mod credentials;
pub mod crypto;
pub mod css_om;
pub mod dom_core;
pub mod encoding;
pub mod events;
pub mod fetch;
pub mod gamepad;
pub mod gpu;
pub mod hid;
pub mod html_elements;
pub mod idb;
pub mod media_apis;
pub mod midi;
pub mod observers;
pub mod payment;
pub mod presentation;
pub mod sensors;
pub mod streams;
pub mod svg;
pub mod url;
pub mod usb;
pub mod web_apis;
pub mod web_audio;
pub mod webgl;
pub mod webrtc;
pub mod webxr;
pub mod workers;

pub mod install_all;
