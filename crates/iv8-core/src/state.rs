//! RuntimeState: per-isolate single container installed via `Isolate::set_slot<T>`.
//!
//! All V8 callbacks access shared state through `RuntimeState::get(isolate)`.
//! Internal mutability via `RefCell` (V8 Isolate is single-threaded, no Mutex needed).
#![expect(
    clippy::expect_used,
    reason = "get_slot: RuntimeState must be installed before use"
)]

use std::cell::RefCell;
use std::sync::Arc;

use crate::config::EnvironmentMap;
use crate::dom::template::DomTemplates;
use crate::dom::Document;
use crate::events::{EventListenerRegistry, EventLoop};
use crate::network::ResourceBundle;
use crate::shims::console::ConsoleMessage;

/// Network handler callback type.
/// Called when fetch/XHR requests a URL not in ResourceBundle.
/// Returns Some((status, headers, body)) or None to reject.
pub type NetworkHandler = Box<dyn Fn(&str, &str) -> Option<(u16, Vec<u8>)> + Send + 'static>;

/// Per-isolate runtime state. Installed once at JSContext creation,
/// accessed from every V8 callback via `RuntimeState::get(isolate)`.
pub struct RuntimeState {
    /// Immutable after construction
    pub strict_compat: bool,
    pub time_mode: TimeMode,
    pub js_api_name: String,
    /// Browser environment (navigator.*, screen.*, etc.) — accessible from V8 callbacks
    pub environment: Arc<EnvironmentMap>,

    /// Active browser identity profile. When set, native getters read from this
    /// before falling back to EnvironmentMap and DEFAULT_PROFILE.
    /// Injected via KernelConfig::with_browser_profile().
    pub profile: Option<&'static crate::shims::browser_profile::BrowserProfile>,

    /// Mutable subsystems (RefCell for interior mutability in single-threaded V8)
    pub eval_count: RefCell<u64>,
    pub disposed: RefCell<bool>,

    /// DOM document (populated by page.load or set_document)
    pub document: RefCell<Option<Document>>,

    /// Event loop (macrotask queue + logical time)
    pub event_loop: RefCell<EventLoop>,

    /// Event listener registry (addEventListener/removeEventListener)
    pub event_listeners: RefCell<EventListenerRegistry>,

    /// Resource bundle (pre-registered HTTP responses)
    pub resource_bundle: RefCell<ResourceBundle>,

    /// DOM node identity cache: same NodeId → same V8 object.
    /// Uses v8::Global (strong reference) to avoid V8 weak handle callback
    /// panic (K-018). Objects in cache prevent GC; cleared on page_load
    /// and periodically via sweep. Memory overhead is bounded because
    /// cache is cleared on navigation.
    pub node_cache: RefCell<std::collections::HashMap<crate::dom::NodeId, v8::Global<v8::Object>>>,

    /// Attr identity cache: (owner Element NodeId, attr name) → Attr object.
    /// Value is live from Element.attrs; cache only stabilizes object identity.
    pub attr_cache:
        RefCell<std::collections::HashMap<(crate::dom::NodeId, String), v8::Global<v8::Object>>>,

    /// Lazy sweep operation counter for periodic full cache sweep.
    pub node_cache_ops: std::cell::Cell<u32>,

    /// Threshold for periodic full sweep (default: 500 operations).
    /// When ops reach this, stale entries (dead nodes) are removed.
    pub node_cache_sweep_threshold: u32,

    /// CSSStyleDeclaration instance cache per element node
    pub style_cache: RefCell<std::collections::HashMap<crate::dom::NodeId, v8::Global<v8::Object>>>,

    /// DOM FunctionTemplate hierarchy (built once per isolate)
    pub dom_templates: RefCell<Option<DomTemplates>>,

    /// BrowserSurface registry — new init chain
    pub surface_registry: RefCell<Option<iv8_surface::BrowserSurfaceRegistry>>,

    /// Behavior callbacks for generated stubs
    pub behavior_callbacks: RefCell<Option<iv8_surface::BehaviorCallbackRegistry>>,

    /// Console messages captured from JS console.log/warn/error etc.
    pub console_messages: RefCell<Vec<ConsoleMessage>>,

    /// Optional Python network handler for fetch/XHR fallback.
    /// Called when URL is not in ResourceBundle.
    pub network_handler: RefCell<Option<NetworkHandler>>,

    /// Optional V8 Inspector session (CDP debugging).
    pub inspector_session: RefCell<Option<crate::inspector::session::InspectorSession>>,

    /// Optional CDP programmatic client (Python-driven Inspector control).
    pub cdp_client: RefCell<Option<crate::inspector::CdpClient>>,

    /// Optional crypto seed for deterministic crypto.getRandomValues.
    pub crypto_seed: RefCell<Option<u64>>,

    /// Canvas 2D instances keyed by canvas ID (for toDataURL/getImageData).
    pub canvases: RefCell<std::collections::HashMap<String, crate::canvas::canvas2d::Canvas2D>>,

    /// Heap registries for Box allocations stored via External pointers.
    /// Each entry: (pointer, free function). Freed on RuntimeState drop.
    pub heap_registry: RefCell<Vec<(*mut std::ffi::c_void, fn(*mut std::ffi::c_void))>>,

    /// Shared cross-kernel localStorage backend.
    pub local_storage: RefCell<Option<crate::dom::local_storage::LocalStorageStore>>,

    /// Global FunctionTemplates from install_all, stored for post-install
    /// prototype chain fixes (e.g., Navigator inherits EventTarget).
    pub codegen_templates: RefCell<std::collections::HashMap<String, v8::Global<v8::FunctionTemplate>>>,

    /// performance.memory: per-page stable, quantized heap snapshot.
    /// Initialized lazily on first access. Quantized to 100KB (102400 bytes)
    /// multiples to avoid per-call-varying unbucketed values (bot-tell).
    /// See date_interceptor.rs::install_performance_memory.
    pub perf_memory: RefCell<Option<PerformanceMemory>>,

    /// performance.now() monotonic jitter state.
    /// Tracks the last returned value to guarantee monotonicity while adding
    /// a small sub-millisecond increment so consecutive samples in a tight
    /// loop do not return identical diffs (bot-tell). See
    /// date_interceptor.rs::performance_now_callback.
    pub perf_now_last: std::cell::Cell<f64>,

    pub workers: RefCell<Vec<crate::shims::worker::WorkerHandle>>,
    pub worker_objects: RefCell<std::collections::HashMap<u64, v8::Global<v8::Object>>>,

    /// Per-node layout/media mutable slots (scrollTop/Left, media currentTime/muted/volume).
    /// Keyed by NodeId; used by DOM FT setters (COMP-3) instead of no-op null_this_check.
    pub node_scroll: RefCell<std::collections::HashMap<crate::dom::NodeId, (f64, f64)>>,
    pub node_media: RefCell<std::collections::HashMap<crate::dom::NodeId, MediaNodeState>>,
    /// Element.attachShadow mode per node ("open"/"closed"); absence ⇒ shadowRoot null.
    pub node_shadow: RefCell<std::collections::HashMap<crate::dom::NodeId, String>>,

    /// K-ESM-LOADER: import map `imports` bare-specifier → absolute URL.
    pub esm_import_map: RefCell<std::collections::HashMap<String, String>>,
    /// K-ESM-LOADER: V8 Module identity_hash → resolved module URL (for relative import).
    pub esm_module_urls: RefCell<std::collections::HashMap<i32, String>>,

    /// Cached navigator.plugins / mimeTypes objects (SameObject).
    /// Without this, each getter access rebuilds a bare Array and drops length.
    pub plugins_array: RefCell<Option<v8::Global<v8::Object>>>,
    pub mime_types_array: RefCell<Option<v8::Global<v8::Object>>>,
}

/// Mutable media element state for HTMLMediaElement-backed FTs.
#[derive(Debug, Clone, Copy)]
pub struct MediaNodeState {
    pub current_time: f64,
    pub muted: bool,
    pub volume: f64,
}

impl Default for MediaNodeState {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            muted: false,
            volume: 1.0,
        }
    }
}

/// Quantized, per-page-stable `performance.memory` snapshot.
///
/// Real Chrome exposes `performance.memory` (non-standard) returning
/// per-call-varying unbucketed heap values derived from V8 Isolate
/// statistics. Returning precise bytes that change every call is a
/// fingerprinting/bot-tell signal. Instead we expose fixed values
/// quantized to 100KB (102400-byte) buckets that remain stable across
/// all calls within a page session.
#[derive(Debug, Clone, Copy)]
pub struct PerformanceMemory {
    /// jsHeapSizeLimit (bytes), multiple of 102400.
    pub js_heap_size_limit: u64,
    /// totalJSHeapSize (bytes), multiple of 102400, <= limit.
    pub total_js_heap_size: u64,
    /// usedJSHeapSize (bytes), multiple of 102400, <= total.
    pub used_js_heap_size: u64,
}

impl PerformanceMemory {
    /// 100KB bucket size in bytes.
    pub const BUCKET: u64 = 102_400;

    /// Quantize a byte count down to the nearest 100KB bucket (>=1 bucket).
    pub fn quantize(bytes: u64) -> u64 {
        let b = (bytes / Self::BUCKET).max(1);
        b * Self::BUCKET
    }

    /// Default per-page-stable quantized snapshot.
    /// Targets: limit ~400MB, total ~10MB, used ~5MB — all snapped to 100KB.
    pub fn default_quantized() -> Self {
        let limit = Self::quantize(419_430_400); // 4096 buckets = 400MB
        let total = Self::quantize(10_485_760); // 102 buckets ≈ 9.77MB
        let used = Self::quantize(5_242_880); // 51 buckets ≈ 4.88MB
        debug_assert!(total <= limit);
        debug_assert!(used <= total);
        Self {
            js_heap_size_limit: limit,
            total_js_heap_size: total,
            used_js_heap_size: used,
        }
    }
}

/// Time mode for the JS context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeMode {
    /// Virtual clock, only advanced by eventLoop.advance/sleep/tick
    Logical,
    /// Real wall clock (Date.now() reflects actual time)
    System,
}

impl RuntimeState {
    /// Create a new RuntimeState with the given configuration.
    pub fn new(
        strict_compat: bool,
        time_mode: TimeMode,
        js_api_name: String,
        environment: Arc<EnvironmentMap>,
        profile: Option<&'static crate::shims::browser_profile::BrowserProfile>,
        local_storage: Option<crate::dom::local_storage::LocalStorageStore>,
    ) -> Self {
        crate::telemetry::state_created(strict_compat, &format!("{:?}", time_mode), &js_api_name, environment.len());
        Self {
            strict_compat,
            time_mode,
            js_api_name,
            environment,
            profile,
            eval_count: RefCell::new(0),
            disposed: RefCell::new(false),
            document: RefCell::new(None),
            event_loop: RefCell::new(EventLoop::new()),
            event_listeners: RefCell::new(EventListenerRegistry::new()),
            resource_bundle: RefCell::new(ResourceBundle::new()),
            node_cache: RefCell::new(std::collections::HashMap::new()),
            attr_cache: RefCell::new(std::collections::HashMap::new()),
            node_cache_ops: std::cell::Cell::new(0),
            node_cache_sweep_threshold: 500,
            style_cache: RefCell::new(std::collections::HashMap::new()),
            dom_templates: RefCell::new(None),
            surface_registry: RefCell::new(None),
            behavior_callbacks: RefCell::new(None),
            console_messages: RefCell::new(Vec::new()),
            network_handler: RefCell::new(None),
            inspector_session: RefCell::new(None),
            cdp_client: RefCell::new(None),
            crypto_seed: RefCell::new(None),
            canvases: RefCell::new(std::collections::HashMap::new()),
            heap_registry: RefCell::new(Vec::new()),
            local_storage: RefCell::new(local_storage),
            codegen_templates: RefCell::new(std::collections::HashMap::new()),
            perf_memory: RefCell::new(None),
            perf_now_last: std::cell::Cell::new(f64::NEG_INFINITY),
            workers: RefCell::new(Vec::new()),
            worker_objects: RefCell::new(std::collections::HashMap::new()),
            node_scroll: RefCell::new(std::collections::HashMap::new()),
            node_media: RefCell::new(std::collections::HashMap::new()),
            node_shadow: RefCell::new(std::collections::HashMap::new()),
            esm_import_map: RefCell::new(std::collections::HashMap::new()),
            esm_module_urls: RefCell::new(std::collections::HashMap::new()),
            plugins_array: RefCell::new(None),
            mime_types_array: RefCell::new(None),
        }
    }

    /// Install this RuntimeState into the given V8 Isolate via `set_slot`.
    /// Panics (debug) if a RuntimeState is already installed.
    pub fn install(isolate: &mut v8::Isolate, state: Self) {
        let is_new = isolate.set_slot(state);
        debug_assert!(is_new, "RuntimeState already installed on this isolate");
    }

    /// Get a reference to the RuntimeState from an Isolate.
    /// Panics if not installed (programming error).
    pub fn get(isolate: &v8::Isolate) -> &Self {
        // SAFETY: get_slot only fails if state not installed (programming error)
        isolate
            .get_slot::<Self>()
            .expect("RuntimeState not installed on this isolate")
    }

    /// Returns true if a RuntimeState is installed on the given Isolate.
    /// Useful when conversion code may run before/without a RuntimeState
    /// (e.g. low-level test contexts).
    pub fn has(isolate: &v8::Isolate) -> bool {
        isolate.get_slot::<Self>().is_some()
    }

    /// Increment eval counter and return new count.
    pub fn increment_eval_count(&self) -> u64 {
        let mut count = self.eval_count.borrow_mut();
        *count += 1;
        *count
    }

    /// Mark as disposed.
    pub fn mark_disposed(&self) {
        *self.disposed.borrow_mut() = true;
    }

    /// Check if disposed.
    pub fn is_disposed(&self) -> bool {
        *self.disposed.borrow()
    }

    /// Register a heap allocation for cleanup when RuntimeState drops.
    /// Used for Box-allocated data stored in V8 External pointers.
    pub fn register_heap(&self, ptr: *mut std::ffi::c_void, free_fn: fn(*mut std::ffi::c_void)) {
        self.heap_registry.borrow_mut().push((ptr, free_fn));
    }
}

impl Drop for RuntimeState {
    fn drop(&mut self) {
        crate::telemetry::state_dropped(*self.eval_count.borrow());
        for (ptr, free_fn) in self.heap_registry.borrow_mut().drain(..) {
            free_fn(ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_state_install_and_get() {
        // Initialize V8
        crate::v8_init::ensure_v8_initialized();

        let mut isolate = v8::Isolate::new(v8::CreateParams::default());

        // Install RuntimeState
        RuntimeState::install(
            &mut isolate,
            RuntimeState::new(
                true,
                TimeMode::Logical,
                "__iv8__".to_string(),
                std::sync::Arc::new(crate::config::EnvironmentMap::defaults()),
                None,
                None,
            ),
        );

        // Get it back
        let state = RuntimeState::get(&isolate);
        assert!(state.strict_compat);
        assert_eq!(state.time_mode, TimeMode::Logical);
        assert_eq!(state.js_api_name, "__iv8__");
        assert_eq!(*state.eval_count.borrow(), 0);
        assert!(!state.is_disposed());

        // Increment eval count
        assert_eq!(state.increment_eval_count(), 1);
        assert_eq!(state.increment_eval_count(), 2);
        assert_eq!(*state.eval_count.borrow(), 2);

        // Mark disposed
        state.mark_disposed();
        assert!(state.is_disposed());
    }

    #[test]
    fn runtime_state_drops_with_isolate() {
        crate::v8_init::ensure_v8_initialized();

        {
            let mut isolate = v8::Isolate::new(v8::CreateParams::default());
            RuntimeState::install(
                &mut isolate,
                RuntimeState::new(
                    false,
                    TimeMode::System,
                    "__test__".to_string(),
                    std::sync::Arc::new(crate::config::EnvironmentMap::defaults()),
                    None,
                    None,
                ),
            );
            let state = RuntimeState::get(&isolate);
            state.increment_eval_count();
            // isolate drops here → RuntimeState::drop called → tracing::info logged
        }
        // If we reach here without crash, drop worked correctly
    }
}
