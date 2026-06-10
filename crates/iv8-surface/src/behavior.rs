//! BehaviorCallbackRegistry — deep behavior callbacks for generated stubs.
//!
//! Stores optional callback functions that override the default stub behavior
//! for specific interface members. When a callback is registered, the generated
//! getter/setter/method stub delegates to it instead of returning the default value.
//!
//! v0.8.19: empty stub — registry exists but no callbacks are registered.
//! v0.8.20: filled during BrowserSurface integration.

pub struct BehaviorCallbackRegistry {}

impl BehaviorCallbackRegistry {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BehaviorCallbackRegistry {
    fn default() -> Self {
        Self::new()
    }
}
