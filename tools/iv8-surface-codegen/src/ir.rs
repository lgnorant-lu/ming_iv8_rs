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
    pub has_put_forwards: bool,
    pub has_replaceable: bool,
    pub return_type: Option<String>,
    pub arguments: Vec<String>,
    pub const_value: Option<String>,
    pub required_arg_count: usize,
    pub special: Option<String>,
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
            // name may be an array of objects with idlType
            if let Some(serde_json::Value::Array(arr)) = map.get("name") {
                if let Some(first) = arr.first() {
                    if let Some(inner_name) = extract_type_name(first) {
                        return Some(inner_name);
                    }
                }
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
            // Generic type (e.g. Promise<T>, sequence<T>, FrozenArray<T>)
            // Return "Generic<Inner>" format so type_mapper can detect Promise
            if let Some(generic) = map.get("generic").and_then(|g| g.as_str()) {
                if let Some(inner) = map.get("inner") {
                    if let Some(inner_name) = extract_type_name(inner) {
                        return Some(format!("{}<{}>", generic, inner_name));
                    }
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

                        let ext_attrs = mobj.and_then(|o| o.get("ext_attrs")).and_then(|v| v.as_array());
                        let has_put_forwards = ext_attrs
                            .map(|arr| arr.iter().any(|ea| ea.get("name").and_then(|v| v.as_str()) == Some("PutForwards")))
                            .unwrap_or(false);
                        let has_replaceable = ext_attrs
                            .map(|arr| arr.iter().any(|ea| ea.get("name").and_then(|v| v.as_str()) == Some("Replaceable")))
                            .unwrap_or(false);

                        let special = m.get("special").and_then(|v| v.as_str()).map(|s| s.to_string());

                        MemberData {
                            kind,
                            name: mname,
                            idl_type,
                            readonly,
                            has_put_forwards,
                            has_replaceable,
                            return_type,
                            arguments: args,
                            const_value,
                            required_arg_count,
                            special,
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

    let definitions = process_includes_and_partials(definitions);

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

fn process_includes_and_partials(mut definitions: Vec<Definition>) -> Vec<Definition> {
    let includes_stmts: Vec<(String, String)> = definitions
        .iter()
        .filter(|d| d.kind == "includes")
        .filter_map(|d| {
            let target = d.target.clone()?;
            let mixin = d.includes.clone()?;
            Some((target, mixin))
        })
        .collect();

    let mixin_members: HashMap<String, Vec<MemberData>> = definitions
        .iter()
        .filter(|d| {
            d.kind == "interface_mixin" || d.kind == "mixin" || d.kind == "callback_interface"
        })
        .filter_map(|d| {
            let name = d.name.clone()?;
            Some((name, d.members.clone()))
        })
        .collect();

    if !includes_stmts.is_empty() {
        let mut by_name: HashMap<String, usize> = HashMap::new();
        for (i, d) in definitions.iter().enumerate() {
            if let Some(ref name) = d.name {
                if d.kind != "includes" && !d.partial {
                    by_name.entry(name.clone()).or_insert(i);
                }
            }
        }

        for (target, mixin_name) in &includes_stmts {
            let target_idx = match by_name.get(target) {
                Some(&idx) => idx,
                None => continue,
            };
            let members_to_copy = match mixin_members.get(mixin_name) {
                Some(m) => m.clone(),
                None => continue,
            };

            let existing_names: std::collections::HashSet<String> = definitions[target_idx]
                .members
                .iter()
                .filter_map(|m| m.name.clone())
                .collect();

            for member in members_to_copy {
                if let Some(ref mname) = member.name {
                    if existing_names.contains(mname) {
                        continue;
                    }
                }
                definitions[target_idx].members.push(member);
            }
        }
    }

    let mut partial_indices: Vec<usize> = Vec::new();
    let mut partial_merges: Vec<(usize, Vec<MemberData>)> = Vec::new();
    {
        let mut base_by_name: HashMap<String, usize> = HashMap::new();
        for (i, d) in definitions.iter().enumerate() {
            if d.kind == "includes" {
                continue;
            }
            if d.partial {
                if let Some(ref name) = d.name {
                    if let Some(&base_idx) = base_by_name.get(name) {
                        partial_merges.push((base_idx, d.members.clone()));
                        partial_indices.push(i);
                    }
                }
            } else {
                if let Some(ref name) = d.name {
                    base_by_name.entry(name.clone()).or_insert(i);
                }
            }
        }
    }
    for (base_idx, members_to_merge) in partial_merges {
        for member in members_to_merge {
            definitions[base_idx].members.push(member);
        }
    }

    let mut remove_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for (i, d) in definitions.iter().enumerate() {
        if d.kind == "includes" {
            remove_set.insert(i);
        }
    }
    for &i in &partial_indices {
        remove_set.insert(i);
    }

    if remove_set.is_empty() {
        definitions
    } else {
        definitions
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !remove_set.contains(i))
            .map(|(_, d)| d)
            .collect()
    }
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

    fn make_member(kind: &str, name: &str) -> MemberData {
        MemberData {
            kind: kind.into(),
            name: Some(name.into()),
            idl_type: None,
            readonly: false,
            has_put_forwards: false,
            has_replaceable: false,
            return_type: None,
            arguments: vec![],
            const_value: None,
            required_arg_count: 0,
            special: None,
        }
    }

    fn make_def(kind: &str, name: &str, members: Vec<MemberData>) -> Definition {
        Definition {
            kind: kind.into(),
            name: Some(name.into()),
            source: None,
            inheritance: None,
            ext_attrs: vec![],
            members,
            partial: false,
            values: vec![],
            target: None,
            includes: None,
        }
    }

    #[test]
    fn test_includes_merges_mixin_members_into_target() {
        let mixin = make_def(
            "interface_mixin",
            "GlobalEventHandlers",
            vec![make_member("attribute", "onclick")],
        );
        let target = make_def("interface", "Window", vec![]);
        let mut includes_def = make_def("includes", "", vec![]);
        includes_def.target = Some("Window".into());
        includes_def.includes = Some("GlobalEventHandlers".into());

        let result = process_includes_and_partials(vec![mixin, target, includes_def]);

        assert_eq!(result.len(), 2);
        let window = result.iter().find(|d| d.name.as_deref() == Some("Window")).unwrap();
        assert_eq!(window.members.len(), 1);
        assert_eq!(window.members[0].name.as_deref(), Some("onclick"));
        assert!(result.iter().all(|d| d.kind != "includes"));
    }

    #[test]
    fn test_includes_does_not_overwrite_existing_members() {
        let mixin = make_def(
            "interface_mixin",
            "GlobalEventHandlers",
            vec![make_member("attribute", "onclick")],
        );
        let target = make_def(
            "interface",
            "Window",
            vec![make_member("attribute", "onclick")],
        );
        let mut includes_def = make_def("includes", "", vec![]);
        includes_def.target = Some("Window".into());
        includes_def.includes = Some("GlobalEventHandlers".into());

        let result = process_includes_and_partials(vec![mixin, target, includes_def]);

        let window = result.iter().find(|d| d.name.as_deref() == Some("Window")).unwrap();
        assert_eq!(window.members.len(), 1);
    }

    #[test]
    fn test_partial_merges_into_base() {
        let base = make_def("interface", "Element", vec![make_member("attribute", "id")]);
        let mut partial = make_def(
            "interface",
            "Element",
            vec![make_member("attribute", "className")],
        );
        partial.partial = true;

        let result = process_includes_and_partials(vec![base, partial]);

        assert_eq!(result.len(), 1);
        let element = &result[0];
        assert!(!element.partial);
        assert_eq!(element.members.len(), 2);
    }

    #[test]
    fn test_partial_without_base_is_kept() {
        let mut partial = make_def(
            "interface",
            "FooPartial",
            vec![make_member("attribute", "bar")],
        );
        partial.partial = true;

        let result = process_includes_and_partials(vec![partial]);

        assert_eq!(result.len(), 1);
        assert!(result[0].partial);
    }

    #[test]
    fn test_includes_with_missing_mixin_is_skipped() {
        let target = make_def("interface", "Window", vec![]);
        let mut includes_def = make_def("includes", "", vec![]);
        includes_def.target = Some("Window".into());
        includes_def.includes = Some("NonExistentMixin".into());

        let result = process_includes_and_partials(vec![target, includes_def]);

        assert_eq!(result.len(), 1);
        let window = &result[0];
        assert_eq!(window.members.len(), 0);
    }

    #[test]
    fn test_required_arg_count_all_required() {
        let json = serde_json::json!([
            {"name": "a", "optional": false, "variadic": false},
            {"name": "b", "optional": false, "variadic": false}
        ]);
        let count = json.as_array().unwrap().iter()
            .take_while(|a| {
                !a.get("optional").and_then(|v| v.as_bool()).unwrap_or(false)
                    && !a.get("variadic").and_then(|v| v.as_bool()).unwrap_or(false)
            })
            .count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_required_arg_count_with_optional() {
        let json = serde_json::json!([
            {"name": "a", "optional": false, "variadic": false},
            {"name": "b", "optional": true, "variadic": false}
        ]);
        let count = json.as_array().unwrap().iter()
            .take_while(|a| {
                !a.get("optional").and_then(|v| v.as_bool()).unwrap_or(false)
                    && !a.get("variadic").and_then(|v| v.as_bool()).unwrap_or(false)
            })
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_required_arg_count_with_variadic() {
        let json = serde_json::json!([
            {"name": "a", "optional": false, "variadic": false},
            {"name": "b", "optional": false, "variadic": true}
        ]);
        let count = json.as_array().unwrap().iter()
            .take_while(|a| {
                !a.get("optional").and_then(|v| v.as_bool()).unwrap_or(false)
                    && !a.get("variadic").and_then(|v| v.as_bool()).unwrap_or(false)
            })
            .count();
        assert_eq!(count, 1);
    }
}
