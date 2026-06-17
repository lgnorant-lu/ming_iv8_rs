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

    // Priority 3: Browserify strong (prelude call shape)
    if raw_signals.has_browserify_strong {
        return SampleKind::BrowserifyRuntime;
    }

    // Priority 4: Vite IIFE bundle
    if raw_signals.has_vite {
        return SampleKind::ViteBundle;
    }

    // Priority 5: Rollup IIFE bundle (PURE annotations, interop helpers)
    if raw_signals.has_rollup_iife {
        return SampleKind::RollupBundle;
    }

    // Priority 6: UMD bundle (AMD/CJS/global branch chain)
    if raw_signals.has_rollup_umd {
        return SampleKind::UmdBundle;
    }

    // Priority 7: Browserify weak (CommonJS wrappers, no prelude)
    if raw_signals.has_browserify_weak {
        return SampleKind::BrowserifyRuntime;
    }

    // Priority 8: Eval-heavy
    if raw_signals.eval_call_count >= 3 {
        return SampleKind::EvalHeavy;
    }

    // Priority 9: Closure-captured runtime (heuristic)
    if raw_signals.has_early_reference_capture {
        return SampleKind::ClosureCapturedRuntime;
    }

    // Priority 10: Unknown IIFE wrapper
    if raw_signals.has_iife_wrapper {
        return SampleKind::UnknownIife;
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
    if set.has_browserify_strong {
        signals.push("browserify_strong".into());
    }
    if set.has_browserify_weak {
        signals.push("browserify_weak".into());
    }
    if set.has_rollup_iife {
        signals.push("rollup_iife".into());
    }
    if set.has_rollup_umd {
        signals.push("rollup_umd".into());
    }
    if set.has_vite {
        signals.push("vite".into());
    }
    if set.has_iife_wrapper {
        signals.push("iife_wrapper".into());
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
    has_browserify_strong: bool,
    has_browserify_weak: bool,
    has_rollup_iife: bool,
    has_rollup_umd: bool,
    has_vite: bool,
    has_iife_wrapper: bool,
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
        let has_browserify_strong = detect_browserify_strong(source);
        let has_browserify_weak = !has_browserify_strong && detect_browserify_weak(source);
        let has_rollup_iife = detect_rollup_iife(source);
        let has_rollup_umd = detect_rollup_umd(source);
        let has_vite = detect_vite(source);
        let has_iife_wrapper = detect_iife_wrapper(source);

        SignalSet {
            has_webpack_require,
            has_chaosvm_dispatch,
            has_switch_vm,
            dispatch_extractable,
            eval_call_count,
            has_early_reference_capture,
            has_browserify_strong,
            has_browserify_weak,
            has_rollup_iife,
            has_rollup_umd,
            has_vite,
            has_iife_wrapper,
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

/// Browserify strong signal: prelude call shape `({id:[fn,{deps}]},{},[entry])`.
/// Looks for the `},{},` separator pattern characteristic of browser-pack prelude.
fn detect_browserify_strong(source: &str) -> bool {
    let has_require_wrapper = source.contains("function(require,module,exports)")
        || source.contains("function(require, module, exports)");
    if !has_require_wrapper {
        return false;
    }
    let has_prelude_separator = source.contains("},{},[")
        || source.contains("},{},{},[");
    has_prelude_separator
}

/// Browserify weak signal: CommonJS `function(require,module,exports)` wrappers
/// without the full prelude call shape.
fn detect_browserify_weak(source: &str) -> bool {
    source.contains("function(require,module,exports)")
        || source.contains("function(require, module, exports)")
}

/// Rollup IIFE signal: PURE annotations or interop namespace helpers.
fn detect_rollup_iife(source: &str) -> bool {
    let has_pure = source.contains("/*#__PURE__*/");
    let has_interop = source.contains("_interopNamespace")
        || source.contains("_interopRequireDefault")
        || source.contains("_interopRequireWildcard");
    (has_pure || has_interop) && !source.contains("__webpack_require__")
}

/// Rollup UMD signal: AMD + CJS + global three-branch chain.
fn detect_rollup_umd(source: &str) -> bool {
    let has_amd = (source.contains("typeof define === 'function'")
        || source.contains("typeof define==='function'"))
        && source.contains("define.amd");
    let has_cjs = source.contains("typeof module === 'object'")
        || source.contains("typeof module==='object'")
        || source.contains("typeof exports === 'object'")
        || source.contains("typeof exports==='object'");
    let has_global = source.contains("factory(global)")
        || source.contains("factory(globalThis)")
        || source.contains("})(global,")
        || source.contains("})(globalThis,")
        || source.contains("}(global,")
        || source.contains("}(globalThis,")
        || source.contains("typeof globalThis")
        || source.contains("typeof global")
        || source.contains("global.");
    has_amd && has_cjs && has_global
}

/// Vite signal: Vite-specific preload helpers or modern flag.
fn detect_vite(source: &str) -> bool {
    let has_vite = source.contains("__vitePreload")
        || source.contains("__VITE_IS_MODERN__")
        || source.contains("__vite__mapDeps");
    has_vite && !source.contains("__webpack_require__")
}

/// IIFE wrapper detection: self-executing function pattern.
/// Strips leading `//` comment lines before checking for IIFE prefix.
fn detect_iife_wrapper(source: &str) -> bool {
    let mut trimmed = source.trim_start();
    while trimmed.starts_with("//") || trimmed.starts_with(' ') || trimmed.starts_with('\n') || trimmed.starts_with('\r') {
        if let Some(lf) = trimmed.find('\n') {
            trimmed = trimmed[lf + 1..].trim_start();
        } else {
            break;
        }
    }
    trimmed.starts_with("(function(")
        || trimmed.starts_with("!function(")
        || trimmed.starts_with("(function(){")
        || trimmed.starts_with("(()=>")
        || trimmed.starts_with("(() =>")
        || trimmed.starts_with(";(function(")
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

    #[test]
    fn test_browserify_strong() {
        let src = r#"(function(){var e={};function r(){return o;}return r})()({1:[function(require,module,exports){module.exports=42},{"dep":2}],2:[function(require,module,exports){}]},{},[1])"#;
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::BrowserifyRuntime);
    }

    #[test]
    fn test_browserify_weak() {
        let src = r#"function(require,module,exports){ module.exports = function(){}; }"#;
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::BrowserifyRuntime);
    }

    #[test]
    fn test_rollup_iife() {
        let src = "var a=/*#__PURE__*/function(){return 1}();";
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::RollupBundle);
    }

    #[test]
    fn test_rollup_interop() {
        let src = "function _interopNamespace(e){if(e&&e.__esModule)return e;var n=Object.create(null);return e&&Object.keys(e).forEach(function(r){var t=Object.getOwnPropertyDescriptor(e,r);Object.defineProperty(n,r,t.get?t:{enumerable:true,get:function(){return e[r]}})}),n.default=e,n}";
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::RollupBundle);
    }

    #[test]
    fn test_rollup_umd() {
        let src = r#"(function(root,factory){if(typeof define==='function'&&define.amd){define([],factory)}else if(typeof module==='object'&&module.exports){module.exports=factory()}else{root.MyLib=factory()}})(global,function(){return{version:'1.0'}});"#;
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::UmdBundle);
    }

    #[test]
    fn test_vite() {
        let src = "const __vitePreload=function(url,dep,as){return Promise.resolve()};";
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::ViteBundle);
    }

    #[test]
    fn test_vite_modern() {
        let src = "const __VITE_IS_MODERN__=true;";
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::ViteBundle);
    }

    #[test]
    fn test_iife_wrapper() {
        let src = "(function(){return 42;})()";
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::UnknownIife);
    }

    #[test]
    fn test_arrow_iife_wrapper() {
        let src = "(()=>{return 42;})()";
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::UnknownIife);
    }

    #[test]
    fn test_extract_signals_browserify() {
        let s = extract_signals(r#"(function(){var e={};function r(){return o;}return r})()({1:[function(require,module,exports){module.exports=42}]},{},[1])"#);
        assert!(s.contains(&"browserify_strong".into()));
    }

    #[test]
    fn test_extract_signals_rollup() {
        let s = extract_signals("var a=/*#__PURE__*/function(){return 1}();");
        assert!(s.contains(&"rollup_iife".into()));
    }

    #[test]
    fn test_extract_signals_vite() {
        let s = extract_signals("const __vitePreload=function(u,d,a){return Promise.resolve()};");
        assert!(s.contains(&"vite".into()));
    }

    #[test]
    fn test_extract_signals_iife() {
        let s = extract_signals("(function(){return 42;})()");
        assert!(s.contains(&"iife_wrapper".into()));
    }

    #[test]
    fn test_browserify_not_false_positive_webpack() {
        let src = "__webpack_require__(1); function(require,module,exports){}";
        let kind = classify(src, &[]);
        assert_eq!(kind, SampleKind::WebpackRuntime);
    }
}
