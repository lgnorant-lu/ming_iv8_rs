use std::collections::HashMap;

use crate::matrix::ProfileMatrix;

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

    /// Build a projection from a materialized ProfileMatrix.
    ///
    /// The projection entries match the dot-path convention used by
    /// `iv8-core`'s `EnvironmentMap` and `env_inject.rs`.
    pub fn from_matrix(matrix: &ProfileMatrix) -> Self {
        Self {
            entries: matrix.flat_env.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defaults::default_profile_source;

    #[test]
    fn projection_from_roundtrip() {
        let source = default_profile_source();
        let (matrix, _) = ProfileMatrix::from_source(&source);
        let projection = EnvironmentProjection::from_matrix(&matrix);
        assert_eq!(projection.len(), matrix.flat_env.len());
        assert_eq!(
            projection
                .get("navigator.userAgent")
                .and_then(|v| v.as_str()),
            Some(source.navigator.user_agent.as_str())
        );
    }

    #[test]
    fn projection_key_consistency() {
        let source = default_profile_source();
        let (matrix, _) = ProfileMatrix::from_source(&source);
        let projection = EnvironmentProjection::from_matrix(&matrix);
        let expected_keys = [
            "navigator.userAgent",
            "navigator.platform",
            "screen.width",
            "screen.height",
        ];
        for key in &expected_keys {
            assert!(
                projection.get(key).is_some(),
                "missing expected key: {}",
                key
            );
        }
    }
}
