//! Configuration and environment map for iv8-rs.
//!
//! `EnvironmentMap` holds 393 dot-path → value entries representing the
//! browser fingerprint (navigator.*, screen.*, webgl.*, etc.).
#![expect(clippy::expect_used, reason = "serde_json::from_str on compile-time embedded JSON")]

use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// The 393 default entries embedded at compile time from iv8-defaults.json.
const DEFAULTS_JSON: &str = include_str!("../../../docs/research/iv8-defaults.json");

/// Browser environment configuration (357 environment + 36 config entries).
/// Immutable after construction.
#[derive(Debug, Clone)]
pub struct EnvironmentMap {
    entries: HashMap<String, JsonValue>,
}

impl EnvironmentMap {
    /// Build from defaults, optionally overriding with user-provided entries.
    /// `user_overrides` is a flat map of dot-path → value.
    pub fn build(user_overrides: Option<&HashMap<String, JsonValue>>) -> Self {
        // SAFETY: DEFAULTS_JSON is compile-time embedded; build breaks if invalid
        let mut entries: HashMap<String, JsonValue> = serde_json::from_str(DEFAULTS_JSON)
            .expect("iv8-defaults.json is invalid JSON");

        if let Some(overrides) = user_overrides {
            for (key, value) in overrides {
                entries.insert(key.clone(), value.clone());
            }
        }

        Self { entries }
    }

    /// Build with only defaults (no overrides).
    pub fn defaults() -> Self {
        Self::build(None)
    }

    /// Get a value by dot-path.
    pub fn get(&self, path: &str) -> Option<&JsonValue> {
        self.entries.get(path)
    }

    /// Get a string value by dot-path.
    pub fn get_str(&self, path: &str) -> Option<&str> {
        self.entries.get(path).and_then(|v| v.as_str())
    }

    /// Get a float value by dot-path.
    pub fn get_f64(&self, path: &str) -> Option<f64> {
        self.entries.get(path).and_then(|v| v.as_f64())
    }

    /// Get a bool value by dot-path.
    pub fn get_bool(&self, path: &str) -> Option<bool> {
        self.entries.get(path).and_then(|v| v.as_bool())
    }

    /// Get the total number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the full entries map (for get_defaults() Python API).
    pub fn as_map(&self) -> &HashMap<String, JsonValue> {
        &self.entries
    }

    /// Iterate over all entries.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &JsonValue)> {
        self.entries.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_loads_393_entries() {
        let env = EnvironmentMap::defaults();
        // iv8 0.1.2 has 393 entries
        assert!(
            env.len() >= 390,
            "expected ~393 defaults, got {}",
            env.len()
        );
    }

    #[test]
    fn defaults_has_navigator_user_agent() {
        let env = EnvironmentMap::defaults();
        let ua = env.get_str("navigator.userAgent");
        assert!(ua.is_some(), "navigator.userAgent should exist");
        assert!(
            ua.unwrap().contains("Chrome"),
            "UA should contain Chrome: {:?}",
            ua
        );
    }

    #[test]
    fn defaults_has_screen_width() {
        let env = EnvironmentMap::defaults();
        let width = env.get_f64("screen.width");
        assert!(width.is_some(), "screen.width should exist");
        assert!(width.unwrap() > 0.0, "screen.width should be positive");
    }

    #[test]
    fn user_override_replaces_default() {
        let mut overrides = HashMap::new();
        overrides.insert(
            "navigator.userAgent".to_string(),
            JsonValue::String("CustomUA/1.0".to_string()),
        );
        let env = EnvironmentMap::build(Some(&overrides));
        assert_eq!(env.get_str("navigator.userAgent").unwrap(), "CustomUA/1.0");
    }

    #[test]
    fn user_override_adds_new_key() {
        let mut overrides = HashMap::new();
        overrides.insert(
            "custom.new.key".to_string(),
            JsonValue::String("hello".to_string()),
        );
        let env = EnvironmentMap::build(Some(&overrides));
        assert_eq!(env.get_str("custom.new.key").unwrap(), "hello");
        // Original defaults still present
        assert!(env.get_str("navigator.userAgent").is_some());
    }
}
