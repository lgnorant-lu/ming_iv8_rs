//! Source code instrumentation for JSVMP (ChaosVM / switch-VM) tracing.
//!
//! Strategy (validated by TDC real-world testing; v0.8.101 Q165 path A robust):
//! 1. **Dispatch expression replacement** (all sites): rewrite every
//!    `H[I[P++]]()` (and whitespace variants) with a logging wrapper so each
//!    VM iteration is traced — works for **closure-scoped** handler tables
//!    without needing a global Proxy (unlike `instrument_chaosvm`).
//! 2. **Source-head Proxy injection**: Prepend global object Proxies at the
//!    very start of source (before ChaosVM IIFE captures references).
//!
//! Output format: "TYPE,PC,target,value" where TYPE is D/R/C/W.
//!
//! Why not V8 internal closure hooks: high cost / fragile / detection surface;
//! path A already captures dispatch without attaching to closed-over locals.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

/// Detect JSVMP pattern and inject unified tracing code.
///
/// Strategy:
/// - Replaces **all** matching dispatch expressions with a logging wrapper
/// - Prepends global object Proxies at source start (captures env reads with PC)
///
/// Returns (patched_source, vm_info_dict) or raises RuntimeError if detection fails.
///
/// Recommended product path for TDC/ChaosVM (Q165 path A). Prefer this over
/// `JSContext.instrument_chaosvm` when the handler table is closure-scoped.
#[pyfunction]
#[pyo3(signature = (
    source,
    mode = "auto",
    capture_stack_depth = 3,
    capture_env = true,
    env_targets = None,
    limit = 100000,
    handler_array = None,
    pc_var = None,
    stack_var = None,
    index_array = None,
    dispatch_pattern = None,
    expose_handlers = false,
))]
pub fn instrument_source(
    source: &str,
    mode: &str,
    capture_stack_depth: u32,
    capture_env: bool,
    env_targets: Option<Vec<String>>,
    limit: u32,
    handler_array: Option<&str>,
    pc_var: Option<&str>,
    stack_var: Option<&str>,
    index_array: Option<&str>,
    dispatch_pattern: Option<&str>,
    expose_handlers: bool,
    py: Python<'_>,
) -> PyResult<(String, PyObject)> {
    // Step 1: Detect or use manual overrides
    let detection = if let (Some(ha), Some(pc), Some(sv)) = (handler_array, pc_var, stack_var) {
        let ia = index_array.unwrap_or("");
        let pattern = dispatch_pattern
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                if ia.is_empty() {
                    format!("{}[{}++]", ha, pc)
                } else {
                    format!("{}[{}[{}++]]()", ha, ia, pc)
                }
            });
        // Prefer exact find; if missing, fall back to regex first match for offsets
        let (dispatch_offset, dispatch_pattern) =
            if let Some(off) = source.find(&pattern) {
                (off, pattern)
            } else if let Some(d) = detect_chaosvm(source) {
                (d.dispatch_offset, d.dispatch_pattern)
            } else {
                (0, pattern)
            };
        VmDetection {
            handler_array: ha.to_string(),
            index_array: ia.to_string(),
            pc_var: pc.to_string(),
            stack_var: sv.to_string(),
            mode: if mode == "auto" {
                "chaosvm".to_string()
            } else {
                mode.to_string()
            },
            dispatch_pattern,
            dispatch_offset,
        }
    } else {
        let detected = match mode {
            "chaosvm" => detect_chaosvm(source),
            "switch_vm" => detect_switch_vm(source),
            "auto" => detect_chaosvm(source).or_else(|| detect_switch_vm(source)),
            _ => None,
        };
        detected.ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err(
                "instrument_source: could not detect JSVMP dispatch pattern. \
                 Use manual parameters (handler_array, pc_var, stack_var, index_array).",
            )
        })?
    };

    // Step 2: Generate source-head Proxy code (for env tracking).
    // Default = full common surface list. Control via:
    //   - capture_env=false  → no env Proxies
    //   - env_targets=[...]  → exact allow-list of global names to wrap
    // Host-object safe: Reflect.get(target, prop, target) so brand-checked
    // getters (screen.width, navigator.userAgent, …) do not Illegal invocation.
    let targets = env_targets.unwrap_or_else(|| {
        vec![
            "navigator".into(),
            "screen".into(),
            "document".into(),
            "location".into(),
            "Math".into(),
            "crypto".into(),
            "performance".into(),
        ]
    });
    let targets_json = serde_json::to_string(&targets).unwrap_or("[]".into());

    let head_code = generate_head_code(capture_env, &targets_json, limit);

    // Step 3: Replace ALL dispatch sites (v0.8.101 robust) + fix offset==0 bug
    let dispatch_replacement =
        generate_dispatch_replacement(&detection, capture_stack_depth, expose_handlers);
    let (body, sites) = replace_all_dispatches(source, &detection, &dispatch_replacement);
    let patched = format!("{}{}", head_code, body);

    // Step 4: Build vm_info dict
    let info = PyDict::new(py);
    info.set_item("handler_array", &detection.handler_array)?;
    info.set_item("index_array", &detection.index_array)?;
    info.set_item("pc_var", &detection.pc_var)?;
    info.set_item("stack_var", &detection.stack_var)?;
    info.set_item("mode", &detection.mode)?;
    info.set_item("dispatch_pattern", &detection.dispatch_pattern)?;
    info.set_item("dispatch_offset", detection.dispatch_offset)?;
    info.set_item("dispatch_count", sites.len())?;
    let offsets = PyList::empty(py);
    for off in &sites {
        offsets.append(*off)?;
    }
    info.set_item("dispatch_offsets", offsets)?;
    info.set_item("head_code_length", head_code.len())?;
    info.set_item("recommended_api", "instrument_source")?;
    info.set_item(
        "q165_note",
        "path A: source rewrite works for closure-scoped handlers; \
         instrument_chaosvm requires global handler table",
    )?;
    info.set_item(
        "env_targets",
        if capture_env {
            targets.clone()
        } else {
            Vec::<String>::new()
        },
    )?;
    info.set_item("capture_env", capture_env)?;
    info.set_item("expose_handlers", expose_handlers)?;
    info.set_item(
        "env_proxy_note",
        "default env_targets = full list (navigator/screen/document/location/Math/crypto/performance); \
         override with env_targets=[...] allow-list, or capture_env=false to disable. \
         Proxies use Reflect.get/set(target, prop, target) for host-object brand safety.",
    )?;
    if expose_handlers {
        info.set_item(
            "expose_handlers_note",
            "on each dispatch, assigns globalThis.__iv8_vm_handlers__ = <handler_array> \
             (in-scope local, no V8 closure hook). Default false (detection surface).",
        )?;
    }

    Ok((patched, info.into_any().unbind()))
}

/// Replace every ChaosVM (or exact) dispatch occurrence. Returns (body, offsets).
fn replace_all_dispatches(
    source: &str,
    detection: &VmDetection,
    replacement: &str,
) -> (String, Vec<usize>) {
    let mut sites: Vec<(usize, usize)> = Vec::new();

    if detection.mode == "chaosvm" {
        // Primary: standard H[I[P++]]()
        if let Ok(re) = regex_lite::Regex::new(
            r"([A-Za-z_$][A-Za-z0-9_$]*)\[([A-Za-z_$][A-Za-z0-9_$]*)\[([A-Za-z_$][A-Za-z0-9_$]*)\+\+\]\]\(\)",
        ) {
            for m in re.find_iter(source) {
                // Only rewrite sites matching detected handler/index/pc when possible
                if let Some(caps) = re.captures(m.as_str()) {
                    let ha = caps.get(1).map(|x| x.as_str()).unwrap_or("");
                    let ia = caps.get(2).map(|x| x.as_str()).unwrap_or("");
                    let pc = caps.get(3).map(|x| x.as_str()).unwrap_or("");
                    if ha == detection.handler_array
                        && (detection.index_array.is_empty() || ia == detection.index_array)
                        && pc == detection.pc_var
                    {
                        sites.push((m.start(), m.end()));
                    }
                }
            }
        }
        // Whitespace-tolerant variant: H [ I [ P ++ ] ] ( )
        if sites.is_empty() {
            if let Ok(re) = regex_lite::Regex::new(
                r"([A-Za-z_$][A-Za-z0-9_$]*)\s*\[\s*([A-Za-z_$][A-Za-z0-9_$]*)\s*\[\s*([A-Za-z_$][A-Za-z0-9_$]*)\s*\+\+\s*\]\s*\]\s*\(\s*\)",
            ) {
                for m in re.find_iter(source) {
                    if let Some(caps) = re.captures(m.as_str()) {
                        let ha = caps.get(1).map(|x| x.as_str()).unwrap_or("");
                        let ia = caps.get(2).map(|x| x.as_str()).unwrap_or("");
                        let pc = caps.get(3).map(|x| x.as_str()).unwrap_or("");
                        if ha == detection.handler_array
                            && (detection.index_array.is_empty() || ia == detection.index_array)
                            && pc == detection.pc_var
                        {
                            sites.push((m.start(), m.end()));
                        }
                    }
                }
            }
        }
    }

    // Exact substring fallback (manual pattern / switch leave-as-is)
    if sites.is_empty() && !detection.dispatch_pattern.is_empty() {
        let pat = &detection.dispatch_pattern;
        let mut start = 0;
        while let Some(rel) = source[start..].find(pat) {
            let abs = start + rel;
            sites.push((abs, abs + pat.len()));
            start = abs + pat.len();
        }
    }

    if sites.is_empty() {
        return (source.to_string(), Vec::new());
    }

    // Rebuild from left to right
    let mut out = String::with_capacity(source.len() + replacement.len() * sites.len());
    let mut last = 0;
    let mut offsets = Vec::with_capacity(sites.len());
    for (s, e) in &sites {
        out.push_str(&source[last..*s]);
        offsets.push(out.len()); // offset in patched body (before head)
        out.push_str(replacement);
        last = *e;
    }
    out.push_str(&source[last..]);
    (out, offsets)
}

// ─── Internal types ──────────────────────────────────────────────────────────

struct VmDetection {
    handler_array: String,
    index_array: String,
    pc_var: String,
    stack_var: String,
    mode: String,
    dispatch_pattern: String,
    dispatch_offset: usize,
}

// ─── Detection logic ─────────────────────────────────────────────────────────

/// Detect ChaosVM pattern: A[Q[U++]]()
fn detect_chaosvm(source: &str) -> Option<VmDetection> {
    // Pattern: X[Y[Z++]]() — handler_array[index_array[pc++]]()
    let re = regex_lite::Regex::new(
        r"([A-Za-z_$][A-Za-z0-9_$]*)\[([A-Za-z_$][A-Za-z0-9_$]*)\[([A-Za-z_$][A-Za-z0-9_$]*)\+\+\]\]\(\)"
    ).ok()?;

    let caps = re.captures(source)?;
    let handler_array = caps.get(1)?.as_str().to_string();
    let index_array = caps.get(2)?.as_str().to_string();
    let pc_var = caps.get(3)?.as_str().to_string();
    let full_match = caps.get(0)?;
    let dispatch_pattern = full_match.as_str().to_string();
    let dispatch_offset = full_match.start();

    // Find stack variable
    let stack_var = detect_stack_var(source, dispatch_offset);

    Some(VmDetection {
        handler_array,
        index_array,
        pc_var,
        stack_var,
        mode: "chaosvm".to_string(),
        dispatch_pattern,
        dispatch_offset,
    })
}

/// Detect switch-based VM: switch(X[Y++]) { case ... }
fn detect_switch_vm(source: &str) -> Option<VmDetection> {
    let re = regex_lite::Regex::new(
        r"switch\s*\(\s*([A-Za-z_$][A-Za-z0-9_$]*)\[([A-Za-z_$][A-Za-z0-9_$]*)\+\+\]\s*\)",
    )
    .ok()?;

    let caps = re.captures(source)?;
    let bytecode_array = caps.get(1)?.as_str().to_string();
    let pc_var = caps.get(2)?.as_str().to_string();
    let full_match = caps.get(0)?;
    let dispatch_pattern = full_match.as_str().to_string();
    let dispatch_offset = full_match.start();

    let stack_var = detect_stack_var(source, dispatch_offset);

    Some(VmDetection {
        handler_array: bytecode_array,
        index_array: String::new(),
        pc_var,
        stack_var,
        mode: "switch_vm".to_string(),
        dispatch_pattern,
        dispatch_offset,
    })
}

/// Find stack variable by searching for .push/.pop patterns near the dispatch
fn detect_stack_var(source: &str, near_offset: usize) -> String {
    let start = near_offset.saturating_sub(3000);
    let end = (near_offset + 3000).min(source.len());
    let window = &source[start..end];

    let re = regex_lite::Regex::new(r"([A-Za-z_$][A-Za-z0-9_$]*)\.push\(");
    if let Ok(re) = re {
        if let Some(caps) = re.captures(window) {
            if let Some(m) = caps.get(1) {
                return m.as_str().to_string();
            }
        }
    }
    "S".to_string()
}

// ─── Code generation ─────────────────────────────────────────────────────────

/// Generate the source-head code (Proxy wrappers for global objects).
/// This is prepended BEFORE the entire source so it runs before ChaosVM
/// captures any references.
fn generate_head_code(capture_env: bool, env_targets_json: &str, limit: u32) -> String {
    let mut code = String::new();

    // Always create the unified log array (non-enumerable to avoid detection by Object.keys)
    // __iv8i_cap__: stack capture switch. Default false (safe for init).
    // User sets `globalThis.__iv8i_cap__ = true` after init to enable stack value capture.
    code.push_str(&format!(
        ";(function(){{var g=globalThis;\
         Object.defineProperty(g,'__iv8i_log__',{{value:[],writable:true,enumerable:false,configurable:true}});\
         Object.defineProperty(g,'__iv8i_lim__',{{value:{limit},writable:true,enumerable:false,configurable:true}});\
         Object.defineProperty(g,'__iv8i_pc__',{{value:-1,writable:true,enumerable:false,configurable:true}});\
         Object.defineProperty(g,'__iv8i_cap__',{{value:false,writable:true,enumerable:false,configurable:true}});}})();\n",
        limit = limit
    ));

    if capture_env {
        // Reflect.get/set use **target** as receiver (not the Proxy). Native host
        // getters (Screen/Navigator/Location/…) brand-check `this`; using the Proxy
        // as receiver throws Illegal invocation and can break init (e.g. TDC setData).
        code.push_str(&format!(r#";(function(){{
var L=globalThis.__iv8i_log__,M=globalThis.__iv8i_lim__;
var T={targets};
T.forEach(function(nm){{
  var obj=globalThis[nm];
  if(!obj||(typeof obj!=='object'&&typeof obj!=='function'))return;
  try{{
    globalThis[nm]=new Proxy(obj,{{
      get:function(t,p,_r){{
        var v;
        try{{ v=Reflect.get(t,p,t); }}catch(_e){{ try{{ v=t[p]; }}catch(_e2){{ return; }} }}
        if(typeof p==='symbol'||p==='then'||p==='toJSON'||p==='constructor')return v;
        if(typeof v==='function'){{
          return function(){{
            var res;
            try{{ res=v.apply(t,arguments); }}catch(_e){{ res=v.apply(this,arguments); }}
            if(L.length<M)L.push('C,'+globalThis.__iv8i_pc__+','+nm+'.'+String(p)+','+String(res).slice(0,50));
            return res;
          }};
        }}
        if(L.length<M)L.push('R,'+globalThis.__iv8i_pc__+','+nm+'.'+String(p)+','+String(v).slice(0,50));
        return v;
      }},
      set:function(t,p,v,_r){{
        if(L.length<M)L.push('W,'+globalThis.__iv8i_pc__+','+nm+'.'+String(p)+','+String(v).slice(0,50));
        try{{ return Reflect.set(t,p,v,t); }}catch(_e){{ try{{ t[p]=v; return true; }}catch(_e2){{ return false; }} }}
      }}
    }});
  }}catch(e){{}}
}});
}})();
"#,
            targets = env_targets_json,
        ));
    }

    code
}

/// Generate the dispatch expression replacement.
///
/// For ChaosVM `A[Q[U++]]()` → `(globalThis.__iv8i_pc__=U, log D entry with stack top values, A[Q[U++]]())`
///
/// When `expose_handlers` is true, also assigns
/// `globalThis.__iv8_vm_handlers__ = <handler_array>` **inside the dispatch
/// expression** (local is in scope) — no V8 closure hooks.
fn generate_dispatch_replacement(
    detection: &VmDetection,
    capture_stack_depth: u32,
    expose_handlers: bool,
) -> String {
    let pc = &detection.pc_var;
    let stack = &detection.stack_var;

    if detection.mode == "chaosvm" {
        let ha = &detection.handler_array;
        let ia = &detection.index_array;

        // Guard: only access stack if it's defined (typeof check prevents ReferenceError
        // when auto-detection guessed wrong stack_var or it's not yet defined during init)
        let stack_guard = format!("typeof {stack}!=='undefined'", stack = stack);

        // Build the stack-value capture expression.
        // D entry format: D,pc,opcode_idx,stack_depth[,stack_top[,stack_top-1[,...]]]
        //
        // Stack values are gated by __iv8i_cap__ (deferred capture switch):
        // - __iv8i_cap__ = false (default): only record depth, no stack values
        //   → safe during VM init (reading stack elements can have side effects)
        // - __iv8i_cap__ = true (user sets after init): record depth + top N values
        //   → enables crypto constant detection in business-logic dispatches
        //
        // User workflow:
        //   ctx.eval(patched)                     # init (cap=false, safe)
        //   ctx.eval("__iv8i_cap__ = true")       # enable stack capture
        //   ctx.eval("TDC.getData(true)")         # business logic (stack values recorded)
        let stack_capture = if capture_stack_depth == 0 {
            // No stack capture requested at all: just depth
            format!(
                "'+(({sg})?{stack}.length:0)",
                sg = stack_guard,
                stack = stack
            )
        } else {
            // Deferred capture: depth always, values only when __iv8i_cap__ is true
            let mut parts = format!(
                "'+(({sg})?{stack}.length:0)",
                sg = stack_guard,
                stack = stack,
            );
            for i in 1..=capture_stack_depth {
                // Gate each stack value read with __iv8i_cap__
                parts.push_str(&format!(
                    "+','+(globalThis.__iv8i_cap__&&{sg}&&{stack}.length>={i}?{stack}[{stack}.length-{i}]:'')",
                    sg = stack_guard, stack = stack, i = i,
                ));
            }
            parts
        };

        let expose_prefix = if expose_handlers {
            format!("globalThis.__iv8_vm_handlers__={ha},", ha = ha)
        } else {
            String::new()
        };

        // Comma expression: (optional expose, set_pc, log, original_dispatch)
        format!(
            "({expose}globalThis.__iv8i_pc__={pc},\
             globalThis.__iv8i_log__.length<globalThis.__iv8i_lim__&&\
             globalThis.__iv8i_log__.push('D,'+{pc}+','+{ia}[{pc}]+',{stack_capture}),\
             {ha}[{ia}[{pc}++]]())",
            expose = expose_prefix,
            pc = pc,
            ia = ia,
            stack_capture = stack_capture,
            ha = ha,
        )
    } else {
        // switch_vm: can't easily replace switch expression, just prepend log
        // For switch VMs, the head code + manual hook is more appropriate
        detection.dispatch_pattern.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_chaosvm_standard_pattern() {
        let src = "function run(){ while(1){ B[g[D++]](); } }";
        let d = detect_chaosvm(src).expect("detect");
        assert_eq!(d.handler_array, "B");
        assert_eq!(d.index_array, "g");
        assert_eq!(d.pc_var, "D");
        assert_eq!(d.mode, "chaosvm");
        assert!(src[d.dispatch_offset..].starts_with("B[g[D++]]()"));
    }

    #[test]
    fn replace_all_dispatches_multi_site_and_offset_zero() {
        // First site at offset 0 — old code skipped rewrite when offset==0
        let src = "B[g[D++]]();foo();B[g[D++]]();";
        let d = detect_chaosvm(src).expect("detect");
        let rep = generate_dispatch_replacement(&d, 0, false);
        let (body, sites) = replace_all_dispatches(src, &d, &rep);
        assert_eq!(sites.len(), 2, "expected two dispatch sites, got {:?}", sites);
        // Replacement still contains B[g[D++]]() as the real call; ensure logging wraps it.
        assert!(body.contains("__iv8i_log__"), "replacement should log");
        // each site injects multiple __iv8i_log__ refs (length check + push)
        assert!(body.matches("__iv8i_log__").count() >= 2);
        assert!(body.starts_with("(globalThis.__iv8i_pc__="), "offset-0 site rewritten");
        assert!(body.contains(";foo();(globalThis.__iv8i_pc__="), "second site rewritten");
    }

    #[test]
    fn replace_ignores_other_handler_names() {
        let src = "B[g[D++]]();X[g[D++]]();";
        let d = detect_chaosvm(src).expect("detect");
        assert_eq!(d.handler_array, "B");
        let rep = "/*patched*/";
        let (body, sites) = replace_all_dispatches(src, &d, rep);
        assert_eq!(sites.len(), 1);
        assert!(body.contains("X[g[D++]]()"));
        assert!(body.contains("/*patched*/"));
    }

    #[test]
    fn expose_handlers_injects_global_assign() {
        let src = "B[g[D++]]();";
        let d = detect_chaosvm(src).expect("detect");
        let rep = generate_dispatch_replacement(&d, 0, true);
        assert!(
            rep.contains("globalThis.__iv8_vm_handlers__=B"),
            "expose rewrite missing: {rep}"
        );
        let rep_off = generate_dispatch_replacement(&d, 0, false);
        assert!(!rep_off.contains("__iv8_vm_handlers__"));
    }
}
