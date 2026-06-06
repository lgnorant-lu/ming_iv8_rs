//! Generalized Dispatch Hook.
//!
//! Generalizes from the existing `instrument_chaosvm` pattern to support
//! multiple dispatch types including handler-array VM, switch-VM, and
//! closure-scoped handler export.

/// Flavor of VM dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchFlavor {
    /// Handler array: A[Q[U++]]()
    HandlerArray,
    /// Switch-based: switch(B[P++]) { case ... }
    SwitchVM,
    /// Closure-scoped handlers captured before dispatch export
    ClosureScoped,
    /// Undetermined
    Unknown,
}

/// Static evidence level produced by dispatch detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchEvidenceLevel {
    /// Handler table and PC/index pattern are statically visible.
    StrongStatic,
    /// Dispatch-like marker is visible but runtime facts are still needed.
    MarkerOnly,
    /// Pattern is too broad to treat as dispatch evidence.
    DiagnosticOnly,
}

/// Result of dispatch detection.
#[derive(Debug, Clone)]
pub struct DispatchDetection {
    pub detected: bool,
    pub flavor: DispatchFlavor,
    pub evidence_level: DispatchEvidenceLevel,
    pub handler_array: Option<String>,
    pub pc_var: Option<String>,
    pub index_array: Option<String>,
    pub stack_var: Option<String>,
    pub diagnostics: Vec<String>,
}

/// Detect dispatch pattern in JS source.
pub fn detect(source: &str) -> DispatchDetection {
    // Handler array: X[Y[Z++]]()
    if let Some(det) = detect_handler_array(source) {
        return det;
    }

    // Switch VM: switch(X[Y++]) or switch(X[Y])
    if let Some(det) = detect_switch_vm(source) {
        return det;
    }

    if detect_ambiguous_computed_member_call(source) {
        return DispatchDetection {
            detected: false,
            flavor: DispatchFlavor::Unknown,
            evidence_level: DispatchEvidenceLevel::DiagnosticOnly,
            handler_array: None,
            pc_var: None,
            index_array: None,
            stack_var: None,
            diagnostics: vec!["DISPATCH_COMPUTED_MEMBER_AMBIGUOUS".to_string()],
        };
    }

    DispatchDetection {
        detected: false,
        flavor: DispatchFlavor::Unknown,
        evidence_level: DispatchEvidenceLevel::DiagnosticOnly,
        handler_array: None,
        pc_var: None,
        index_array: None,
        stack_var: None,
        diagnostics: Vec::new(),
    }
}

/// Generate dispatch hook JS prelude for instrumenting a handler-array VM.
pub fn handler_array_prelude(
    handler_array: &str,
    pc_var: &str,
    _index_array: &str,
    stack_var: &str,
) -> String {
    format!(
        r#"
(function() {{
    if (typeof globalThis.__iv8_dispatch_installed !== 'undefined') return;
    globalThis.__iv8_dispatch_installed = true;

    var __iv8_dispatch_log = [];
    var __orig_{ha} = {ha};
    var __iv8_orig_handlers__ = {ha}.slice();

    {ha} = new Proxy({ha}, {{
        get: function(target, prop) {{
            var val = Reflect.get(target, prop);
            if (typeof val === 'function') {{
                return new Proxy(val, {{
                    apply: function(fn, thisArg, args) {{
                        var pc = {pc};
                        var stack = {stack};
                        __iv8_dispatch_log.push('D,' + pc + ',' + prop + ',' + (stack ? stack.length : 0));
                        return fn.apply(thisArg, args);
                    }}
                }});
            }}
            return val;
        }}
    }});

    globalThis.__iv8_dispatch_flavor = 'handler_array';
    globalThis.__iv8_dispatch_log = __iv8_dispatch_log;
}})();
"#,
        ha = handler_array,
        pc = pc_var,
        stack = stack_var,
    )
}

/// Generate dispatch hook JS prelude for a switch-VM.
pub fn switch_vm_prelude() -> String {
    r#"
(function() {
    if (typeof globalThis.__iv8_dispatch_installed !== 'undefined') return;
    globalThis.__iv8_dispatch_installed = true;

    var __iv8_dispatch_log = [];
    // Switch VM is harder to instrument generically.
    // For switch-based dispatch, mark that we detected it but
    // cannot instrument transparently.
    __iv8_dispatch_log.push('D,switch_vm_detected');
    globalThis.__iv8_dispatch_flavor = 'switch_vm';
    globalThis.__iv8_dispatch_log = __iv8_dispatch_log;
})();
"#
    .to_string()
}

// ───
// Internal detection helpers
// ───

/// Detect handler array VM: X[Y[Z++]]()
fn detect_handler_array(source: &str) -> Option<DispatchDetection> {
    // Find each '[' and check if the surrounding pattern matches X[Y[Z++]]()
    // where X (before [) is the handler array, Y is the index array, Z is PC
    let bytes = source.as_bytes();
    let len = source.len();
    for i in 0..len.saturating_sub(10) {
        if bytes[i] != b'[' {
            continue;
        }
        let after_first = &source[i + 1..];
        // Look for second '['
        let second_brk = after_first.find('[')?;
        if second_brk > 30 {
            continue;
        }
        let after_second = &after_first[second_brk + 1..];
        // Look for "++"
        let inc = after_second.find("++")?;
        let after_inc = &after_second[inc + 2..];
        // Check closing pattern: X[Y[Z++]](...) with any argument list.
        let after_inc_trimmed = after_inc.trim_start();
        if !after_inc_trimmed.starts_with("]]") {
            continue;
        }
        let after_nested = after_inc_trimmed[2..].trim_start();
        if !after_nested.starts_with('(') {
            continue;
        }

        // Extract handler array: walk backwards from position i
        let handler_start = source[..i].trim_end().rfind(|c: char| {
            c == ' ' || c == '\n' || c == '\r' || c == '\t' || c == '=' || c == '(' || c == ';'
        });
        let handler_array = match handler_start {
            Some(pos) => source[pos + 1..i].trim(),
            None => source[..i].trim(),
        };
        if handler_array.is_empty() || handler_array.len() > 30 {
            continue;
        }

        // Extract index array: between first '[' and second '['
        let index_array_candidate = after_first[..second_brk].trim();
        if index_array_candidate.is_empty() || index_array_candidate.len() > 30 {
            continue;
        }

        // Extract PC variable: before "++" in after_second
        let pc_var = after_second[..inc].trim();
        if pc_var.is_empty() || pc_var.len() > 30 {
            continue;
        }

        let stack_var = detect_stack_var(source, i);

        return Some(DispatchDetection {
            detected: true,
            flavor: DispatchFlavor::HandlerArray,
            evidence_level: DispatchEvidenceLevel::StrongStatic,
            handler_array: Some(handler_array.to_string()),
            pc_var: Some(pc_var.to_string()),
            index_array: Some(index_array_candidate.to_string()),
            stack_var,
            diagnostics: vec!["DISPATCH_CANDIDATE_DETECTED".to_string()],
        });
    }
    None
}

fn detect_switch_vm(source: &str) -> Option<DispatchDetection> {
    let sw_pos = source.find("switch(")?;
    let after_switch = &source[sw_pos + 7..];
    if after_switch.contains('[') && (after_switch.contains("++") || after_switch.contains("++]")) {
        Some(DispatchDetection {
            detected: true,
            flavor: DispatchFlavor::SwitchVM,
            evidence_level: DispatchEvidenceLevel::MarkerOnly,
            handler_array: None,
            pc_var: None,
            index_array: None,
            stack_var: None,
            diagnostics: vec![
                "DISPATCH_CANDIDATE_DETECTED".to_string(),
                "SWITCHVM_MARKER_ONLY".to_string(),
            ],
        })
    } else {
        None
    }
}

fn detect_ambiguous_computed_member_call(source: &str) -> bool {
    let bytes = source.as_bytes();
    for i in 0..bytes.len().saturating_sub(3) {
        if bytes[i] != b'[' {
            continue;
        }
        let Some(close_rel) = source[i + 1..].find(']') else {
            continue;
        };
        let after = source[i + 1 + close_rel + 1..].trim_start();
        if !after.starts_with('(') {
            continue;
        }
        let inside = source[i + 1..i + 1 + close_rel].trim();
        if inside.is_empty() || inside.contains('[') || inside.contains("++") {
            continue;
        }
        return true;
    }
    false
}

fn detect_stack_var(source: &str, near_offset: usize) -> Option<String> {
    let start = near_offset.saturating_sub(3000);
    let end = (near_offset + 3000).min(source.len());
    let window = &source[start..end];

    if let Some(pos) = window.find(".push(") {
        let before = &window[..pos];
        if let Some(dot) = before.rfind([' ', '\n', ';', '{']) {
            let name = &before[dot + 1..].trim();
            if !name.is_empty() && name.len() < 30 {
                return Some(name.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_handler_array() {
        let det = detect("var result = A[Q[U++]]();");
        assert!(det.detected);
        assert_eq!(det.flavor, DispatchFlavor::HandlerArray);
        assert_eq!(det.evidence_level, DispatchEvidenceLevel::StrongStatic);
        assert_eq!(det.handler_array.as_deref(), Some("A"));
        assert_eq!(det.pc_var.as_deref(), Some("U"));
        assert!(det
            .diagnostics
            .contains(&"DISPATCH_CANDIDATE_DETECTED".to_string()));
    }

    #[test]
    fn test_detect_handler_array_with_args() {
        let det = detect("var result = A[Q[U++]](stack, ctx);");
        assert!(det.detected);
        assert_eq!(det.flavor, DispatchFlavor::HandlerArray);
        assert_eq!(det.handler_array.as_deref(), Some("A"));
        assert_eq!(det.index_array.as_deref(), Some("Q"));
        assert_eq!(det.pc_var.as_deref(), Some("U"));
        assert_eq!(det.evidence_level, DispatchEvidenceLevel::StrongStatic);
    }

    #[test]
    fn test_detect_handler_array_with_whitespace_before_call() {
        let det = detect("var result = A[Q[U++]]   (stack);");
        assert!(det.detected);
        assert_eq!(det.flavor, DispatchFlavor::HandlerArray);
    }

    #[test]
    fn test_detect_switch_vm() {
        let det = detect("switch(B[P++]) { case 0: break; }");
        assert!(det.detected);
        assert_eq!(det.flavor, DispatchFlavor::SwitchVM);
        assert_eq!(det.evidence_level, DispatchEvidenceLevel::MarkerOnly);
        assert!(det
            .diagnostics
            .contains(&"SWITCHVM_MARKER_ONLY".to_string()));
    }

    #[test]
    fn test_computed_member_call_is_ambiguous_not_dispatch() {
        let det = detect("var r = obj[key]();");
        assert!(!det.detected);
        assert_eq!(det.flavor, DispatchFlavor::Unknown);
        assert_eq!(det.evidence_level, DispatchEvidenceLevel::DiagnosticOnly);
        assert!(det
            .diagnostics
            .contains(&"DISPATCH_COMPUTED_MEMBER_AMBIGUOUS".to_string()));
    }

    #[test]
    fn test_no_dispatch() {
        let det = detect("var x = 1 + 1;");
        assert!(!det.detected);
    }

    #[test]
    fn test_handler_array_prelude_generation() {
        let js = handler_array_prelude("A", "U", "Q", "S");
        assert!(js.contains("__iv8_dispatch_log"));
        assert!(js.contains("A = new Proxy(A"));
        assert!(js.contains("U"));
    }

    #[test]
    fn test_switch_vm_prelude_generation() {
        let js = switch_vm_prelude();
        assert!(js.contains("switch_vm_detected"));
    }
}
