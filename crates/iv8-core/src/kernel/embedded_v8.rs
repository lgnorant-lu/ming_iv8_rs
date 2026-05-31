//! EmbeddedV8Kernel: the primary kernel implementation using v8 crate.

use crate::config::EnvironmentMap;
use crate::error::IV8Error;
use crate::kernel::{EvalOpts, KernelConfig};
use crate::state::RuntimeState;
use crate::v8_init::ensure_v8_initialized;
use std::sync::Arc;

/// document.write workaround shim (REQ-DOM-008).
/// Replaces document.write with insertAdjacentHTML-based implementation.
const DOCUMENT_WRITE_SHIM: &str = r#"
(function() {
    if (typeof document === 'undefined') return;
    var origWrite = document.write;
    document.write = function() {
        var html = Array.prototype.join.call(arguments, '');
        // Try to insert after currentScript, fallback to body
        var anchor = document.currentScript;
        if (anchor && anchor.parentNode) {
            try {
                anchor.insertAdjacentHTML('afterend', html);
                return;
            } catch(e) {}
        }
        if (document.body) {
            try {
                document.body.insertAdjacentHTML('beforeend', html);
                return;
            } catch(e) {}
        }
        // Last resort: append to document element
        if (document.documentElement) {
            try {
                document.documentElement.insertAdjacentHTML('beforeend', html);
            } catch(e) {}
        }
    };
    document.writeln = function() {
        var args = Array.prototype.slice.call(arguments);
        document.write(args.join(' ') + '\n');
    };
    document.open = function() { return document; };
    document.close = function() {};
})();
"#;

/// Minimal TextEncoder/TextDecoder polyfill for V8 (not included by default).
const TEXT_ENCODER_SHIM: &str = r#"
(function() {
    if (typeof TextEncoder === 'undefined') {
        globalThis.TextEncoder = function TextEncoder() {};
        TextEncoder.prototype.encode = function(str) {
            var arr = [];
            for (var i = 0; i < str.length; i++) {
                var c = str.charCodeAt(i);
                if (c < 128) { arr.push(c); }
                else if (c < 2048) { arr.push((c >> 6) | 192); arr.push((c & 63) | 128); }
                else { arr.push((c >> 12) | 224); arr.push(((c >> 6) & 63) | 128); arr.push((c & 63) | 128); }
            }
            return new Uint8Array(arr);
        };
    }
    if (typeof TextDecoder === 'undefined') {
        globalThis.TextDecoder = function TextDecoder() {};
        TextDecoder.prototype.decode = function(buf) {
            var arr = new Uint8Array(buf);
            var str = '';
            for (var i = 0; i < arr.length; i++) { str += String.fromCharCode(arr[i]); }
            return str;
        };
    }
})();
"#;

/// The embedded V8 kernel — owns an Isolate + Context.
pub struct EmbeddedV8Kernel {
    pub(crate) isolate: v8::OwnedIsolate,
    pub(crate) context: v8::Global<v8::Context>,
    environment: Arc<EnvironmentMap>,
    creator_thread: std::thread::ThreadId,
}

impl EmbeddedV8Kernel {
    /// Create a new embedded V8 kernel with the given configuration.
    pub fn new(config: KernelConfig) -> Result<Self, IV8Error> {
        ensure_v8_initialized();

        // Extract deterministic config before moving config fields
        let random_seed = config.random_seed;
        let crypto_seed = config.crypto_seed;
        let time_freeze = config.time_freeze;

        let environment = Arc::new(EnvironmentMap::build(config.environment_overrides.as_ref()));

        let mut isolate = v8::Isolate::new(v8::CreateParams::default());

        // Set microtask policy to Explicit (we drive microtasks manually)
        isolate.set_microtasks_policy(v8::MicrotasksPolicy::Explicit);

        // Install RuntimeState (with environment reference for V8 callbacks)
        RuntimeState::install(
            &mut isolate,
            RuntimeState::new(
                config.strict_compat,
                config.time_mode,
                config.js_api_name,
                environment.clone(),
            ),
        );

        // Create the main context
        let context = {
            v8::scope!(handle_scope, &mut isolate);
            let context = v8::Context::new(handle_scope, Default::default());
            v8::Global::new(handle_scope, context)
        };

        let mut kernel = Self {
            isolate,
            context,
            environment,
            creator_thread: std::thread::current().id(),
        };

        // Install environment fields (navigator.*, screen.*, etc.) into global
        kernel.install_environment();

        // Install anti-detection shims (__iv8__ + wrapNative + window.chrome)
        kernel.install_undetect_shims();

        // Install deterministic overrides (random_seed / crypto_seed / time_freeze)
        kernel.install_deterministic_overrides_from(random_seed, crypto_seed, time_freeze);

        // Install DOM templates (FunctionTemplate hierarchy)
        kernel.install_dom_templates();

        // Exit the isolate so it's not "entered" at rest.
        // This allows multiple JSContext instances to coexist without LIFO drop panic.
        // We re-enter before each eval/operation and exit after.
        // SAFETY: isolate was entered by v8::Isolate::new, we exit it here.
        unsafe { kernel.isolate.exit(); }

        Ok(kernel)
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
    pub fn eval(&mut self, source: &str, opts: EvalOpts) -> Result<v8::Global<v8::Value>, IV8Error> {
        self.assert_thread();

        // Enter isolate for this operation
        // SAFETY: we exit at the end of this function (or on error return)
        unsafe { self.isolate.enter(); }

        let result = self.eval_inner(source, opts);

        // Run microtasks after each eval (matches browser behavior)
        self.isolate.perform_microtask_checkpoint();

        // Exit isolate after operation
        unsafe { self.isolate.exit(); }

        result
    }

    /// Inner eval implementation (isolate must already be entered).
    fn eval_inner(&mut self, source: &str, opts: EvalOpts) -> Result<v8::Global<v8::Value>, IV8Error> {

        v8::scope!(handle_scope, &mut self.isolate);
        let context = v8::Local::new(handle_scope, &self.context);
        v8::scope_with_context!(scope, handle_scope, context);

        // Create source string
        let source_str = v8::String::new(scope, source).ok_or_else(|| {
            IV8Error::Internal("failed to create V8 source string (too long?)".into())
        })?;

        // Set up script origin if provided
        let origin = if let Some(ref url) = opts.source_url {
            let name = v8::String::new(scope, url).unwrap_or_else(|| {
                v8::String::new(scope, "<eval>").expect("infallible")
            });
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
            let exception = tc.exception().expect("has_caught but no exception");
            let message = exception.to_rust_string_lossy(tc);
            let stack = tc
                .stack_trace()
                .map(|s| s.to_rust_string_lossy(tc))
                .unwrap_or_default();

            let name = if exception.is_native_error() {
                if let Some(obj) = exception.to_object(tc) {
                    let name_key = v8::String::new(tc, "name").expect("infallible");
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
        unsafe { self.isolate.enter(); }
        self.isolate.perform_microtask_checkpoint();
        unsafe { self.isolate.exit(); }
    }

    /// Expose a Rust function to JS global scope.
    /// The function receives args as Vec<String> and returns Result<String, String>.
    /// (Simplified for v0.1 — M2 will add proper V8 value conversion.)
    pub fn expose_fn(
        &mut self,
        name: &str,
        callback: Box<dyn Fn(&[String]) -> Result<String, String> + Send + 'static>,
    ) {
        unsafe { self.isolate.enter(); }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);
            crate::expose::expose_function(scope, global, name, callback);
        }
        unsafe { self.isolate.exit(); }
    }

    /// Execute a closure with access to the V8 scope and global object.
    /// The isolate is entered before and exited after the closure runs.
    /// Use this for operations that need direct V8 API access from outside iv8-core.
    pub fn with_global_scope<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&v8::PinScope<'_, '_>, v8::Local<v8::Object>) -> R,
    {
        unsafe { self.isolate.enter(); }
        let result = {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);
            f(scope, global)
        };
        unsafe { self.isolate.exit(); }
        result
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
            let js = format!(r#"
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
"#, seed = seed);
            self.eval(&js, EvalOpts::default()).ok();
        }

        // time_freeze: override Date.now, performance.now, new Date()
        if let Some(freeze_ms) = time_freeze {
            let js = format!(r#"
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
"#, freeze_ms = freeze_ms as u64);
            self.eval(&js, EvalOpts::default()).ok();
        }

        // crypto_seed: store in RuntimeState for Rust-side random.rs to use
        if let Some(seed) = crypto_seed {
            let state = crate::state::RuntimeState::get(&self.isolate);
            *state.crypto_seed.borrow_mut() = Some(seed);
        }
    }
}

/// No-op call-as-function handler for the undetectable __iv8__ tool object.
/// V8 requires this when MarkAsUndetectable is set on an ObjectTemplate.
unsafe extern "C" fn undetectable_noop_handler(_info: *const v8::FunctionCallbackInfo) {
    // Returns undefined implicitly (no rv.set call).
}

impl EmbeddedV8Kernel {
    /// Install anti-detection shims (__iv8__ tool object + wrapNative + hookNative + window.chrome).
    pub fn install_undetect_shims(&mut self) {
        let js_api_name = {
            let state = crate::state::RuntimeState::get(&self.isolate);
            state.js_api_name.clone()
        };

        // 1. Create __iv8__ tool object with MarkAsUndetectable (DontEnum)
        //    This gives [[IsHTMLDDA]] semantics: typeof === 'undefined', == null, falsy
        unsafe { self.isolate.enter(); }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);

            let templ = v8::ObjectTemplate::new(scope);
            crate::v8_extra::mark_as_undetectable(&templ);
            crate::v8_extra::set_call_as_function_handler(
                &templ,
                undetectable_noop_handler,
                None,
            );
            let tool_obj = templ
                .new_instance(scope)
                .expect("failed to create undetectable __iv8__ instance");

            let key = v8::String::new(scope, &js_api_name).expect("api name");
            global.define_own_property(
                scope,
                key.into(),
                tool_obj.into(),
                v8::PropertyAttribute::DONT_ENUM,
            );
        }
        unsafe { self.isolate.exit(); }

        // 2. Install wrapNative shim
        let wrap_script = format!("{}({})", include_str!("../../../iv8-undetect/src/shims/wrap_native.js"), js_api_name);
        self.eval(&wrap_script, crate::kernel::EvalOpts::default()).ok();

        // 3. Install hookNative shim
        let hook_script = format!("{}({})", include_str!("../../../iv8-undetect/src/shims/hook_native.js"), js_api_name);
        self.eval(&hook_script, crate::kernel::EvalOpts::default()).ok();

        // 4. Install window.chrome shim
        let chrome_script = format!("{}({}.wrapNative)", include_str!("../../../iv8-undetect/src/shims/window_chrome.js"), js_api_name);
        self.eval(&chrome_script, crate::kernel::EvalOpts::default()).ok();

        // 5. Install eventLoop API on __iv8__
        unsafe { self.isolate.enter(); }
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
            // NOTE: install_dom_navigation removed — navigation is now handled by
            // native accessors in dom/template.rs (ObjectTemplate refactor).
            // Install __iv8__.page.load(snapshot) API
            crate::events::page_api::install_page_api(scope, global);
            // Install __iv8__.input.dispatchMouseEvent/dispatchPointerEvent
            crate::events::input_sim::install_input_api(scope, global);
        }
        unsafe { self.isolate.exit(); }

        // 6. Install Date constructor shim (JS-level, needs __iv8_now__ to be ready)
        self.eval(crate::events::date_interceptor::DATE_SHIM_JS, crate::kernel::EvalOpts::default()).ok();

        // 7. Install WebGL context shim
        self.eval(crate::canvas::webgl::WEBGL_SHIM_JS, crate::kernel::EvalOpts::default()).ok();

        // 8. Install XMLHttpRequest class shim
        self.eval(crate::network::xhr::XHR_SHIM_JS, crate::kernel::EvalOpts::default()).ok();

        // 9. Install TextEncoder/TextDecoder polyfill
        self.eval(TEXT_ENCODER_SHIM, crate::kernel::EvalOpts::default()).ok();

        // 10. Install Event/CustomEvent/MouseEvent/KeyboardEvent/PointerEvent constructors
        self.eval(crate::shims::event_constructors::EVENT_CONSTRUCTORS_JS, crate::kernel::EvalOpts::default()).ok();

        // 11. Install getBoundingClientRect + getComputedStyle + DOMRect
        self.eval(crate::shims::geometry::GEOMETRY_SHIM_JS, crate::kernel::EvalOpts::default()).ok();

        // 12. Install URL + URLSearchParams
        self.eval(crate::shims::url::URL_SHIM_JS, crate::kernel::EvalOpts::default()).ok();

        // 13. Install MessageChannel
        self.eval(crate::shims::message_channel::MESSAGE_CHANNEL_JS, crate::kernel::EvalOpts::default()).ok();

        // 14. Install localStorage/sessionStorage
        self.eval(crate::shims::storage::STORAGE_JS, crate::kernel::EvalOpts::default()).ok();

        // 15. Install navigator.mimeTypes/plugins/connection + history
        self.eval(crate::shims::navigator_extras::NAVIGATOR_EXTRAS_JS, crate::kernel::EvalOpts::default()).ok();

        // 16. Install Tier 1 browser API surface stubs (empty constructors for typeof checks)
        // NOTE: DOM_PROTOTYPES_JS and ELEMENT_PROTOTYPES_JS removed — the ObjectTemplate
        // refactor (dom/template.rs) now handles the full prototype chain natively.
        // HTMLDivElement, HTMLElement, etc. are installed by install_dom_templates().
        self.eval(crate::shims::tier1_stubs::TIER1_STUBS_JS, crate::kernel::EvalOpts::default()).ok();

        // 17. Install timezone shim (override Intl.DateTimeFormat default timezone)
        {
            let tz = {
                let state = crate::state::RuntimeState::get(&self.isolate);
                state.environment.get_str("timezone").unwrap_or("UTC").to_string()
            };
            let tz_shim = format!(r#"
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
        try {{ Intl.DateTimeFormat = _wrappedDTF; }} catch(e) {{}}
    }}
}})();
"#, tz);
            self.eval(&tz_shim, crate::kernel::EvalOpts::default()).ok();
        }

        // 18. Install default empty document so document.* methods are always available
        self.set_document("<!DOCTYPE html><html><head></head><body></body></html>", None);

        // 19. Install document properties (cookie, referrer, hidden, visibilityState, DOM methods)
        self.eval(crate::shims::document_props::DOCUMENT_PROPS_JS, crate::kernel::EvalOpts::default()).ok();

        // 20. Install Canvas2D shim (after document.createElement is available)
        self.eval(crate::canvas::binding::CANVAS2D_SHIM_JS, crate::kernel::EvalOpts::default()).ok();
    }

    /// Install DOM FunctionTemplate hierarchy into the isolate.
    /// Called once after kernel creation.
    pub fn install_dom_templates(&mut self) {
        unsafe { self.isolate.enter(); }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);

            // Build templates
            let templates = crate::dom::template::build_dom_templates(scope);

            // Install constructor functions on global
            crate::dom::template::install_dom_constructors(scope, global, &templates);

            // Store in RuntimeState
            let state = crate::state::RuntimeState::get(&*scope);
            *state.dom_templates.borrow_mut() = Some(templates);
        }
        unsafe { self.isolate.exit(); }
    }

    /// Install environment fields into the V8 global object.
    /// Called once after kernel creation to populate navigator.*, screen.*, etc.
    /// Phase 1: static value injection (all 393 entries via env_inject)
    /// Phase 2: native getter override for key objects (navigator, screen)
    pub fn install_environment(&mut self) {
        unsafe { self.isolate.enter(); }
        {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let global = context.global(scope);
            // Phase 1: static injection (all 393 dot-path entries)
            crate::env_inject::install_environment(scope, global);
            // Phase 2: override navigator + screen with native-getter ObjectTemplates
            // This makes Object.getOwnPropertyDescriptor(navigator, 'userAgent')
            // return a native getter instead of a plain value descriptor.
            crate::shims::native_env::install_native_env(scope, global);
        }
        unsafe { self.isolate.exit(); }
    }

    /// Dispose the kernel (explicit cleanup before drop).
    pub fn dispose(&mut self) {
        let state = RuntimeState::get(&self.isolate);
        state.mark_disposed();
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
        }

        // Install V8 bindings
        self.with_global_scope(|scope, global| {
            crate::dom::binding::install_document_bindings(scope, global);
        });
        // NOTE: DOM_NAV_SHIM_JS removed — navigation properties (parentNode, childNodes, etc.)
        // are now native accessors on the ObjectTemplate prototype chain (dom/template.rs).

        // Re-install Canvas2D shim (DOM bindings may reset HTMLCanvasElement.prototype)
        self.eval(crate::canvas::binding::CANVAS2D_SHIM_JS, crate::kernel::EvalOpts::default()).ok();
    }

    /// Full page.load: parse HTML, install DOM, execute inline <script> tags,
    /// fire DOMContentLoaded event.
    pub fn page_load(&mut self, html: &str, base_url: Option<&str>) {
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
                let src = doc.get(nid).and_then(|n| n.value().get_attr("src")).map(|s| s.to_string());
                ScriptInfo {
                    inline: if inline.is_empty() { None } else { Some(inline) },
                    src,
                }
            })
            .collect();

        // 3. Store document in RuntimeState
        {
            let state = RuntimeState::get(&self.isolate);
            *state.document.borrow_mut() = Some(doc);
            state.node_cache.borrow_mut().clear();
        }

        // 4. Install DOM V8 bindings
        self.with_global_scope(|scope, global| {
            crate::dom::binding::install_document_bindings(scope, global);
        });

        // 4b. Re-install Canvas2D shim (DOM bindings may have reset HTMLCanvasElement.prototype)
        self.eval(crate::canvas::binding::CANVAS2D_SHIM_JS, crate::kernel::EvalOpts::default()).ok();

        // 4c. Install document.write workaround shim
        self.eval(DOCUMENT_WRITE_SHIM, crate::kernel::EvalOpts::default()).ok();

        // 4d. Re-install document properties (readyState, cookie, etc.)
        // These are reset when install_document_bindings creates a new document object
        self.eval(crate::shims::document_props::DOCUMENT_PROPS_JS, crate::kernel::EvalOpts::default()).ok();

        // 4e. Update location.href if base_url is provided
        if let Some(url) = base_url {
            let url_escaped = url.replace('\'', "\\'");
            let update_location = format!(r#"
(function() {{
    try {{
        var u = new URL('{}');
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
        location.href = '{}';
    }}
}})();
"#, url_escaped, url_escaped);
            self.eval(&update_location, crate::kernel::EvalOpts::default()).ok();
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
                        base_url_parsed.join(src).map(|u| u.to_string()).unwrap_or_else(|_| src.clone())
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
                    bundle.get(&resolved_url).map(|r| String::from_utf8_lossy(&r.body).to_string())
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
        self.eval("try { document.readyState = 'interactive'; } catch(e) {}", crate::kernel::EvalOpts::default()).ok();

        // 7. Fire DOMContentLoaded on document (simplified: dispatch on root)
        // In v0.1, we just advance the event loop slightly to process any pending microtasks
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
    ) -> String {
        let config = crate::inspector::session::InspectorConfig {
            port,
            watch_apis,
            enable_console,
        };

        let mut session = crate::inspector::session::InspectorSession::new(config);

        // Initialize inspector: create V8Inspector + session
        // Must be done with isolate entered but without an active scope
        unsafe { self.isolate.enter(); }

        // Step 1: Create inspector (needs &mut Isolate, no scope)
        let client = v8::inspector::V8InspectorClient::new(Box::new(
            crate::inspector::session::InspectorClientImpl::new(session.channel_state.clone())
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
            crate::inspector::session::InspectorChannelImpl::new(session.channel_state.clone())
        ));
        let state_str = v8::inspector::StringView::from(b"{}" as &[u8]);
        let v8_session = inspector.connect(
            1,
            channel,
            state_str,
            v8::inspector::V8InspectorClientTrustLevel::FullyTrusted,
        );

        session.set_inspector(inspector, v8_session);

        unsafe { self.isolate.exit(); }

        let devtools_url = session.devtools_url.clone();

        // Install vdebugger
        let vdebugger_js = crate::inspector::session::InspectorSession::vdebugger_js().to_string();
        self.eval(&vdebugger_js, crate::kernel::EvalOpts::default()).ok();

        // Install watch_apis
        if let Some(watch_js) = session.watch_apis_js() {
            self.eval(&watch_js, crate::kernel::EvalOpts::default()).ok();
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

        devtools_url
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
        unsafe { self.isolate.enter(); }
        let result = self.cdp_set_breakpoint_inner(url, line, column, condition);
        unsafe { self.isolate.exit(); }
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
        let session = session_guard.as_ref()
            .and_then(|s| s.session_ref())
            .ok_or_else(|| "Inspector not started. Call with_devtools() first.".to_string())?;
        let mut cdp = state.cdp_client.borrow_mut();
        let cdp = cdp.as_mut()
            .ok_or_else(|| "CDP client not initialized.".to_string())?;
        cdp.ensure_debugger_enabled(session);
        cdp.set_breakpoint_by_url(session, url, line, column, condition)
    }

    /// Remove a breakpoint by id.
    pub fn cdp_remove_breakpoint(&mut self, breakpoint_id: &str) -> Result<(), String> {
        unsafe { self.isolate.enter(); }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard.as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.remove_breakpoint(session, breakpoint_id)
        };
        unsafe { self.isolate.exit(); }
        result
    }

    /// Evaluate expression on a call frame while paused.
    pub fn cdp_evaluate_on_frame(
        &mut self,
        call_frame_id: &str,
        expression: &str,
    ) -> Result<serde_json::Value, String> {
        unsafe { self.isolate.enter(); }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard.as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.evaluate_on_call_frame(session, call_frame_id, expression)
        };
        unsafe { self.isolate.exit(); }
        result
    }

    /// Resume execution (after pause).
    pub fn cdp_resume(&mut self) -> Result<(), String> {
        unsafe { self.isolate.enter(); }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard.as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.resume(session)
        };
        unsafe { self.isolate.exit(); }
        result
    }

    /// Step over (after pause).
    pub fn cdp_step_over(&mut self) -> Result<(), String> {
        unsafe { self.isolate.enter(); }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard.as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.step_over(session)
        };
        unsafe { self.isolate.exit(); }
        result
    }

    /// Step into (after pause).
    pub fn cdp_step_into(&mut self) -> Result<(), String> {
        unsafe { self.isolate.enter(); }
        let result = {
            let state = RuntimeState::get(&self.isolate);
            let session_guard = state.inspector_session.borrow();
            let session = session_guard.as_ref()
                .and_then(|s| s.session_ref())
                .ok_or_else(|| "Inspector not started.".to_string())?;
            let cdp = state.cdp_client.borrow();
            let cdp = cdp.as_ref().ok_or("CDP client not initialized.")?;
            cdp.step_into(session)
        };
        unsafe { self.isolate.exit(); }
        result
    }

    /// Get call frames from last Debugger.paused event.
    pub fn cdp_get_call_frames(&self) -> Option<serde_json::Value> {
        let state = RuntimeState::get(&self.isolate);
        let cdp = state.cdp_client.borrow();
        cdp.as_ref().and_then(|c| c.get_call_frames().cloned())
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

    /// Eval and if the result is a Promise, await it by draining the event loop.
    /// Returns the resolved value or Null on rejection/timeout.
    pub fn eval_await(&mut self, source: &str, max_ticks: u32) -> Result<v8::Global<v8::Value>, crate::error::IV8Error> {
        let global = self.eval(source, crate::kernel::EvalOpts::default())?;

        // Check if result is a Promise
        let is_promise = {
            unsafe { self.isolate.enter(); }
            let result = {
                v8::scope!(handle_scope, &mut self.isolate);
                let context = v8::Local::new(handle_scope, &self.context);
                v8::scope_with_context!(scope, handle_scope, context);
                let local = v8::Local::new(scope, &global);
                local.is_promise()
            };
            unsafe { self.isolate.exit(); }
            result
        };

        if !is_promise {
            return Ok(global);
        }

        // It's a Promise — set up resolve/reject callbacks and drain the event loop
        let _resolved_value: std::cell::Cell<Option<v8::Global<v8::Value>>> = std::cell::Cell::new(None);
        let _rejected_value: std::cell::Cell<Option<v8::Global<v8::Value>>> = std::cell::Cell::new(None);

        // Use JS to attach .then/.catch and store the result
        let settle_script = r#"
(function(__promise__) {
    var __settled__ = false;
    var __result__ = undefined;
    var __error__ = undefined;
    __promise__.then(function(v) { __settled__ = true; __result__ = v; })
               .catch(function(e) { __settled__ = true; __error__ = e; });
    return { settled: function() { return __settled__; }, result: function() { return __result__; }, error: function() { return __error__; } };
})
"#;

        unsafe { self.isolate.enter(); }
        let tracker = {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            v8::tc_scope!(tc, scope);

            let promise_local = v8::Local::new(tc, &global);
            let fn_src = v8::String::new(tc, settle_script).expect("str");
            let script = v8::Script::compile(tc, fn_src, None).expect("compile");
            let fn_val = script.run(tc).expect("run");
            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(fn_val) };
            let undefined = v8::undefined(tc);
            let tracker = func.call(tc, undefined.into(), &[promise_local]).expect("call");
            v8::Global::new(tc, tracker)
        };
        unsafe { self.isolate.exit(); }

        // Drain microtasks + macrotasks until settled or max_ticks
        for _ in 0..max_ticks {
            self.drain_microtasks();

            // Check if settled
            let settled = {
                unsafe { self.isolate.enter(); }
                let result = {
                    v8::scope!(handle_scope, &mut self.isolate);
                    let context = v8::Local::new(handle_scope, &self.context);
                    v8::scope_with_context!(scope, handle_scope, context);
                    v8::tc_scope!(tc, scope);
                    let tracker_local = v8::Local::new(tc, &tracker);
                    let tracker_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(tracker_local) };
                    let settled_key = v8::String::new(tc, "settled").expect("key");
                    if let Some(settled_fn) = tracker_obj.get(tc, settled_key.into()) {
                        if settled_fn.is_function() {
                            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(settled_fn) };
                            let undefined = v8::undefined(tc);
                            if let Some(result) = func.call(tc, undefined.into(), &[]) {
                                result.is_true()
                            } else { false }
                        } else { false }
                    } else { false }
                };
                unsafe { self.isolate.exit(); }
                result
            };

            if settled {
                // Extract the result
                unsafe { self.isolate.enter(); }
                let result = {
                    v8::scope!(handle_scope, &mut self.isolate);
                    let context = v8::Local::new(handle_scope, &self.context);
                    v8::scope_with_context!(scope, handle_scope, context);
                    v8::tc_scope!(tc, scope);
                    let tracker_local = v8::Local::new(tc, &tracker);
                    let tracker_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(tracker_local) };
                    let result_key = v8::String::new(tc, "result").expect("key");
                    if let Some(result_fn) = tracker_obj.get(tc, result_key.into()) {
                        if result_fn.is_function() {
                            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(result_fn) };
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
                unsafe { self.isolate.exit(); }
                return Ok(result);
            }

            // Advance event loop by one tick to process pending timers
            let _ = self.eval("__iv8__.eventLoop.tick()", crate::kernel::EvalOpts::default());
        }

        // Timed out — return undefined
        unsafe { self.isolate.enter(); }
        let result = {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            let undef: v8::Local<v8::Value> = v8::undefined(scope).into();
            v8::Global::new(scope, undef)
        };
        unsafe { self.isolate.exit(); }
        Ok(result)
    }

    /// Convert a V8 Global<Value> to RustValue using this kernel's context.
    pub fn global_to_rust_value(&mut self, global: &v8::Global<v8::Value>) -> crate::convert::RustValue {
        unsafe { self.isolate.enter(); }
        let result = {
            v8::scope!(handle_scope, &mut self.isolate);
            let context = v8::Local::new(handle_scope, &self.context);
            v8::scope_with_context!(scope, handle_scope, context);
            v8::tc_scope!(tc, scope);
            let local = v8::Local::new(tc, global);
            crate::convert::v8_to_rust_impl(tc, local, 0)
        };
        unsafe { self.isolate.exit(); }
        result
    }
}

impl Drop for EmbeddedV8Kernel {
    fn drop(&mut self) {
        // Re-enter the isolate before drop — OwnedIsolate expects to be entered
        // SAFETY: we exited after new(), now re-enter for proper cleanup
        unsafe { self.isolate.enter(); }
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
        assert_eq!(result, crate::RustValue::Int(3));
    }

    #[test]
    fn kernel_eval_string() {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        let result = kernel.eval_to_rust_value("'hello' + ' world'");
        assert_eq!(result, crate::RustValue::String("hello world".into()));
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
        assert_eq!(result, crate::RustValue::Int(43));
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
}
