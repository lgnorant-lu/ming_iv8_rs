//! User-defined property overrides for the V8 global object.
//!
//! install_user_overrides() allows Python API users to override browser
//! environment properties (navigator.userAgent, screen.width, etc.) with
//! custom values. Overrides are applied after DOM template installation
//! and deterministic overrides, giving them the highest priority.

use serde_json::Value as JsonValue;

/// A single user-defined override value.
#[derive(Debug, Clone)]
pub enum OverrideValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<OverrideValue>),
    Object(Vec<(String, OverrideValue)>),
    Null,
}

/// Collection of user-defined property overrides.
///
/// Each entry maps a dot-separated path (e.g. "navigator.userAgent")
/// to an OverrideValue.
#[derive(Debug, Clone, Default)]
pub struct UserOverrides {
    entries: Vec<(String, OverrideValue)>,
}

impl UserOverrides {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Add an override from a path and JSON value.
    pub fn insert(&mut self, path: &str, value: OverrideValue) {
        self.entries.push((path.to_string(), value));
    }

    /// Build overrides from a JSON dictionary. Non-object JSON is ignored.
    pub fn from_json_value(value: &JsonValue) -> Self {
        let mut overrides = Self::new();
        if let Some(obj) = value.as_object() {
            for (k, v) in obj {
                overrides.insert(k, json_to_override(v));
            }
        }
        overrides
    }
}

fn json_to_override(value: &JsonValue) -> OverrideValue {
    match value {
        JsonValue::String(s) => OverrideValue::String(s.clone()),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_f64() {
                OverrideValue::Number(i)
            } else {
                OverrideValue::Null
            }
        }
        JsonValue::Bool(b) => OverrideValue::Boolean(*b),
        JsonValue::Array(arr) => {
            OverrideValue::Array(arr.iter().map(json_to_override).collect())
        }
        JsonValue::Object(obj) => {
            OverrideValue::Object(
                obj.iter()
                    .map(|(k, v)| (k.clone(), json_to_override(v)))
                    .collect(),
            )
        }
        JsonValue::Null => OverrideValue::Null,
    }
}

/// Install user-defined property overrides on the V8 global object.
///
/// Must be called AFTER install_deterministic_overrides to ensure
/// user overrides take the highest priority.
///
/// For each override:
/// 1. Parse the dot-separated path to locate the target object
/// 2. Delete any existing property (accessor or data) on the target
/// 3. Set the new value as a writable, enumerable, configurable data property
pub fn install_user_overrides(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    overrides: &UserOverrides,
) {
    if overrides.is_empty() {
        return;
    }

    for (path, value) in &overrides.entries {
        let segments: Vec<&str> = path.split('.').collect();
        if segments.is_empty() {
            continue;
        }
        // Reject empty segments (e.g. "a..b") and prototype-chain paths
        if segments.iter().any(|s| s.is_empty()) {
            continue;
        }
        if segments.iter().any(|s| {
            *s == "__proto__" || *s == "constructor" || *s == "prototype"
        }) {
            continue;
        }

        let last_idx = segments.len() - 1;
        let mut current = global;

        for (i, &seg) in segments.iter().enumerate() {
            let key = match v8::String::new(scope, seg) {
                Some(s) => s,
                None => break,
            };
            if i == last_idx {
                let v8_value = override_value_to_v8(scope, value);
                let _ = current.delete(scope, key.into());
                current.set(scope, key.into(), v8_value);
            } else {
                let next_val = current.get(scope, key.into());
                if let Some(next_val) = next_val {
                    if let Ok(next_obj) = next_val.try_into() {
                        current = next_obj;
                        continue;
                    }
                }
                // Path segment not found or not an object — skip this override
                break;
            }
        }
    }
}

fn override_value_to_v8<'s>(
    scope: &v8::PinScope<'s, '_>,
    value: &OverrideValue,
) -> v8::Local<'s, v8::Value> {
    match value {
        OverrideValue::String(s) => v8::String::new(scope, s)
            .map(|v| v.into())
            .unwrap_or_else(|| v8::undefined(scope).into()),
        OverrideValue::Number(n) => v8::Number::new(scope, *n).into(),
        OverrideValue::Boolean(b) => v8::Boolean::new(scope, *b).into(),
        OverrideValue::Null => v8::null(scope).into(),
        OverrideValue::Array(items) => {
            let arr = v8::Array::new(scope, items.len() as i32);
            for (i, item) in items.iter().enumerate() {
                let v = override_value_to_v8(scope, item);
                arr.set_index(scope, i as u32, v);
            }
            arr.into()
        }
        OverrideValue::Object(entries) => {
            let obj = v8::Object::new(scope);
            for (k, v) in entries {
                let key = match v8::String::new(scope, k) {
                    Some(s) => s,
                    None => continue,
                };
                let val = override_value_to_v8(scope, v);
                obj.set(scope, key.into(), val);
            }
            obj.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_override_value_conversions() {
        assert!(matches!(json_to_override(&JsonValue::String("hi".into())), OverrideValue::String(_)));
        assert!(matches!(json_to_override(&JsonValue::Number(serde_json::Number::from_f64(42.0).unwrap())), OverrideValue::Number(_)));
        assert!(matches!(json_to_override(&JsonValue::Bool(true)), OverrideValue::Boolean(_)));
        assert!(matches!(json_to_override(&JsonValue::Null), OverrideValue::Null));
    }

    #[test]
    fn test_user_overrides_from_json() {
        let json: JsonValue = serde_json::json!({
            "navigator.userAgent": "TestUA/1.0",
            "screen.width": 1920,
            "navigator.languages": ["zh-CN", "en-US"]
        });
        let overrides = UserOverrides::from_json_value(&json);
        assert_eq!(overrides.len(), 3);
    }

    #[test]
    fn test_empty_overrides() {
        let overrides = UserOverrides::new();
        assert!(overrides.is_empty());
        assert_eq!(overrides.len(), 0);
    }
}
