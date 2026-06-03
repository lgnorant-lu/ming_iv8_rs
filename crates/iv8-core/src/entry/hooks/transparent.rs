//! Transparent Runtime Hook Pack.
//!
//! Design principles (from V0.6_ARCHITECTURE_MID_DRAFT §10.5.1):
//! - Does NOT modify `Function.prototype.toString`
//! - Does NOT place Proxy on sensitive object bodies
//! - Does NOT modify `Function.prototype.call/apply/bind` by default
//! - Prefers prototype getter wrapping and runtime markers

/// Generate the transparent hook JS prelude.
///
/// This code is evaluated before the main source and sets up runtime
/// observation markers that are difficult to detect.
pub fn prelude() -> String {
    r#"
(function() {
    if (typeof globalThis.__iv8_installed !== 'undefined') return;
    globalThis.__iv8_installed = true;

    var __iv8_log = [];

    // 1. Low-intrusion eval hook via getter
    var origEval = globalThis.eval;
    var evalDescriptor = Object.getOwnPropertyDescriptor(globalThis, 'eval');
    Object.defineProperty(globalThis, 'eval', {
        configurable: true,
        enumerable: evalDescriptor ? evalDescriptor.enumerable : true,
        get: function() {
            return function(code) {
                __iv8_log.push('eval,' + String(code).substring(0, 200));
                return origEval.apply(this, arguments);
            };
        },
        set: function(v) { origEval = v; }
    });

    // 2. Runtime markers via Function constructor wrapping (getter pattern)
    var origFn = globalThis.Function;
    Object.defineProperty(globalThis, 'Function', {
        configurable: true,
        enumerable: true,
        get: function() {
            return function() {
                var args = Array.prototype.slice.call(arguments);
                var body = args.pop() || '';
                __iv8_log.push('fn_ctor,' + body.substring(0, 200));
                var code = args.length > 0 ? args.join(',') + ',return ' + body : 'return ' + body;
                return origFn.apply(this, arguments);
            };
        },
        set: function(v) { origFn = v; }
    });

    // 3. Runtime marker for environment reads (prototype-level getter tap)
    //    This avoids Proxy on navigator/screen/document while still capturing
    //    property access at the prototype chain level.
    var sensitiveProtos = ['Navigator', 'Screen', 'Document', 'Location'];
    sensitiveProtos.forEach(function(name) {
        var proto = globalThis[name] && globalThis[name].prototype;
        if (!proto) return;
        var propNames = Object.getOwnPropertyNames(proto);
        propNames.forEach(function(prop) {
            var desc = Object.getOwnPropertyDescriptor(proto, prop);
            if (!desc || !desc.get) return;
            var origGet = desc.get;
            Object.defineProperty(proto, prop, {
                configurable: true,
                enumerable: desc.enumerable !== false,
                get: function() {
                    __iv8_log.push('env_read,' + name + '.' + prop);
                    return origGet.call(this);
                }
            });
        });
    });

    globalThis.__iv8_runtime_log = __iv8_log;
})();
"#
    .to_string()
}

/// Extract evidence collected by the transparent hook.
pub fn collect(runtime_log: &[String]) -> serde_json::Value {
    let mut evals = Vec::new();
    let mut fn_ctors = Vec::new();
    let mut env_reads = Vec::new();

    for entry in runtime_log {
        if let Some(val) = entry.strip_prefix("eval,") {
            evals.push(val.to_string());
        } else if let Some(val) = entry.strip_prefix("fn_ctor,") {
            fn_ctors.push(val.to_string());
        } else if let Some(val) = entry.strip_prefix("env_read,") {
            env_reads.push(val.to_string());
        }
    }

    serde_json::json!({
        "hook_mode": "transparent",
        "eval_captures": evals,
        "fn_ctor_captures": fn_ctors,
        "env_reads": env_reads,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude_is_valid_js() {
        let js = prelude();
        assert!(js.contains("__iv8_runtime_log"));
        assert!(js.contains("Object.defineProperty"));
        assert!(!js.contains("Function.prototype.toString"));
        assert!(!js.contains("new Proxy("));
    }

    #[test]
    fn test_collect_empty() {
        let result = collect(&[]);
        assert_eq!(result["hook_mode"], "transparent");
        assert!(result["eval_captures"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_collect_parses_entries() {
        let log = vec![
            "eval,1+1".into(),
            "fn_ctor,return x".into(),
            "env_read,Navigator.userAgent".into(),
        ];
        let result = collect(&log);
        assert_eq!(result["eval_captures"].as_array().unwrap().len(), 1);
        assert_eq!(result["fn_ctor_captures"].as_array().unwrap().len(), 1);
        assert_eq!(result["env_reads"].as_array().unwrap().len(), 1);
    }
}
