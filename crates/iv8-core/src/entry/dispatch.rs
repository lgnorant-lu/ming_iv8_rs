//! Generalized Dispatch Hook.
//!
//! Generalizes from the existing `instrument_chaosvm` pattern to support
//! multiple dispatch types including handler-array VM, switch-VM, and
//! closure-scoped handler export.
//!
//! Also produces structured `DiagnosticRecord` and `EvidenceRecord` output
//! aligned with the `crate::entry::diagnostics` shared types.

use super::diagnostics;

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

impl DispatchFlavor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HandlerArray => "handler_array",
            Self::SwitchVM => "switch_vm",
            Self::ClosureScoped => "closure_scoped",
            Self::Unknown => "unknown",
        }
    }
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

/// Structured dispatch candidate matching the v0.7 dispatch-generalization spec.
#[derive(Debug, Clone)]
pub struct DispatchCandidate {
    pub candidate_id: String,
    pub variant: String,
    pub source_kind: String,
    pub static_score: f64,
    pub handler_count_hint: Option<usize>,
    pub bytecode_hint: bool,
    pub pc_hint: Option<String>,
    pub risk_level: String,
    pub decision: String,
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
    /// Number of handlers in the detected handler array (if known).
    pub handler_count_hint: Option<usize>,
    /// Number of arguments in the dispatch call (0 for zero-arg, >0 for multi-arg).
    pub argc_hint: Option<usize>,
}

impl DispatchDetection {
    pub fn to_candidate(&self) -> Option<DispatchCandidate> {
        if !self.detected {
            return None;
        }
        let decision = match self.evidence_level {
            DispatchEvidenceLevel::StrongStatic => "selected",
            DispatchEvidenceLevel::MarkerOnly => "selected",
            DispatchEvidenceLevel::DiagnosticOnly => "rejected",
        };
        Some(DispatchCandidate {
            candidate_id: format!("dispatch.candidate.{}", self.flavor.as_str()),
            variant: self.flavor.as_str().to_string(),
            source_kind: "source_regex".to_string(),
            static_score: match self.evidence_level {
                DispatchEvidenceLevel::StrongStatic => 0.82,
                DispatchEvidenceLevel::MarkerOnly => 0.45,
                DispatchEvidenceLevel::DiagnosticOnly => 0.15,
            },
            handler_count_hint: self.handler_count_hint,
            bytecode_hint: self.handler_array.is_some(),
            pc_hint: self.pc_var.clone(),
            risk_level: match self.evidence_level {
                DispatchEvidenceLevel::StrongStatic => "medium".to_string(),
                DispatchEvidenceLevel::MarkerOnly => "low".to_string(),
                DispatchEvidenceLevel::DiagnosticOnly => "high".to_string(),
            },
            decision: decision.to_string(),
        })
    }

    /// Convert detection results into structured diagnostic records.
    pub fn to_diagnostic_records(&self) -> Vec<diagnostics::DiagnosticRecord> {
        let mut records = Vec::new();

        if !self.detected {
            if self.evidence_level == DispatchEvidenceLevel::DiagnosticOnly {
                records.push(diagnostics::warn_diag(
                    diagnostics::codes::dispatch::CANDIDATE_REJECTED,
                    "dispatch.probe",
                    "computed member call without VM indicators; classified as ambiguous",
                ));
            } else {
                records.push(diagnostics::warn_diag(
                    diagnostics::codes::dispatch::CANDIDATE_REJECTED,
                    "dispatch.probe",
                    "no dispatch pattern detected",
                ));
            }
            records.push(diagnostics::warn_diag(
                diagnostics::codes::dispatch::STATIC_WEAK,
                "dispatch.probe",
                "static indicators below strong threshold",
            ));
            return records;
        }

        // Candidate detected
        records.push(diagnostics::info_diag(
            diagnostics::codes::dispatch::CANDIDATE_DETECTED,
            "dispatch.probe",
            &format!("dispatch variant '{}' detected", self.flavor.as_str()),
        ));

        // SwitchVM marker-only
        if matches!(self.flavor, DispatchFlavor::SwitchVM) {
            records.push(diagnostics::warn_diag(
                diagnostics::codes::dispatch::SWITCHVM_MARKER_ONLY,
                "dispatch.validate",
                "switch dispatch marker detected without case-level trace",
            ));
        }

        // Multi-arg evidence
        if self.argc_hint.unwrap_or(0) > 0 {
            records.push(diagnostics::info_diag(
                diagnostics::codes::dispatch::MULTI_ARG_OBSERVED,
                "dispatch.execute",
                &format!("multi-arg dispatch observed with {} argument(s)", self.argc_hint.unwrap()),
            ));
        }

        // Closure captured
        if matches!(self.flavor, DispatchFlavor::ClosureScoped) {
            records.push(diagnostics::warn_diag(
                diagnostics::codes::dispatch::CLOSURE_CAPTURED,
                "dispatch.instrument",
                "hook installed after closure capture; runtime proxy may not observe all calls",
            ));
        }

        records
    }

    /// Convert detection results into structured evidence records.
    pub fn to_evidence_records(&self) -> Vec<diagnostics::EvidenceRecord> {
        let mut records = Vec::new();

        if !self.detected {
            return records;
        }

        // dispatch_pattern_detected (weak, static)
        records.push(
            diagnostics::EvidenceRecord::new(
                "dispatch_pattern_detected",
                diagnostics::EvidenceStrength::Weak,
                "dispatch",
                "dispatch.probe",
                &format!("dispatch pattern '{}' detected statically", self.flavor.as_str()),
            )
            .with_producer("dispatch.main"),
        );

        // handler_array_captured (strong) -- only for HandlerArray with known handlers
        if matches!(self.flavor, DispatchFlavor::HandlerArray)
            && self.handler_count_hint.unwrap_or(0) > 0
        {
            records.push(
                diagnostics::EvidenceRecord::new(
                    "handler_array_captured",
                    diagnostics::EvidenceStrength::Strong,
                    "dispatch",
                    "dispatch.capture",
                    &format!("handler array captured with {} handler(s)", self.handler_count_hint.unwrap()),
                )
                .with_producer("dispatch.main")
                .with_payload(serde_json::json!({"handler_count": self.handler_count_hint})),
            );
        }

        // multi_arg_dispatch_observed (strong) -- if argc > 0
        if self.argc_hint.unwrap_or(0) > 0 {
            records.push(
                diagnostics::EvidenceRecord::new(
                    "multi_arg_dispatch_observed",
                    diagnostics::EvidenceStrength::Strong,
                    "dispatch",
                    "dispatch.execute",
                    &format!("multi-arg dispatch observed with {} argument(s)", self.argc_hint.unwrap()),
                )
                .with_producer("dispatch.main")
                .with_payload(serde_json::json!({"argc": self.argc_hint})),
            );
        }

        // SwitchVM marker-only (diagnostic_only)
        if matches!(self.flavor, DispatchFlavor::SwitchVM) {
            records.push(
                diagnostics::EvidenceRecord::new(
                    "switchvm_marker_only",
                    diagnostics::EvidenceStrength::MarkerOnly,
                    "dispatch",
                    "dispatch.validate",
                    "switch dispatch marker detected without runtime case-level trace",
                )
                .with_producer("dispatch.main"),
            );
        }

        records
    }
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
            handler_count_hint: None,
            argc_hint: None,
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
        handler_count_hint: None,
        argc_hint: None,
    }
}

/// Generate dispatch hook JS prelude for instrumenting a handler-array VM.
///
/// Produces extended trace events: `D,pc,opcode,stack_depth,handler_count,argc`
/// when handler_count and argc metadata are available.
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
    var __iv8_handler_count = {ha}.length;

    {ha} = new Proxy({ha}, {{
        get: function(target, prop) {{
            var val = Reflect.get(target, prop);
            if (typeof val === 'function') {{
                return new Proxy(val, {{
                    apply: function(fn, thisArg, args) {{
                        var pc = {pc};
                        var stack = {stack};
                        var argc = args ? args.length : 0;
                        __iv8_dispatch_log.push('D,' + pc + ',' + prop + ',' + (stack ? stack.length : 0) + ',' + __iv8_handler_count + ',' + argc);
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
///
/// v0.7: emits marker-only evidence since case-level instrumentation
/// requires AST-level switch-case rewriting.
pub fn switch_vm_prelude() -> String {
    r#"
(function() {
    if (typeof globalThis.__iv8_dispatch_installed !== 'undefined') return;
    globalThis.__iv8_dispatch_installed = true;

    var __iv8_dispatch_log = [];
    // Switch VM cannot be instrumented generically at the regex level.
    // AST-level switch-case instrumentation is needed for case-level trace.
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

        // Determine argc from the call pattern
        let argc_hint = extract_argc(after_nested);

        return Some(DispatchDetection {
            detected: true,
            flavor: DispatchFlavor::HandlerArray,
            evidence_level: DispatchEvidenceLevel::StrongStatic,
            handler_array: Some(handler_array.to_string()),
            pc_var: Some(pc_var.to_string()),
            index_array: Some(index_array_candidate.to_string()),
            stack_var,
            diagnostics: vec!["DISPATCH_CANDIDATE_DETECTED".to_string()],
            handler_count_hint: None,
            argc_hint,
        });
    }
    None
}

/// Estimate the number of arguments in the dispatch call from the call expression.
fn extract_argc(after_nested: &str) -> Option<usize> {
    let trimmed = after_nested.trim_start();
    if !trimmed.starts_with('(') {
        return None;
    }
    let paren = &trimmed[1..];
    // Find the matching closing paren, tracking balance
    let mut depth = 1;
    let mut args_str = String::new();
    for (i, c) in paren.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    args_str = paren[..i].to_string();
                    break;
                }
            }
            _ => {}
        }
    }
    if args_str.trim().is_empty() {
        return Some(0);
    }
    // Count top-level commas (not nested)
    let mut depth2 = 0;
    let mut count = 1;
    for c in args_str.chars() {
        match c {
            '(' | '[' | '{' => depth2 += 1,
            ')' | ']' | '}' => depth2 -= 1,
            ',' if depth2 == 0 => count += 1,
            _ => {}
        }
    }
    Some(count)
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
            handler_count_hint: None,
            argc_hint: None,
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

/// Check whether a dispatch detection represents an overbroad trap.
///
/// Per spec section 11, computed member calls must have at least two independent
/// VM indicators to be treated as dispatch. Returns true if the candidate
/// should be rejected as overbroad.
pub fn is_overbroad_trap(det: &DispatchDetection) -> bool {
    if !det.detected {
        return true;
    }
    // Count independent VM indicators
    let mut indicators = 0;
    if det.handler_array.is_some() {
        indicators += 1;
    }
    if det.pc_var.is_some() {
        indicators += 1;
    }
    if det.index_array.is_some() {
        indicators += 1;
    }
    if det.stack_var.is_some() {
        indicators += 1;
    }
    if det.handler_count_hint.unwrap_or(0) > 0 {
        indicators += 1;
    }
    // Need at least 2 indicators
    indicators < 2
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
        assert_eq!(det.argc_hint, Some(2));
    }

    #[test]
    fn test_detect_handler_array_with_whitespace_before_call() {
        let det = detect("var result = A[Q[U++]]   (stack);");
        assert!(det.detected);
        assert_eq!(det.flavor, DispatchFlavor::HandlerArray);
        assert_eq!(det.argc_hint, Some(1));
    }

    #[test]
    fn test_detect_handler_array_zero_arg() {
        let det = detect("var result = A[Q[U++]]();");
        assert!(det.detected);
        assert_eq!(det.argc_hint, Some(0));
    }

    #[test]
    fn test_detect_handler_array_multi_arg() {
        let det = detect("var result = A[Q[U++]](stack, ctx, extra);");
        assert!(det.detected);
        assert_eq!(det.argc_hint, Some(3));
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
        assert!(js.contains("__iv8_handler_count"));
        assert!(js.contains("argc = args ? args.length : 0"));
        assert!(js.contains("__iv8_handler_count + ',' + argc"));
    }

    #[test]
    fn test_switch_vm_prelude_generation() {
        let js = switch_vm_prelude();
        assert!(js.contains("switch_vm_detected"));
    }

    #[test]
    fn test_dispatch_candidate_from_detection() {
        let det = detect("A[Q[U++]]();");
        let cand = det.to_candidate().expect("candidate");
        assert_eq!(cand.variant, "handler_array");
        assert_eq!(cand.decision, "selected");
    }

    #[test]
    fn test_diagnostic_records_from_detection() {
        let det = detect("A[Q[U++]]();");
        let records = det.to_diagnostic_records();
        assert!(records.iter().any(|d| d.code == "DISPATCH_CANDIDATE_DETECTED"));
    }

    #[test]
    fn test_evidence_records_from_detection() {
        let det = detect("A[Q[U++]]();");
        let records = det.to_evidence_records();
        assert!(records.iter().any(|e| e.kind == "dispatch_pattern_detected"));
    }

    #[test]
    fn test_evidence_records_from_multi_arg_detection() {
        let det = detect("A[Q[U++]](a, b);");
        let records = det.to_evidence_records();
        assert!(records.iter().any(|e| e.kind == "multi_arg_dispatch_observed"));
    }

    #[test]
    fn test_overbroad_trap_caught_for_ordinary_computed_member() {
        let det = detect("var r = obj[key]();");
        assert!(is_overbroad_trap(&det));
    }

    #[test]
    fn test_overbroad_trap_not_caught_for_handler_array() {
        let det = detect("A[Q[U++]]();");
        assert!(!is_overbroad_trap(&det));
    }

    #[test]
    fn test_diagnostic_records_from_switch_vm() {
        let det = detect("switch(B[P++]) { case 0: break; }");
        let records = det.to_diagnostic_records();
        assert!(records.iter().any(|d| d.code == "DISPATCH_CANDIDATE_DETECTED"));
        assert!(records.iter().any(|d| d.code == "SWITCHVM_MARKER_ONLY"));
    }

    #[test]
    fn test_evidence_records_from_switch_vm() {
        let det = detect("switch(B[P++]) { case 0: break; }");
        let records = det.to_evidence_records();
        assert!(records.iter().any(|e| e.kind == "switchvm_marker_only"));
    }

    #[test]
    fn test_no_diagnostic_records_when_not_detected() {
        let det = detect("var x = 1;");
        let records = det.to_diagnostic_records();
        assert!(!records.is_empty());
        assert!(records.iter().any(|d| d.code == "DISPATCH_CANDIDATE_REJECTED"));
    }
}