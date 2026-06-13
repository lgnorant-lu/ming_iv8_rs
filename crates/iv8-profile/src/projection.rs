use std::collections::HashMap;

/// Flat dot-path compatibility map for existing `EnvironmentMap` consumers.
///
/// Keyed by the same dot-path convention used by `env_inject.rs` and
/// `RuntimeState.environment`.
#[derive(Clone, Debug)]
pub struct EnvironmentProjection {
    pub entries: HashMap<String, serde_json::Value>,
}

impl EnvironmentProjection {
    pub fn new(entries: HashMap<String, serde_json::Value>) -> Self {
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.entries.get(key)
    }

    pub fn into_inner(self) -> HashMap<String, serde_json::Value> {
        self.entries
    }
}
