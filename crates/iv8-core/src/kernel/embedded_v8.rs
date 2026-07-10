//! EmbeddedV8Kernel: the primary kernel implementation using v8 crate.
// SAFETY: remaining expects are OOM-only or logic invariants
#![expect(clippy::expect_used, reason = "OOM or logic invariant")]

use crate::config::EnvironmentMap;
use crate::error::IV8Error;
use crate::kernel::{EvalOpts, KernelConfig};
use crate::state::RuntimeState;

type ExposedCallback = Box<dyn Fn(&[String]) -> Result<String, String> + Send + 'static>;
use crate::shims::browser_profile::DEFAULT_PROFILE;
use crate::v8_init::ensure_v8_initialized;
use iv8_profile::BehaviorConfig;
use std::sync::Arc;

/// document.write workaround shim (REQ-DOM-008).
/// Replaces document.write with insertAdjacentHTML-based implementation.
///
/// Uses a persistent insertion-point tracker so that multiple document.write
/// calls append sequentially rather than all inserting at the same anchor.
/// Primary path: body.insertAdjacentHTML('beforeend'), creating body if needed.
/// If a currentScript anchor exists, the first write inserts after it and
/// subsequent writes append after the previously written content.
pub(crate) const DOCUMENT_WRITE_SHIM: &str = r#"
(function() {
    if (typeof document === 'undefined') return;

    // Persistent insertion point: tracks where the next write should go.
    // - null  => no write has happened yet; use currentScript or body
    // - node  => insert after this node (a sentinel comment we insert)
    var __iv8_write_anchor = null;

    function ensureBody() {
        if (document.body) return document.body;
        if (!document.documentElement) {
            // No documentElement either — create the full chain
            var html = document.createElement('html');
            document.appendChild(html);
        }
        var body = document.createElement('body');
        document.documentElement.appendChild(body);
        return body;
    }

    function doWrite(html) {
        // Case 1: We have a currentScript with a parent — insert after it
        // and track the position via a sentinel comment so subsequent
        // writes append after the previously written content.
        if (__iv8_write_anchor === null) {
            var script = document.currentScript;
            if (script && script.parentNode) {
                try {
                    // Insert a sentinel comment after the script to act as
                    // our tracking anchor. Content goes before the sentinel
                    // so it appears in document order.
                    var sentinel = document.createComment('iv8-write');
                    script.parentNode.insertBefore(sentinel, script.nextSibling);
                    sentinel.insertAdjacentHTML('beforebegin', html);
                    __iv8_write_anchor = sentinel;
                    return;
                } catch(e) {
                    __iv8_write_anchor = null;
                }
            }
        }

        // Case 2: Subsequent write — append before the existing sentinel
        if (__iv8_write_anchor && __iv8_write_anchor.parentNode) {
            try {
                __iv8_write_anchor.insertAdjacentHTML('beforebegin', html);
                return;
            } catch(e) {
                // Sentinel was detached, fall through to body path
                __iv8_write_anchor = null;
            }
        }

        // Case 3: Body path (primary fallback / post-load)
        var body = ensureBody();
        try {
            body.insertAdjacentHTML('beforeend', html);
            return;
        } catch(e) {}

        // Case 4: Last resort — append to documentElement
        if (document.documentElement) {
            try {
                document.documentElement.insertAdjacentHTML('beforeend', html);
            } catch(e) {}
        }
    }

    document.write = function() {
        var html = Array.prototype.join.call(arguments, '');
        doWrite(html);
    };
    document.writeln = function() {
        var args = Array.prototype.slice.call(arguments);
        document.write(args.join(' ') + '\n');
    };
    document.open = function() {
        // Reset the insertion point on explicit open()
        __iv8_write_anchor = null;
        return document;
    };
    document.close = function() {};
})();
"#;

/// Minimal TextEncoder/TextDecoder polyfill for V8.
///
/// The generated IDL surface (iv8-surface) installs `TextEncoder`/`TextDecoder`
/// constructors with correct class name + toStringTag for fingerprint fidelity,
/// but their `encode`/`decode` methods are non-functional skeletons (returned
/// `v8::null` via `default_value_for_type` before the v0.8.63 type_conv fix).
/// This shim overrides the prototype methods unconditionally so they produce
/// real Uint8Array / string results regardless of which constructor is in use.
const TEXT_ENCODER_SHIM: &str = r#"
(function() {
    // Override the (possibly skeleton) prototype methods with working ones.
    TextEncoder.prototype.encode = function(str) {
        str = str === undefined ? '' : String(str);
        var arr = [];
        for (var i = 0; i < str.length; i++) {
            var c = str.charCodeAt(i);
            if (c < 128) { arr.push(c); }
            else if (c < 2048) { arr.push((c >> 6) | 192); arr.push((c & 63) | 128); }
            else { arr.push((c >> 12) | 224); arr.push(((c >> 6) & 63) | 128); arr.push((c & 63) | 128); }
        }
        return new Uint8Array(arr);
    };
    TextDecoder.prototype.decode = function(buf) {
        if (buf === undefined || buf === null) { return ''; }
        var arr = new Uint8Array(buf.buffer ? buf.buffer : buf);
        var str = '';
        for (var i = 0; i < arr.length; i++) { str += String.fromCharCode(arr[i]); }
        return str;
    };
})();
"#;

fn is_valid_js_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch == '$' || ch.is_ascii_alphanumeric())
}

/// The embedded V8 kernel — owns an Isolate + Context.
pub struct EmbeddedV8Kernel {
    pub(crate) isolate: v8::OwnedIsolate,
    pub(crate) context: v8::Global<v8::Context>,
    environment: Arc<EnvironmentMap>,
    pub creator_thread: std::thread::ThreadId,
    pub(crate) worker_mode: bool,
}

// SAFETY: EmbeddedV8Kernel is effectively single-threaded. The Isolate
// is entered by at most one thread at a time (enforced by creator_thread
// check). We need Send to move the kernel from the init thread (with
// large stack) back to the caller thread after V8 template creation.
unsafe impl Send for EmbeddedV8Kernel {}

// ─── Window dimension native getter callbacks ─────────────────────
// Defined locally per v0.8.65 design: keep window getters in
// embedded_v8.rs to avoid expanding native_env.rs API surface.
macro_rules! window_f64_getter_cb {
    ($name:ident, $path:literal, $field:ident, $default:expr) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = crate::state::RuntimeState::get(isolate);
                let val = match state.profile {
                    Some(p) => p.$field,
                    None => state.environment.get_f64($path).unwrap_or($default),
                };
                rv.set(v8::Number::new(scope, val).into());
            }));
        }
    };
}
window_f64_getter_cb!(
    window_inner_width_cb,
    "window.innerWidth",
    window_inner_width,
    DEFAULT_PROFILE.window_inner_width
);
window_f64_getter_cb!(
    window_inner_height_cb,
    "window.innerHeight",
    window_inner_height,
    DEFAULT_PROFILE.window_inner_height
);
window_f64_getter_cb!(
    window_outer_width_cb,
    "window.outerWidth",
    window_outer_width,
    DEFAULT_PROFILE.window_outer_width
);
window_f64_getter_cb!(
    window_outer_height_cb,
    "window.outerHeight",
    window_outer_height,
    DEFAULT_PROFILE.window_outer_height
);
window_f64_getter_cb!(
    window_device_pixel_ratio_cb,
    "window.devicePixelRatio",
    device_pixel_ratio,
    DEFAULT_PROFILE.device_pixel_ratio
);

unsafe extern "C" fn worker_constructor_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = crate::state::RuntimeState::get(isolate);
        if !args.is_construct_call() {
            let msg = crate::v8_utils::v8_string(scope, "Failed to construct 'Worker': Please use the 'new' operator");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(scope, "Failed to construct 'Worker': 1 argument required");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let script_url = args.get(0).to_rust_string_lossy(scope);
        let script_source = resolve_worker_script(isolate, &script_url);
        let profile = state.profile.unwrap_or(&DEFAULT_PROFILE);
        let worker_id = state.workers.borrow().len() as u64;
        let handle = crate::shims::worker::spawn_worker(script_source, script_url, profile, worker_id);
        let worker_obj = v8::Object::new(scope);
        let new_target = args.new_target();
        if new_target.is_object() {
            let nt_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(new_target) };
            let proto_key = crate::v8_utils::v8_string(scope, "prototype");
            if let Some(proto_val) = nt_obj.get(scope, proto_key.into()) {
                if proto_val.is_object() {
                    let _ = worker_obj.set_prototype(scope, proto_val);
                }
            }
        }
        let id_key = crate::v8_utils::v8_string(scope, "__iv8WorkerId");
        let _ = worker_obj.set(scope, id_key.into(), v8::Number::new(scope, worker_id as f64).into());
        let onmsg_key = crate::v8_utils::v8_string(scope, "onmessage");
        let _ = worker_obj.set(scope, onmsg_key.into(), v8::null(scope).into());
        let onerror_key = crate::v8_utils::v8_string(scope, "onerror");
        let _ = worker_obj.set(scope, onerror_key.into(), v8::null(scope).into());
        state.workers.borrow_mut().push(handle);
        let worker_global = v8::Global::new(scope, worker_obj);
        state.worker_objects.borrow_mut().insert(worker_id, worker_global);
        let worker_obj_local = v8::Local::new(scope, state.worker_objects.borrow().get(&worker_id).unwrap());
        rv.set(worker_obj_local.into());
    }));
}

unsafe extern "C" fn worker_post_message_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = crate::state::RuntimeState::get(isolate);
        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(scope, "Failed to execute 'postMessage' on 'Worker': 1 argument required");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let this = args.this();
        let id_key = crate::v8_utils::v8_string(scope, "__iv8WorkerId");
        let id_val = this.get(scope, id_key.into());
        let worker_id = match id_val.and_then(|v| v.number_value(scope)) {
            Some(n) => n as u64,
            None => return,
        };
        let data = args.get(0);
        let context = scope.get_current_context();
        match crate::shims::structured_clone::serialize_value(scope, context, data) {
            Ok(bytes) => {
                let workers = state.workers.borrow();
                for handle in workers.iter() {
                    if handle.worker_id == worker_id {
                        let _ = handle.tx.send(crate::shims::worker::WorkerMessage::PostMessage(bytes));
                        break;
                    }
                }
            }
            Err(e) => {
                let msg = crate::v8_utils::v8_string(scope, &e);
                let exc = v8::Exception::type_error(scope, msg);
                scope.throw_exception(exc);
            }
        }
    }));
}

unsafe extern "C" fn worker_terminate_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = crate::state::RuntimeState::get(isolate);
        let this = args.this();
        let id_key = crate::v8_utils::v8_string(scope, "__iv8WorkerId");
        let id_val = this.get(scope, id_key.into());
        if let Some(n) = id_val.and_then(|v| v.number_value(scope)) {
            let worker_id = n as u64;
            let mut workers = state.workers.borrow_mut();
            if let Some(idx) = workers.iter().position(|h| h.worker_id == worker_id) {
                let mut handle = workers.remove(idx);
                handle.terminate();
            }
            state.worker_objects.borrow_mut().remove(&worker_id);
        }
    }));
}

fn resolve_worker_script(isolate: &v8::Isolate, url: &str) -> String {
    let state = crate::state::RuntimeState::get(isolate);
    let bundle = state.resource_bundle.borrow();
    if let Some(resource) = bundle.get(url) {
        return String::from_utf8_lossy(&resource.body).to_string();
    }
    if url.starts_with("blob:") || url.starts_with("data:") {
        if let Some(src) = url.split(',').nth(1) {
            return percent_decode(src);
        }
    }
    String::new()
}

fn percent_decode(input: &str) -> String {
    let mut result = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex_digit(bytes[i + 1]), hex_digit(bytes[i + 2])) {
                result.push(h * 16 + l);
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'+' {
            result.push(b' ');
        } else {
            result.push(bytes[i]);
        }
        i += 1;
    }
    String::from_utf8_lossy(&result).to_string()
}

fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

impl EmbeddedV8Kernel {
    /// Create a new embedded V8 kernel with the given configuration.
    ///
    /// The caller MUST ensure sufficient thread stack size (>= 32MB) for
    /// V8 template creation. On the Python side, JSContext() handles this
    /// via threading.stack_size(). On the Rust test side, the test harness
    /// must use thread::Builder::new().stack_size(32 * 1024 * 1024).
    pub fn new(config: KernelConfig) -> Result<Self, IV8Error> {
        ensure_v8_initialized();

        if !is_valid_js_identifier(&config.js_api_name) {
            return Err(IV8Error::Internal(format!(
                "invalid js_api name '{}': expected a JavaScript identifier",
                config.js_api_name
            )));
        }

        let random_seed = config.random_seed;
        let crypto_seed = config.crypto_seed;
        let time_freeze = config.time_freeze;
        let user_overrides = config.user_overrides;
        let browser_profile: Option<&'static crate::shims::browser_profile::BrowserProfile> =
            config.browser_profile.map(|bp| &*Box::leak(bp));
        let storage_path = config.storage_path;
        let mut local_storage_backend = config.local_storage;
        if let Some(ref path) = storage_path {
            if path.exists() {
                match local_storage_backend {
                    Some(ref store) => {
                        let _ = store.load_from_file(path);
                    }
                    None => {
                        let store = crate::dom::local_storage::LocalStorageStore::new();
                        let _ = store.load_from_file(path);
                        local_storage_backend = Some(store);
                    }
                }
            }
        }
        let strict_compat = config.strict_compat;
        let time_mode = config.time_mode;
        let js_api_name = config.js_api_name;
        let environment = Arc::new(EnvironmentMap::build(config.environment_overrides.as_ref()));
        let worker_mode = config.worker_mode;

        Self::init_kernel(
            environment,
            strict_compat,
            time_mode,
            js_api_name,
            browser_profile,
            local_storage_backend,
            random_seed,
            crypto_seed,
            time_freeze,
            user_overrides,
            worker_mode,
        )
    }

    fn init_kernel(
        environment: Arc<EnvironmentMap>,
        strict_compat: bool,
        time_mode: crate::state::TimeMode,
        js_api_name: String,
        browser_profile: Option<&'static crate::shims::browser_profile::BrowserProfile>,
        local_storage_backend: Option<crate::dom::local_storage::LocalStorageStore>,
        random_seed: Option<u64>,
        crypto_seed: Option<u64>,
        time_freeze: Option<f64>,
        user_overrides: crate::user_overrides::UserOverrides,
        worker_mode: bool,
    ) -> Result<Self, IV8Error> {
        // Install panic hook once — ensures panics are logged via telemetry
        // before PyO3's catch_unwind converts them to PanicException.
        static PANIC_HOOK_INSTALLED: std::sync::Once = std::sync::Once::new();
        PANIC_HOOK_INSTALLED.call_once(|| {
            let default_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                let msg = info.to_string();
                crate::telemetry::rust_panic(&msg);
                eprintln!("[iv8-panic] {}", msg);
                default_hook(info);
            }));
        });

        // Install V8 fatal error handler (process-wide, once).
        static V8_FATAL_INSTALLED: std::sync::Once = std::sync::Once::new();
        V8_FATAL_INSTALLED.call_once(|| {
            fn fatal_handler(file: &str, line: i32, message: &str) {
                crate::telemetry::v8_fatal_error(file, line, message);
                eprintln!("[v8-fatal] {}:{} {}", file, line, message);
            }
            v8::V8::set_fatal_error_handler(fatal_handler);
        });

        let mut isolate = v8::Isolate::new(
            v8::CreateParams::default().heap_limits(512 * 1024 * 1024, 4 * 1024 * 1024 * 1024),
        );

        // Set microtask policy to Explicit (we drive microtasks manually)
        isolate.set_microtasks_policy(v8::MicrotasksPolicy::Explicit);

        // Capture stack traces for uncaught exceptions (like Deno does).
        isolate.set_capture_stack_trace_for_uncaught_exceptions(true, 10);

        // Set OOM error handler.
        extern "C" fn oom_handler(location: *const std::ffi::c_char, details: &v8::OomDetails) {
            let loc = if location.is_null() {
                "<unknown>"
            } else {
                unsafe { std::ffi::CStr::from_ptr(location) }
                    .to_str()
                    .unwrap_or("<invalid>")
            };
            crate::telemetry::v8_oom(loc, details.is_heap_oom);
            eprintln!("[v8-oom] {} heap={}", loc, details.is_heap_oom);
        }
        isolate.set_oom_error_handler(oom_handler);

        // Install RuntimeState (with environment reference for V8 callbacks)
        RuntimeState::install(
            &mut isolate,
            RuntimeState::new(
                strict_compat,
                time_mode,
                js_api_name,
                environment.clone(),
                browser_profile,
                local_storage_backend,
            ),
        );

        // Create the main context with global_template for native window accessors
        let context = {
            v8::scope!(handle_scope, &mut isolate);

            let global_tmpl = v8::ObjectTemplate::new(handle_scope);
            macro_rules! window_f64_getter {
                ($name:literal, $cb:ident) => {
                    let getter = v8::FunctionTemplate::builder_raw($cb).build(handle_scope);
                    let key = v8::String::new(handle_scope, $name).unwrap();
                    getter.set_class_name(key);
                    getter.remove_prototype();
                    global_tmpl.set_accessor_property(
                        key.into(),
                        Some(getter),
                        None,
                        v8::PropertyAttribute::NONE,
                    );
                };
            }
            window_f64_getter!("innerWidth", window_inner_width_cb);
            window_f64_getter!("innerHeight", window_inner_height_cb);
            window_f64_getter!("outerWidth", window_outer_width_cb);
            window_f64_getter!("outerHeight", window_outer_height_cb);
            window_f64_getter!("devicePixelRatio", window_device_pixel_ratio_cb);

            let context = v8::Context::new(
                handle_scope,
                v8::ContextOptions {
                    global_template: Some(global_tmpl),
                    ..Default::default()
                },
            );
            v8::Global::new(handle_scope, context)
        };

        let mut kernel = Self {
            isolate,
            context,
            environment,
            creator_thread: std::thread::current().id(),
            worker_mode,
        };

        // Install environment fields (navigator.*, screen.*, etc.) into global
        // Phase 1 only: static value injection via env_inject.
        // Phase 2 (native_env) runs after install_browser_surface_init
        // so that codegen EventTarget template is available for inheritance.
        let t0 = std::time::Instant::now();
        crate::telemetry::init_phase_start("install_environment");
        kernel.install_environment();
        crate::telemetry::init_phase_complete("install_environment", t0.elapsed().as_millis() as u64);

        // Install BrowserSurface (1284 IDL templates + 14 native behaviors).
        // Heap limits increased from default 1.4GB to 4GB to accommodate
        // 1284 FunctionTemplate creation without V8 GC IsOnCentralStack crash.
        let t1 = std::time::Instant::now();
        crate::telemetry::init_phase_start("install_browser_surface");
        kernel.install_browser_surface_init(worker_mode);
        crate::telemetry::init_phase_complete("install_browser_surface", t1.elapsed().as_millis() as u64);

        // Phase 2: install native environment objects (navigator, screen)

        // Phase 2: install native environment objects (navigator, screen)
        let t2 = std::time::Instant::now();
        crate::telemetry::init_phase_start("install_native_environment");
        kernel.install_native_environment();
        crate::telemetry::init_phase_complete("install_native_environment", t2.elapsed().as_millis() as u64);

        // Install anti-detection shims + JS shims (skip native behaviors
        // — already installed by install_browser_surface_init above).
        let t3 = std::time::Instant::now();
        crate::telemetry::init_phase_start("install_undetect_shims");
        kernel.install_undetect_shims(true);
        crate::telemetry::init_phase_complete("install_undetect_shims", t3.elapsed().as_millis() as u64);

        let t4 = std::time::Instant::now();
        crate::telemetry::init_phase_start("install_worker_constructor");
        kernel.install_worker_constructor();
        crate::telemetry::init_phase_complete("install_worker_constructor", t4.elapsed().as_millis() as u64);

        // Note: XHR_SHIM_JS is eval'd twice (install_xhr in install_browser_surface_init
        // + step 8 in install_undetect_shims). The second eval overwrites the codegen
        // constructor but prototype chain remains intact (fix_prototype_chains ran
        // before the overwrite, and V8 preserves the link). See TODO-infrastructure
        // "init flow double-eval" for long-term cleanup plan.

        // Install deterministic overrides (random_seed / crypto_seed / time_freeze)
        kernel.install_deterministic_overrides_from(random_seed, crypto_seed, time_freeze);

        kernel.freeze_all_prototypes();

        // DOM templates are installed inside install_browser_surface_init
        // via install_dom_constructors() — no separate call needed.

        // Step 8: Install user-defined property overrides (highest priority).
        if !user_overrides.is_empty() {
            unsafe {
                kernel.isolate.enter();
            }
            {
                v8::scope!(handle_scope, &mut kernel.isolate);
                let context = v8::Local::new(handle_scope, &kernel.context);
                v8::scope_with_context!(scope, handle_scope, context);
                let global = context.global(scope);
                crate::user_overrides::install_user_overrides(scope, global, &user_overrides);
            }
            unsafe {
                kernel.isolate.exit();
            }
        }

        // Exit the isolate so it's not "entered" at rest.
        // This allows multiple JSContext instances to coexist without LIFO drop panic.
        // We re-enter before each eval/operation and exit after.
        // SAFETY: isolate was entered by v8::Isolate::new, we exit it here.
        unsafe {
            kernel.isolate.exit();
        }

        Ok(kernel)
    }

    fn freeze_all_prototypes(&mut self) {
        let worker_mode = self.worker_mode;
        self.with_global_scope(|scope, global| {
            let move_js = crate::v8_utils::v8_string(scope, iv8_surface::generated::install_all::GLOBAL_MOVE_JS);
            let _ = v8::Script::compile(scope, move_js, None).and_then(|s| s.run(scope));

            // P0 boundary fix: delete navigator.webdriver from Navigator.prototype.
            // Real Chrome: Object.getOwnPropertyDescriptor(Navigator.prototype, 'webdriver') === undefined.
            // IV8 installs it as a getter returning false, which is detectable.
            let webdriver_fix = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    try { delete Navigator.prototype.webdriver; } catch(e) {}
                })();
            "#);
            let _ = v8::Script::compile(scope, webdriver_fix, None).and_then(|s| s.run(scope));

            // P0 boundary fix: patch document.createElement toString to return native code.
            // Current shim exposes JS source in toString().
            let create_element_fix = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    if (typeof document !== 'undefined' && document.createElement) {
                        var orig = document.createElement;
                        var origStr = orig.toString();
                        if (origStr.indexOf('[native code]') < 0) {
                            var patched = function createElement(tagName) { return orig.call(document, tagName); };
                            patched.toString = function() { return 'function createElement() { [native code] }'; };
                            patched.toString.toString = function() { return 'function toString() { [native code] }'; };
                            try { Object.defineProperty(document, 'createElement', { value: patched, writable: true, configurable: true, enumerable: true }); } catch(e) {}
                        }
                    }
                })();
            "#);
            let _ = v8::Script::compile(scope, create_element_fix, None).and_then(|s| s.run(scope));

            // P0 boundary fix: navigator.plugins instanceof PluginArray must be true.
            // Shim replaces plugins with plain object; wrap with Proxy that lies instanceof.
            let plugins_fix = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    if (typeof PluginArray === 'undefined' || typeof MimeTypeArray === 'undefined') return;
                    if (typeof navigator === 'undefined' || !navigator.plugins) return;
                    // If plugins is not instanceof PluginArray, wrap it
                    if (!(navigator.plugins instanceof PluginArray)) {
                        var realPlugins = navigator.plugins;
                        var pa = Object.create(PluginArray.prototype);
                        for (var i = 0; i < realPlugins.length; i++) {
                            pa[i] = realPlugins[i];
                        }
                        pa.length = realPlugins.length;
                        pa.item = function(i) { return realPlugins[i]; };
                        pa.namedItem = function(n) { return realPlugins[n]; };
                        pa[Symbol.toStringTag] = 'PluginArray';
                        try { Object.defineProperty(navigator, 'plugins', { value: pa, writable: true, configurable: true, enumerable: true }); } catch(e) {}
                    }
                    if (!(navigator.mimeTypes instanceof MimeTypeArray)) {
                        var realMT = navigator.mimeTypes;
                        var ma = Object.create(MimeTypeArray.prototype);
                        for (var i = 0; i < realMT.length; i++) {
                            ma[i] = realMT[i];
                        }
                        ma.length = realMT.length;
                        ma.item = function(i) { return realMT[i]; };
                        ma.namedItem = function(n) { return realMT[n]; };
                        ma[Symbol.toStringTag] = 'MimeTypeArray';
                        try { Object.defineProperty(navigator, 'mimeTypes', { value: ma, writable: true, configurable: true, enumerable: true }); } catch(e) {}
                    }
                })();
            "#);
            let _ = v8::Script::compile(scope, plugins_fix, None).and_then(|s| s.run(scope));

            // P0-BT-5 fix: iframe contentWindow.navigator missing.
            // Root cause: contentWindow getter returns bare Object or null
            // (looks for nonexistent "WindowProxy" global).
            // Fix: wrap contentWindow to create a Window-like proxy with navigator.
            let iframe_fix = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    if (typeof HTMLIFrameElement === 'undefined') return;
                    var proto = HTMLIFrameElement.prototype;
                    var origGetter = Object.getOwnPropertyDescriptor(proto, 'contentWindow');
                    if (!origGetter || !origGetter.get) return;
                    var origGet = origGetter.get;
                    Object.defineProperty(proto, 'contentWindow', {
                        get: function contentWindow() {
                            var cw = origGet.call(this);
                            // If null/undefined, create a Window-like object
                            if (!cw || typeof cw !== 'object') {
                                cw = {};
                            }
                            // Install navigator if missing (shares top frame values)
                            if (!cw.navigator) {
                                try {
                                    Object.defineProperty(cw, 'navigator', {
                                        get: function() { return navigator; },
                                        enumerable: true,
                                        configurable: true,
                                    });
                                } catch(e) {}
                            }
                            // Install basic Window properties
                            if (!cw.document) {
                                try {
                                    Object.defineProperty(cw, 'document', {
                                        get: function() { return this._contentDocument || document; },
                                        enumerable: true,
                                        configurable: true,
                                    });
                                } catch(e) {}
                            }
                            if (!('parent' in cw)) {
                                try {
                                    Object.defineProperty(cw, 'parent', { value: window, enumerable: true, configurable: true });
                                    Object.defineProperty(cw, 'top', { value: window, enumerable: true, configurable: true });
                                    Object.defineProperty(cw, 'self', { value: cw, enumerable: true, configurable: true });
                                    Object.defineProperty(cw, 'window', { value: cw, enumerable: true, configurable: true });
                                } catch(e) {}
                            }
                            return cw;
                        },
                        set: undefined,
                        enumerable: true,
                        configurable: true,
                    });
                })();
            "#);
            let _ = v8::Script::compile(scope, iframe_fix, None).and_then(|s| s.run(scope));

            // ROOT CAUSE: Element.prototype.shadowRoot returns {} by default but
            // should return null. VMP checks this API and takes wrong branch.
            let shadow_root_fix = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    if (typeof Element === 'undefined' || typeof Element.prototype === 'undefined') {
                        console.log('[shadowRoot] Element not available');
                        return;
                    }
                    try {
                        var oldDesc = Object.getOwnPropertyDescriptor(Element.prototype, 'shadowRoot');
                        if (!oldDesc) { console.log('[shadowRoot] no desc'); return; }
                        console.log('[shadowRoot] configurable=' + oldDesc.configurable + ' hasGetter=' + (typeof oldDesc.get));
                        Object.defineProperty(Element.prototype, 'shadowRoot', {
                            get: function() {
                                if (!this || typeof this !== 'object') return null;
                                return this.__iv8_shadowRoot || null;
                            },
                            enumerable: true, configurable: true
                        });
                        console.log('[shadowRoot] patched OK');
                    } catch(e) { console.log('[shadowRoot] error: ' + e.message); }
                    if (typeof Element.prototype.attachShadow === 'function') {
                        Element.prototype.attachShadow = function(init) {
                            var root = {};
                            try { root = Object.create(ShadowRoot.prototype); } catch(ex) {}
                            this.__iv8_shadowRoot = root;
                            return root;
                        };
                    }
                })();
            "#);
            let _ = v8::Script::compile(scope, shadow_root_fix, None).and_then(|s| s.run(scope));

            // P1: Request constructor — codegen creates empty object, store url/method
            let request_fix = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    if (typeof Request === 'undefined') return;
                    var origCtor = Request;
                    function RequestShim(input, init) {
                        var url = '';
                        if (typeof input === 'string') {
                            url = input;
                        } else if (input && typeof input === 'object' && input.url) {
                            url = input.url;
                        }
                        var method = (init && init.method) || 'GET';
                        Object.defineProperty(this, 'url', { value: url, writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'method', { value: method, writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'headers', { value: (init && init.headers) || {}, writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'body', { value: (init && init.body) || null, writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'cache', { value: 'default', writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'credentials', { value: 'same-origin', writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'destination', { value: '', writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'integrity', { value: '', writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'keepalive', { value: false, writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'mode', { value: 'cors', writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'redirect', { value: 'follow', writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'referrer', { value: 'about:client', writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'referrerPolicy', { value: '', writable: true, configurable: true, enumerable: true });
                        Object.defineProperty(this, 'signal', { value: null, writable: true, configurable: true, enumerable: true });
                    }
                    RequestShim.prototype = origCtor.prototype;
                    try { Object.defineProperty(globalThis, 'Request', {
                        value: RequestShim, writable: true, configurable: true, enumerable: true
                    }); } catch(e) {}
                })();
            "#);
            let _ = v8::Script::compile(scope, request_fix, None).and_then(|s| s.run(scope));

            iv8_surface::generated::install_all::fix_accessor_properties(scope, global);
            // [Global] accessor fix is Window-only — skip in worker mode to avoid
            // installing document/window/top etc. on worker globalThis.
            if !worker_mode {
                iv8_surface::generated::install_all::fix_global_accessor_properties(scope, global);
                iv8_surface::generated::install_all::fix_global_operation_lengths(scope, global);
            }
            // fix_operation_callbacks runs in install_browser_surface_init
            // (before shim JS evals) to avoid overwriting shim operations.

            // Post-fix: convert [Global] data properties to accessor properties.
            // Shims install real values as data properties (history, customElements, etc.).
            // idlharness expects accessor properties (descriptor.get === function).
            // For each [Global] attribute that is a configurable data property:
            //   read value → store in closure → install accessor getter returning value.
            // Skip non-configurable (V8 built-in name/length) and already-accessor.
            let global_attr_names: &[&str] = iv8_surface::generated::install_all::GLOBAL_ATTR_NAMES;
            let attr_names_js = global_attr_names
                .iter()
                .map(|n| format!("'{}'", n))
                .collect::<Vec<_>>()
                .join(",");
            let convert_js = format!(r#"
                (function() {{
                    var attrs = [{names}];
                    var windowCtor = globalThis.Window;
                    var windowProto = windowCtor && windowCtor.prototype;
                    for (var i = 0; i < attrs.length; i++) {{
                        (function(name) {{
                        try {{
                            // Skip length/name: these are function-intrinsic
                            // name/length: Function-intrinsic data properties
                            // on the global object. idlharness [Global]
                            // checks require them to be accessor properties
                            // (descriptor.get === function). Since they are
                            // configurable on V8's global, we can convert them.
                            var desc = Object.getOwnPropertyDescriptor(globalThis, name);
                            if (!desc) return;
                            if (!desc.configurable) return;
                            if (desc.get || desc.set) {{
                                // Accessor property (shim-installed getter/setter)
                                // Wrap getter with receiver check
                                if (desc.get && typeof desc.get === 'function') {{
                                    let origGet = desc.get;
                                    let wproto2 = windowProto;
                                    var wrappedGet = function() {{
                                        if (wproto2 && this !== globalThis && this !== wproto2) {{
                                            var cur = Object.getPrototypeOf(this);
                                            var found = false;
                                            for (var k = 0; k < 30; k++) {{
                                                if (cur === wproto2) {{ found = true; break; }}
                                                if (!cur) break;
                                                cur = Object.getPrototypeOf(cur);
                                            }}
                                            if (!found) throw new TypeError('Illegal invocation');
                                        }}
                                        // Use globalThis directly for self/window to avoid
                                        // getter recursion (self returns window, window returns window)
                                        if (name === 'self' || name === 'window' || name === 'top' || name === 'parent' || name === 'frames') {{
                                            return globalThis;
                                        }}
                                        return origGet.call(globalThis);
                                    }};
                                    try {{ Object.defineProperty(wrappedGet, 'name', {{ value: 'get ' + name }}); }} catch(e) {{}}
                                    Object.defineProperty(globalThis, name, {{
                                        get: wrappedGet,
                                        set: desc.set,
                                        enumerable: desc.enumerable !== false,
                                        configurable: true
                                    }});
                                }}
                                return;
                            }}
                            var value = desc.value;
                            var getter = (function(v, wproto) {{
                                return function() {{
                                    // Receiver check: this must be the global object
                                    // (which has Window.prototype in its chain)
                                    if (wproto && this !== globalThis && this !== wproto) {{
                                        var cur = Object.getPrototypeOf(this);
                                        var found = false;
                                        for (var k = 0; k < 30; k++) {{
                                            if (cur === wproto) {{ found = true; break; }}
                                            if (!cur) break;
                                            cur = Object.getPrototypeOf(cur);
                                        }}
                                        if (!found) throw new TypeError('Illegal invocation');
                                    }}
                                    return v;
                                }};
                            }})(value, windowProto);
                            try {{ Object.defineProperty(getter, 'name', {{ value: 'get ' + name }}); }} catch(e) {{}}
                            // Create setter for writable attributes.
                            // idlharness checks typeof desc.set === "function"
                            // for non-readonly / PutForwards / Replaceable attrs.
                            var setter = (function(nm, wproto) {{
                                return function(v) {{
                                    // Receiver check
                                    if (wproto && this !== globalThis && this !== wproto) {{
                                        var cur = Object.getPrototypeOf(this);
                                        var found = false;
                                        for (var k = 0; k < 30; k++) {{
                                            if (cur === wproto) {{ found = true; break; }}
                                            if (!cur) break;
                                            cur = Object.getPrototypeOf(cur);
                                        }}
                                        if (!found) throw new TypeError('Illegal invocation');
                                    }}
                                    // Store value as data property (Replaceable semantics)
                                    Object.defineProperty(globalThis, nm, {{
                                        value: v, writable: true,
                                        enumerable: true, configurable: true
                                    }});
                                }};
                            }})(name, windowProto);
                            try {{ Object.defineProperty(setter, 'name', {{ value: 'set ' + name }}); }} catch(e) {{}}
                            try {{ Object.defineProperty(setter, 'length', {{ value: 1, writable: false, enumerable: false, configurable: true }}); }} catch(e) {{}}
                            Object.defineProperty(globalThis, name, {{
                                get: getter,
                                set: setter,
                                enumerable: desc.enumerable !== false,
                                configurable: true
                            }});
                        }} catch(e) {{}}
                        }})(attrs[i]);
                    }}
                }})();
            "#, names = attr_names_js);
            let convert_js_str = crate::v8_utils::v8_string(scope, &convert_js);
            let _ = v8::Script::compile(scope, convert_js_str, None).and_then(|s| s.run(scope));

            let extra_accessor_js = crate::v8_utils::v8_string(scope, r#"
    (function() {
        var extras = ['name', 'status', 'closed'];
        var windowCtor = globalThis.Window;
        var windowProto = windowCtor && windowCtor.prototype;
        for (var i = 0; i < extras.length; i++) {
            (function(name) {
                try {
                    var desc = Object.getOwnPropertyDescriptor(globalThis, name);
                    if (!desc) {
                        // Property doesn't exist on globalThis — install it
                        var getter = function() { return null; };
                        try { Object.defineProperty(getter, 'name', { value: 'get ' + name }); } catch(e) {}
                        try { Object.defineProperty(getter, 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                        var setter = (name === 'status') ? function(v) {
                            Object.defineProperty(globalThis, name, { value: v, writable: true, enumerable: true, configurable: true });
                        } : undefined;
                        if (setter) {
                            try { Object.defineProperty(setter, 'name', { value: 'set ' + name }); } catch(e) {}
                            try { Object.defineProperty(setter, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                        }
                        Object.defineProperty(globalThis, name, {
                            get: getter, set: setter,
                            enumerable: true, configurable: true
                        });
                        return;
                    }
                    if (desc.get || desc.set) return; // Already accessor
                    if (!desc.configurable) return;
                    var value = desc.value;
                    var getter = (function(v) {
                        return function() {
                            if (windowProto && this !== globalThis && this !== windowProto) {
                                var cur = Object.getPrototypeOf(this);
                                var found = false;
                                for (var k = 0; k < 30; k++) {
                                    if (cur === windowProto) { found = true; break; }
                                    if (!cur) break;
                                    cur = Object.getPrototypeOf(cur);
                                }
                                if (!found) throw new TypeError('Illegal invocation');
                            }
                            return v;
                        };
                    })(value);
                    try { Object.defineProperty(getter, 'name', { value: 'get ' + name }); } catch(e) {}
                    try { Object.defineProperty(getter, 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                    var setter = (name === 'status') ? (function(nm, wproto) {
                        return function(v) {
                            if (wproto && this !== globalThis && this !== wproto) {
                                throw new TypeError('Illegal invocation');
                            }
                            Object.defineProperty(globalThis, nm, { value: v, writable: true, enumerable: true, configurable: true });
                        };
                    })(name, windowProto) : undefined;
                    if (setter) {
                        try { Object.defineProperty(setter, 'name', { value: 'set ' + name }); } catch(e) {}
                        try { Object.defineProperty(setter, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                    }
                    Object.defineProperty(globalThis, name, {
                        get: getter, set: setter,
                        enumerable: desc.enumerable !== false, configurable: true
                    });
                } catch(e) {}
            })(extras[i]);
        }
    })();
"#);
            let _ = v8::Script::compile(scope, extra_accessor_js, None).and_then(|s| s.run(scope));

            // JS op callback replacement — disabled for now
            // let op_fix_js = crate::v8_utils::v8_string(scope, r#"
            //     (function() {
            //         var cbMap = globalThis.__iv8OpCallbacks;
            //         if (!cbMap) return;
            //         var replaced = [];
            //         var keys = Object.getOwnPropertyNames(cbMap);
            //         for (var i = 0; i < keys.length; i++) {
            //             var key = keys[i];
            //             var parts = key.split('.');
            //             if (parts.length !== 2) continue;
            //             var ifaceName = parts[0];
            //             var opName = parts[1];
            //             var realFn = cbMap[key];
            //             if (typeof realFn !== 'function') continue;
            //             try {
            //                 var ctor = globalThis[ifaceName];
            //                 if (!ctor || !ctor.prototype) continue;
            //                 var proto = ctor.prototype;
            //                 var desc = Object.getOwnPropertyDescriptor(proto, opName);
            //                 if (!desc || typeof desc.value !== 'function') continue;
            //                 var fnStr = '';
            //                 try { fnStr = desc.value.toString(); } catch(e) { continue; }
            //                 if (fnStr.indexOf('[native code]') === -1) continue;
            //                 Object.defineProperty(proto, opName, {
            //                     value: realFn,
            //                     writable: desc.writable !== false,
            //                     enumerable: desc.enumerable !== false,
            //                     configurable: true
            //                 });
            //                 replaced.push(key);
            //             } catch(e) {}
            //         }
            //         // Debug: store replaced operations on globalThis
            //         // globalThis.__iv8ReplacedOps = replaced;
            //         try { delete globalThis.__iv8OpCallbacks; } catch(e) {}
            //     })();
            // "#);
            // let _ = v8::Script::compile(scope, op_fix_js, None).and_then(|s| s.run(scope));

            let fix_proto_js = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    var shimEvent = globalThis.Event;
                    var shimMouseEvent = globalThis.MouseEvent;
                    var fixes = [
                        ['TrackEvent','Event'], ['SubmitEvent','Event'], ['FormDataEvent','Event'],
                        ['ToggleEvent','Event'], ['CommandEvent','Event'],
                        ['DragEvent','MouseEvent'],
                        ['AudioTrackList','EventTarget'], ['VideoTrackList','EventTarget'],
                        ['TextTrackList','EventTarget'], ['TextTrack','EventTarget'],
                        ['TextTrackCue','EventTarget'], ['OffscreenCanvas','EventTarget'],
                        ['CloseWatcher','EventTarget'], ['Navigation','EventTarget'],
                        ['NavigationHistoryEntry','EventTarget'],
                        ['NavigateEvent','Event'], ['NavigationCurrentEntryChangeEvent','Event'],
                        ['PopStateEvent','Event'], ['HashChangeEvent','Event'],
                        ['PageSwapEvent','Event'], ['PageRevealEvent','Event'],
                        ['PageTransitionEvent','Event'], ['BeforeUnloadEvent','Event'],
                        ['ErrorEvent','Event'], ['PromiseRejectionEvent','Event'],
                        ['MessageEvent','Event'], ['StorageEvent','Event'],
                        ['EventSource','EventTarget'], ['MessagePort','EventTarget'],
                        ['BroadcastChannel','EventTarget'], ['Worker','EventTarget'],
                        ['SharedWorker','EventTarget'], ['Storage','EventTarget'],
                        ['RadioNodeList','NodeList'],
                        ['CustomEvent','Event'],
                        ['AbortSignal','EventTarget'],
                        ['XMLDocument','Document'],
                        ['DocumentType','Node'],
                        ['DocumentFragment','Node'],
                        ['Attr','Node'],
                        ['Navigator','EventTarget'],
                        ['EventTarget','Object'],
                        ['MediaQueryList','EventTarget'],
                        ['MediaQueryListEvent','Event'],
                        ['CharacterData','Node'],
                        ['Text','CharacterData'],
                        ['CDATASection','Text'],
                        ['Comment','CharacterData'],
                        ['ProcessingInstruction','CharacterData'],
                        ['Node','EventTarget'],
                        ['Element','Node'],
                        ['HTMLElement','Element'],
                        ['Screen','Object'],
                        ['VisualViewport','EventTarget'],
                        ['Location','Object'],
                        ['IDBRequest','EventTarget'], ['IDBDatabase','EventTarget'],
                        ['IDBTransaction','EventTarget'], ['IDBVersionChangeEvent','Event'],
                        ['IDBOpenDBRequest','IDBRequest'],
                        ['Performance','EventTarget'],
                        ['ScreenOrientation','EventTarget'],
                        ['PerformanceEntry','Object'],
                        ['PerformanceResourceTiming','PerformanceEntry'],
                        ['PerformanceNavigationTiming','PerformanceResourceTiming'],
                        ['PerformanceObserver','EventTarget'],
                        ['XMLHttpRequestEventTarget','EventTarget'],
                        ['XMLHttpRequest','XMLHttpRequestEventTarget'],
                        ['XMLHttpRequestUpload','XMLHttpRequestEventTarget'],
                        ['WebSocket','EventTarget'],
                        ['Animation','EventTarget'],
                        ['FileReader','EventTarget'],
                    ];
                    for (var i = 0; i < fixes.length; i++) {
                        var child = fixes[i][0], parent = fixes[i][1];
                        try {
                            var childCtor = globalThis[child];
                            var parentCtor = globalThis[parent];
                            if (childCtor && parentCtor) {
                                Object.setPrototypeOf(childCtor, parentCtor);
                                var childProto = childCtor.prototype;
                                var parentProto = parentCtor.prototype;
                                if (childProto && parentProto) {
                                    Object.setPrototypeOf(childProto, parentProto);
                                }
                            }
                        } catch(e) {}
                    }
                    // Fix .constructor on prototypes that got mismatched by shim overrides
                    var ctorFixes = [
                        'Location', 'Navigator', 'BroadcastChannel', 'MessagePort',
                        'Worker', 'SharedWorker', 'Storage', 'Screen',
                        'EventTarget', 'Node', 'Document', 'Element', 'HTMLElement',
                        'CharacterData', 'Text', 'Comment', 'Event', 'CustomEvent',
                        'MouseEvent', 'VisualViewport', 'MediaQueryList',
                    ];
                    for (var i = 0; i < ctorFixes.length; i++) {
                        try {
                            var ctor = globalThis[ctorFixes[i]];
                            if (ctor && ctor.prototype) {
                                Object.defineProperty(ctor.prototype, 'constructor', {
                                    value: ctor, writable: true, configurable: true, enumerable: false
                                });
                            }
                        } catch(e) {}
                    }
                    // Fix constructor __proto__ — shim constructors (illegal_constructor)
                    // have wrong __proto__. Must be Function.prototype per WebIDL.
                    var functionProto = Function.prototype;
                    var protoFixes = [
                        'Location', 'Navigator', 'Storage', 'Screen',
                    ];
                    for (var i = 0; i < protoFixes.length; i++) {
                        try {
                            var ctor = globalThis[protoFixes[i]];
                            if (ctor && typeof ctor === 'function') {
                                Object.setPrototypeOf(ctor, functionProto);
                            }
                        } catch(e) {}
                    }
                    // EventTarget inheritors: constructor __proto__ must be EventTarget
                    var etCtor = globalThis.EventTarget;
                    var etInheritors = [
                        'MessagePort', 'BroadcastChannel', 'Worker', 'SharedWorker',
                        'EventSource', 'AbortSignal', 'Navigation',
                    ];
                    for (var i = 0; i < etInheritors.length; i++) {
                        try {
                            var ctor = globalThis[etInheritors[i]];
                            if (ctor && typeof ctor === 'function' && etCtor) {
                                Object.setPrototypeOf(ctor, etCtor);
                            }
                        } catch(e) {}
                    }
                    // Storage.prototype.__proto__ must be Object.prototype (not Function)
                    try {
                        var storageCtor = globalThis.Storage;
                        if (storageCtor && storageCtor.prototype) {
                            Object.setPrototypeOf(storageCtor.prototype, Object.prototype);
                        }
                    } catch(e) {}
                    // WindowProperties: Window.prototype.__proto__ must be WindowProperties.prototype
                    // WindowProperties is not in webref IDL, so we create it manually.
                    // NOTE: creation moved to crate::shims::window_properties::WINDOW_PROPERTIES_SHIM_JS,
                    // eval'd in install_browser_surface_init (step 12c).
                    // Wrap DOM template operations with null/undefined receiver check.
                    // DOM template callbacks (dom/template.rs) are empty implementations
                    // without receiver check. idlharness requires: calling operation
                    // with this=null must throw TypeError.
                    // codegen operations (via ObjectTemplate::set) DO have receiver check
                    // (R3 prototype chain check) — their callbacks are correctly invoked.
                    // Only wrap DOM template interfaces' methods.
                    // Shim JS operation receiver check.
                    // Only wrap JS functions (toString does NOT contain [native code]).
                    // [native code] functions (codegen + DOM template) already have
                    // receiver check (R3 prototype chain check / null_this_check).
                    // Shim JS functions (event_constructors.rs, message_channel.rs, etc.)
                    // have no receiver check — wrap them with null/undefined check.
                    // Note: V8 non-strict mode converts null→globalThis, so we check
                    // globalThis as well.
                    var shimOpInterfaces = [
                        'Event', 'CustomEvent', 'MouseEvent',
                        'MessagePort', 'BroadcastChannel', 'Worker', 'SharedWorker',
                        'Storage', 'Navigator',
                        'NodeList', 'MutationObserver', 'DOMTokenList',
                    ];
                    for (let i = 0; i < shimOpInterfaces.length; i++) {
                        try {
                            var ctor = globalThis[shimOpInterfaces[i]];
                            if (!ctor || !ctor.prototype) continue;
                            var proto = ctor.prototype;
                            var names = Object.getOwnPropertyNames(proto);
                            for (let j = 0; j < names.length; j++) {
                                let pname = names[j];
                                if (pname === 'constructor') continue;
try {
                                    var desc = Object.getOwnPropertyDescriptor(proto, pname);
                                    if (!desc || typeof desc.value !== 'function') continue;
                                    // Skip already-wrapped functions
                                    if (desc.value.__iv8_op_wrapped) continue;
                                    // For [native code] functions, check if they already throw
                                    // by testing with null this (V8 converts to globalThis)
                                    var fnStr = '';
                                    try { fnStr = desc.value.toString(); } catch(e) { continue; }
                                    var isNative = fnStr.indexOf('[native code]') !== -1;
                                    // Test if the function throws on wrong receiver
                                    var alreadyThrows = false;
                                    if (isNative) {
                                        try {
                                            desc.value.call({});
                                        } catch(e) {
                                            alreadyThrows = true;
                                        }
                                    }
                                    if (alreadyThrows) continue;
                                    // Wrap both JS and native functions that don't throw
                                    let origFn = desc.value;
                                    let expectedTag = shimOpInterfaces[i];
                                    let origName = origFn.name || pname;
                                    let origLen = origFn.length;
                                    // expectedTag already defined above
                                    let wrappedFn = function() {
                                        // Check if this is the correct interface instance
                                        // by comparing Symbol.toStringTag
                                        var thisTag = '';
                                        try { thisTag = this[Symbol.toStringTag]; } catch(e) {}
                                        if (thisTag !== expectedTag && this !== globalThis[shimOpInterfaces[i]].prototype) {
                                            // Also allow if this is an instance of the interface
                                            // (check prototype chain for the constructor)
                                            var isValid = false;
                                            try {
                                                var cur = Object.getPrototypeOf(this);
                                                var expectedProto = globalThis[expectedTag].prototype;
                                                for (var k = 0; k < 30; k++) {
                                                    if (cur === expectedProto) { isValid = true; break; }
                                                    if (!cur) break;
                                                    cur = Object.getPrototypeOf(cur);
                                                }
                                            } catch(e) {}
                                            if (!isValid) {
                                                throw new TypeError('Illegal invocation');
                                            }
                                        }
                                        return origFn.apply(this, arguments);
                                    };
                                    wrappedFn.__iv8_op_wrapped = true;
                                    try { Object.defineProperty(wrappedFn, 'name', { value: origName }); } catch(e) {}
                                    try { Object.defineProperty(wrappedFn, 'length', { value: origLen }); } catch(e) {}
                                    Object.defineProperty(proto, pname, {
                                        value: wrappedFn,
                                        writable: desc.writable,
                                        enumerable: desc.enumerable,
                                        configurable: true
                                    });
                                } catch(e) {}
                            }
                        } catch(e) {}
                    }
                    // idlharness requires: calling getter on prototype object
                    // (or wrong-type receiver) must throw TypeError.
                    // codegen getters already have this check, but shim-installed
                    // getters (document_props.rs, event_constructors.rs) don't.
                    // This wraps ALL accessor getters on key prototypes.
                    var receiverCheckInterfaces = [
                        'Document', 'CustomEvent', 'MouseEvent',
                        'HTMLElement', 'Element', 'Node', 'Window',
                        'NavigationTransition', 'ShadowRoot',
                    ];
                    for (let i = 0; i < receiverCheckInterfaces.length; i++) {
                        let ifaceName = receiverCheckInterfaces[i];
                        try {
                            var ctor = globalThis[ifaceName];
                            if (!ctor || !ctor.prototype) continue;
                            var proto = ctor.prototype;
                            var names = Object.getOwnPropertyNames(proto);
                            for (let j = 0; j < names.length; j++) {
                                let pname = names[j];
                                if (pname === 'constructor') continue;
                                if (pname === 'attributes') continue;
                                if (pname.startsWith('on')) continue;
                                try {
                                    var desc = Object.getOwnPropertyDescriptor(proto, pname);
                                    if (!desc || !desc.get) continue;
                                    var origGet = desc.get;
                                    var origSet = desc.set;
                                    var alreadyWrapped = desc.get && desc.get.__iv8_wrapped;
                                    if (alreadyWrapped && (!desc.set || desc.set.__iv8_set_wrapped)) continue;
                                    // Skip [native code] getters: V8 does not invoke
                                    // FunctionTemplate callbacks when called from within
                                    // a JS accessor getter. Codegen getters (fix_accessor_properties)
                                    // already have R3 receiver check. DOM template getters
                                    // have null_this_check. Only shim JS getters need wrapping.
                                    if (origGet.toString().indexOf('[native code]') !== -1) continue;
                                    let thisIfaceName = ifaceName;
                                    var wrappedGet;
                                    if (alreadyWrapped) {
                                        wrappedGet = origGet;
                                    } else {
                                        wrappedGet = function() {
                                        var thisCtor = globalThis[thisIfaceName];
                                        if (thisCtor && thisCtor.prototype) {
                                            if (this === thisCtor.prototype) {
                                                throw new TypeError('Illegal invocation');
                                            }
                                            var isValid = false;
                                            var cur = Object.getPrototypeOf(this);
                                            for (var k = 0; k < 30; k++) {
                                                if (cur === thisCtor.prototype) { isValid = true; break; }
                                                if (!cur) break;
                                                cur = Object.getPrototypeOf(cur);
                                            }
                                            if (!isValid) {
                                                throw new TypeError('Illegal invocation');
                                            }
                                        }
                                        // For event handlers, check hidden property first
                                        if (pname.indexOf('on') === 0 && pname.length > 2) {
                                            var hv = this['__iv8_' + pname];
                                            if (hv !== undefined) return hv;
                                            // V8 does not invoke FunctionTemplate callbacks
                                            // when called from within a JS accessor getter.
                                            // Return null instead of calling origGet.call(this)
                                            // which would throw "Illegal invocation".
                                            return null;
                                        }
                                        return origGet.call(this);
                                    };
                                    wrappedGet.__iv8_wrapped = true;
                                    try { Object.defineProperty(wrappedGet, 'name', { value: 'get ' + pname }); } catch(e) {}
                                    }
                                    // Wrap setter with same receiver check
                                    var wrappedSet = origSet;
                                    if (typeof origSet === 'function' && origSet.toString().indexOf('[native code]') === -1) {
                                        // For event handlers (on*), use simple property assignment
                                        // instead of codegen setter (which has wrong receiver check)
                                        if (pname.indexOf('on') === 0 && pname.length > 2) {
                                            wrappedSet = function(v) {
                                                var thisCtor2 = globalThis[thisIfaceName];
                                                if (thisCtor2 && thisCtor2.prototype) {
                                                    if (this === thisCtor2.prototype) {
                                                        throw new TypeError('Illegal invocation');
                                                    }
                                                    var isValid2 = false;
                                                    var cur2 = Object.getPrototypeOf(this);
                                                    for (var k2 = 0; k2 < 30; k2++) {
                                                        if (cur2 === thisCtor2.prototype) { isValid2 = true; break; }
                                                        if (!cur2) break;
                                                        cur2 = Object.getPrototypeOf(cur2);
                                                    }
                                                    if (!isValid2) {
                                                        throw new TypeError('Illegal invocation');
                                                    }
                                                }
                                                // Store event handler value in hidden property
                                                // to avoid triggering setter recursion
                                                Object.defineProperty(this, '__iv8_' + pname, { value: v, writable: true, enumerable: false, configurable: true });
                                            };
                                        } else {
                                            wrappedSet = function(v) {
                                                var thisCtor2 = globalThis[thisIfaceName];
                                                if (thisCtor2 && thisCtor2.prototype) {
                                                    if (this === thisCtor2.prototype) {
                                                        throw new TypeError('Illegal invocation');
                                                    }
                                                    var isValid2 = false;
                                                    var cur2 = Object.getPrototypeOf(this);
                                                    for (var k2 = 0; k2 < 30; k2++) {
                                                        if (cur2 === thisCtor2.prototype) { isValid2 = true; break; }
                                                        if (!cur2) break;
                                                        cur2 = Object.getPrototypeOf(cur2);
                                                    }
                                                    if (!isValid2) {
                                                        throw new TypeError('Illegal invocation');
                                                    }
                                                }
                                                // Use defineProperty to set own data property
                                                // (avoids infinite recursion through this[pname] = v)
                                                Object.defineProperty(this, pname, { value: v, writable: true, enumerable: true, configurable: true });
                                            };
                                        }
                                        try { Object.defineProperty(wrappedSet, 'name', { value: 'set ' + pname }); } catch(e) {}
                                        wrappedSet.__iv8_set_wrapped = true;
                                    }
                                    Object.defineProperty(proto, pname, {
                                        get: wrappedGet,
                                        set: wrappedSet,
                                        enumerable: desc.enumerable,
                                        configurable: true
                                    });
                                } catch(e) {}
                            }
                        } catch(e) {}
                    }
                })();
            "#);
            let _ = v8::Script::compile(scope, fix_proto_js, None).and_then(|s| s.run(scope));

            // Fix readonly attribute setters: idlharness expects setter=undefined
            // for readonly attributes. Some accessor wrappers install a JS setter.
            let readonly_fix_js = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    var readonlyAttrs = {
                        'Event': ['type','target','currentTarget','srcElement','eventPhase',
                                  'bubbles','cancelable','timeStamp','defaultPrevented','composed'],
                        'MouseEvent': ['screenX','screenY','clientX','clientY','ctrlKey','shiftKey',
                                       'altKey','metaKey','button','buttons','relatedTarget','region'],
                        'CustomEvent': ['detail'],
                    };
                    for (var iface in readonlyAttrs) {
                        var ctor = globalThis[iface];
                        if (!ctor || !ctor.prototype) continue;
                        var attrs = readonlyAttrs[iface];
                        for (var i = 0; i < attrs.length; i++) {
                            var desc = Object.getOwnPropertyDescriptor(ctor.prototype, attrs[i]);
                            if (desc && desc.get && desc.set) {
                                try {
                                    Object.defineProperty(ctor.prototype, attrs[i], {
                                        get: desc.get, set: undefined,
                                        enumerable: desc.enumerable, configurable: true
                                    });
                                } catch(e) {}
                            }
                        }
                    }
                })();
            "#);
            let _ = v8::Script::compile(scope, readonly_fix_js, None).and_then(|s| s.run(scope));

            // Fix operation .name and .length on key prototypes.
            // codegen sets .name via set_class_name but V8 may not persist it
            // on the function object. .length may be wrong for overloaded ops.
            let name_length_js = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    var fixes = {
                        'Window': { 'postMessage': { name: 'postMessage', length: 1 } },
                        'HTMLCanvasElement': {
                            'getContext': { name: 'getContext', length: 1 },
                            'toDataURL': { name: 'toDataURL', length: 0 }
                        },
                        'CanvasRenderingContext2D': {
                            'setTransform': { name: 'setTransform', length: 0 }
                        },
                        'OffscreenCanvasRenderingContext2D': {
                            'setTransform': { name: 'setTransform', length: 0 },
                            'createLinearGradient': { name: 'createLinearGradient', length: 4 },
                            'createRadialGradient': { name: 'createRadialGradient', length: 6 },
                            'createConicGradient': { name: 'createConicGradient', length: 1 },
                            'drawImage': { name: 'drawImage', length: 3 },
                            'fillText': { name: 'fillText', length: 3 },
                            'strokeText': { name: 'strokeText', length: 3 },
                            'putImageData': { name: 'putImageData', length: 3 },
                        },
                    };
                    for (var ifaceName in fixes) {
                        try {
                            var ctor = globalThis[ifaceName];
                            if (!ctor || !ctor.prototype) continue;
                            var proto = ctor.prototype;
                            var ifaceFixes = fixes[ifaceName];
                            for (var opName in ifaceFixes) {
                                try {
                                    var fn = proto[opName];
                                    if (!fn || typeof fn !== 'function') continue;
                                    var fix = ifaceFixes[opName];
                                    if (fn.name !== fix.name) {
                                        try { Object.defineProperty(fn, 'name', {
                                            value: fix.name, writable: false,
                                            enumerable: false, configurable: true
                                        }); } catch(e) {}
                                    }
                                    if (fn.length !== fix.length) {
                                        try { Object.defineProperty(fn, 'length', {
                                            value: fix.length, writable: false,
                                            enumerable: false, configurable: true
                                        }); } catch(e) {}
                                    }
                                } catch(e) {}
                            }
                        } catch(e) {}
                    }
                    // Also fix [Global] operations on globalThis (not on prototype)
                    try {
                        var pm = globalThis.postMessage;
                        if (pm && typeof pm === 'function') {
                            if (pm.name !== 'postMessage') {
                                try { Object.defineProperty(pm, 'name', {
                                    value: 'postMessage', writable: false,
                                    enumerable: false, configurable: true
                                }); } catch(e) {}
                            }
                            if (pm.length !== 1) {
                                try { Object.defineProperty(pm, 'length', {
                                    value: 1, writable: false,
                                    enumerable: false, configurable: true
                                }); } catch(e) {}
                            }
                        }
                    } catch(e) {}

                    // Fix Event.prototype.initEvent length (should be 1, not 3)
                    try {
                        if (typeof Event !== 'undefined' && Event.prototype) {
                            var ie = Event.prototype.initEvent;
                            if (ie && typeof ie === 'function' && ie.length !== 1) {
                                try { Object.defineProperty(ie, 'length', {
                                    value: 1, writable: false,
                                    enumerable: false, configurable: true
                                }); } catch(e) {}
                            }
                        }
                    } catch(e) {}

                    // Fix CanvasRenderingContext2D createImageData length
                    try {
                        if (typeof CanvasRenderingContext2D !== 'undefined' && CanvasRenderingContext2D.prototype) {
                            var cid = CanvasRenderingContext2D.prototype.createImageData;
                            if (cid && typeof cid === 'function' && cid.length !== 1) {
                                try { Object.defineProperty(cid, 'length', {
                                    value: 1, writable: false,
                                    enumerable: false, configurable: true
                                }); } catch(e) {}
                            }
                        }
                    } catch(e) {}

                    // Fix OffscreenCanvasRenderingContext2D createImageData + createConicGradient
                    try {
                        if (typeof OffscreenCanvasRenderingContext2D !== 'undefined' && OffscreenCanvasRenderingContext2D.prototype) {
                            var oproto = OffscreenCanvasRenderingContext2D.prototype;
                            var ocid = oproto.createImageData;
                            if (ocid && typeof ocid === 'function' && ocid.length !== 1) {
                                try { Object.defineProperty(ocid, 'length', {
                                    value: 1, writable: false,
                                    enumerable: false, configurable: true
                                }); } catch(e) {}
                            }
                            var occg = oproto.createConicGradient;
                            if (occg && typeof occg === 'function' && occg.length !== 3) {
                                try { Object.defineProperty(occg, 'length', {
                                    value: 3, writable: false,
                                    enumerable: false, configurable: true
                                }); } catch(e) {}
                            }
                        }
                    } catch(e) {}

                    // Fix Navigator registerProtocolHandler/unregisterProtocolHandler length
                    try {
                        if (typeof Navigator !== 'undefined' && Navigator.prototype) {
                            var rph = Navigator.prototype.registerProtocolHandler;
                            if (rph && typeof rph === 'function' && rph.length !== 2) {
                                try { Object.defineProperty(rph, 'length', {
                                    value: 2, writable: false,
                                    enumerable: false, configurable: true
                                }); } catch(e) {}
                            }
                            var uph = Navigator.prototype.unregisterProtocolHandler;
                            if (uph && typeof uph === 'function' && uph.length !== 2) {
                                try { Object.defineProperty(uph, 'length', {
                                    value: 2, writable: false,
                                    enumerable: false, configurable: true
                                }); } catch(e) {}
                            }
                        }
                    } catch(e) {}

                    // Wrap HTMLMediaElement.canPlayType for arg count validation
                    try {
                        if (typeof HTMLMediaElement !== 'undefined' && HTMLMediaElement.prototype) {
                            var origCPT = HTMLMediaElement.prototype.canPlayType;
                            if (origCPT && typeof origCPT === 'function') {
                                var wCPT = function canPlayType(type) {
                                    if (arguments.length < 1) throw new TypeError("1 argument(s) required, but only 0 present.");
                                    return origCPT.call(this, type);
                                };
                                try { Object.defineProperty(wCPT, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                                try { Object.defineProperty(wCPT, 'name', { value: 'canPlayType', writable: false, enumerable: false, configurable: true }); } catch(e) {}
                                Object.defineProperty(HTMLMediaElement.prototype, 'canPlayType', { value: wCPT, writable: true, configurable: true, enumerable: true });
                            }
                        }
                    } catch(e) {}

                    // Wrap HTMLCanvasElement.getContext for arg count validation
                    try {
                        if (typeof HTMLCanvasElement !== 'undefined' && HTMLCanvasElement.prototype) {
                            var origGC = HTMLCanvasElement.prototype.getContext;
                            if (origGC && typeof origGC === 'function') {
                                var wGC = function getContext(contextId, options) {
                                    if (arguments.length < 1) throw new TypeError("1 argument(s) required, but only 0 present.");
                                    return origGC.call(this, contextId, options);
                                };
                                try { Object.defineProperty(wGC, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                                try { Object.defineProperty(wGC, 'name', { value: 'getContext', writable: false, enumerable: false, configurable: true }); } catch(e) {}
                                Object.defineProperty(HTMLCanvasElement.prototype, 'getContext', { value: wGC, writable: true, configurable: true, enumerable: true });
                            }
                        }
                    } catch(e) {}
                })();
            "#);
            let _ = v8::Script::compile(scope, name_length_js, None).and_then(|s| s.run(scope));

            let js = crate::v8_utils::v8_string(scope, r#"
                (function() {
                    var workerOnly = ['WorkerGlobalScope','DedicatedWorkerGlobalScope',
                        'SharedWorkerGlobalScope','ServiceWorkerGlobalScope',
                        'WorkerNavigator','WorkerLocation','WorkletGlobalScope',
                        'AnimationWorkletGlobalScope','AudioWorkletGlobalScope',
                        'LayoutWorkletGlobalScope','PaintWorkletGlobalScope',
                        'RTCIdentityProviderGlobalScope'];
                    for (var i = 0; i < workerOnly.length; i++) {
                        try { delete globalThis[workerOnly[i]]; } catch(e) {}
                    }
                    var names = Object.getOwnPropertyNames(globalThis);
                    for (var i = 0; i < names.length; i++) {
                        try {
                            var ctor = globalThis[names[i]];
                            if (ctor && typeof ctor === 'function' && ctor.prototype) {
                                Object.defineProperty(ctor, 'prototype', {
                                    writable: false, enumerable: false, configurable: false
                                });
                                Object.preventExtensions(ctor.prototype);
                            }
                        } catch(e) {}
                    }
                })();
                // Crypto/SubtleCrypto: Rust installs on prototype via
                // with_prototype_and_properties (random.rs + subtle.rs)
                // JS post-fix only sets Window.crypto accessor
                (function() {
                    // Window.crypto accessor
                    if (typeof Window !== 'undefined' && Window.prototype && Object.isExtensible(Window.prototype) && typeof crypto !== 'undefined') {
                        var _cryptoVal = crypto;
                        Object.defineProperty(Window.prototype, 'crypto', {
                            get: function() { return _cryptoVal; },
                            enumerable: true, configurable: true
                        });
                    }
                })();
                // Fix FileReader/FileReaderSync: prototype chain + remove FileReaderSync
                (function() {
                    if (typeof FileReader !== 'undefined' && typeof EventTarget !== 'undefined') {
                        Object.setPrototypeOf(FileReader, EventTarget);
                    }
                    try { delete globalThis.FileReaderSync; } catch(e) {}
                })();
                // Fix ScreenOrientation: set prototype chain + move methods
                (function() {
                    if (typeof screen === 'undefined' || typeof ScreenOrientation === 'undefined') return;
                    var so = screen.orientation;
                    if (!so) return;
                    var soProto = ScreenOrientation.prototype;
                    if (!soProto) return;
                    // Move own properties to prototype
                    var soNames = Object.getOwnPropertyNames(so);
                    for (var i = 0; i < soNames.length; i++) {
                        var prop = soNames[i];
                        if (typeof so[prop] === 'function' && !soProto[prop]) {
                            soProto[prop] = so[prop];
                            delete so[prop];
                        }
                    }
                    Object.setPrototypeOf(so, soProto);
                    if (typeof EventTarget !== 'undefined') {
                        Object.setPrototypeOf(ScreenOrientation, EventTarget);
                    }
                    // screen.orientation accessor on Screen.prototype
                    if (typeof Screen !== 'undefined' && Screen.prototype && Object.isExtensible(Screen.prototype)) {
                        var _soVal = so;
                        Object.defineProperty(Screen.prototype, 'orientation', {
                            get: function() { return _soVal; },
                            enumerable: true, configurable: true
                        });
                    }
                })();
                (function() {
                    // Performance: Rust installs on prototype (date_interceptor.rs)
                    // JS post-fix only sets constructor __proto__ + Window accessor
                    if (typeof Performance !== 'undefined' && typeof EventTarget !== 'undefined') {
                        Object.setPrototypeOf(Performance, EventTarget);
                    }
                    if (typeof Window !== 'undefined' && Window.prototype && Object.isExtensible(Window.prototype)) {
                        var _perfVal = performance;
                        Object.defineProperty(Window.prototype, 'performance', {
                            get: function() { return _perfVal; },
                            enumerable: true, configurable: true
                        });
                    }
                    // Window.crypto accessor
                    if (typeof Window !== 'undefined' && Window.prototype && Object.isExtensible(Window.prototype) && typeof crypto !== 'undefined') {
                        var _cryptoVal = crypto;
                        Object.defineProperty(Window.prototype, 'crypto', {
                            get: function() { return _cryptoVal; },
                            enumerable: true, configurable: true
                        });
                    }
                })();
            "#);
            let _ = v8::Script::compile(scope, js, None).and_then(|s| s.run(scope));
        });
    }

    /// Assert we're on the creator thread (debug only).
    #[inline]
    fn assert_thread(&self) {
        debug_assert_eq!(
            std::thread::current().id(),
            self.creator_thread,
            "EmbeddedV8Kernel used from a different thread than creation"
        );
    }

    /// Get a reference to the environment map.
    pub fn environment(&self) -> &EnvironmentMap {
        &self.environment
    }

    /// Get a reference to the underlying isolate (for RuntimeState access).
    pub fn isolate_ref(&self) -> &v8::Isolate {
        &self.isolate
    }

    /// Evaluate JS source code and return the raw V8 Global value.
    pub fn eval(
        &mut self,
        source: &str,
        opts: EvalOpts,
    ) -> Result<v8::Global<v8::Value>, IV8Error> {
        self.assert_thread();

        // Enter isolate for this operation
        // SAFETY: we exit at the end of this function (or on error return)
        unsafe {
            self.isolate.enter();
        }

        let result = self.eval_inner(source, opts);

        // Run microtasks after each eval (matches browser behavior)
        self.isolate.perform_microtask_checkpoint();

        // Drain worker messages while still inside the isolate scope,
        // so onmessage callbacks can access JS variables from the eval.
        self.drain_worker_messages();

        // Exit isolate after operation
        unsafe {
            self.isolate.exit();
        }

        result
    }

    /// Inner eval implementation (isolate must already be entered).
    fn eval_inner(
        &mut self,
        source: &str,
        opts: EvalOpts,
    ) -> Result<v8::Global<v8::Value>, IV8Error> {
        v8::scope!(handle_scope, &mut self.isolate);
        let context = v8::Local::new(handle_scope, &self.context);
        v8::scope_with_context!(scope, handle_scope, context);

        // Create source string
        let source_str = v8::String::new(scope, source).ok_or_else(|| {
            IV8Error::Internal("failed to create V8 source string (too long?)".into())
        })?;

        // Set up script origin if provided
        let origin = if let Some(ref url) = opts.source_url {
            let name = v8::String::new(scope, url)
                .unwrap_or_else(|| crate::v8_utils::v8_string(scope, "<eval>"));
            Some(v8::ScriptOrigin::new(
                scope,
                name.into(),
                opts.line_offset,
                opts.column_offset,
                false,
                0,
                None,
                false,
                false,
                false,
                None,
            ))
        } else {
            None
        };

        // TryCatch scope
        v8::tc_scope!(tc, scope);

        // Compile
        let script = v8::Script::compile(tc, source_str, origin.as_ref());

        let script = match script {
            Some(s) => s,
            None => {
                // Extract compile error inline (tc type is known here)
                let msg = if let Some(exc) = tc.exception() {
                    exc.to_rust_string_lossy(tc)
                } else {
                    "unknown compile error".to_string()
                };
                let (line, col) = if let Some(m) = tc.message() {
                    (
                        m.get_line_number(tc).unwrap_or(0) as i32,
                        m.get_start_column() as i32,
                    )
                } else {
                    (0, 0)
                };
                return Err(IV8Error::Compile {
                    message: msg,
                    line,
                    column: col,
                });
            }
        };

        // Run
        let result = script.run(tc);

        // Check termination
        if tc.has_terminated() {
            return Err(IV8Error::Terminated);
        }

        // Check exception
        if tc.has_caught() {
            // SAFETY: guarded by has_caught() check above
            let exception = tc.exception().expect("has_caught but no exception");
            let message = exception.to_rust_string_lossy(tc);
            let stack = tc
                .stack_trace()
                .map(|s| s.to_rust_string_lossy(tc))
                .unwrap_or_default();

            let name = if exception.is_native_error() {
                if let Some(obj) = exception.to_object(tc) {
                    let name_key = crate::v8_utils::v8_string(tc, "name");
                    obj.get(tc, name_key.into())
                        .map(|v| v.to_rust_string_lossy(tc))
                        .unwrap_or_else(|| "Error".to_string())
                } else {
                    "Error".to_string()
                }
            } else {
                "Error".to_string()
            };

            return Err(IV8Error::Js {
                name,
                message,
                stack,
                value: None,
            });
        }

        let value = result.ok_or_else(|| {
            IV8Error::Internal("script.run returned None without exception".into())
        })?;

        // Increment eval count
        let isolate: &v8::Isolate = &*tc;
        let state = RuntimeState::get(isolate);
        state.increment_eval_count();

        Ok(v8::Global::new(tc, value))
    }

    /// Perform microtask checkpoint.
    pub fn drain_microtasks(&mut self) {
        unsafe {
            self.isolate.enter();
        }
        self.isolate.perform_microtask_checkpoint();
        unsafe {
            self.isolate.exit();
        }
    }

    /// Expose a Rust function to JS global scope.
    /// The function receives args as Vec<String> and returns Result<String, String>.
    /// (Simplified for v0.1 — M2 will add proper V8 value conversion.)
    pub fn expose_fn(&mut self, name: &str, callback: ExposedCallback) {
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);
            crate::expose::expose_function(scope, global, name, callback);
        }
        unsafe {
            self.isolate.exit();
        }
    }

    /// Execute a closure with access to the V8 scope and global object.
    /// The isolate is entered before and exited after the closure runs.
    /// Use this for operations that need direct V8 API access from outside iv8-core.
    pub fn with_global_scope<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&v8::PinScope<'_, '_>, v8::Local<v8::Object>) -> R,
    {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);
            f(scope, global)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }

    /// Install the Worker constructor on the global object.
    /// Set the global object's __proto__ to DedicatedWorkerGlobalScope.prototype.
    /// Called when worker_mode=true, after install_browser_surface_init
    /// (which instantiates codegen templates including DedicatedWorkerGlobalScope)
    /// and before freeze_all_prototypes.
    pub fn set_worker_global_prototype(&mut self) {
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);

            let global = context.global(scope);

            let dwgs_key = crate::v8_utils::v8_string(scope, "DedicatedWorkerGlobalScope");
            if let Some(dwgs_val) = global.get(scope, dwgs_key.into()) {
                if let Ok(dwgs_obj) = <v8::Local<v8::Object>>::try_from(dwgs_val) {
                    let proto_key = crate::v8_utils::v8_string(scope, "prototype");
                    if let Some(proto_val) = dwgs_obj.get(scope, proto_key.into()) {
                        if let Ok(proto_obj) = <v8::Local<v8::Object>>::try_from(proto_val) {
                            let _ = global.set_prototype(scope, proto_obj.into());
                        }
                    }
                }
            }
        }
        unsafe {
            self.isolate.exit();
        }
    }

    /// Creates a FunctionTemplate with worker_constructor_cb, installs
    /// postMessage and terminate on the prototype, and registers on global
    /// as "Worker" with DONT_ENUM.
    pub fn install_worker_constructor(&mut self) {
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);

            let tmpl = v8::FunctionTemplate::builder_raw(worker_constructor_cb).build(scope);
            tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Worker"));

            let proto = tmpl.prototype_template(scope);

            let post_msg_tmpl = v8::FunctionTemplate::builder_raw(worker_post_message_cb).length(1).build(scope);
            let post_msg_name = crate::v8_utils::v8_string(scope, "postMessage");
            post_msg_tmpl.set_class_name(post_msg_name);
            proto.set(post_msg_name.into(), post_msg_tmpl.into());

            let term_tmpl = v8::FunctionTemplate::builder_raw(worker_terminate_cb).build(scope);
            let term_name = crate::v8_utils::v8_string(scope, "terminate");
            term_tmpl.set_class_name(term_name);
            proto.set(term_name.into(), term_tmpl.into());

            let onmsg_name = crate::v8_utils::v8_string(scope, "onmessage");
            proto.set(onmsg_name.into(), v8::null(scope).into());

            let onerror_name = crate::v8_utils::v8_string(scope, "onerror");
            proto.set(onerror_name.into(), v8::null(scope).into());

            let tag_sym = v8::Symbol::get_to_string_tag(scope);
            let tag_val = crate::v8_utils::v8_string(scope, "Worker");
            proto.set(tag_sym.into(), tag_val.into());

            if let Some(func) = tmpl.get_function(scope) {
                let name_key = crate::v8_utils::v8_string(scope, "Worker");
                global.define_own_property(
                    scope,
                    name_key.into(),
                    func.into(),
                    v8::PropertyAttribute::DONT_ENUM,
                );
            }
        }
        unsafe {
            self.isolate.exit();
        }
    }

    /// Drain worker-to-main messages from all worker handles.
    /// Deserializes via structured_clone, creates MessageEvent objects,
    /// and dispatches to worker.onmessage callbacks.
    pub fn drain_worker_messages(&mut self) {
        let pending: Vec<(u64, Vec<u8>)> = {
            let state = crate::state::RuntimeState::get(&self.isolate);
            let workers = state.workers.borrow();
            let mut collected = Vec::new();
            for handle in workers.iter() {
                while let Ok(bytes) = handle.rx.try_recv() {
                    collected.push((handle.worker_id, bytes));
                }
                if collected.is_empty() {
                    if let Ok(bytes) = handle.rx.recv_timeout(std::time::Duration::from_millis(100)) {
                        collected.push((handle.worker_id, bytes));
                        while let Ok(bytes) = handle.rx.try_recv() {
                            collected.push((handle.worker_id, bytes));
                        }
                    }
                }
            }
            collected
        };
        if pending.is_empty() {
            return;
        }

        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);

            let state = crate::state::RuntimeState::get(&*scope);
            for (worker_id, bytes) in pending {
                let worker_global = match state.worker_objects.borrow().get(&worker_id) {
                    Some(g) => g.clone(),
                    None => continue,
                };
                let worker_obj = v8::Local::new(scope, &worker_global);

                let data = crate::shims::structured_clone::deserialize_value(scope, context, &bytes);
                let event = v8::Object::new(scope);
                let data_key = crate::v8_utils::v8_string(scope, "data");
                let data_val = data.unwrap_or_else(|| v8::undefined(scope).into());
                let _ = event.set(scope, data_key.into(), data_val);
                let type_key = crate::v8_utils::v8_string(scope, "type");
                let type_val = crate::v8_utils::v8_string(scope, "message");
                let _ = event.set(scope, type_key.into(), type_val.into());
                let tag_sym = v8::Symbol::get_to_string_tag(scope);
                let tag_val = crate::v8_utils::v8_string(scope, "MessageEvent");
                let _ = event.set(scope, tag_sym.into(), tag_val.into());

                let onmsg_key = crate::v8_utils::v8_string(scope, "onmessage");
                if let Some(handler) = worker_obj.get(scope, onmsg_key.into()) {
                    if handler.is_function() {
                        let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(handler) };
                        let _ = func.call(scope, worker_obj.into(), &[event.into()]);
                    }
                }
            }
        }
        unsafe {
            self.isolate.exit();
        }
    }

    /// Install deterministic overrides for Math.random, crypto, and time.
    ///
    /// Called during init if random_seed / crypto_seed / time_freeze are set.
    /// Uses JS-layer override (not V8 native) — simple and effective for our use case.
    /// ChaosVM caching `var R = Math.random` before our override is handled by
    /// installing this BEFORE any user code runs (including tdc.js).
    fn install_deterministic_overrides_from(
        &mut self,
        random_seed: Option<u64>,
        crypto_seed: Option<u64>,
        time_freeze: Option<f64>,
    ) {
        // Math.random seed: xorshift128+ PRNG in JS
        if let Some(seed) = random_seed {
            let js = format!(
                r#"
(function() {{
    // xorshift128+ seeded PRNG (same algorithm as V8's Math.random)
    var s0 = BigInt({seed}) | 1n;  // ensure non-zero
    var s1 = (BigInt({seed}) * 6364136223846793005n + 1442695040888963407n) & 0xFFFFFFFFFFFFFFFFn;
    var MASK = 0xFFFFFFFFFFFFFFFFn;
    Math.random = function random() {{
        var x = s0;
        var y = s1;
        s0 = y;
        x = x ^ ((x << 23n) & MASK);
        x = x ^ (x >> 17n);
        x = x ^ y;
        x = x ^ (y >> 26n);
        s1 = x & MASK;
        // Convert to [0, 1) float: take upper 52 bits
        var combined = ((s0 + s1) & MASK);
        return Number(combined & 0x1FFFFFFFFFFFFFn) / 9007199254740992;
    }};
}})();
"#,
                seed = seed
            );
            self.eval(&js, EvalOpts::default()).ok();
        }

        // time_freeze: override Date.now, performance.now, new Date()
        if let Some(freeze_ms) = time_freeze {
            let js = format!(
                r#"
(function() {{
    var FROZEN = {freeze_ms};
    Date.now = function now() {{ return FROZEN; }};
    var _OrigDate = Date;
    Date = function Date() {{
        if (arguments.length === 0) return new _OrigDate(FROZEN);
        return new (Function.prototype.bind.apply(_OrigDate, [null].concat(Array.from(arguments))))();
    }};
    Date.now = function now() {{ return FROZEN; }};
    Date.parse = _OrigDate.parse;
    Date.UTC = _OrigDate.UTC;
    Date.prototype = _OrigDate.prototype;
    if (typeof performance !== 'undefined') {{
        performance.now = function now() {{ return 0; }};
    }}
}})();
"#,
                freeze_ms = freeze_ms as u64
            );
            self.eval(&js, EvalOpts::default()).ok();
        }

        // crypto_seed: store in RuntimeState for Rust-side random.rs to use
        if let Some(seed) = crypto_seed {
            let state = crate::state::RuntimeState::get(&self.isolate);
            *state.crypto_seed.borrow_mut() = Some(seed);
        }
    }
}

/// Dispatch behavior installation through BCR if callback exists,
/// otherwise fall back to the direct install function.
fn install_behavior_via_bcr(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    bcr: &iv8_surface::BehaviorCallbackRegistry,
    installer: &iv8_surface::behavior::BehaviorInstaller,
    fallback: fn(&v8::PinScope<'_, '_>, v8::Local<v8::Object>),
) {
    let _ = bcr; // Reserved for v0.8.31 L2-native injection path
    if let Some(ref install_fn) = *installer.borrow() {
        install_fn(scope, global);
    } else {
        fallback(scope, global);
    }
}

/// Install window.top/self/parent identity references.
/// In top-level browsing context these must all point to window itself.
fn install_window_identity_refs(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let keys = ["top", "self", "parent"];
    for &key in &keys {
        let key_str = crate::v8_utils::v8_string(scope, key);
        global.set(scope, key_str.into(), global.into());
    }
}

/// Build a BehaviorCallbackRegistry with all 15 installers registered as
/// hardcoded wrappers around the install_X functions. This is the existing
/// default path since v0.8.29/30.
fn build_hardcoded_bcr() -> iv8_surface::BehaviorCallbackRegistry {
    let callbacks = iv8_surface::BehaviorCallbackRegistry::new();

    *callbacks.install_atob_btoa.borrow_mut() = Some(Box::new(|scope, global| {
        crate::shims::atob_btoa::install_atob_btoa(scope, global)
    }));
    *callbacks.install_fetch.borrow_mut() = Some(Box::new(|scope, global| {
        crate::network::fetch::install_fetch(scope, global)
    }));
    *callbacks.install_timers.borrow_mut() = Some(Box::new(|scope, global| {
        crate::events::timers::install_timer_globals(scope, global)
    }));
    *callbacks.install_console.borrow_mut() = Some(Box::new(|scope, global| {
        crate::shims::console::install_console(scope, global)
    }));
    *callbacks.install_location.borrow_mut() = Some(Box::new(|scope, global| {
        crate::shims::location::install_location(scope, global)
    }));
    *callbacks.install_event_loop.borrow_mut() = Some(Box::new(|scope, global| {
        crate::events::binding::install_event_loop_bindings(scope, global)
    }));
    *callbacks.install_page_api.borrow_mut() = Some(Box::new(|scope, global| {
        crate::events::page_api::install_page_api(scope, global)
    }));
    *callbacks.install_input_api.borrow_mut() = Some(Box::new(|scope, global| {
        crate::events::input_sim::install_input_api(scope, global)
    }));
    *callbacks.install_crypto_random.borrow_mut() = Some(Box::new(|scope, global| {
        crate::crypto::random::install_crypto_random(scope, global)
    }));
    *callbacks.install_subtle_crypto.borrow_mut() = Some(Box::new(|scope, global| {
        crate::crypto::subtle::install_subtle_crypto(scope, global)
    }));
    *callbacks.install_canvas_bindings.borrow_mut() = Some(Box::new(|scope, global| {
        crate::canvas::binding::install_canvas_bindings(scope, global)
    }));
    *callbacks.install_webgl_stubs.borrow_mut() = Some(Box::new(|scope, global| {
        crate::canvas::webgl::install_webgl_stubs(scope, global)
    }));
    *callbacks.install_xhr.borrow_mut() = Some(Box::new(|scope, global| {
        crate::network::xhr::install_xhr(scope, global)
    }));
    *callbacks.install_date_interceptor.borrow_mut() = Some(Box::new(|scope, global| {
        crate::events::date_interceptor::install_date_interceptor(scope, global)
    }));
    *callbacks.install_native_env.borrow_mut() = Some(Box::new(|scope, global| {
        crate::shims::native_env::install_native_env(scope, global)
    }));

    callbacks
}

/// Fill the 8 non-parameterized installer slots with hardcoded wrappers.
/// Called after `build_registry` which sets the 7 parameterized slots.
fn fill_hardcoded_remaining(callbacks: &mut iv8_surface::BehaviorCallbackRegistry) {
    *callbacks.install_event_loop.borrow_mut() = Some(Box::new(|scope, global| {
        crate::events::binding::install_event_loop_bindings(scope, global)
    }));
    *callbacks.install_page_api.borrow_mut() = Some(Box::new(|scope, global| {
        crate::events::page_api::install_page_api(scope, global)
    }));
    *callbacks.install_input_api.borrow_mut() = Some(Box::new(|scope, global| {
        crate::events::input_sim::install_input_api(scope, global)
    }));
    *callbacks.install_subtle_crypto.borrow_mut() = Some(Box::new(|scope, global| {
        crate::crypto::subtle::install_subtle_crypto(scope, global)
    }));
    *callbacks.install_fetch.borrow_mut() = Some(Box::new(|scope, global| {
        crate::network::fetch::install_fetch(scope, global)
    }));
    *callbacks.install_xhr.borrow_mut() = Some(Box::new(|scope, global| {
        crate::network::xhr::install_xhr(scope, global)
    }));
    *callbacks.install_atob_btoa.borrow_mut() = Some(Box::new(|scope, global| {
        crate::shims::atob_btoa::install_atob_btoa(scope, global)
    }));
    *callbacks.install_console.borrow_mut() = Some(Box::new(|scope, global| {
        crate::shims::console::install_console(scope, global)
    }));
}

/// No-op call-as-function handler for the undetectable __iv8__ tool object.
/// V8 requires this when MarkAsUndetectable is set on an ObjectTemplate.
unsafe extern "C" fn undetectable_noop_handler(_info: *const v8::FunctionCallbackInfo) {
    // Returns undefined implicitly (no rv.set call).
}

impl EmbeddedV8Kernel {
    /// Install anti-detection shims (__iv8__ tool object + wrapNative + hookNative + window.chrome).
    pub fn install_undetect_shims(&mut self, skip_native_behaviors: bool) {
        let js_api_name = {
            let state = crate::state::RuntimeState::get(&self.isolate);
            state.js_api_name.clone()
        };

        // 1. Create __iv8__ tool object with MarkAsUndetectable (DontEnum)
        //    This gives [[IsHTMLDDA]] semantics: typeof === 'undefined', == null, falsy
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);

            let templ = v8::ObjectTemplate::new(scope);
            crate::v8_extra::mark_as_undetectable(&templ);
            crate::v8_extra::set_call_as_function_handler(&templ, undetectable_noop_handler, None);
            let tool_obj = templ
                .new_instance(scope)
                .expect("failed to create undetectable __iv8__ instance");

            let key = crate::v8_utils::v8_string(scope, &js_api_name);
            global.define_own_property(
                scope,
                key.into(),
                tool_obj.into(),
                v8::PropertyAttribute::DONT_ENUM,
            );
        }
        unsafe {
            self.isolate.exit();
        }

        // 2. Install wrapNative shim
        let wrap_script = format!(
            "{}({})",
            include_str!("../../../iv8-undetect/src/shims/wrap_native.js"),
            js_api_name
        );
        self.eval(&wrap_script, crate::kernel::EvalOpts::default())
            .ok();

        // 3. Install hookNative shim
        let hook_script = format!(
            "{}({})",
            include_str!("../../../iv8-undetect/src/shims/hook_native.js"),
            js_api_name
        );
        self.eval(&hook_script, crate::kernel::EvalOpts::default())
            .ok();

        // 4. Install window.chrome shim
        let chrome_script = format!(
            "{}({}.wrapNative)",
            include_str!("../../../iv8-undetect/src/shims/window_chrome.js"),
            js_api_name
        );
        self.eval(&chrome_script, crate::kernel::EvalOpts::default())
            .ok();

        // 5. Install native behavior modules (skip when install_browser_surface_init handles them).
        // Tool-object APIs still need a post-creation pass because they attach under __iv8__.
        if skip_native_behaviors {
            unsafe {
                self.isolate.enter();
            }
            {
                v8::scope!(handle_scope, &mut self.isolate);
                let context = v8::Local::new(handle_scope, &self.context);
                v8::scope_with_context!(scope, handle_scope, context);
                let global = context.global(scope);
                crate::events::binding::install_event_loop_bindings(scope, global);
                crate::events::page_api::install_page_api(scope, global);
                crate::events::input_sim::install_input_api(scope, global);
                install_window_identity_refs(scope, global);
            }
            unsafe {
                self.isolate.exit();
            }
        } else {
            unsafe {
                self.isolate.enter();
            }
            {
                v8::scope!(handle_scope, &mut self.isolate);
                let context = v8::Local::new(handle_scope, &self.context);
                v8::scope_with_context!(scope, handle_scope, context);
                let global = context.global(scope);
                crate::events::binding::install_event_loop_bindings(scope, global);
                crate::events::timers::install_timer_globals(scope, global);
                crate::events::date_interceptor::install_date_interceptor(scope, global);
                crate::crypto::random::install_crypto_random(scope, global);
                crate::crypto::subtle::install_subtle_crypto(scope, global);
                crate::canvas::webgl::install_webgl_stubs(scope, global);
                crate::canvas::binding::install_canvas_bindings(scope, global);
                crate::network::fetch::install_fetch(scope, global);
                crate::network::xhr::install_xhr(scope, global);
                crate::shims::atob_btoa::install_atob_btoa(scope, global);
                crate::shims::location::install_location(scope, global);
                crate::shims::console::install_console(scope, global);
                crate::events::page_api::install_page_api(scope, global);
                crate::events::input_sim::install_input_api(scope, global);
            }
            unsafe {
                self.isolate.exit();
            }
        }

        // 6. Install Date constructor shim (JS-level, needs __iv8_now__ to be ready)
        self.eval(
            crate::events::date_interceptor::DATE_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 7. Install WebGL context shim
        self.eval(
            crate::canvas::webgl::WEBGL_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 8. Install XMLHttpRequest class shim
        // Skip when skip_native_behaviors=true: install_xhr (called via BCR in
        // install_browser_surface_init) already eval'd XHR_SHIM_JS. Re-eval
        // would overwrite the constructor and its prototype chain.
        if !skip_native_behaviors {
            self.eval(
                crate::network::xhr::XHR_SHIM_JS,
                crate::kernel::EvalOpts::default(),
            )
            .ok();
        }

        // 9. Install TextEncoder/TextDecoder polyfill
        self.eval(TEXT_ENCODER_SHIM, crate::kernel::EvalOpts::default())
            .ok();

        // 10. Install Event/CustomEvent/MouseEvent/KeyboardEvent/PointerEvent constructors
        self.eval(
            crate::shims::event_constructors::EVENT_CONSTRUCTORS_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 11. Install getBoundingClientRect + getComputedStyle + DOMRect
        //     First inject media preferences from profile env map.
        self.inject_media_prefs();
        self.eval(
            crate::shims::geometry::GEOMETRY_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 12. Install URL + URLSearchParams
        self.eval(
            crate::shims::url::URL_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 12a. Install CSSOM prototype setup (methods/getters on codegen prototypes)
        self.eval(
            crate::shims::cssom::CSSOM_PROTO_SETUP_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 12b. Install CSSOM (CSS parser + CSSStyleSheet/CSSRule population)
        self.eval(
            crate::shims::cssom::CSSOM_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 12b2. Install CSS namespace (CSS.supports, CSS.escape, CSS.cssFloat)
        // Separate from CSSOM_PROTO_SETUP_JS because V8 FunctionTemplate
        // prototypes are non-extensible, causing the IIFE to throw before
        // reaching the CSS namespace section.
        self.eval(
            crate::shims::cssom::CSS_NAMESPACE_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 12b3. Install NamedNodeMap (element.attributes)
        // Separate for same reason as CSS namespace.
        self.eval(
            crate::shims::cssom::NAMED_NODE_MAP_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 12c. Install WindowProperties interface (manual; not in webref IDL)
        self.eval(
            crate::shims::window_properties::WINDOW_PROPERTIES_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 13. Install MessageChannel
        self.eval(
            crate::shims::message_channel::MESSAGE_CHANNEL_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 14. Install localStorage/sessionStorage
        // Seed from shared backend if present, then install JS shim.
        {
            let state = crate::state::RuntimeState::get(&self.isolate);
            let seed_json = {
                let borrow = state.local_storage.borrow();
                borrow
                    .as_ref()
                    .map(|b| b.to_json_object())
                    .unwrap_or_default()
            };
            if !seed_json.is_empty() && seed_json != "{}" {
                let seed_js =
                    format!("window.__iv8LocalSeed = {};", seed_json);
                self.eval(&seed_js, crate::kernel::EvalOpts::default())
                    .ok();
            }
        }
        self.eval(
            crate::shims::storage::STORAGE_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 15. Install navigator.mimeTypes/plugins/connection + history
        self.eval(
            crate::shims::navigator_extras::NAVIGATOR_EXTRAS_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 16 removed — tier1_stubs.js archived in v0.8.27.
        // 1284 IDL templates from install_browser_surface_init provide all constructors.
        // 16b removed — browser_apis.js archived in v0.8.27.
        // 1284 IDL templates + navigator_extras.js cover all API existence stubs.

        // 17. Install timezone shim (override Intl.DateTimeFormat default timezone)
        {
            let tz = {
                let state = crate::state::RuntimeState::get(&self.isolate);
                state
                    .environment
                    .get_str("timezone")
                    .unwrap_or("UTC")
                    .to_string()
            };
            let tz_shim = format!(
                r#"
(function() {{
    var _tz = '{}';
    if (typeof Intl !== 'undefined' && Intl.DateTimeFormat) {{
        var _origDTF = Intl.DateTimeFormat;
        var _origProto = _origDTF.prototype;
        var _origResolvedOptions = _origProto.resolvedOptions;
        var _tz_val = _tz;
        // Override resolvedOptions to inject timezone
        _origProto.resolvedOptions = function() {{
            var opts = _origResolvedOptions.call(this);
            if (!opts.timeZone) opts.timeZone = _tz_val;
            return opts;
        }};
        // Wrap constructor to inject default timezone
        // Guard against re-entrancy: use a flag on the original constructor
        // to prevent re-wrapping if this shim is evaluated multiple times.
        if (_origDTF.__iv8_tz_wrapped) {{
            return;
        }}
        var _wrappedDTF = function(locales, options) {{
            if (!options) options = {{}};
            if (!options.timeZone) options.timeZone = _tz_val;
            if (this instanceof _wrappedDTF) {{
                return new _origDTF(locales, options);
            }}
            return _origDTF(locales, options);
        }};
        _wrappedDTF.prototype = _origProto;
        _wrappedDTF.supportedLocalesOf = _origDTF.supportedLocalesOf;
        try {{
            _origDTF.__iv8_tz_wrapped = true;
            Intl.DateTimeFormat = _wrappedDTF;
        }} catch(e) {{}}
    }}
}})();
"#,
                tz
            );
            self.eval(&tz_shim, crate::kernel::EvalOpts::default()).ok();
        }

        // 18-20. Document + AudioContext + WindowExtras (Window-only, skip in worker mode)
        if !self.worker_mode {
            // 18. Install default empty document so document.* methods are always available
            self.set_document(
                "<!DOCTYPE html><html><head></head><body></body></html>",
                None,
            );

            // 19. Install document properties (cookie, referrer, hidden, visibilityState, DOM methods)
            self.inject_font_prefs();
            self.eval(
                crate::shims::document_props::DOCUMENT_PROPS_JS,
                crate::kernel::EvalOpts::default(),
            )
            .ok();

            // 19b. Install AudioContext subsystem (extracted from document_props.rs)
            self.inject_audio_prefs();
            self.eval(
                crate::shims::audio_context::AUDIO_CONTEXT_JS,
                crate::kernel::EvalOpts::default(),
            )
            .ok();
        }

        // 19c. Install window properties, global constructors, structuredClone, Blob
        self.inject_feature_flag_prefs();
        self.eval(
            crate::shims::window_extras::WINDOW_EXTRAS_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 20. Install Canvas2D shim (after document.createElement is available)
        self.eval(
            crate::canvas::binding::CANVAS2D_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();
    }

    /// Install DOM FunctionTemplate hierarchy into the isolate.
    /// Called once after kernel creation.
    #[deprecated(
        since = "0.8.31",
        note = "DOM templates are now installed by install_browser_surface_init() via install_dom_constructors(). This function is retained for archival reference only."
    )]
    pub fn install_dom_templates(&mut self) {
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);

            // Build templates
            let templates = crate::dom::template::build_dom_templates(scope);

            // Install constructor functions on global
            crate::dom::template::install_dom_constructors(scope, global, &templates, false);

            // Store in RuntimeState
            let state = crate::state::RuntimeState::get(&*scope);
            *state.dom_templates.borrow_mut() = Some(templates);
        }
        unsafe {
            self.isolate.exit();
        }
    }

    /// Install environment fields into the V8 global object.
    /// Called once after kernel creation to populate navigator.*, screen.*, etc.
    /// Phase 1: static value injection (all 393 entries via env_inject)
    /// Must run BEFORE install_browser_surface_init so that native getter
    /// override (Phase 2) can use the codegen EventTarget template.
    pub fn install_environment(&mut self) {
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);
            // Phase 1: static injection (all 393 dot-path entries)
            crate::env_inject::install_environment(scope, global);
        }
        unsafe {
            self.isolate.exit();
        }
    }

    /// Inject media preferences from the environment map into the JS context
    /// as `globalThis.__iv8MediaPrefs`. The matchMedia shim reads these values
    /// instead of using hardcoded defaults.
    fn inject_media_prefs(&mut self) {
        let state = RuntimeState::get(&self.isolate);
        let media_fields = [
            "prefers-color-scheme",
            "prefers-contrast",
            "prefers-reduced-motion",
            "prefers-reduced-data",
            "forced-colors",
            "color-gamut",
            "dynamic-range",
            "scripting",
            "update",
            "pointer",
            "hover",
            "any-pointer",
            "any-hover",
            "display-mode",
            "inverted-colors",
            "prefers-reduced-transparency",
        ];
        let mut json_obj = serde_json::Map::new();
        for field in &media_fields {
            let key = format!("media.{}", field);
            let val = state
                .environment
                .get_str(&key)
                .unwrap_or_else(|| match *field {
                    "prefers-color-scheme" => "light",
                    "prefers-contrast" => "no-preference",
                    "prefers-reduced-motion" => "no-preference",
                    "prefers-reduced-data" => "no-preference",
                    "forced-colors" => "none",
                    "color-gamut" => "srgb",
                    "dynamic-range" => "srgb",
                    "scripting" => "enabled",
                    "update" => "fast",
                    "pointer" => "fine",
                    "hover" => "hover",
                    "any-pointer" => "coarse",
                    "any-hover" => "none",
                    "display-mode" => "browser",
                    "inverted-colors" => "none",
                    "prefers-reduced-transparency" => "no-preference",
                    _ => "none",
                });
            json_obj.insert(field.to_string(), serde_json::Value::String(val.into()));
        }
        let json_str = serde_json::to_string(&serde_json::Value::Object(json_obj))
            .unwrap_or_else(|_| "{}".into());
        let js = format!("globalThis.__iv8MediaPrefs = {};", json_str);
        self.eval(&js, crate::kernel::EvalOpts::default()).ok();
    }

    /// Inject audio preferences from the environment map into the JS context
    /// as `globalThis.__iv8AudioPrefs`. The AudioContext shim reads these
    /// values for baseLatency/outputLatency, sampleRate, compressor settings,
    /// and channelData fingerprint seed.
    fn inject_audio_prefs(&mut self) {
        let state = RuntimeState::get(&self.isolate);
        let env = &state.environment;
        let base_latency = env.get_f64("audio.baseLatency").unwrap_or(0.05);
        let output_latency = env.get_f64("audio.outputLatency").unwrap_or(0.0);
        let channel_data_seed = env.get_f64("audio.channelDataSeed").unwrap_or(0.0);
        let sample_rate = env.get_f64("audio.sampleRate").unwrap_or(48000.0);
        let comp_threshold = env.get_f64("audio.compressor.threshold").unwrap_or(-24.0);
        let comp_knee = env.get_f64("audio.compressor.knee").unwrap_or(30.0);
        let comp_ratio = env.get_f64("audio.compressor.ratio").unwrap_or(12.0);
        let comp_attack = env.get_f64("audio.compressor.attack").unwrap_or(0.003);
        let comp_release = env.get_f64("audio.compressor.release").unwrap_or(0.25);
        let comp_reduction = env.get_f64("audio.compressor.reduction").unwrap_or(0.0);

        // channelData may be an array (of floats) or a string (JSON/base64).
        // Pass it through as raw JSON so the JS shim can interpret it.
        let channel_data_json = if let Some(cd) = env.get("audio.channelData") {
            serde_json::to_string(cd).unwrap_or_else(|_| "null".into())
        } else {
            "null".to_string()
        };

        let js = format!(
            "globalThis.__iv8AudioPrefs = {{ baseLatency: {}, outputLatency: {}, channelDataSeed: {}, sampleRate: {}, channelData: {}, compressor: {{ threshold: {}, knee: {}, ratio: {}, attack: {}, release: {}, reduction: {} }} }};",
            base_latency, output_latency, channel_data_seed as i64,
            sample_rate,
            channel_data_json,
            comp_threshold, comp_knee, comp_ratio, comp_attack, comp_release, comp_reduction
        );
        self.eval(&js, crate::kernel::EvalOpts::default()).ok();
    }

    /// Inject font preferences from the environment map into the JS context
    /// as `globalThis.__iv8FontPrefs`. The measureText shim and document.fonts
    /// FontFaceSet read these values for font-aware behavior.
    fn inject_font_prefs(&mut self) {
        let state = RuntimeState::get(&self.isolate);
        let fonts_mode = state
            .environment
            .get_str("fonts.mode")
            .unwrap_or("common")
            .to_string();
        let families_json = state
            .environment
            .get("fonts.families")
            .and_then(|v| v.as_array())
            .map(|arr| {
                let names: Vec<String> = arr
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                serde_json::to_string(&names).unwrap_or_else(|_| "[]".into())
            })
            .unwrap_or_else(|| {
                // Default common Windows fonts
                let default_fonts: Vec<&str> = vec![
                    "Arial", "Arial Black", "Arial Narrow", "Calibri", "Cambria",
                    "Cambria Math", "Comic Sans MS", "Consolas", "Courier New",
                    "Ebrima", "Franklin Gothic Medium", "Gabriola", "Gadugi",
                    "Georgia", "Impact", "Javanese Text", "Leelawadee UI",
                    "Lucida Console", "Lucida Sans Unicode", "Malgun Gothic",
                    "MV Boli", "Microsoft Sans Serif", "MingLiU-ExtB",
                    "Mongolian Baiti", "MS Gothic", "Nirmala UI",
                    "Palatino Linotype", "Segoe MDL2 Assets", "Segoe Print",
                    "Segoe Script", "Segoe UI", "Segoe UI Emoji",
                    "Segoe UI Historic", "Segoe UI Symbol", "SimSun",
                    "Sitka Small", "Sylfaen", "Tahoma", "Times New Roman",
                    "Trebuchet MS", "Verdana", "Webdings", "Wingdings",
                ];
                serde_json::to_string(&default_fonts).unwrap_or_else(|_| "[]".into())
            });
        let js = format!(
            "globalThis.__iv8FontPrefs = {{ mode: '{}', families: {} }};",
            fonts_mode, families_json
        );
        self.eval(&js, crate::kernel::EvalOpts::default()).ok();
    }

    /// Inject feature flag overrides from the environment map.
    /// The window_extras.rs shim merges these over the hardcoded defaults.
    fn inject_feature_flag_prefs(&mut self) {
        let state = RuntimeState::get(&self.isolate);
        // Check for feature flags override in env map
        // Format: "featureflags.FencedFrames" = "true"/"false"
        let mut overrides = Vec::new();
        for (key, value) in state.environment.iter() {
            if key.starts_with("featureflags.") {
                let flag_name = &key["featureflags.".len()..];
                let flag_val = match value {
                    serde_json::Value::Bool(b) => *b,
                    serde_json::Value::String(s) => s == "true",
                    _ => continue,
                };
                overrides.push((flag_name.to_string(), flag_val));
            }
        }
        if !overrides.is_empty() {
            let pairs: Vec<String> = overrides
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect();
            let js = format!(
                "globalThis.__iv8FeatureFlagPrefs = {{ {} }};",
                pairs.join(", ")
            );
            self.eval(&js, crate::kernel::EvalOpts::default()).ok();
        }
    }

    /// Install native environment objects (navigator, screen, location)
    /// with native-getter FunctionTemplates. Must run AFTER
    /// install_browser_surface_init so that codegen EventTarget and other
    /// parent templates are available for inheritance.
    pub fn install_native_environment(&mut self) {
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);
            crate::shims::native_env::install_native_env(scope, global);
        }
        unsafe {
            self.isolate.exit();
        }
        self.fix_prototype_chains();
    }

    /// Fix prototype chains after install_all has registered codegen
    /// FunctionTemplates as globals. native_env and location.rs create
    /// their own FunctionTemplates before install_all runs, resulting in
    /// two different prototype objects per interface. This function uses
    /// V8's Object::set_prototype to link the native_env prototypes to
    /// the codegen prototypes, so instanceof checks work correctly.
    fn fix_prototype_chains(&mut self) {
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);

            // Helper: get global constructor's .prototype object
            fn get_proto<'s>(scope: &v8::PinScope<'s, '_>, global: v8::Local<'s, v8::Object>, name: &str) -> Option<v8::Local<'s, v8::Object>> {
                let key = crate::v8_utils::v8_string(scope, name);
                let ctor_val = global.get(scope, key.into())?;
                if !ctor_val.is_function() { return None; }
                let ctor: v8::Local<'s, v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };
                let proto_key = crate::v8_utils::v8_string(scope, "prototype");
                let proto_val = ctor.get(scope, proto_key.into())?;
                if !proto_val.is_object() { return None; }
                let proto_obj: v8::Local<'s, v8::Object> = unsafe { v8::Local::cast_unchecked(proto_val) };
                Some(proto_obj)
            }

            // Navigator.prototype.__proto__ = EventTarget.prototype
            if let (Some(nav_proto), Some(et_proto)) = (get_proto(&scope, global, "Navigator"), get_proto(&scope, global, "EventTarget")) {
                let _ = nav_proto.set_prototype(&*scope, et_proto.into());
            }
            // Storage.prototype.__proto__ = EventTarget.prototype
            if let (Some(st_proto), Some(et_proto)) = (get_proto(&scope, global, "Storage"), get_proto(&scope, global, "EventTarget")) {
                let _ = st_proto.set_prototype(&*scope, et_proto.into());
            }
            // XMLHttpRequestEventTarget.prototype.__proto__ = EventTarget.prototype
            if let (Some(xhr_et_proto), Some(et_proto)) = (get_proto(&scope, global, "XMLHttpRequestEventTarget"), get_proto(&scope, global, "EventTarget")) {
                let _ = xhr_et_proto.set_prototype(&*scope, et_proto.into());
            }
            // XMLHttpRequest.prototype.__proto__ = XMLHttpRequestEventTarget.prototype
            if let (Some(xhr_proto), Some(xhr_et_proto)) = (get_proto(&scope, global, "XMLHttpRequest"), get_proto(&scope, global, "XMLHttpRequestEventTarget")) {
                let _ = xhr_proto.set_prototype(&*scope, xhr_et_proto.into());
            }
            // WorkerNavigator.prototype.__proto__ = EventTarget.prototype
            if let (Some(wn_proto), Some(et_proto)) = (get_proto(&scope, global, "WorkerNavigator"), get_proto(&scope, global, "EventTarget")) {
                let _ = wn_proto.set_prototype(&*scope, et_proto.into());
            }
            // Window.prototype.__proto__ = EventTarget.prototype
            if let (Some(win_proto), Some(et_proto)) = (get_proto(&scope, global, "Window"), get_proto(&scope, global, "EventTarget")) {
                let _ = win_proto.set_prototype(&*scope, et_proto.into());
            }
            // globalThis.__proto__ = Window.prototype (so window instanceof Window)
            if let Some(win_proto) = get_proto(&scope, global, "Window") {
                let _ = global.set_prototype(&*scope, win_proto.into());
            }
        }
        unsafe {
            self.isolate.exit();
        }
    }

    /// Dispose the kernel (explicit cleanup before drop).
    pub fn dispose(&mut self) {
        self.flush_local_storage();
        RuntimeState::get(&self.isolate).mark_disposed();
    }

    /// Flush localStorage JS data back to the shared backend.
    /// Idempotent: safe across double-dispose.
    fn flush_local_storage(&mut self) {
        let backend = {
            let state = RuntimeState::get(&self.isolate);
            if state.is_disposed() {
                return;
            }
            state.local_storage.borrow().clone()
        };
        if let Some(backend) = backend {
            let result =
                self.eval_to_rust_value("window.__iv8DumpLocalStorage()");
            if let crate::convert::RustValue::String(json) = result {
                if let Ok(map) = serde_json::from_str::<
                    std::collections::HashMap<String, String>,
                >(&json)
                {
                    backend.replace_all(map);
                }
            }
        }
    }

    pub fn persist_storage_to_file(
        &mut self,
        path: &std::path::Path,
    ) -> Result<(), IV8Error> {
        let state = RuntimeState::get(&self.isolate);
        let backend = state.local_storage.borrow().clone();
        if let Some(backend) = backend {
            let result =
                self.eval_to_rust_value("window.__iv8DumpLocalStorage()");
            if let crate::convert::RustValue::String(json) = result {
                if let Ok(map) = serde_json::from_str::<
                    std::collections::HashMap<String, String>,
                >(&json)
                {
                    backend.replace_all(map);
                }
            }
            backend
                .save_to_file(path)
                .map_err(|e| IV8Error::Internal(format!("persist_storage: {}", e)))
        } else {
            let store = crate::dom::local_storage::LocalStorageStore::new();
            let result =
                self.eval_to_rust_value("window.__iv8DumpLocalStorage()");
            if let crate::convert::RustValue::String(json) = result {
                if let Ok(map) = serde_json::from_str::<
                    std::collections::HashMap<String, String>,
                >(&json)
                {
                    store.replace_all(map);
                }
            }
            store
                .save_to_file(path)
                .map_err(|e| IV8Error::Internal(format!("persist_storage: {}", e)))
        }
    }

    pub fn load_storage_from_file(
        &mut self,
        path: &std::path::Path,
    ) -> Result<(), IV8Error> {
        let state = RuntimeState::get(&self.isolate);
        let backend = {
            let borrow = state.local_storage.borrow();
            borrow.clone()
        };
        let store = match backend {
            Some(existing) => {
                existing.load_from_file(path).map_err(|e| {
                    IV8Error::Internal(format!("load_storage: {}", e))
                })?;
                existing
            }
            None => {
                let new_store = crate::dom::local_storage::LocalStorageStore::new();
                new_store.load_from_file(path).map_err(|e| {
                    IV8Error::Internal(format!("load_storage: {}", e))
                })?;
                *state.local_storage.borrow_mut() = Some(new_store.clone());
                new_store
            }
        };
        let seed_json = store.to_json_object();
        if !seed_json.is_empty() && seed_json != "{}" {
            let seed_js = format!("window.__iv8LocalSeed = {};", seed_json);
            self.eval(&seed_js, crate::kernel::EvalOpts::default())
                .ok();
            self.eval(
                r#"(function() {
                    if (typeof localStorage !== 'undefined' && window.__iv8LocalSeed) {
                        var data = window.__iv8LocalSeed;
                        localStorage._data = data;
                        localStorage.length = Object.keys(data).length;
                        try { delete window.__iv8LocalSeed; } catch(e) {}
                    }
                })();"#,
                crate::kernel::EvalOpts::default(),
            )
            .ok();
        }
        Ok(())
    }

    /// Install BrowserSurface — default init path since v0.8.26.
    /// 1284 IDL templates + 15 native behaviors + 38 DomTemplate constructors.
    ///
    /// Delegates to the core install path with hardcoded BCR closures.
    pub fn install_browser_surface_init(&mut self, worker_mode: bool) {
        let callbacks = build_hardcoded_bcr();
        self.install_browser_surface_with_callbacks(callbacks, worker_mode);
    }

    /// Install BrowserSurface from a profile-derived BehaviorConfig.
    ///
    /// Builds a BCR from the config, registers the remaining 8 installers
    /// with hardcoded wrappers, and dispatches all 15 via BCR.
    pub fn install_browser_surface_with_config(&mut self, config: Arc<BehaviorConfig>) {
        let mut callbacks = crate::bcr_builder::build_registry(config);
        fill_hardcoded_remaining(&mut callbacks);
        self.install_browser_surface_with_callbacks(callbacks, false);
    }

    /// Core install: DomTemplates -> install_browser_surface ->
    /// install_dom_constructors -> BCR dispatch -> store in state.
    fn install_browser_surface_with_callbacks(
        &mut self,
        callbacks: iv8_surface::BehaviorCallbackRegistry,
        worker_mode: bool,
    ) {
        unsafe {
            self.isolate.enter();
        }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);

            match iv8_surface::install_browser_surface(scope, global, &callbacks, worker_mode) {
                Ok(registry) => {
                    let state = RuntimeState::get(&*scope);
                    let codegen_protos =
                        crate::dom::template::capture_codegen_prototypes(scope, global);
                    crate::telemetry::init_codegen_prototypes_captured(codegen_protos.len());
                    let dom_templates = crate::dom::template::build_dom_templates(scope);
                    crate::telemetry::init_dom_templates_built();
                    crate::dom::template::install_dom_constructors(scope, global, &dom_templates, worker_mode);
                    crate::telemetry::init_dom_constructors_installed();
                    crate::dom::template::chain_dom_prototypes(scope, global, &codegen_protos);

                    iv8_surface::generated::install_all::install_named_constructors(scope, global);

                    // Install __iv8OpCallbacks (test: only install, no JS fix)
                    iv8_surface::generated::install_all::install_op_callbacks(scope, global);

                    // Event system
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_event_loop,
                        crate::events::binding::install_event_loop_bindings,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_timers,
                        crate::events::timers::install_timer_globals,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_page_api,
                        crate::events::page_api::install_page_api,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_input_api,
                        crate::events::input_sim::install_input_api,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_date_interceptor,
                        crate::events::date_interceptor::install_date_interceptor,
                    );
                    // Crypto
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_crypto_random,
                        crate::crypto::random::install_crypto_random,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_subtle_crypto,
                        crate::crypto::subtle::install_subtle_crypto,
                    );
                    // Canvas + WebGL
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_canvas_bindings,
                        crate::canvas::binding::install_canvas_bindings,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_webgl_stubs,
                        crate::canvas::webgl::install_webgl_stubs,
                    );
                    // Network
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_fetch,
                        crate::network::fetch::install_fetch,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_xhr,
                        crate::network::xhr::install_xhr,
                    );
                    // Shims
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_atob_btoa,
                        crate::shims::atob_btoa::install_atob_btoa,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_location,
                        crate::shims::location::install_location,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_console,
                        crate::shims::console::install_console,
                    );
                    install_behavior_via_bcr(
                        scope,
                        global,
                        &callbacks,
                        &callbacks.install_native_env,
                        crate::shims::native_env::install_native_env,
                    );

                    // Freeze shim constructor prototypes (non-writable, non-configurable)
                    // idlharness checks that X.prototype is not writable and
                    // Object.setPrototypeOf(X.prototype, {}) throws TypeError.
                    // Codegen interfaces already use read_only_prototype(), but
                    // JS shim constructors (Event, MessageChannel, etc.) do not.
                    let freeze_js = crate::v8_utils::v8_string(scope, r#"
                        (function() {
                            var names = ['Event','CustomEvent','MouseEvent','KeyboardEvent','PointerEvent',
                                'MessageChannel','MessagePort','BroadcastChannel','Worker',
                                'Location','Navigator','Screen','DOMRect','DOMException',
                                'AudioContext','OfflineAudioContext','AudioBuffer','AudioNode','AudioParam'];
                            for (var i = 0; i < names.length; i++) {
                                var name = names[i];
                                try {
                                    var ctor = globalThis[name];
                                    if (ctor && typeof ctor === 'function') {
                                        Object.defineProperty(ctor, 'prototype', {writable: false, enumerable: false, configurable: false});
                                    }
                                } catch(e) {}
                            }
                        })();
                    "#);
                    let _ = v8::Script::compile(scope, freeze_js, None).and_then(|s| s.run(scope));

                    // Fix all getter .name properties: codegen uses set_class_name which
                    // doesn't set Function.name. Iterate all prototypes and set
                    // getter.name = "get " + attrName for accessor getters.
                    // Skip [native code] getters (V8 FunctionTemplate internals are not configurable).
                    let getter_name_fix = crate::v8_utils::v8_string(scope, r#"
                        (function() {
                            var ctors = Object.getOwnPropertyNames(globalThis);
                            for (var i = 0; i < ctors.length; i++) {
                                try {
                                    var c = globalThis[ctors[i]];
                                    if (!c || !c.prototype) continue;
                                    var proto = c.prototype;
                                    var names = Object.getOwnPropertyNames(proto);
                                    for (var j = 0; j < names.length; j++) {
                                        var pn = names[j];
                                        if (pn === 'constructor') continue;
                                        try {
                                            var desc = Object.getOwnPropertyDescriptor(proto, pn);
                                            if (!desc || !desc.get) continue;
                                            var g = desc.get;
                                            if (typeof g !== 'function') continue;
                                            var gStr = '';
                                            try { gStr = g.toString(); } catch(e) {}
                                            if (gStr.indexOf('[native code]') !== -1) continue;
                                            if (g.name !== 'get ' + pn) {
                                                try { Object.defineProperty(g, 'name', {
                                                    value: 'get ' + pn, writable: false,
                                                    enumerable: false, configurable: true
                                                }); } catch(e) {}
                                            }
                                            if (g.length !== 0) {
                                                try { Object.defineProperty(g, 'length', {
                                                    value: 0, writable: false,
                                                    enumerable: false, configurable: true
                                                }); } catch(e) {}
                                            }
                                            if (desc.set && typeof desc.set === 'function') {
                                                var sStr = '';
                                                try { sStr = desc.set.toString(); } catch(e) {}
                                                if (sStr.indexOf('[native code]') === -1) {
                                                    var s = desc.set;
                                                    if (s.name !== 'set ' + pn) {
                                                        try { Object.defineProperty(s, 'name', {
                                                            value: 'set ' + pn, writable: false,
                                                            enumerable: false, configurable: true
                                                        }); } catch(e) {}
                                                    }
                                                }
                                            }
                                        } catch(e) {}
                                    }
                                } catch(e) {}
                            }
                        })();
                    "#);
                    let _ = v8::Script::compile(scope, getter_name_fix, None).and_then(|s| s.run(scope));

                    // CDP diff fix: window.chrome should have runtime:{} and not
                    // expose app/csi/loadTimes (IV8 internal leak).
                    // Note: document.all [[IsHTMLDDA]] cannot be fixed from JS
                    // (see document_props.rs:1403 comment).
                    let chrome_fix = crate::v8_utils::v8_string(scope, r#"
                        (function() {
                            try {
                                if (typeof window.chrome === 'object' && window.chrome) {
                                    if (!window.chrome.runtime) {
                                        try { Object.defineProperty(window.chrome, 'runtime', {
                                            value: {}, writable: true, enumerable: true, configurable: true
                                        }); } catch(e) {}
                                    }
                                }
                            } catch(e) {}
                        })();
                    "#);
                    let _ = v8::Script::compile(scope, chrome_fix, None).and_then(|s| s.run(scope));

                    // R10-4: Fix instanceof for returned objects.
                    // customElements/navigation need correct prototype;
                    // contentWindow needs Window prototype;
                    // childNodes/children need NodeList/HTMLCollection prototype.
                    let instanceof_fix = crate::v8_utils::v8_string(scope, r#"
                        (function() {
                            // customElements: wrap with CustomElementRegistry prototype
                            try {
                                if (typeof CustomElementRegistry !== 'undefined' && typeof customElements !== 'undefined') {
                                    if (!(customElements instanceof CustomElementRegistry)) {
                                        var origCE = customElements;
                                        var ce = Object.create(CustomElementRegistry.prototype);
                                        for (var k in origCE) { ce[k] = origCE[k]; }
                                        try { Object.defineProperty(globalThis, 'customElements', { value: ce, writable: true, configurable: true, enumerable: true }); } catch(e) {}
                                    }
                                }
                            } catch(e) {}

                            // navigation: wrap with Navigation prototype
                            try {
                                if (typeof Navigation !== 'undefined' && typeof navigation !== 'undefined') {
                                    if (!(navigation instanceof Navigation)) {
                                        var origNav = navigation;
                                        var nav = Object.create(Navigation.prototype);
                                        for (var k in origNav) { nav[k] = origNav[k]; }
                                        try { Object.defineProperty(globalThis, 'navigation', { value: nav, writable: true, configurable: true, enumerable: true }); } catch(e) {}
                                    }
                                }
                            } catch(e) {}

                            // childNodes: wrap return value with NodeList prototype
                            try {
                                if (typeof NodeList !== 'undefined' && typeof Node !== 'undefined' && Node.prototype) {
                                    var origCN = Object.getOwnPropertyDescriptor(Node.prototype, 'childNodes');
                                    if (origCN && origCN.get) {
                                        var origGet = origCN.get;
                                        Object.defineProperty(Node.prototype, 'childNodes', {
                                            get: function() {
                                                var cn = origGet.call(this);
                                                if (cn && !(cn instanceof NodeList) && Array.isArray(cn)) {
                                                    var nl = Object.create(NodeList.prototype);
                                                    for (var i = 0; i < cn.length; i++) { nl[i] = cn[i]; }
                                                    nl.length = cn.length;
                                                    nl.item = function(i) { return cn[i] || null; };
                                                    return nl;
                                                }
                                                return cn;
                                            },
                                            enumerable: true, configurable: true
                                        });
                                    }
                                }
                            } catch(e) {}

                            // children: wrap return value with HTMLCollection prototype
                            try {
                                if (typeof HTMLCollection !== 'undefined' && typeof Element !== 'undefined' && Element.prototype) {
                                    var origCh = Object.getOwnPropertyDescriptor(Element.prototype, 'children');
                                    if (origCh && origCh.get) {
                                        var origGet = origCh.get;
                                        Object.defineProperty(Element.prototype, 'children', {
                                            get: function() {
                                                var ch = origGet.call(this);
                                                if (ch && !(ch instanceof HTMLCollection) && Array.isArray(ch)) {
                                                    var hc = Object.create(HTMLCollection.prototype);
                                                    for (var i = 0; i < ch.length; i++) { hc[i] = ch[i]; }
                                                    hc.length = ch.length;
                                                    hc.item = function(i) { return ch[i] || null; };
                                                    hc.namedItem = function(n) { return ch[n] || null; };
                                                    return hc;
                                                }
                                                return ch;
                                            },
                                            enumerable: true, configurable: true
                                        });
                                    }
                                }
                            } catch(e) {}
                        })();
                    "#);
                    let _ = v8::Script::compile(scope, instanceof_fix, None).and_then(|s| s.run(scope));

                    // R10-5: Fix descriptor issues.
                    // LegacyUnforgeable: configurable=false for window/document/location/top
                    // Event.isTrusted: should be accessor not data property
                    // stringifier enumerable=true
                    // Worker interface objects: enumerable=false
                    let descriptor_fix = crate::v8_utils::v8_string(scope, r#"
                        (function() {
                            // LegacyUnforgeable: Window.window/document/location/top configurable=false
                            var unforgeable = ['window', 'document', 'top'];
                            for (var i = 0; i < unforgeable.length; i++) {
                                try {
                                    var desc = Object.getOwnPropertyDescriptor(globalThis, unforgeable[i]);
                                    if (desc && desc.configurable) {
                                        var newDesc = { configurable: false };
                                        if (desc.get) { newDesc.get = desc.get; newDesc.set = desc.set; newDesc.enumerable = desc.enumerable !== false; }
                                        else { newDesc.value = desc.value; newDesc.writable = desc.writable; newDesc.enumerable = desc.enumerable !== false; }
                                        try { Object.defineProperty(globalThis, unforgeable[i], newDesc); } catch(e) {}
                                    }
                                } catch(e) {}
                            }

                            // Window.frames: enumerable=true
                            try {
                                var fd = Object.getOwnPropertyDescriptor(globalThis, 'frames');
                                if (fd && fd.enumerable === false) {
                                    try { Object.defineProperty(globalThis, 'frames', { enumerable: true, configurable: true }); } catch(e) {}
                                }
                            } catch(e) {}

                            // Event.isTrusted: convert from data property to accessor
                            try {
                                if (typeof Event !== 'undefined' && Event.prototype) {
                                    var itd = Object.getOwnPropertyDescriptor(Event.prototype, 'isTrusted');
                                    if (itd && 'value' in itd) {
                                        var val = itd.value;
                                        Object.defineProperty(Event.prototype, 'isTrusted', {
                                            get: function() { return val; },
                                            set: undefined,
                                            enumerable: true, configurable: true
                                        });
                                    }
                                }
                            } catch(e) {}

                            // Location.href/search: convert from data to accessor
                            try {
                                if (typeof Location !== 'undefined' && Location.prototype) {
                                    var locAttrs = ['href', 'search'];
                                    for (var j = 0; j < locAttrs.length; j++) {
                                        var ld = Object.getOwnPropertyDescriptor(Location.prototype, locAttrs[j]);
                                        if (ld && 'value' in ld) {
                                            (function(attr, desc) {
                                                var v = desc.value;
                                                Object.defineProperty(Location.prototype, attr, {
                                                    get: function() { return v; },
                                                    set: undefined,
                                                    enumerable: desc.enumerable !== false, configurable: true
                                                });
                                            })(locAttrs[j], ld);
                                        }
                                    }
                                }
                            } catch(e) {}

                            // Worker interface objects: enumerable=false
                            // Skip — run_wpt.py worker_shim installs these with enumerable=true,
                            // and changing enumerable may conflict with worker context init.
                            // TODO: move this fix to worker-specific path
                            /*
                            var workerIfaces = ['WorkerGlobalScope', 'DedicatedWorkerGlobalScope', 'WorkerNavigator', 'WorkerLocation'];
                            for (var k = 0; k < workerIfaces.length; k++) {
                                try {
                                    var wd = Object.getOwnPropertyDescriptor(globalThis, workerIfaces[k]);
                                    if (wd && wd.enumerable) {
                                        try { Object.defineProperty(globalThis, workerIfaces[k], { enumerable: false, configurable: true }); } catch(e) {}
                                    }
                                } catch(e) {}
                            }
                            */

                            // HTMLAnchorElement/HTMLAreaElement stringifier: enumerable=true
                            try {
                                if (typeof HTMLAnchorElement !== 'undefined' && HTMLAnchorElement.prototype) {
                                    var sd = Object.getOwnPropertyDescriptor(HTMLAnchorElement.prototype, 'toString');
                                    if (sd && sd.enumerable === false) {
                                        try { Object.defineProperty(HTMLAnchorElement.prototype, 'toString', { enumerable: true, configurable: true }); } catch(e) {}
                                    }
                                }
                            } catch(e) {}
                            try {
                                if (typeof HTMLAreaElement !== 'undefined' && HTMLAreaElement.prototype) {
                                    var sd2 = Object.getOwnPropertyDescriptor(HTMLAreaElement.prototype, 'toString');
                                    if (sd2 && sd2.enumerable === false) {
                                        try { Object.defineProperty(HTMLAreaElement.prototype, 'toString', { enumerable: true, configurable: true }); } catch(e) {}
                                    }
                                }
                            } catch(e) {}
                        })();
                    "#);
                    let _ = v8::Script::compile(scope, descriptor_fix, None).and_then(|s| s.run(scope));

                    *state.dom_templates.borrow_mut() = Some(dom_templates);
                    let count = registry.interface_count();
                    *state.surface_registry.borrow_mut() = Some(registry);
                    *state.behavior_callbacks.borrow_mut() = Some(callbacks);
                    tracing::info!(interfaces = count, "BrowserSurface installation complete");
                }
                Err(e) => {
                    tracing::error!("BrowserSurface installation failed: {}", e);
                }
            }
        }
        unsafe {
            self.isolate.exit();
        }
    }

    /// Load an HTML document into the context, making DOM query APIs available.
    /// Parses the HTML, stores the Document in RuntimeState, and installs
    /// document.getElementById / querySelector / querySelectorAll on the global.
    pub fn set_document(&mut self, html: &str, base_url: Option<&str>) {
        let doc = crate::dom::parse_html(html, base_url);

        // Store in RuntimeState
        {
            let state = RuntimeState::get(&self.isolate);
            *state.document.borrow_mut() = Some(doc);
            state.node_cache.borrow_mut().clear();
            state.style_cache.borrow_mut().clear();
        }

        // Install V8 bindings (skip in worker mode — document is Window-only)
        if !self.worker_mode {
            self.with_global_scope(|scope, global| {
                crate::dom::binding::install_document_bindings(scope, global);
            });
        } else {
            // Worker mode: document should not exist on globalThis.
            // Log for debugging.
            tracing::debug!("set_document: skipping install_document_bindings (worker_mode)");
        }
        // NOTE: DOM_NAV_SHIM_JS removed — navigation properties (parentNode, childNodes, etc.)
        // are now native accessors on the ObjectTemplate prototype chain (dom/template.rs).

        // Re-install Canvas2D shim (DOM bindings may reset HTMLCanvasElement.prototype)
        self.eval(
            crate::canvas::binding::CANVAS2D_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();
    }

    /// Full page.load: parse HTML, install DOM, execute inline <script> tags,
    /// fire DOMContentLoaded event.
    pub fn page_load(&mut self, html: &str, base_url: Option<&str>) {
        self.page_load_with_headers(html, base_url, &[])
    }

    /// Load HTML with response headers (for Set-Cookie processing).
    /// headers: slice of (name, value) pairs.
    pub fn page_load_with_headers(
        &mut self,
        html: &str,
        base_url: Option<&str>,
        headers: &[(String, String)],
    ) {
        // 1. Parse HTML into DOM
        let doc = crate::dom::parse_html(html, base_url);

        // 2. Collect script info (inline content + external src) before storing document
        struct ScriptInfo {
            inline: Option<String>,
            src: Option<String>,
        }
        let scripts: Vec<ScriptInfo> = doc
            .get_elements_by_tag_name("script")
            .iter()
            .map(|&nid| {
                let inline = doc.text_content_of(nid);
                let src = doc
                    .get(nid)
                    .and_then(|n| n.value().get_attr("src"))
                    .map(|s| s.to_string());
                ScriptInfo {
                    inline: if inline.is_empty() {
                        None
                    } else {
                        Some(inline)
                    },
                    src,
                }
            })
            .collect();

        // 3. Store document in RuntimeState
        {
            let state = RuntimeState::get(&self.isolate);
            *state.document.borrow_mut() = Some(doc);
            state.node_cache.borrow_mut().clear();
            state.style_cache.borrow_mut().clear();
        }

        // 4. Install DOM V8 bindings
        self.with_global_scope(|scope, global| {
            crate::dom::binding::install_document_bindings(scope, global);
        });

        // 4a. Pre-populate cookie store from Set-Cookie headers
        if !headers.is_empty() {
            let mut js_lines = Vec::new();
            js_lines.push("(function(){var s=window._iv8CookieStore||(window._iv8CookieStore={});".to_string());
            for (name, value) in headers {
                if name.eq_ignore_ascii_case("set-cookie") {
                    let escaped = value.replace('\\', "\\\\").replace('\'', "\\'");
                    js_lines.push(format!(
                        "(function(v){{var p=v.split(';');var kv=p[0].split('=');if(kv.length<2)return;var n=kv[0].trim();var val=kv.slice(1).join('=').trim();var a={{}};var h=false;for(var i=1;i<p.length;i++){{var at=p[i].trim();var lo=at.toLowerCase();if(lo.indexOf('path=')===0){{a.path=at.substring(5);h=true;}}else if(lo.indexOf('domain=')===0){{a.domain=at.substring(7);h=true;}}else if(lo==='secure'){{a.secure=true;h=true;}}else if(lo==='httponly'){{a.httpOnly=true;h=true;}}}}if(h){{a.v=val;s[n]=a;}}else{{s[n]=val;}}}})('{}');",
                        escaped
                    ));
                }
            }
            js_lines.push("})();".to_string());
            self.eval(&js_lines.join(""), crate::kernel::EvalOpts::default()).ok();
        }

        // 4b. Re-install Canvas2D shim (DOM bindings may have reset HTMLCanvasElement.prototype)
        self.eval(
            crate::canvas::binding::CANVAS2D_SHIM_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 4c. Install document.write workaround shim
        self.eval(DOCUMENT_WRITE_SHIM, crate::kernel::EvalOpts::default())
            .ok();

        // 4d. Re-install document properties (readyState, cookie, etc.)
        // These are reset when install_document_bindings creates a new document object
        self.eval(
            crate::shims::document_props::DOCUMENT_PROPS_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 4d2. Re-install AudioContext subsystem
        self.eval(
            crate::shims::audio_context::AUDIO_CONTEXT_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 4d3. Re-install window properties, global constructors, structuredClone, Blob
        self.eval(
            crate::shims::window_extras::WINDOW_EXTRAS_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 4e. Update location.href if base_url is provided
        if let Some(url) = base_url {
            let url_literal = serde_json::to_string(url).unwrap_or_else(|_| "\"\"".to_string());
            let update_location = format!(
                r#"
(function() {{
    try {{
        var u = new URL({url});
        location.href = u.href;
        location.origin = u.origin;
        location.protocol = u.protocol;
        location.host = u.host;
        location.hostname = u.hostname;
        location.port = u.port;
        location.pathname = u.pathname;
        location.search = u.search;
        location.hash = u.hash;
    }} catch(e) {{
        location.href = {url};
    }}
}})();
"#,
                url = url_literal
            );
            self.eval(&update_location, crate::kernel::EvalOpts::default())
                .ok();
        }

        // 5. Execute scripts in order (inline first, then external)
        for (i, script) in scripts.iter().enumerate() {
            // Handle external script (src attribute)
            if let Some(ref src) = script.src {
                // Resolve URL relative to base_url
                let resolved_url = if src.starts_with("http://") || src.starts_with("https://") {
                    src.clone()
                } else if let Some(base) = base_url {
                    // Simple URL resolution: join base + src
                    if let Ok(base_url_parsed) = url::Url::parse(base) {
                        base_url_parsed
                            .join(src)
                            .map(|u| u.to_string())
                            .unwrap_or_else(|_| src.clone())
                    } else {
                        src.clone()
                    }
                } else {
                    src.clone()
                };

                // Look up in ResourceBundle
                let script_src = {
                    let state = RuntimeState::get(&self.isolate);
                    let bundle = state.resource_bundle.borrow();
                    bundle
                        .get(&resolved_url)
                        .map(|r| String::from_utf8_lossy(&r.body).to_string())
                };

                if let Some(src_code) = script_src {
                    let opts = crate::kernel::EvalOpts {
                        source_url: Some(resolved_url),
                        line_offset: 0,
                        column_offset: 0,
                    };
                    if let Err(e) = self.eval(&src_code, opts) {
                        tracing::warn!("external script {} error: {:?}", i, e);
                    }
                }

                #[test]
                fn screen_profile_runtime_batch_v044() {
                    use crate::convert::RustValue;
                    let source = iv8_profile::defaults::default_profile_source();
                    let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
                    let config = KernelConfig::default().with_profile_matrix(&matrix);
                    let mut kernel = EmbeddedV8Kernel::new(config).unwrap();

                    assert_eq!(
                        kernel.eval_to_rust_value("screen.width"),
                        RustValue::Int(source.display.screen.width as i64)
                    );
                    assert_eq!(
                        kernel.eval_to_rust_value("screen.height"),
                        RustValue::Int(source.display.screen.height as i64)
                    );
                    assert_eq!(
                        kernel.eval_to_rust_value("screen.availWidth"),
                        RustValue::Int(source.display.screen.avail_width as i64)
                    );
                    assert_eq!(
                        kernel.eval_to_rust_value("screen.availHeight"),
                        RustValue::Int(source.display.screen.avail_height as i64)
                    );
                    assert_eq!(
                        kernel.eval_to_rust_value("screen.colorDepth"),
                        RustValue::Int(source.display.screen.color_depth as i64)
                    );
                    assert_eq!(
                        kernel.eval_to_rust_value("screen.pixelDepth"),
                        RustValue::Int(source.display.screen.color_depth as i64)
                    );

                    let dpr = kernel.eval_to_rust_value("window.devicePixelRatio");
                    match dpr {
                        RustValue::Int(v) => {
                            assert_eq!(v as f64, source.display.window.device_pixel_ratio)
                        }
                        RustValue::Float(v) => assert!(
                            (v - source.display.window.device_pixel_ratio).abs() < f64::EPSILON
                        ),
                        other => panic!("expected numeric devicePixelRatio, got {:?}", other),
                    }
                }
            }

            // Handle inline script
            if let Some(ref inline_src) = script.inline {
                let opts = crate::kernel::EvalOpts {
                    source_url: Some(format!("inline-script-{}", i)),
                    line_offset: 0,
                    column_offset: 0,
                };
                if let Err(e) = self.eval(inline_src, opts) {
                    tracing::warn!("inline script {} error: {:?}", i, e);
                }
            }
        }

        // 6. Set readyState to interactive (update JS-side too)
        {
            let state = RuntimeState::get(&self.isolate);
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                doc.set_ready_state(crate::dom::node::DocumentReadyState::Interactive);
            }
        }
        // Update JS-side readyState
        self.eval(
            "try { document.readyState = 'interactive'; } catch(e) {}",
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 7. Dispatch DOMContentLoaded event on document root
        self.with_global_scope(|scope, _global| {
            let state = RuntimeState::get(&*scope);
            let doc = state.document.borrow();
            if let Some(ref document) = *doc {
                let root_id = document.root_id();
                let registry = &state.event_listeners;
                crate::events::target::dispatch_event(
                    scope,
                    registry,
                    document,
                    root_id,
                    "DOMContentLoaded",
                    false,
                );
            }
        });

        // 8. Set readyState to complete (Rust + JS side)
        {
            let state = RuntimeState::get(&self.isolate);
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                doc.set_ready_state(crate::dom::node::DocumentReadyState::Complete);
            }
        }
        self.eval(
            "try { document.readyState = 'complete'; } catch(e) {}",
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 9. Dispatch load event on document root
        self.with_global_scope(|scope, _global| {
            let state = RuntimeState::get(&*scope);
            let doc = state.document.borrow();
            if let Some(ref document) = *doc {
                let root_id = document.root_id();
                let registry = &state.event_listeners;
                crate::events::target::dispatch_event(
                    scope,
                    registry,
                    document,
                    root_id,
                    "load",
                    false,
                );
            }
        });

        // 9b. Re-install cookie accessor after all scripts executed.
        // Inline scripts may have interfered with the cookie accessor via
        // Object.defineProperty. Only re-install cookie (not full
        // DOCUMENT_PROPS_JS) to avoid Intl/Date lastModified getter
        // re-entrancy OOM in callback contexts.
        self.eval(
            crate::shims::document_props::COOKIE_REINSTALL_JS,
            crate::kernel::EvalOpts::default(),
        )
        .ok();

        // 10. Drain microtasks
        self.drain_microtasks();
    }

    /// Add a resource to the bundle (for offline fetch/XHR).
    pub fn add_resource(
        &self,
        url: &str,
        body: Vec<u8>,
        status: u16,
        headers: Option<std::collections::HashMap<String, String>>,
    ) {
        let state = RuntimeState::get(&self.isolate);
        state
            .resource_bundle
            .borrow_mut()
            .add_raw(url, body, status, headers);
    }

    /// Set a network handler for fetch/XHR fallback.
    /// Called when URL is not in ResourceBundle.
    pub fn set_network_handler(&self, handler: crate::state::NetworkHandler) {
        let state = RuntimeState::get(&self.isolate);
        *state.network_handler.borrow_mut() = Some(handler);
    }

    /// Clear the network handler.
    pub fn clear_network_handler(&self) {
        let state = RuntimeState::get(&self.isolate);
        *state.network_handler.borrow_mut() = None;
    }

    /// Start the V8 Inspector (CDP WebSocket server).
    /// Returns the DevTools URL to open in Chrome.
    pub fn start_inspector(
        &mut self,
        port: u16,
        watch_apis: Vec<String>,
        enable_console: bool,
    ) -> Result<String, String> {
        let config = crate::inspector::session::InspectorConfig {
            port,
            watch_apis,
            enable_console,
        };

        let mut session = crate::inspector::session::InspectorSession::new(config)
            .map_err(|e| format!("failed to start DevTools server on port {}: {}", port, e))?;

        // Initialize inspector: create V8Inspector + session
        // Must be done with isolate entered but without an active scope
        unsafe {
            self.isolate.enter();
        }

        // Step 1: Create inspector (needs &mut Isolate, no scope)
        let client = v8::inspector::V8InspectorClient::new(Box::new(
            crate::inspector::session::InspectorClientImpl::new(session.channel_state.clone()),
        ));
        let inspector = v8::inspector::V8Inspector::create(&mut self.isolate, client);

        // Step 2: Register context (needs a scope to get Local<Context>)
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            let context_name = v8::inspector::StringView::from(b"iv8-rs" as &[u8]);
            let aux_data = v8::inspector::StringView::from(b"{}" as &[u8]);
            inspector.context_created(context, 1, context_name, aux_data);
        }

        // Step 3: Create session channel + session
        let channel = v8::inspector::Channel::new(Box::new(
            crate::inspector::session::InspectorChannelImpl::new(session.channel_state.clone()),
        ));
        let state_str = v8::inspector::StringView::from(b"{}" as &[u8]);
        let v8_session = inspector.connect(
            1,
            channel,
            state_str,
            v8::inspector::V8InspectorClientTrustLevel::FullyTrusted,
        );

        session.set_inspector(inspector, v8_session);

        unsafe {
            self.isolate.exit();
        }

        let devtools_url = session.devtools_url.clone();

        // Install vdebugger
        let vdebugger_js = crate::inspector::session::InspectorSession::vdebugger_js().to_string();
        self.eval(&vdebugger_js, crate::kernel::EvalOpts::default())
            .ok();

        // Install watch_apis
        if let Some(watch_js) = session.watch_apis_js() {
            self.eval(&watch_js, crate::kernel::EvalOpts::default())
                .ok();
        }

        // Store session in RuntimeState
        let state = RuntimeState::get(&self.isolate);
        let channel_state = {
            // Clone channel_state before moving session
            let cs = session.channel_state.clone();
            *state.inspector_session.borrow_mut() = Some(session);
            cs
        };

        // Initialize CDP programmatic client
        *state.cdp_client.borrow_mut() = Some(crate::inspector::CdpClient::new(channel_state));

        Ok(devtools_url)
    }

    /// Process pending CDP messages (call periodically when inspector is active).
    pub fn process_inspector_messages(&mut self) {
        let state = RuntimeState::get(&self.isolate);
        let mut session = state.inspector_session.borrow_mut();
        if let Some(ref mut s) = *session {
            s.process_messages();
        }
    }

    /// Wait for DevTools to connect.
    pub fn wait_for_devtools(&self, timeout_ms: u64) {
        let state = RuntimeState::get(&self.isolate);
        let session = state.inspector_session.borrow();
        if let Some(ref s) = *session {
            s.wait_for_connection(timeout_ms);
        }
    }

    // ─── CDP Programmatic API (v0.3 M15) ─────────────────────────────────────

    /// Set a breakpoint by URL. Returns breakpoint_id or error.
    pub fn cdp_set_breakpoint(
        &mut self,
        url: &str,
        line: u32,
        column: Option<u32>,
        condition: Option<&str>,
    ) -> Result<String, String> {
        // V8 Inspector requires the isolate to be entered.
        unsafe {
            self.isolate.enter();
        }
        let result = self.cdp_set_breakpoint_inner(url, line, column, condition);
        unsafe {
            self.isolate.exit();
        }
        result
    }

    fn cdp_set_breakpoint_inner(
        &mut self,
        url: &str,
        line: u32,
        column: Option<u32>,
        condition: Option<&str>,
    ) -> Result<String, String> {
        let state = RuntimeState::get(&self.isolate);
        let session_guard = state.inspector_session.borrow();
        let session = session_guard
            .as_ref()
            .and_then(|s| s.session_ref())
            .ok_or_else(|| "Inspector not started. Call with_devtools() first.".to_string())?;
        let mut cdp = state.cdp_client.borrow_mut();
        let cdp = cdp
            .as_mut()
            .ok_or_else(|| "CDP client not initialized.".to_string())?;
        cdp.ensure_debugger_enabled(session);
        cdp.set_breakpoint_by_url(session, url, line, column, condition)
    }

    /// Remove a breakpoint by id.
    pub fn cdp_remove_breakpoint(&mut self, breakpoint_id: &str) -> Result<(), String> {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard
                .as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.remove_breakpoint(session, breakpoint_id)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }

    /// Evaluate expression on a call frame while paused.
    pub fn cdp_evaluate_on_frame(
        &mut self,
        call_frame_id: &str,
        expression: &str,
    ) -> Result<serde_json::Value, String> {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard
                .as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.evaluate_on_call_frame(session, call_frame_id, expression)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }

    /// Resume execution (after pause).
    pub fn cdp_resume(&mut self) -> Result<(), String> {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard
                .as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.resume(session)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }

    /// Step over (after pause).
    pub fn cdp_step_over(&mut self) -> Result<(), String> {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard
                .as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.step_over(session)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }

    /// Step into (after pause).
    pub fn cdp_step_into(&mut self) -> Result<(), String> {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard
                .as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.step_into(session)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }

    /// Step out (exit current function, return to caller).
    pub fn cdp_step_out(&mut self) -> Result<(), String> {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard
                .as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.step_out(session)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }

    /// Get call frames from last Debugger.paused event.
    pub fn cdp_get_call_frames(&self) -> Option<serde_json::Value> {
        let state = RuntimeState::get(&self.isolate);
        let cdp = state.cdp_client.borrow();
        cdp.as_ref().and_then(|c| c.get_call_frames().cloned())
    }

    /// Get properties of a remote object (scope variable enumeration).
    pub fn cdp_get_properties(
        &mut self,
        object_id: &str,
        own_properties: bool,
    ) -> Result<serde_json::Value, String> {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard
                .as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let mut cdp = state.cdp_client.borrow_mut();
            let cdp = cdp.as_mut().ok_or("CDP client not initialized.")?;
            cdp.get_properties(session, object_id, own_properties)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }

    /// Process CDP events (check for paused/resumed).
    pub fn cdp_process_events(&mut self) -> bool {
        let state = RuntimeState::get(&self.isolate);
        let mut cdp = state.cdp_client.borrow_mut();
        if let Some(ref mut c) = *cdp {
            c.process_events()
        } else {
            false
        }
    }

    /// Convenience: eval + convert to RustValue in one call.
    /// Used by tests and by the Python binding layer.
    pub fn eval_to_rust_value(&mut self, source: &str) -> crate::convert::RustValue {
        use crate::convert::RustValue;

        let global = match self.eval(source, crate::kernel::EvalOpts::default()) {
            Ok(g) => g,
            Err(_) => return RustValue::Null,
        };

        self.global_to_rust_value(&global)
    }

    /// Evaluate a JavaScript module (ESM) using v8::Module API.
    ///
    /// Compiles the source as an ES module, instantiates it with a
    /// minimal resolve callback, evaluates it, and runs microtask
    /// checkpoint for top-level await support.
    ///
    /// Returns the module namespace object.
    pub fn eval_module(
        &mut self,
        source: &str,
        specifier: Option<&str>,
        _opts: EvalOpts,
    ) -> Result<v8::Global<v8::Value>, IV8Error> {
        self.assert_thread();

        // Enter isolate
        unsafe {
            self.isolate.enter();
        }

        let result = (|| -> Result<v8::Global<v8::Value>, IV8Error> {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);

            let name = specifier.unwrap_or("<module>");
            let source_str = v8::String::new(scope, source)
                .ok_or_else(|| IV8Error::Internal("failed to create V8 source string".into()))?;

            let origin = v8::ScriptOrigin::new(
                scope,
                v8::String::new(scope, name)
                    .unwrap_or_else(|| v8::String::empty(scope))
                    .into(),
                0,
                0,
                true,
                0,
                None,
                false,
                false,
                true,
                None,
            );

            let mut source = v8::script_compiler::Source::new(source_str, Some(&origin));

            let module = v8::script_compiler::compile_module(scope, &mut source)
                .ok_or_else(|| IV8Error::Internal("module compilation failed".into()))?;

            let instantiated =
                module.instantiate_module(scope, |_context, _specifier, _referrer, _module| None);

            if instantiated.is_none() {
                return Err(IV8Error::Internal("module instantiation failed".into()));
            }

            let result = module
                .evaluate(scope)
                .ok_or_else(|| IV8Error::Internal("module evaluation failed".into()))?;

            Ok(v8::Global::new(scope, result))
        })();

        // Microtask checkpoint after scope exit (avoids double borrow)
        self.isolate.perform_microtask_checkpoint();

        // Exit isolate
        unsafe {
            self.isolate.exit();
        }

        result
    }

    /// Eval and if the result is a Promise, await it by draining the event loop.
    /// Returns the resolved value. Rejections become IV8Error::Js and timeouts
    /// become IV8Error::Terminated.
    pub fn eval_await(
        &mut self,
        source: &str,
        max_ticks: u32,
    ) -> Result<v8::Global<v8::Value>, crate::error::IV8Error> {
        let global = self.eval(source, crate::kernel::EvalOpts::default())?;

        // Check if result is a Promise
        let is_promise = {
            unsafe {
                self.isolate.enter();
            }
            let result = {
                v8::scope!(handle_scope, &mut self.isolate);
                let context = v8::Local::new(handle_scope, &self.context);
                v8::scope_with_context!(scope, handle_scope, context);
                let local = v8::Local::new(scope, &global);
                local.is_promise()
            };
            unsafe {
                self.isolate.exit();
            }
            result
        };

        if !is_promise {
            return Ok(global);
        }

        // Use JS to attach .then/.catch and store the result
        let settle_script = r#"
(function(__promise__) {
    var __status__ = 'pending';
    var __result__ = undefined;
    var __error_name__ = 'Error';
    var __error_message__ = '';
    var __error_stack__ = '';
    __promise__.then(function(v) {
        __status__ = 'fulfilled';
        __result__ = v;
    }).catch(function(e) {
        __status__ = 'rejected';
        __error_name__ = e && e.name ? String(e.name) : 'Error';
        __error_message__ = e && e.message != null ? String(e.message) : String(e);
        __error_stack__ = e && e.stack ? String(e.stack) : '';
    });
    return {
        status: function() { return __status__; },
        result: function() { return __result__; },
        errorName: function() { return __error_name__; },
        errorMessage: function() { return __error_message__; },
        errorStack: function() { return __error_stack__; }
    };
})
"#;

        unsafe {
            self.isolate.enter();
        }
        let tracker = {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            v8::tc_scope!(tc, scope);

            let promise_local = v8::Local::new(tc, &global);
            let fn_src = crate::v8_utils::v8_string(tc, settle_script);
            let script = v8::Script::compile(tc, fn_src, None).expect("compile");
            let fn_val = script.run(tc).expect("run");
            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(fn_val) };
            let undefined = v8::undefined(tc);
            let tracker = func
                .call(tc, undefined.into(), &[promise_local])
                .expect("call");
            v8::Global::new(tc, tracker)
        };
        unsafe {
            self.isolate.exit();
        }

        // Drain microtasks + macrotasks until settled or max_ticks
        for _ in 0..max_ticks {
            self.drain_microtasks();

            // Check settlement status
            let status = {
                unsafe {
                    self.isolate.enter();
                }
                let result = {
                    v8::scope!(handle_scope, &mut self.isolate);
                    let context = v8::Local::new(handle_scope, &self.context);
                    v8::scope_with_context!(scope, handle_scope, context);
                    v8::tc_scope!(tc, scope);
                    let tracker_local = v8::Local::new(tc, &tracker);
                    let tracker_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(tracker_local) };
                    let status_key = crate::v8_utils::v8_string(tc, "status");
                    if let Some(status_fn) = tracker_obj.get(tc, status_key.into()) {
                        if status_fn.is_function() {
                            let func: v8::Local<v8::Function> =
                                unsafe { v8::Local::cast_unchecked(status_fn) };
                            let undefined = v8::undefined(tc);
                            if let Some(result) = func.call(tc, undefined.into(), &[]) {
                                result.to_rust_string_lossy(tc)
                            } else {
                                "pending".to_string()
                            }
                        } else {
                            "pending".to_string()
                        }
                    } else {
                        "pending".to_string()
                    }
                };
                unsafe {
                    self.isolate.exit();
                }
                result
            };

            if status == "fulfilled" {
                // Extract the result
                unsafe {
                    self.isolate.enter();
                }
                let result = {
                    v8::scope!(handle_scope, &mut self.isolate);
                    let context = v8::Local::new(handle_scope, &self.context);
                    v8::scope_with_context!(scope, handle_scope, context);
                    v8::tc_scope!(tc, scope);
                    let tracker_local = v8::Local::new(tc, &tracker);
                    let tracker_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(tracker_local) };
                    let result_key = crate::v8_utils::v8_string(tc, "result");
                    if let Some(result_fn) = tracker_obj.get(tc, result_key.into()) {
                        if result_fn.is_function() {
                            let func: v8::Local<v8::Function> =
                                unsafe { v8::Local::cast_unchecked(result_fn) };
                            let undefined = v8::undefined(tc);
                            if let Some(val) = func.call(tc, undefined.into(), &[]) {
                                v8::Global::new(tc, val)
                            } else {
                                let undef: v8::Local<v8::Value> = v8::undefined(tc).into();
                                v8::Global::new(tc, undef)
                            }
                        } else {
                            let undef: v8::Local<v8::Value> = v8::undefined(tc).into();
                            v8::Global::new(tc, undef)
                        }
                    } else {
                        let undef: v8::Local<v8::Value> = v8::undefined(tc).into();
                        v8::Global::new(tc, undef)
                    }
                };
                unsafe {
                    self.isolate.exit();
                }
                return Ok(result);
            } else if status == "rejected" {
                unsafe {
                    self.isolate.enter();
                }
                let (name, message, stack) = {
                    v8::scope!(handle_scope, &mut self.isolate);
                    let context = v8::Local::new(handle_scope, &self.context);
                    v8::scope_with_context!(scope, handle_scope, context);
                    v8::tc_scope!(tc, scope);
                    let tracker_local = v8::Local::new(tc, &tracker);
                    let tracker_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(tracker_local) };
                    fn call_tracker_string<'s>(
                        tc: &mut v8::PinScope<'s, '_>,
                        tracker_obj: v8::Local<'s, v8::Object>,
                        key: &str,
                    ) -> String {
                        let key = crate::v8_utils::v8_string(tc, key);
                        if let Some(value) = tracker_obj.get(tc, key.into()) {
                            if value.is_function() {
                                let func: v8::Local<v8::Function> =
                                    unsafe { v8::Local::cast_unchecked(value) };
                                let undefined = v8::undefined(tc);
                                if let Some(result) = func.call(tc, undefined.into(), &[]) {
                                    return result.to_rust_string_lossy(tc);
                                }
                            }
                        }
                        String::new()
                    }
                    (
                        call_tracker_string(tc, tracker_obj, "errorName"),
                        call_tracker_string(tc, tracker_obj, "errorMessage"),
                        call_tracker_string(tc, tracker_obj, "errorStack"),
                    )
                };
                unsafe {
                    self.isolate.exit();
                }
                return Err(crate::error::IV8Error::Js {
                    name: if name.is_empty() {
                        "Error".to_string()
                    } else {
                        name
                    },
                    message,
                    stack,
                    value: None,
                });
            }

            // Advance event loop by one tick to process pending timers
            let _ = self.eval(
                "__iv8__.eventLoop.tick()",
                crate::kernel::EvalOpts::default(),
            );
        }

        Err(crate::error::IV8Error::Terminated)
    }

    /// Convert a V8 Global<Value> to RustValue using this kernel's context.
    pub fn global_to_rust_value(
        &mut self,
        global: &v8::Global<v8::Value>,
    ) -> crate::convert::RustValue {
        unsafe {
            self.isolate.enter();
        }
        let result = {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            v8::tc_scope!(tc, scope);
            let local = v8::Local::new(tc, global);
            crate::convert::v8_to_rust_impl(tc, local, 0)
        };
        unsafe {
            self.isolate.exit();
        }
        result
    }
}

impl Drop for EmbeddedV8Kernel {
    fn drop(&mut self) {
        // Flush localStorage before isolate disposal.
        self.flush_local_storage();
        // Re-enter the isolate before drop — OwnedIsolate expects to be entered
        // SAFETY: we exited after new(), now re-enter for proper cleanup
        unsafe {
            self.isolate.enter();
        }
        // OwnedIsolate::drop will exit and dispose
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_eval_basic_arithmetic() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        let result = kernel.eval_to_rust_value("1 + 2");
        assert_eq!(result, crate::convert::RustValue::Int(3));
    }

    #[test]
    fn kernel_eval_string() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        let result = kernel.eval_to_rust_value("'hello' + ' world'");
        assert_eq!(
            result,
            crate::convert::RustValue::String("hello world".into())
        );
    }

    #[test]
    fn kernel_eval_syntax_error() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        let err = kernel.eval("function(", EvalOpts::default()).unwrap_err();
        match err {
            IV8Error::Compile { message, line, .. } => {
                // V8 reports various syntax error messages depending on version
                assert!(
                    !message.is_empty(),
                    "compile error message should not be empty"
                );
                assert!(line >= 0, "line should be non-negative");
            }
            other => panic!("expected Compile error, got: {:?}", other),
        }
    }

    #[test]
    fn kernel_eval_runtime_error() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        let err = kernel
            .eval("throw new TypeError('boom')", EvalOpts::default())
            .unwrap_err();
        match err {
            IV8Error::Js { name, message, .. } => {
                assert_eq!(name, "TypeError");
                assert!(message.contains("boom"), "message: {}", message);
            }
            other => panic!("expected Js error, got: {:?}", other),
        }
    }

    #[test]
    fn kernel_eval_persists_state() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.eval("var x = 42", EvalOpts::default()).unwrap();
        let result = kernel.eval_to_rust_value("x + 1");
        assert_eq!(result, crate::convert::RustValue::Int(43));
    }

    #[test]
    fn kernel_eval_increments_count() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        // Note: kernel creation runs shim evals internally, so count starts > 0
        let baseline = {
            let state = RuntimeState::get(&kernel.isolate);
            *state.eval_count.borrow()
        };

        kernel.eval("1", EvalOpts::default()).unwrap();
        kernel.eval("2", EvalOpts::default()).unwrap();
        kernel.eval("3", EvalOpts::default()).unwrap();

        let state = RuntimeState::get(&kernel.isolate);
        assert_eq!(*state.eval_count.borrow(), baseline + 3);
    }

    #[test]
    fn kernel_microtask_policy_is_explicit() {
        let kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        assert_eq!(
            kernel.isolate.get_microtasks_policy(),
            v8::MicrotasksPolicy::Explicit
        );
    }

    #[test]
    fn kernel_environment_accessible() {
        let mut config = KernelConfig::default();
        let mut overrides = std::collections::HashMap::new();
        overrides.insert(
            "navigator.userAgent".to_string(),
            serde_json::Value::String("TestAgent/1.0".to_string()),
        );
        config.environment_overrides = Some(overrides);

        let kernel = EmbeddedV8Kernel::new(config).unwrap();
        assert_eq!(
            kernel.environment().get_str("navigator.userAgent").unwrap(),
            "TestAgent/1.0"
        );
    }

    #[test]
    fn profile_matrix_environment_overrides_are_observable_in_js() {
        use crate::convert::RustValue;

        fn assert_number_eq(actual: RustValue, expected: f64) {
            match actual {
                RustValue::Int(v) => assert_eq!(v as f64, expected),
                RustValue::Float(v) => assert!((v - expected).abs() < f64::EPSILON),
                other => panic!("expected numeric RustValue, got {:?}", other),
            }
        }

        let source = iv8_profile::defaults::default_profile_source();
        let (matrix, validation) = iv8_profile::ProfileMatrix::from_source(&source);
        assert!(
            validation.is_valid(),
            "default profile should validate: {}",
            validation
        );

        let config = KernelConfig::default().with_profile_matrix(&matrix);
        let mut kernel = EmbeddedV8Kernel::new(config).unwrap();

        assert_eq!(
            kernel.eval_to_rust_value("navigator.userAgent"),
            RustValue::String(source.navigator.user_agent)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.platform"),
            RustValue::String(source.navigator.platform)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.vendor"),
            RustValue::String(source.navigator.vendor)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.language"),
            RustValue::String(source.navigator.language)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.languages[0]"),
            RustValue::String(source.navigator.languages[0].clone())
        );
        assert_eq!(
            kernel.eval_to_rust_value("screen.width"),
            RustValue::Int(source.display.screen.width as i64)
        );
        assert_eq!(
            kernel.eval_to_rust_value("screen.availWidth"),
            RustValue::Int(source.display.screen.avail_width as i64)
        );
        assert_eq!(
            kernel.eval_to_rust_value("screen.colorDepth"),
            RustValue::Int(source.display.screen.color_depth as i64)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.webdriver"),
            RustValue::Bool(source.navigator.webdriver)
        );
        assert_number_eq(
            kernel.eval_to_rust_value("window.devicePixelRatio"),
            source.display.window.device_pixel_ratio,
        );
        assert_eq!(
            kernel.eval_to_rust_value("location.href"),
            RustValue::String("about:blank".into())
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.userAgentData.platform"),
            RustValue::String(source.navigator.user_agent_data.platform)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.userAgentData.mobile"),
            RustValue::Bool(source.navigator.user_agent_data.mobile)
        );
        assert!(
            (kernel
                .environment()
                .get_f64("timers.raf_interval_ms")
                .expect("timer projection should be present")
                - (1000.0 / source.timing.fps as f64))
                .abs()
                < f64::EPSILON
        );
        assert_eq!(
            kernel.eval_to_rust_value(
                "document.createElement('canvas').getContext('webgl').getParameter(0x1F00)"
            ),
            RustValue::String(source.identity.gpu.vendor)
        );
        assert_eq!(
            kernel.eval_to_rust_value(
                "document.createElement('canvas').getContext('webgl').getParameter(0x9246)"
            ),
            RustValue::String(source.identity.gpu.webgl_unmasked_renderer)
        );
    }

    #[test]
    fn navigator_profile_runtime_batch_v043() {
        use crate::convert::RustValue;

        let source = iv8_profile::defaults::default_profile_source();
        let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
        let config = KernelConfig::default().with_profile_matrix(&matrix);
        let mut kernel = EmbeddedV8Kernel::new(config).unwrap();

        assert_eq!(
            kernel.eval_to_rust_value("navigator.language"),
            RustValue::String(source.navigator.language)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.languages[0]"),
            RustValue::String(source.navigator.languages[0].clone())
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.platform"),
            RustValue::String(source.navigator.platform)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.webdriver"),
            RustValue::Bool(source.navigator.webdriver)
        );

        let hw = kernel.eval_to_rust_value("navigator.hardwareConcurrency");
        match hw {
            RustValue::Int(v) => assert_eq!(v as f64, source.navigator.hardware_concurrency as f64),
            RustValue::Float(v) => {
                assert!((v - source.navigator.hardware_concurrency as f64).abs() < f64::EPSILON)
            }
            other => panic!("expected numeric hardwareConcurrency, got {:?}", other),
        }

        let dm = kernel.eval_to_rust_value("navigator.deviceMemory");
        match dm {
            RustValue::Int(v) => assert_eq!(v as f64, source.navigator.device_memory as f64),
            RustValue::Float(v) => {
                assert!((v - source.navigator.device_memory as f64).abs() < f64::EPSILON)
            }
            other => panic!("expected numeric deviceMemory, got {:?}", other),
        }
    }

    #[test]
    fn uadata_low_entropy_boundary_v045() {
        use crate::convert::RustValue;

        let source = iv8_profile::defaults::default_profile_source();
        let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
        let config = KernelConfig::default().with_profile_matrix(&matrix);
        let mut kernel = EmbeddedV8Kernel::new(config).unwrap();

        assert_eq!(
            kernel.eval_to_rust_value("navigator.userAgentData.platform"),
            RustValue::String(source.navigator.user_agent_data.platform)
        );
        assert_eq!(
            kernel.eval_to_rust_value("navigator.userAgentData.mobile"),
            RustValue::Bool(source.navigator.user_agent_data.mobile)
        );

        let brands = kernel.eval_to_rust_value("navigator.userAgentData.brands");
        match brands {
            RustValue::Array(ref entries) => {
                assert!(!entries.is_empty(), "brands array should not be empty");
                if let RustValue::Object(ref obj) = entries[0] {
                    assert!(
                        obj.contains_key("brand"),
                        "brands[0] should have 'brand' key"
                    );
                    assert!(
                        obj.contains_key("version"),
                        "brands[0] should have 'version' key"
                    );
                } else {
                    panic!("brands[0] should be an Object");
                }
            }
            other => panic!("expected brands to be an Array, got {:?}", other),
        }
    }

    #[test]
    fn timing_performance_now_boundary_v046() {
        use crate::convert::RustValue;

        let source = iv8_profile::defaults::default_profile_source();
        let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
        let config = KernelConfig::default().with_profile_matrix(&matrix);
        let mut kernel = EmbeddedV8Kernel::new(config).unwrap();

        assert_eq!(
            kernel.eval_to_rust_value("typeof performance.now"),
            RustValue::String("function".into())
        );

        let t0 = kernel.eval_to_rust_value("performance.now()");
        match t0 {
            RustValue::Float(v) => assert!(v >= 0.0, "performance.now() >= 0, got {}", v),
            RustValue::Int(v) => assert!(v >= 0, "performance.now() >= 0, got {}", v),
            other => panic!("expected numeric from performance.now(), got {:?}", other),
        }

        let t1 = kernel.eval_to_rust_value("performance.now()");
        fn as_f64(v: &RustValue) -> f64 {
            match v {
                RustValue::Float(f) => *f,
                RustValue::Int(i) => *i as f64,
                _ => panic!("not numeric"),
            }
        }
        let a = as_f64(&t0);
        let b = as_f64(&t1);
        assert!(b >= a, "performance.now() monotonic: {} then {}", a, b);
    }

    #[test]
    fn performance_memory_quantized_and_stable_v080() {
        use crate::convert::RustValue;

        let source = iv8_profile::defaults::default_profile_source();
        let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
        let config = KernelConfig::default().with_profile_matrix(&matrix);
        let mut kernel = EmbeddedV8Kernel::new(config).unwrap();

        // performance.memory must exist and be an object.
        assert_eq!(
            kernel.eval_to_rust_value("typeof performance.memory"),
            RustValue::String("object".into())
        );

        fn num(kernel: &mut EmbeddedV8Kernel, expr: &str) -> f64 {
            match kernel.eval_to_rust_value(expr) {
                RustValue::Float(f) => f,
                RustValue::Int(i) => i as f64,
                other => panic!("expected number from `{}`, got {:?}", expr, other),
            }
        }

        let limit = num(&mut kernel, "performance.memory.jsHeapSizeLimit");
        let total = num(&mut kernel, "performance.memory.totalJSHeapSize");
        let used = num(&mut kernel, "performance.memory.usedJSHeapSize");

        // All values must be multiples of the 100KB bucket (102400 bytes).
        const BUCKET: f64 = 102_400.0;
        assert!(
            limit > 0.0 && (limit % BUCKET).abs() < f64::EPSILON,
            "jsHeapSizeLimit must be a positive 100KB multiple, got {}",
            limit
        );
        assert!(
            total > 0.0 && (total % BUCKET).abs() < f64::EPSILON,
            "totalJSHeapSize must be a positive 100KB multiple, got {}",
            total
        );
        assert!(
            used > 0.0 && (used % BUCKET).abs() < f64::EPSILON,
            "usedJSHeapSize must be a positive 100KB multiple, got {}",
            used
        );

        // Heap invariant: used <= total <= limit.
        assert!(used <= total, "used ({}) must be <= total ({})", used, total);
        assert!(total <= limit, "total ({}) must be <= limit ({})", total, limit);

        // Per-page stability: repeated reads return identical values.
        let limit2 = num(&mut kernel, "performance.memory.jsHeapSizeLimit");
        let total2 = num(&mut kernel, "performance.memory.totalJSHeapSize");
        let used2 = num(&mut kernel, "performance.memory.usedJSHeapSize");
        assert_eq!(limit, limit2, "jsHeapSizeLimit must be stable across calls");
        assert_eq!(total, total2, "totalJSHeapSize must be stable across calls");
        assert_eq!(used, used2, "usedJSHeapSize must be stable across calls");
    }

    #[test]
    fn performance_now_jitter_breaks_identical_diffs_v080() {
        use crate::convert::RustValue;

        let source = iv8_profile::defaults::default_profile_source();
        let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
        let config = KernelConfig::default().with_profile_matrix(&matrix);
        let mut kernel = EmbeddedV8Kernel::new(config).unwrap();

        fn now(kernel: &mut EmbeddedV8Kernel) -> f64 {
            match kernel.eval_to_rust_value("performance.now()") {
                RustValue::Float(f) => f,
                RustValue::Int(i) => i as f64,
                other => panic!("expected number, got {:?}", other),
            }
        }

        // Sample 10 times in a tight loop with no task advancement.
        let mut samples: Vec<f64> = Vec::with_capacity(10);
        for _ in 0..10 {
            samples.push(now(&mut kernel));
        }

        // Monotonicity: each sample >= previous.
        for w in samples.windows(2) {
            assert!(w[1] >= w[0], "performance.now() not monotonic: {} then {}", w[0], w[1]);
        }

        // Bot-tell guard: not all consecutive diffs are identical. At least
        // one diff must differ from the others (jitter is active).
        let diffs: Vec<f64> = samples.windows(2).map(|w| w[1] - w[0]).collect();
        let first = diffs[0];
        let all_same = diffs.iter().all(|d| (*d - first).abs() < f64::EPSILON);
        // With the monotonic jitter, consecutive calls strictly increase, so
        // at least the first diff (0 if base didn't move) differs from later
        // diffs (0.0011). Assert not-all-identical.
        assert!(!all_same, "performance.now() diffs all identical ({}) — jitter not active", first);
    }

    #[test]
    fn native_code_tostring_boundary_v047() {
        use crate::convert::RustValue;

        let source = iv8_profile::defaults::default_profile_source();
        let (matrix, _) = iv8_profile::ProfileMatrix::from_source(&source);
        let config = KernelConfig::default().with_profile_matrix(&matrix);
        let mut kernel = EmbeddedV8Kernel::new(config).unwrap();

        assert_eq!(
            kernel.eval_to_rust_value("typeof eval.toString"),
            RustValue::String("function".into())
        );

        let ts = kernel.eval_to_rust_value("eval.toString()");
        match ts {
            RustValue::String(s) => {
                assert!(
                    s.contains("function"),
                    "eval.toString() must contain 'function', got '{}'",
                    s
                );
                assert!(
                    s.contains("[native code]"),
                    "eval.toString() must contain '[native code]', got '{}'",
                    s
                );
            }
            other => panic!("expected string from eval.toString(), got {:?}", other),
        }
    }

    #[test]
    fn event_loop_microtask_drain_alignment_v080() {
        // B4: Verify microtask drain happens after eval (HTML spec alignment).
        // Promise.then callbacks are microtasks that should drain before
        // the next eval call.
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();

        // Set up a microtask via Promise.resolve().then()
        kernel
            .eval(
                "globalThis.__mt_result = 'before'; Promise.resolve().then(function() { globalThis.__mt_result = 'after'; })",
                EvalOpts::default(),
            )
            .ok();

        // After eval returns, microtasks should have drained
        let result = kernel.eval_to_rust_value("globalThis.__mt_result");
        assert_eq!(
            result,
            crate::convert::RustValue::String("after".into()),
            "microtask should drain after eval (HTML spec microtask checkpoint)"
        );
    }

    #[test]
    fn event_loop_settimeout_order_alignment_v080() {
        // B4: Verify setTimeout callbacks execute in registration order.
        // Note: setTimeout timers require event loop time advancement to fire.
        // page_load does not auto-advance time, so we verify the timer queue
        // accepts registrations and IDs are sequential.
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();

        // setTimeout returns timer IDs that should be sequential
        let id1 = kernel.eval_to_rust_value("setTimeout(function(){}, 0)");
        let id2 = kernel.eval_to_rust_value("setTimeout(function(){}, 0)");
        let id3 = kernel.eval_to_rust_value("setTimeout(function(){}, 0)");

        // Timer IDs should be sequential positive integers
        let id_vals: Vec<i64> = [id1, id2, id3]
            .iter()
            .map(|v| match v {
                crate::convert::RustValue::Int(i) => *i,
                _ => -1,
            })
            .collect();
        assert!(id_vals[0] >= 1, "first timer ID should be >= 1, got {}", id_vals[0]);
        assert_eq!(
            id_vals[1],
            id_vals[0] + 1,
            "second timer ID should be sequential"
        );
        assert_eq!(
            id_vals[2],
            id_vals[0] + 2,
            "third timer ID should be sequential"
        );
    }

    #[test]
    fn event_loop_microtask_before_macrotask_v080() {
        // B4: Verify microtasks drain after eval (HTML spec microtask checkpoint).
        // Promise.then is a microtask that drains before the next eval.
        // setTimeout is a macrotask that requires event loop time advancement.
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();

        // Register a microtask and a macrotask
        kernel
            .eval(
                r#"
                globalThis.__seq = [];
                setTimeout(function() { globalThis.__seq.push('timeout'); }, 0);
                Promise.resolve().then(function() { globalThis.__seq.push('microtask'); });
                "#,
                EvalOpts::default(),
            )
            .ok();

        // After eval, microtask should have drained but macrotask should not
        let result = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
        assert_eq!(
            result,
            crate::convert::RustValue::String("microtask".into()),
            "microtask must drain after eval; setTimeout macrotask should NOT fire yet"
        );
    }
}
