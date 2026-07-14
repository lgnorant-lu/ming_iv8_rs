//! Minimal consumer source-map support (S7-09).
//!
//! Parses `//# sourceMappingURL=` comments and data-URL JSON maps enough to
//! expose file list / sourcesContent presence. Full VLQ original-position
//! lookup remains a later refinement. No external crate dependency.

/// Extract sourceMappingURL value from JS/CSS source text (last occurrence wins).
pub fn extract_source_mapping_url(source: &str) -> Option<String> {
    let mut last = None;
    for line in source.lines().rev() {
        let t = line.trim();
        for prefix in ["//# sourceMappingURL=", "//@ sourceMappingURL="] {
            if let Some(rest) = t.strip_prefix(prefix) {
                let url = rest.trim();
                if !url.is_empty() {
                    last = Some(url.to_string());
                    break;
                }
            }
        }
        if last.is_some() {
            break;
        }
    }
    last
}

fn decode_b64(input: &str) -> Option<Vec<u8>> {
    const T: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::new();
    let mut buf: u32 = 0;
    let mut bits: i32 = 0;
    for &b in input.as_bytes() {
        if b == b'=' || b.is_ascii_whitespace() {
            continue;
        }
        let v = T.iter().position(|&c| c == b)? as u32;
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    Some(out)
}

/// Parse a data-URL application/json source map into a small JSON summary.
pub fn parse_inline_source_map_json(data_url: &str) -> Option<serde_json::Value> {
    let rest = data_url.strip_prefix("data:")?;
    let (meta, payload) = rest.split_once(',')?;
    let json_text = if meta.contains(";base64") {
        let bytes = decode_b64(payload.trim())?;
        String::from_utf8(bytes).ok()?
    } else {
        payload.to_string()
    };
    let v: serde_json::Value = serde_json::from_str(&json_text).ok()?;
    let sources = v
        .get("sources")
        .and_then(|s| s.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let has_content = v
        .get("sourcesContent")
        .and_then(|s| s.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    Some(serde_json::json!({
        "version": v.get("version").cloned().unwrap_or(serde_json::json!(null)),
        "file": v.get("file").cloned().unwrap_or(serde_json::json!(null)),
        "source_count": sources,
        "has_sources_content": has_content,
        "mappings_present": v.get("mappings").and_then(|m| m.as_str()).map(|s| !s.is_empty()).unwrap_or(false),
    }))
}

/// Summarize source map for a source string (inline data URL only for full parse).
pub fn summarize_source_map(source: &str) -> serde_json::Value {
    let url = extract_source_mapping_url(source);
    match url {
        None => serde_json::json!({ "present": false }),
        Some(u) if u.starts_with("data:") => {
            let parsed = parse_inline_source_map_json(&u);
            serde_json::json!({
                "present": true,
                "url_kind": "data",
                "summary": parsed,
            })
        }
        Some(u) => serde_json::json!({
            "present": true,
            "url_kind": "external",
            "url": u,
            "summary": null,
            "note": "external map fetch not performed (fixture/local path only)",
        }),
    }
}

/// Tree-shaking / sideEffects markers (S7-11 diagnostics only).
pub fn detect_treeshaking_markers(source: &str) -> serde_json::Value {
    let pure = source.contains("/*#__PURE__*/") || source.contains("/*@__PURE__*/");
    let side_effects_false = source.contains("\"sideEffects\":false")
        || source.contains("'sideEffects':false")
        || source.contains("\"sideEffects\": false");
    let unused = source.contains("/* unused harmony export")
        || source.contains("/* unused harmony reexport");
    let harmony = source.contains("__webpack_exports__")
        || source.contains("/* harmony export")
        || source.contains("/* harmony import");
    let pure_count = source.matches("/*#__PURE__*/").count()
        + source.matches("/*@__PURE__*/").count();
    serde_json::json!({
        "pure_annotation": pure,
        "pure_annotation_count": pure_count,
        "side_effects_false": side_effects_false,
        "unused_export_comment": unused,
        "harmony_markers": harmony,
        "note": "diagnostics only; no semantic DCE",
    })
}

/// Minimal VLQ segment decoder for source-map `mappings` (A-P2-1).
/// Returns list of {generated_line, generated_column, source_index, original_line,
/// original_column, name_index?}.
pub fn decode_vlq_mappings_preview(mappings: &str, max_entries: usize) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    let mut gen_line = 0i64;
    let mut gen_col = 0i64;
    let mut src_idx = 0i64;
    let mut orig_line = 0i64;
    let mut orig_col = 0i64;
    let mut name_idx = 0i64;
    for line in mappings.split(';') {
        gen_col = 0;
        if line.is_empty() {
            gen_line += 1;
            continue;
        }
        for seg in line.split(',') {
            if seg.is_empty() {
                continue;
            }
            let vals = decode_vlq_segment(seg);
            if vals.is_empty() {
                continue;
            }
            gen_col += vals[0];
            if vals.len() >= 4 {
                src_idx += vals[1];
                orig_line += vals[2];
                orig_col += vals[3];
                let mut map = serde_json::Map::new();
                map.insert("generated_line".into(), serde_json::json!(gen_line));
                map.insert("generated_column".into(), serde_json::json!(gen_col));
                map.insert("source_index".into(), serde_json::json!(src_idx));
                map.insert("original_line".into(), serde_json::json!(orig_line));
                map.insert("original_column".into(), serde_json::json!(orig_col));
                if vals.len() >= 5 {
                    name_idx += vals[4];
                    map.insert("name_index".into(), serde_json::json!(name_idx));
                }
                out.push(serde_json::Value::Object(map));
            } else {
                out.push(serde_json::json!({
                    "generated_line": gen_line,
                    "generated_column": gen_col,
                }));
            }
            if out.len() >= max_entries {
                return out;
            }
        }
        gen_line += 1;
    }
    out
}

fn decode_vlq_segment(seg: &str) -> Vec<i64> {
    let mut values = Vec::new();
    let mut value = 0i64;
    let mut shift = 0;
    for c in seg.chars() {
        let digit = vlq_char(c);
        if digit < 0 {
            break;
        }
        let d = digit as i64;
        value |= (d & 31) << shift;
        if (d & 32) == 0 {
            let neg = (value & 1) == 1;
            let mut v = value >> 1;
            if neg {
                v = -v;
            }
            values.push(v);
            value = 0;
            shift = 0;
        } else {
            shift += 5;
        }
    }
    values
}

fn vlq_char(c: char) -> i32 {
    match c {
        'A'..='Z' => (c as i32) - ('A' as i32),
        'a'..='z' => (c as i32) - ('a' as i32) + 26,
        '0'..='9' => (c as i32) - ('0' as i32) + 52,
        '+' => 62,
        '/' => 63,
        _ => -1,
    }
}

/// originalPositionFor-style lookup: first mapping at or before (line, column).
/// When `sources` / `names` are provided, resolve indices to path / identifier.
pub fn original_position_for(
    mappings: &str,
    generated_line: i64,
    generated_column: i64,
) -> Option<serde_json::Value> {
    original_position_for_resolved(mappings, generated_line, generated_column, None, None)
}

/// Like [`original_position_for`], optionally resolving `source` / `name` strings.
pub fn original_position_for_resolved(
    mappings: &str,
    generated_line: i64,
    generated_column: i64,
    sources: Option<&[String]>,
    names: Option<&[String]>,
) -> Option<serde_json::Value> {
    let entries = decode_vlq_mappings_preview(mappings, 10_000);
    let mut best: Option<serde_json::Value> = None;
    for e in &entries {
        let gl = e.get("generated_line").and_then(|v| v.as_i64()).unwrap_or(-1);
        let gc = e
            .get("generated_column")
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);
        if gl < generated_line || (gl == generated_line && gc <= generated_column) {
            if e.get("source_index").is_some() {
                best = Some(e.clone());
            }
        }
        if gl > generated_line {
            break;
        }
    }
    let mut pos = best?;
    if let Some(srcs) = sources {
        if let Some(idx) = pos.get("source_index").and_then(|v| v.as_i64()) {
            if idx >= 0 {
                let i = idx as usize;
                if i < srcs.len() {
                    pos.as_object_mut()
                        .unwrap()
                        .insert("source".into(), serde_json::json!(srcs[i]));
                }
            }
        }
    }
    if let Some(nms) = names {
        if let Some(idx) = pos.get("name_index").and_then(|v| v.as_i64()) {
            if idx >= 0 {
                let i = idx as usize;
                if i < nms.len() {
                    pos.as_object_mut()
                        .unwrap()
                        .insert("name".into(), serde_json::json!(nms[i]));
                }
            }
        }
    }
    Some(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_source_mapping_url() {
        let src = "var x=1;\n//# sourceMappingURL=app.js.map\n";
        assert_eq!(
            extract_source_mapping_url(src).as_deref(),
            Some("app.js.map")
        );
    }

    #[test]
    fn test_parse_inline_source_map() {
        let json = r#"{"version":3,"file":"out.js","sources":["a.js"],"mappings":"AAAA"}"#;
        let b64 = {
            // manual encode using same table
            const T: &[u8] =
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let bytes = json.as_bytes();
            let mut out = String::new();
            let mut i = 0;
            while i < bytes.len() {
                let b0 = bytes[i] as u32;
                let b1 = if i + 1 < bytes.len() {
                    bytes[i + 1] as u32
                } else {
                    0
                };
                let b2 = if i + 2 < bytes.len() {
                    bytes[i + 2] as u32
                } else {
                    0
                };
                let triple = (b0 << 16) | (b1 << 8) | b2;
                out.push(T[((triple >> 18) & 63) as usize] as char);
                out.push(T[((triple >> 12) & 63) as usize] as char);
                if i + 1 < bytes.len() {
                    out.push(T[((triple >> 6) & 63) as usize] as char);
                } else {
                    out.push('=');
                }
                if i + 2 < bytes.len() {
                    out.push(T[(triple & 63) as usize] as char);
                } else {
                    out.push('=');
                }
                i += 3;
            }
            out
        };
        let url = format!("data:application/json;base64,{b64}");
        let src = format!("var x=1;\n//# sourceMappingURL={url}\n");
        let sum = summarize_source_map(&src);
        assert_eq!(sum["present"], true);
        assert_eq!(sum["url_kind"], "data");
        assert_eq!(sum["summary"]["source_count"], 1);
        assert_eq!(sum["summary"]["mappings_present"], true);
    }

    #[test]
    fn test_treeshaking_markers() {
        let m = detect_treeshaking_markers("var a=/*#__PURE__*/fn(); /* harmony export */");
        assert_eq!(m["pure_annotation"], true);
        assert_eq!(m["pure_annotation_count"], 1);
        assert_eq!(m["harmony_markers"], true);
    }

    #[test]
    fn test_vlq_original_position_resolves_source_and_name() {
        // [0,0,0,0,0] -> AAAAA (col, src, ol, oc, name)
        let mappings = "AAAAA";
        let sources = vec!["app.ts".to_string()];
        let names = vec!["foo".to_string()];
        let pos = original_position_for_resolved(mappings, 0, 0, Some(&sources), Some(&names))
            .expect("pos");
        assert_eq!(pos["source"], "app.ts");
        assert_eq!(pos["name"], "foo");
    }

    #[test]
    fn test_vlq_original_position_for() {
        // Minimal mappings: one segment at 0,0 -> source 0, line 0, col 0
        // VLQ for [0,0,0,0] is "AAAA"
        let mappings = "AAAA";
        let pos = original_position_for(mappings, 0, 0).expect("pos");
        assert_eq!(pos["source_index"], 0);
        assert_eq!(pos["original_line"], 0);
        assert_eq!(pos["original_column"], 0);
    }
}
