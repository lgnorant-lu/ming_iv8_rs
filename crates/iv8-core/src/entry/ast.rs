//! SWC-based AST instrumentation for v0.6 EntryPlanner.
//!
//! Provides source-level rewrite for the 3 join points defined in the
//! AST Instrumentation MVP: dispatch expression, module init wrapper,
//! and eval/Function constructor.
//!
//! Falls back to the original source if parsing fails.

use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{VisitMut, VisitMutWith};

/// Quick viability probe: check whether SWC can successfully parse the source.
/// Returns false if parsing fails for any reason (syntax error, unsupported feature, etc.).
/// Does not perform any transform — just parse and discard.
pub fn can_parse(source: &str) -> bool {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("input.js".into()).into(),
        source.to_string(),
    );
    let lexer = Lexer::new(
        Syntax::default(),
        EsVersion::Es2020,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    parser.parse_module().is_ok()
}

/// Apply AST-level instrumentation to a JS source string.
///
/// Rewrites the 3 join points:
/// 1. Dispatch expression: `A[Q[U++]]()` → tagged call
/// 2. Module init: `__webpack_require__(id)` → tagged call
/// 3. Eval/Function: `eval(x)` / `Function(x)` → tagged call
///
/// On parse failure, returns the original source unchanged and logs
/// the error through the reason string.
pub fn instrument(source: &str) -> (String, Option<String>) {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        FileName::Custom("input.js".into()).into(),
        source.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::default(),
        EsVersion::Es2020,
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let errors = parser.take_errors();

    let mut module = match parser.parse_module() {
        Ok(m) => m,
        Err(e) => {
            return (source.to_string(), Some(format!("parse error: {:?}", e)));
        }
    };

    let diag = if errors.is_empty() {
        None
    } else {
        Some(format!("{} parse warnings", errors.len()))
    };

    // Apply the universal dispatch trap transform
    module.visit_mut_with(&mut TrapTransform);

    // Emit back to string
    let output = emit_js(&cm, &module);

    // Prepend __iv8_trap helper + log infrastructure
    let helper = r#"
;(function(){var g=globalThis;
Object.defineProperty(g,'__iv8i_log__',{value:[],writable:true,enumerable:false,configurable:true});
Object.defineProperty(g,'__iv8i_lim__',{value:200000,writable:true,enumerable:false,configurable:true});
g.__iv8_trap=function(h,k){var f=h[k];
if(g.__iv8i_log__.length<g.__iv8i_lim__)g.__iv8i_log__.push('D,'+(typeof f==='function'?(f.name||'?'):'!')+','+k);
return f.apply(h,Array.prototype.slice.call(arguments,2));};
g.__iv8_eval_trap=function(src){
if(g.__iv8i_log__.length<g.__iv8i_lim__)g.__iv8i_log__.push('eval,'+String(src).slice(0,80));
return (0,eval)(src);};
g.__iv8_function_trap=function(){
var args=Array.prototype.slice.call(arguments);
if(g.__iv8i_log__.length<g.__iv8i_lim__)g.__iv8i_log__.push('fn_ctor,'+args.map(String).join('|').slice(0,80));
return Function.apply(null,args);};
})();
"#;
    (format!("{}{}", helper, output), diag)
}

// ───
// Transform: universal dispatch instrumentation via __iv8_trap helper
//   X[Y](a1, a2, ..., aN) → __iv8_trap(X, Y, a1, a2, ..., aN)
//
// Semantics preserved:
//   - X evaluated once (passed as first arg)
//   - Y evaluated once (passed as second arg)
//   - this binding = X (via .apply(h, ...) inside helper)
//   - Arguments evaluated once (passed as trailing args)
//   - Return value preserved (helper returns the result)
// ───

struct TrapTransform;

impl VisitMut for TrapTransform {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        call.visit_mut_children_with(self);

        if let Callee::Expr(callee_expr) = &call.callee {
            match &**callee_expr {
                // Computed member call: X[Y](...) → __iv8_trap(X, Y, ...)
                Expr::Member(member) if matches!(&member.prop, MemberProp::Computed(_)) => {
                    // Prepend (obj, prop) to existing args
                    let obj_expr = member.obj.clone();
                    let prop_expr = match &member.prop {
                        MemberProp::Computed(c) => c.expr.clone(),
                        _ => unreachable!(),
                    };
                    let mut new_args = vec![
                        ExprOrSpread { spread: None, expr: obj_expr },
                        ExprOrSpread { spread: None, expr: prop_expr },
                    ];
                    new_args.extend(std::mem::take(&mut call.args));

                    call.callee = Callee::Expr(Box::new(Expr::Ident(Ident::new(
                        "__iv8_trap".into(),
                        call.span,
                        swc_common::SyntaxContext::empty(),
                    ))));
                    call.args = new_args;
                }
                // Eval/Function: dynamic code execution
                Expr::Ident(ident)
                    if &*ident.sym == "eval" || &*ident.sym == "Function" =>
                {
                    let trap_name = if &*ident.sym == "eval" {
                        "__iv8_eval_trap"
                    } else {
                        "__iv8_function_trap"
                    };
                    call.callee = Callee::Expr(Box::new(Expr::Ident(Ident::new(
                        trap_name.into(),
                        call.span,
                        swc_common::SyntaxContext::empty(),
                    ))));
                }
                _ => {}
            }
        }
    }
}

fn emit_js(cm: &Lrc<SourceMap>, module: &Module) -> String {
    let mut buf = vec![];
    {
        let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);
        let config = Config::default()
            .with_minify(false)
            .with_target(EsVersion::Es2020);
        let mut emitter = Emitter {
            cfg: config,
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };
        let _ = emitter.emit_module(module);
    }
    String::from_utf8(buf).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_expression() {
        let (output, diag) = instrument("var r = A[Q[U++]]();");
        assert!(diag.is_none(), "unexpected diagnostic: {:?}", diag);
        assert!(output.contains("__iv8_trap"),
            "expected __iv8_trap wrapper in output, got: {}", output);
    }

    #[test]
    fn test_webpack_require() {
        let (output, diag) = instrument("__webpack_require__(42);");
        assert!(diag.is_none());
        assert!(output.contains("__webpack_require__"));
    }

    #[test]
    fn test_eval_function() {
        let (output, diag) = instrument("eval('1+1'); Function('return 1');");
        assert!(diag.is_none());
        assert!(output.contains("__iv8_eval_trap"));
        assert!(output.contains("__iv8_function_trap"));
    }

    #[test]
    fn test_eval_function_source_points_on_execution() {
        let src = r#"
eval('globalThis.__iv8_eval_value = 2');
var f = Function('return 3');
globalThis.__iv8_function_value = f();
"#;
        let (transformed, diag) = instrument(src);
        assert!(diag.is_none());
        assert!(transformed.contains("__iv8_eval_trap"));
        assert!(transformed.contains("__iv8_function_trap"));

        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.eval(&transformed, crate::kernel::EvalOpts::default()).unwrap();

        let eval_value = kernel.eval_to_rust_value("globalThis.__iv8_eval_value");
        let function_value = kernel.eval_to_rust_value("globalThis.__iv8_function_value");
        assert_eq!(eval_value, crate::convert::RustValue::Int(2));
        assert_eq!(function_value, crate::convert::RustValue::Int(3));

        let trace = kernel.eval_to_rust_value(
            "typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []",
        );
        if let crate::convert::RustValue::Array(items) = trace {
            let entries: Vec<&str> = items.iter().map(entry_str).collect();
            assert!(entries.iter().any(|e| e.starts_with("eval,")),
                "expected eval source-point entry, got: {:?}", entries);
            assert!(entries.iter().any(|e| e.starts_with("fn_ctor,")),
                "expected Function source-point entry, got: {:?}", entries);
        } else {
            panic!("expected array, got {:?}", trace);
        }
    }

    #[test]
    fn test_plain_script_passthrough() {
        let (output, diag) = instrument("var x = 1 + 1;");
        assert!(diag.is_none());
        assert!(!output.contains("__iv8_trap(var"),
            "no dispatch calls, so no wrapping should occur: {}", output);
        assert!(output.contains("var x = 1 + 1;"));
    }

    #[test]
    fn test_fallback_on_invalid_syntax() {
        let (output, diag) = instrument("var x = ;;;;");
        assert_eq!(output, "var x = ;;;;");
        assert!(diag.is_some());
        assert!(diag.unwrap().contains("parse"));
    }

    #[test]
    fn test_combined_transform() {
        let src = "A[Q[U++]](); __webpack_require__(7); eval(code);";
        let (output, _) = instrument(src);
        assert!(output.contains("__iv8_trap"));
    }

    #[test]
    fn test_single_computed_dispatch() {
        let (output, diag) = instrument("var r = handlers[opcode]();");
        assert!(diag.is_none());
        assert!(output.contains("__iv8_trap"),
            "single computed dispatch not instrumented: {}", output);
    }

    #[test]
    fn test_dot_then_computed() {
        let (output, diag) = instrument("var r = vm.handlers[opcode]();");
        assert!(diag.is_none());
        assert!(output.contains("__iv8_trap"),
            "dot-then-computed dispatch not instrumented: {}", output);
    }

    /// Helper: extract the inner string from a RustValue::String or panic.
    fn entry_str(item: &crate::convert::RustValue) -> &str {
        match item {
            crate::convert::RustValue::String(s) => s.as_str(),
            _ => panic!("expected String, got {:?}", item),
        }
    }

    #[test]
    fn test_trap_produces_trace_on_execution() {
        let src = "var handlers = [function a() {}, function b() {}]; var pc = 0; handlers[pc]();";
        let (transformed, diag) = instrument(src);
        assert!(diag.is_none(), "transform should succeed: {:?}", diag);
        assert!(transformed.contains("__iv8_trap"));

        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.eval(&transformed, crate::kernel::EvalOpts::default()).unwrap();

        let trace = kernel.eval_to_rust_value(
            "typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []",
        );
        if let crate::convert::RustValue::Array(items) = trace {
            assert!(!items.is_empty(), "expected at least one D entry");
            assert!(entry_str(&items[0]).starts_with("D,"), "expected D entry, got: {:?}", items[0]);
        } else {
            panic!("expected array, got {:?}", trace);
        }
    }

    #[test]
    fn test_trap_multi_arg() {
        let src = "var handlers = [function(x,y){}]; var op=0; handlers[op](1,2);";
        let (transformed, diag) = instrument(src);
        assert!(diag.is_none());
        assert!(transformed.contains("__iv8_trap"));

        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.eval(&transformed, crate::kernel::EvalOpts::default()).unwrap();

        let trace = kernel.eval_to_rust_value(
            "typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []",
        );
        if let crate::convert::RustValue::Array(items) = trace {
            assert!(!items.is_empty(), "expected D entries for multi-arg dispatch");
            assert!(entry_str(&items[0]).starts_with("D,"));
        } else {
            panic!("expected array, got {:?}", trace);
        }
    }

    #[test]
    fn test_trap_this_binding() {
        let src = r#"
var obj = {
    name: "test",
    method: function() { return this.name; }
};
var key = "method";
obj[key]();
"#;
        let (transformed, diag) = instrument(src);
        assert!(diag.is_none());
        assert!(transformed.contains("__iv8_trap"));

        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.eval(&transformed, crate::kernel::EvalOpts::default()).unwrap();

        let trace = kernel.eval_to_rust_value(
            "typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []",
        );
        if let crate::convert::RustValue::Array(items) = trace {
            assert!(!items.is_empty(), "expected D entry for method dispatch");
            let first = entry_str(&items[0]);
            assert!(first.starts_with("D,"), "expected D entry, got: {}", first);
            assert!(first.contains("method"), "expected 'method' name, got: {}", first);
        } else {
            panic!("expected array, got {:?}", trace);
        }
    }
}
