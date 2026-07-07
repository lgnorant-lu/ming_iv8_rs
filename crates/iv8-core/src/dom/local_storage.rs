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
use std::path::Path;
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
        serde_json::to_string(&*map).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let map = self.data.lock().unwrap();
        let json = serde_json::to_string(&*map).unwrap_or_else(|_| "{}".to_string());
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, json)
    }

    pub fn load_from_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let map: HashMap<String, String> =
            serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new());
        let mut data = self.data.lock().unwrap();
        *data = map;
        Ok(())
    }
}

impl Default for LocalStorageStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_object_empty() {
        let store = LocalStorageStore::new();
        assert_eq!(store.to_json_object(), "{}");
    }

    #[test]
    fn test_to_json_object_simple() {
        let store = LocalStorageStore::new();
        store.set("key".into(), "value".into());
        assert_eq!(store.to_json_object(), "{\"key\":\"value\"}");
    }

    #[test]
    fn test_to_json_object_special_chars() {
        let store = LocalStorageStore::new();
        store.set("k\nnewline".into(), "val\"quote".into());
        let json = store.to_json_object();
        // serde_json properly escapes control characters and quotes
        assert!(json.contains("\\n"));
        assert!(json.contains("\\\""));
    }

    #[test]
    fn test_to_json_object_multiple() {
        let store = LocalStorageStore::new();
        store.set("a".into(), "1".into());
        store.set("b".into(), "2".into());
        let json = store.to_json_object();
        assert!(json.contains("\"a\":\"1\""));
        assert!(json.contains("\"b\":\"2\""));
    }
}
