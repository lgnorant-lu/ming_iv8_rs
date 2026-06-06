//! Source code instrumentation for JSVMP (ChaosVM / switch-VM) tracing.
//!
//! Strategy (validated by TDC real-world testing):
//! 1. **Dispatch expression replacement**: Replace `A[Q[U++]]()` with
//!    `(log_push, A[Q[U++]]())` — captures EVERY iteration including recursive calls.
//! 2. **Source-head Proxy injection**: Prepend global object Proxies at the very
//!    start of source (before ChaosVM IIFE captures references).
//!
//! Output format: "TYPE,PC,target,value" where TYPE is D/R/C/W.

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Detect JSVMP pattern and inject unified tracing code.
///
/// Strategy:
/// - Replaces the dispatch expression (e.g. `A[Q[U++]]()`) with a logging wrapper
/// - Prepends global object Proxies at source start (captures env reads with PC)
///
/// Returns (patched_source, vm_info_dict) or raises RuntimeError if detection fails.
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
    py: Python<'_>,
) -> PyResult<(String, PyObject)> {
    // Step 1: Detect or use manual overrides
    let detection = if let (Some(ha), Some(pc), Some(sv)) = (handler_array, pc_var, stack_var) {
        let ia = index_array.unwrap_or("");
        // Find dispatch pattern in source
        let pattern = dispatch_pattern
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}[{}[{}++]]()", ha, ia, pc));
        let dispatch_offset = source.find(&pattern).unwrap_or(0);
        VmDetection {
            handler_array: ha.to_string(),
            index_array: ia.to_string(),
            pc_var: pc.to_string(),
            stack_var: sv.to_string(),
            mode: mode.to_string(),
            dispatch_pattern: pattern,
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

    // Step 2: Generate source-head Proxy code (for env tracking)
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

    // Step 3: Replace dispatch expression with logging wrapper
    let dispatch_replacement = generate_dispatch_replacement(&detection, capture_stack_depth);

    let patched = if detection.dispatch_offset > 0 {
        let before = &source[..detection.dispatch_offset];
        let after = &source[detection.dispatch_offset + detection.dispatch_pattern.len()..];
        format!("{}{}{}{}", head_code, before, dispatch_replacement, after)
    } else {
        // Fallback: just prepend head code (no dispatch replacement)
        format!("{}{}", head_code, source)
    };

    // Step 4: Build vm_info dict
    let info = PyDict::new(py);
    info.set_item("handler_array", &detection.handler_array)?;
    info.set_item("index_array", &detection.index_array)?;
    info.set_item("pc_var", &detection.pc_var)?;
    info.set_item("stack_var", &detection.stack_var)?;
    info.set_item("mode", &detection.mode)?;
    info.set_item("dispatch_pattern", &detection.dispatch_pattern)?;
    info.set_item("dispatch_offset", detection.dispatch_offset)?;
    info.set_item("head_code_length", head_code.len())?;

    Ok((patched, info.into_any().unbind()))
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
        code.push_str(&format!(r#";(function(){{
var L=globalThis.__iv8i_log__,M=globalThis.__iv8i_lim__;
var T={targets};
T.forEach(function(nm){{
  var obj=globalThis[nm];
  if(!obj||typeof obj!=='object')return;
  try{{
    globalThis[nm]=new Proxy(obj,{{
      get:function(t,p,r){{
        var v=Reflect.get(t,p,r);
        if(typeof p==='symbol'||p==='then'||p==='toJSON'||p==='constructor')return v;
        if(typeof v==='function'){{
          return function(){{
            var res=v.apply(t,arguments);
            if(L.length<M)L.push('C,'+globalThis.__iv8i_pc__+','+nm+'.'+p+','+String(res).slice(0,50));
            return res;
          }};
        }}
        if(L.length<M)L.push('R,'+globalThis.__iv8i_pc__+','+nm+'.'+p+','+String(v).slice(0,50));
        return v;
      }},
      set:function(t,p,v,r){{
        if(L.length<M)L.push('W,'+globalThis.__iv8i_pc__+','+nm+'.'+p+','+String(v).slice(0,50));
        return Reflect.set(t,p,v,r);
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
fn generate_dispatch_replacement(detection: &VmDetection, capture_stack_depth: u32) -> String {
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

        // Comma expression: (set_pc, log, original_dispatch)
        format!(
            "(globalThis.__iv8i_pc__={pc},\
             globalThis.__iv8i_log__.length<globalThis.__iv8i_lim__&&\
             globalThis.__iv8i_log__.push('D,'+{pc}+','+{ia}[{pc}]+',{stack_capture}),\
             {ha}[{ia}[{pc}++]]())",
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
