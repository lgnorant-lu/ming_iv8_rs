//! localStorage cross-kernel persistence backend.
//!
//! `LocalStorageStore` is a thread-safe shared map that can be passed to
//! multiple `EmbeddedV8Kernel` instances. Kernels that share the same store
//! see the same localStorage data (in-memory only; no filesystem).
//!
//! sessionStorage remains session-scoped JS-only (cleared on page unload).
//!
//! Track A of v0.8.72.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct LocalStorageStore {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl LocalStorageStore {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        self.data.lock().unwrap().insert(key, value);
    }

    pub fn remove(&self, key: &str) {
        self.data.lock().unwrap().remove(key);
    }

    pub fn clear(&self) {
        self.data.lock().unwrap().clear();
    }

    pub fn replace_all(&self, map: HashMap<String, String>) {
        let mut data = self.data.lock().unwrap();
        *data = map;
    }

    pub fn to_json_object(&self) -> String {
        let map = self.data.lock().unwrap();
        let parts: Vec<String> = map
            .iter()
            .map(|(k, v)| {
                format!(
                    "\"{}\":\"{}\"",
                    k.replace('\\', "\\\\").replace('"', "\\\""),
                    v.replace('\\', "\\\\").replace('"', "\\\"")
                )
            })
            .collect();
        format!("{{{}}}", parts.join(","))
    }
}

impl Default for LocalStorageStore {
    fn default() -> Self {
        Self::new()
    }
}
