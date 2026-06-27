//! IR data structures — deserialize unified_ir.json from v0.8.18.
//!
//! Uses serde_json::Value to handle structural variations in the IR data,
//! then extracts typed data through helper functions.

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Definition {
    pub kind: String,
    pub name: Option<String>,
    pub source: Option<String>,
    pub inheritance: Option<String>,
    pub ext_attrs: Vec<String>,
    pub members: Vec<MemberData>,
    pub partial: bool,
    pub values: Vec<String>,
    pub target: Option<String>,
    pub includes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MemberData {
    pub kind: String,
    pub name: Option<String>,
    pub idl_type: Option<String>,
    pub readonly: bool,
    pub return_type: Option<String>,
    pub arguments: Vec<String>,
    pub const_value: Option<String>,
    pub required_arg_count: usize,
}

/// Extract the type name from an IDL type JSON value (recursive).
fn extract_type_name(val: &serde_json::Value) -> Option<String> {
    match val {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Object(map) => {
            // Try "name" field first
            if let Some(serde_json::Value::String(name)) = map.get("name") {
                return Some(name.clone());
            }
            // Try "idlType" for primitive references
            if let Some(inner) = map.get("idlType") {
                return extract_type_name(inner);
            }
            // Try "idl_type" (snake_case variant)
            if let Some(inner) = map.get("idl_type") {
                return extract_type_name(inner);
            }
            // Union type: take first type
            if let Some(serde_json::Value::Array(types)) =
                map.get("idType").or_else(|| map.get("types"))
            {
                if let Some(first) = types.first() {
                    return extract_type_name(first);
                }
            }
            // Generic type
            if let Some(generic) = map.get("generic").and_then(|g| g.as_str()) {
                if let Some(inner) = map.get("inner") {
                    return extract_type_name(inner);
                }
                return Some(generic.to_string());
            }
            None
        }
        serde_json::Value::Array(arr) => arr.first().and_then(extract_type_name),
        _ => None,
    }
}

/// Load and parse unified_ir.json, extracting structured definitions.
pub fn load_ir(path: &str) -> Result<(Vec<Definition>, JsonStats), String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;
    let root: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse {}: {}", path, e))?;

    let defs_raw = root
        .get("definitions")
        .and_then(|d| d.as_array())
        .ok_or("Missing 'definitions' array")?;

    let mut definitions = Vec::new();
    let mut counts = HashMap::new();

    for raw in defs_raw {
        let obj = match raw.as_object() {
            Some(o) => o,
            None => continue,
        };

        let kind = obj
            .get("kind")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        *counts.entry(kind.clone()).or_insert(0) += 1;

        let name = obj
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let source = obj
            .get("source")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let inheritance = obj
            .get("inheritance")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let partial = obj
            .get("partial")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Ext attrs: serialize as "name" or "name=value"
        let ext_attrs: Vec<String> = obj
            .get("ext_attrs")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|ea| {
                        let name = ea.get("name").and_then(|n| n.as_str())?;
                        if let Some(val) = ea.get("value").and_then(|v| v.as_str()) {
                            Some(format!("{}={}", name, val))
                        } else if let Some(val) = ea.get("value") {
                            Some(format!("{}={}", name, val))
                        } else {
                            Some(name.to_string())
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Members
        let members: Vec<MemberData> = obj
            .get("members")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|m| {
                        let mobj = m.as_object();
                        let kind = mobj
                            .and_then(|o| o.get("kind"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let mname = mobj
                            .and_then(|o| o.get("name"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let idl_type = mobj.and_then(|o| o.get("type")).and_then(extract_type_name);
                        let readonly = mobj
                            .and_then(|o| o.get("readonly"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let return_type = mobj
                            .and_then(|o| o.get("return_type"))
                            .and_then(extract_type_name);
                        let args: Vec<String> = mobj
                            .and_then(|o| o.get("arguments"))
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|a| {
                                        a.get("name")
                                            .and_then(|n| n.as_str())
                                            .map(|s| s.to_string())
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        let required_arg_count = mobj
                            .and_then(|o| o.get("arguments"))
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .take_while(|a| {
                                        !a.get("optional").and_then(|v| v.as_bool()).unwrap_or(false)
                                            && !a.get("variadic").and_then(|v| v.as_bool()).unwrap_or(false)
                                    })
                                    .count()
                            })
                            .unwrap_or(0);
                        let const_value = mobj
                            .and_then(|o| o.get("value"))
                            .and_then(|v| v.get("value"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());

                        MemberData {
                            kind,
                            name: mname,
                            idl_type,
                            readonly,
                            return_type,
                            arguments: args,
                            const_value,
                            required_arg_count,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Enum values
        let values: Vec<String> = obj
            .get("values")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let target = obj
            .get("target")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let includes = obj
            .get("includes")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        definitions.push(Definition {
            kind,
            name,
            source,
            inheritance,
            ext_attrs,
            members,
            partial,
            values,
            target,
            includes,
        });
    }

    let stats = JsonStats {
        definitions: defs_raw.len(),
        interfaces: *counts.get("interface").unwrap_or(&0),
        dictionaries: *counts.get("dictionary").unwrap_or(&0),
        enums: *counts.get("enum").unwrap_or(&0),
        typedefs: *counts.get("typedef").unwrap_or(&0),
        callbacks: *counts.get("callback").unwrap_or(&0)
            + *counts.get("callback_interface").unwrap_or(&0),
        namespaces: *counts.get("namespace").unwrap_or(&0),
    };

    Ok((definitions, stats))
}

#[derive(Debug, Clone, Serialize)]
pub struct JsonStats {
    pub definitions: usize,
    pub interfaces: usize,
    pub dictionaries: usize,
    pub enums: usize,
    pub typedefs: usize,
    pub callbacks: usize,
    pub namespaces: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_type_name_primitive() {
        let v = serde_json::Value::String("DOMString".into());
        assert_eq!(extract_type_name(&v), Some("DOMString".into()));
    }

    #[test]
    fn test_extract_type_name_object_name() {
        let mut map = serde_json::Map::new();
        map.insert("name".into(), serde_json::Value::String("Node".into()));
        let v = serde_json::Value::Object(map);
        assert_eq!(extract_type_name(&v), Some("Node".into()));
    }

    #[test]
    fn test_extract_type_name_none() {
        let v = serde_json::Value::Bool(true);
        assert_eq!(extract_type_name(&v), None);
    }
}
