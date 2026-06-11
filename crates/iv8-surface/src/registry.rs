//! BrowserSurfaceRegistry — tracks generated FunctionTemplate instances.
//!
//! Provides a HashMap-based registry for looking up generated FunctionTemplates
//! by interface name. Used by code that needs to reference generated templates
//! (e.g. type conversion, inter-interface references).
//!
//! v0.8.19: stub — registry is instantiated but not populated with callbacks.

use std::collections::HashMap;
use v8::Global;
use v8::FunctionTemplate;

/// Tracks generated FunctionTemplate instances by interface name.
pub struct SurfaceRegistry {
    templates: HashMap<String, Global<FunctionTemplate>>,
    count: usize,
}

/// Alias for public API surface.
pub type BrowserSurfaceRegistry = SurfaceRegistry;

impl SurfaceRegistry {
    pub fn new() -> Self {
        Self {
            templates: HashMap::with_capacity(1024),
            count: 0,
        }
    }

    pub fn set_count(&mut self, n: usize) {
        self.count = n;
    }

    pub fn register(&mut self, name: &str, tmpl: Global<FunctionTemplate>) {
        self.templates.insert(name.to_string(), tmpl);
    }

    pub fn get(&self, name: &str) -> Option<&Global<FunctionTemplate>> {
        self.templates.get(name)
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    /// Return the number of registered templates (interface count).
    pub fn interface_count(&self) -> usize {
        if self.count > 0 { self.count } else { self.templates.len() }
    }
}

impl Default for SurfaceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
