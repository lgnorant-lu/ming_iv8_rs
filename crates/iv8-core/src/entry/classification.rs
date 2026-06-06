//! Sample classification for v0.6 EntryPlanner.
//!
//! Detects the kind of JS source being processed (plain script, VM dispatch,
//! webpack bundle, eval-heavy, etc.) by examining source content and runtime signals.
//! This drives strategy selection in the EntryPlanner.

use crate::entry::types::SampleKind;

/// Classify a JS source into a SampleKind.
///
/// Uses a combination of source-level patterns and pre-extracted runtime signals.
/// Not intended to be perfectly accurate for edge samples — the EntryPlanner
/// always has fallback strategies.
pub fn classify(source: &str, _signals: &[String]) -> SampleKind {
    // HTML content should be classified as plain_script
    // (executor will correctly fail on SyntaxError)
    let trimmed = source.trim_start();
    if trimmed.starts_with("<!") || trimmed.starts_with('<') {
        return SampleKind::PlainScript;
    }

    let raw_signals = SignalSet::from_source(source);

    // Priority 1: VM dispatch patterns
    if raw_signals.has_chaosvm_dispatch || raw_signals.has_switch_vm {
        if raw_signals.has_webpack_require {
            return SampleKind::WebpackVmHybrid;
        }
        if !raw_signals.dispatch_extractable {
            return SampleKind::VmDispatchUnknown;
        }
        return SampleKind::VmDispatchKnown;
    }

    // Priority 2: Webpack bundle patterns
    if raw_signals.has_webpack_require {
        return SampleKind::WebpackRuntime;
    }

    // Priority 3: Eval-heavy
    if raw_signals.eval_call_count >= 3 {
        return SampleKind::EvalHeavy;
    }

    // Priority 4: Closure-captured runtime (heuristic)
    if raw_signals.has_early_reference_capture {
        return SampleKind::ClosureCapturedRuntime;
    }

    // Fallback: plain script
    SampleKind::PlainScript
}

/// Extract notable signals from source text for classifier and diagnostics.
pub fn extract_signals(source: &str) -> Vec<String> {
    let set = SignalSet::from_source(source);
    let mut signals = Vec::new();

    if set.has_webpack_require {
        signals.push("webpack_require".into());
    }
    if set.has_chaosvm_dispatch {
        signals.push("chaosvm_dispatch".into());
    }
    if set.has_switch_vm {
        signals.push("switch_vm".into());
    }
    if set.eval_call_count > 0 {
        signals.push(format!("eval_calls={}", set.eval_call_count));
    }
    if set.has_early_reference_capture {
        signals.push("early_reference_capture".into());
    }
    if !set.dispatch_extractable {
        signals.push("dispatch_not_extractable".into());
    }

    signals
}

// ───
// Internal pattern analysis
// ───

struct SignalSet {
    has_webpack_require: bool,
    has_chaosvm_dispatch: bool,
    has_switch_vm: bool,
    dispatch_extractable: bool,
    eval_call_count: usize,
    has_early_reference_capture: bool,
}

impl SignalSet {
    fn from_source(source: &str) -> Self {
        let has_webpack_require = source.contains("__webpack_require__")
            || source.contains("webpackChunk")
            || source.contains("window.webpackJsonp");
        let has_chaosvm_dispatch = detect_chaosvm(source);
        let has_switch_vm = detect_switch_vm(source);
        let dispatch_extractable = has_chaosvm_dispatch || has_switch_vm;
        let eval_call_count = count_eval_calls(source);
        let has_early_reference_capture = detect_early_capture(source);

        SignalSet {
            has_webpack_require,
            has_chaosvm_dispatch,
            has_switch_vm,
            dispatch_extractable,
            eval_call_count,
            has_early_reference_capture,
        }
    }
}

/// Detect ChaosVM pattern: X[Y[Z++]]()
fn detect_chaosvm(source: &str) -> bool {
    let bytes = source.as_bytes();
    if bytes.len() < 9 {
        return false;
    }
    for pos in 0..bytes.len().saturating_sub(8) {
        if bytes[pos] != b'[' {
            continue;
        }
        // Check subsequent bytes for the pattern: X[Y[Z++]]()
        // Look for second '[' somewhere after position 1
        let rest = &bytes[pos + 1..];
        let Some(second_brk) = rest.iter().position(|&b| b == b'[') else {
            continue;
        };
        if second_brk > 30 {
            continue;
        }
        let sb = second_brk + 1;
        let after_sb = &bytes[pos + 1 + sb..];
        // Look for "++"
        let Some(pp) = after_sb
            .windows(2)
            .position(|w| w[0] == b'+' && w[1] == b'+')
        else {
            continue;
        };
        let inc_pos = pp + 2;
        let after_inc = &bytes[pos + 1 + sb + inc_pos..];
        // Check for "]]" followed by a call with any argument list.
        if after_inc.len() >= 3
            && after_inc[0] == b']'
            && after_inc[1] == b']'
            && after_inc[2..]
                .iter()
                .copied()
                .find(|b| !b.is_ascii_whitespace())
                == Some(b'(')
        {
            return true;
        }
    }
    false
}

/// Detect switch-VM pattern: switch(X[Y++]) or switch(X[Y]) with bytecode array
fn detect_switch_vm(source: &str) -> bool {
    // Look for `switch(` followed closely by `[` and `++`
    if let Some(sw_pos) = source.find("switch(") {
        let after_switch = &source[sw_pos + 7..];
        // Check if there's array indexing with increment within the switch expression
        if after_switch.contains('[')
            && (after_switch.contains("++") || after_switch.contains("++]"))
        {
            return true;
        }
    }
    false
}

/// Count eval / Function constructor calls.
fn count_eval_calls(source: &str) -> usize {
    let mut count = 0;
    let mut pos = 0;
    while pos < source.len() {
        // Look for "eval("
        if let Some(p) = source[pos..].find("eval(") {
            count += 1;
            pos += p + 5;
        } else {
            break;
        }
    }
    pos = 0;
    // Also count "Function(" — this is more aggressive
    while pos < source.len() {
        if let Some(p) = source[pos..].find("Function(") {
            count += 1;
            pos += p + 9;
        } else {
            break;
        }
    }
    count
}

/// Heuristic: check for patterns that suggest early reference capture.
/// E.g. `var R = navigator;` before an IIFE.
pub fn detect_early_capture(source: &str) -> bool {
    // Look for global-object reference captured before a function/IIFE
    let globals = ["navigator", "screen", "document", "window", "location"];
    let mut has_capture_before_iife = false;
    if let Some(iife_pos) = source.find("function(") {
        let prefix = &source[..iife_pos];
        for g in &globals {
            if prefix.contains(g) {
                has_capture_before_iife = true;
                break;
            }
        }
    } else if let Some(iife_pos) = source.find("(()=>") {
        let prefix = &source[..iife_pos];
        for g in &globals {
            if prefix.contains(g) {
                has_capture_before_iife = true;
                break;
            }
        }
    }
    has_capture_before_iife
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_script() {
        assert_eq!(classify("var x = 1 + 1;", &[]), SampleKind::PlainScript);
    }

    #[test]
    fn test_chaosvm_dispatch() {
        let src = "(function() { var A = []; var Q = [0,1]; var U = 0; return A[Q[U++]](); })();";
        assert_eq!(classify(src, &[]), SampleKind::VmDispatchKnown);
    }

    #[test]
    fn test_chaosvm_dispatch_with_args() {
        let src = "var r = A[Q[U++]](stack, ctx);";
        assert_eq!(classify(src, &[]), SampleKind::VmDispatchKnown);
    }

    #[test]
    fn test_switch_vm() {
        let src = "switch(B[P++]) { case 0: break; }";
        assert_eq!(classify(src, &[]), SampleKind::VmDispatchKnown);
    }

    #[test]
    fn test_webpack_runtime() {
        let src = "var r = __webpack_require__(42);";
        assert_eq!(classify(src, &[]), SampleKind::WebpackRuntime);
    }

    #[test]
    fn test_webpack_vm_hybrid() {
        let src = "__webpack_require__(7); A[Q[U++]]();";
        assert_eq!(classify(src, &[]), SampleKind::WebpackVmHybrid);
    }

    #[test]
    fn test_eval_heavy() {
        let src = "eval('a'); eval('b'); Function('c'); eval('d');";
        assert_eq!(classify(src, &[]), SampleKind::EvalHeavy);
    }

    #[test]
    fn test_extract_signals_webpack() {
        let signals = extract_signals("__webpack_require__(1);");
        assert!(signals.contains(&"webpack_require".into()));
    }

    #[test]
    fn test_extract_signals_chaosvm() {
        let signals = extract_signals("A[Q[U++]]()");
        assert!(signals.contains(&"chaosvm_dispatch".into()));
    }
}
