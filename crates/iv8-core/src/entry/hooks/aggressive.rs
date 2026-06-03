//! Aggressive Runtime Hook Pack.
//!
//! Design principles (from V0.6_ARCHITECTURE_MID_DRAFT §10.5.2):
//! - May use Proxy on global objects
//! - May intercept call/apply/eval/Function at runtime
//! - Only usable via analysis persona explicit opt-in
//! - All evidence must be tagged as "aggressive"

/// Generate the aggressive hook JS prelude.
pub fn prelude() -> String {
    r#"
(function() {
    if (typeof globalThis.__iv8_aggressive_installed !== 'undefined') return;
    globalThis.__iv8_aggressive_installed = true;

    var __iv8_log = [];
    __iv8_log.push('!!aggressive_mode');

    // 1. Proxy-based global call interception
    var origCall = Function.prototype.call;
    var origApply = Function.prototype.apply;
    var origBind = Function.prototype.bind;

    Function.prototype.call = function(thisArg) {
        __iv8_log.push('aggressive_call,' + (this.name || 'anonymous'));
        return origCall.apply(this, arguments);
    };

    Function.prototype.apply = function(thisArg, argsArray) {
        __iv8_log.push('aggressive_apply,' + (this.name || 'anonymous'));
        return origApply.apply(this, arguments);
    };

    Function.prototype.bind = function(thisArg) {
        __iv8_log.push('aggressive_bind,' + (this.name || 'anonymous'));
        return origBind.apply(this, arguments);
    };

    // 2. Full eval/Function interception
    var origEval = globalThis.eval;
    globalThis.eval = function(code) {
        __iv8_log.push('aggressive_eval,' + String(code).substring(0, 300));
        return origEval.apply(this, arguments);
    };

    var origFn = globalThis.Function;
    globalThis.Function = function() {
        var body = Array.prototype.slice.call(arguments).pop() || '';
        __iv8_log.push('aggressive_fn,' + body.substring(0, 300));
        return origFn.apply(this, arguments);
    };
    Object.keys(origFn).forEach(function(k) { globalThis.Function[k] = origFn[k]; });

    // 3. Proxy on globalThis for full property access trace
    try {
        var handler = {
            get: function(target, prop, receiver) {
                var val = Reflect.get(target, prop, receiver);
                if (typeof prop === 'string' && prop !== 'Symbol'
                    && prop !== '__iv8_log' && prop !== '__iv8_runtime_log'
                    && !prop.startsWith('__iv8')) {
                    __iv8_log.push('aggressive_get,' + prop + ',' + typeof val);
                }
                return val;
            },
            set: function(target, prop, value) {
                __iv8_log.push('aggressive_set,' + prop);
                return Reflect.set(target, prop, value);
            }
        };
        globalThis.__iv8_orig_global = Object.assign({}, globalThis);
        // Only wrap in Proxy if we can intercept safely
        // (Skip actual Proxy install for now — too risky for most targets)
    } catch(e) {
        __iv8_log.push('aggressive_proxy_failed,' + String(e));
    }

    globalThis.__iv8_runtime_log = __iv8_log;
})();
"#
    .to_string()
}

/// Extract evidence collected by the aggressive hook.
pub fn collect(runtime_log: &[String]) -> serde_json::Value {
    let mut evals = Vec::new();
    let mut fn_ctors = Vec::new();
    let mut calls = Vec::new();
    let mut props = Vec::new();

    for entry in runtime_log {
        if entry == "!!aggressive_mode" {
            continue;
        }
        if let Some(val) = entry.strip_prefix("aggressive_eval,") {
            evals.push(val.to_string());
        } else if let Some(val) = entry.strip_prefix("aggressive_fn,") {
            fn_ctors.push(val.to_string());
        } else if let Some(val) = entry.strip_prefix("aggressive_call,") {
            calls.push(format!("call:{}", val));
        } else if let Some(val) = entry.strip_prefix("aggressive_apply,") {
            calls.push(format!("apply:{}", val));
        } else if let Some(val) = entry.strip_prefix("aggressive_bind,") {
            calls.push(format!("bind:{}", val));
        } else if entry.starts_with("aggressive_") {
            props.push(entry.to_string());
        }
    }

    serde_json::json!({
        "hook_mode": "aggressive",
        "eval_captures": evals,
        "fn_ctor_captures": fn_ctors,
        "call_intercepts": calls,
        "other_events": props,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude_marks_aggressive() {
        let js = prelude();
        assert!(js.contains("!!aggressive_mode"));
    }

    #[test]
    fn test_collect_marks_mode() {
        let result = collect(&[]);
        assert_eq!(result["hook_mode"], "aggressive");
    }

    #[test]
    fn test_collect_parses_entries() {
        let log = vec![
            "!!aggressive_mode".into(),
            "aggressive_eval,1+1".into(),
            "aggressive_call,test".into(),
        ];
        let result = collect(&log);
        assert_eq!(result["eval_captures"].as_array().unwrap().len(), 1);
        assert_eq!(result["call_intercepts"].as_array().unwrap().len(), 1);
    }
}
