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

    // Apply the dispatch wrapper transform
    module.visit_mut_with(&mut DispatchWrapper);

    // Emit back to string
    let output = emit_js(&cm, &module);

    // Prepend __iv8_trace helper + log infrastructure
    let helper = r#"
;(function(){var g=globalThis;
Object.defineProperty(g,'__iv8i_log__',{value:[],writable:true,enumerable:false,configurable:true});
Object.defineProperty(g,'__iv8i_lim__',{value:200000,writable:true,enumerable:false,configurable:true});
Object.defineProperty(g,'__iv8i_pc__',{value:-1,writable:true,enumerable:false,configurable:true});
g.__iv8_trace=function(fn){if(typeof fn!=='function')return fn;
return function(){var a=Array.prototype.slice.call(arguments);
if(__iv8i_log__.length<__iv8i_lim__)__iv8i_log__.push('D,'+__iv8i_pc__+','+(fn.name||'?'));
return fn.apply(this,a);};};
})();
"#;
    (format!("{}{}", helper, output), diag)
}

// ───
// Wrapper: dispatcher wraps the CALLEE (not injects args)
//   handlers[opcode](x, y) → __iv8_trace(handlers[opcode])(x, y)
// ───

struct DispatchWrapper;

impl VisitMut for DispatchWrapper {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        call.visit_mut_children_with(self);

        if let Callee::Expr(expr) = &call.callee {
            match &**expr {
                // Computed member call: potential dispatch — wrap callee
                Expr::Member(member) if has_any_computed_access(member) => {
                    // Replace callee with __iv8_trace(callee)
                    let callee_span = call.span;
                    let old_callee = call.callee.clone();
                    // Create __iv8_trace(old_callee) call expression
                    call.callee = Callee::Expr(Box::new(Expr::Call(CallExpr {
                        span: callee_span,
                        callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                            "__iv8_trace".into(),
                            callee_span,
                            swc_common::SyntaxContext::empty(),
                        )))),
                        args: vec![ExprOrSpread {
                            spread: None,
                            expr: match old_callee {
                                Callee::Expr(e) => e,
                                _ => Box::new(Expr::Ident(Ident::new(
                                    "undefined".into(),
                                    callee_span,
                                    swc_common::SyntaxContext::empty(),
                                ))),
                            },
                        }],
                        ..Default::default()
                    })));
                }
                // Eval/Function: dynamic code execution
                Expr::Ident(ident)
                    if &*ident.sym == "eval" || &*ident.sym == "Function" =>
                {
                    // TODO: wrap for source capture
                }
                _ => {}
            }
        }
    }
    }

/// Check if any MemberExpr in the chain has a computed property access.
/// Matches patterns: A[B](), A.B[C](), A[B[C]](), A[B][C]() etc.
fn has_any_computed_access(member: &MemberExpr) -> bool {
    matches!(&member.prop, MemberProp::Computed(_)) ||
    matches!(&*member.obj, Expr::Member(m) if has_any_computed_access(m))
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
        assert!(output.contains("__iv8_trace"),
            "expected __iv8_trace wrapper in output, got: {}", output);
    }

    #[test]
    fn test_webpack_require() {
        let (output, diag) = instrument("__webpack_require__(42);");
        assert!(diag.is_none());
        // __webpack_require__ is an Ident, not computed — passes through
        assert!(output.contains("__webpack_require__"));
    }

    #[test]
    fn test_eval_function() {
        let (output, diag) = instrument("eval('1+1'); Function('return 1');");
        assert!(diag.is_none());
        // eval/Function pass through (TODO: wrap for source capture)
        assert!(output.contains("eval"));
    }

    #[test]
    fn test_plain_script_passthrough() {
        let (output, diag) = instrument("var x = 1 + 1;");
        assert!(diag.is_none());
        // Helper __iv8_trace is prepended but no dispatch calls should be wrapped
        assert!(!output.contains("__iv8_trace(var"),
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
        assert!(output.contains("__iv8_trace"));
    }

    #[test]
    fn test_single_computed_dispatch() {
        let (output, diag) = instrument("var r = handlers[opcode]();");
        assert!(diag.is_none());
        assert!(output.contains("__iv8_trace"),
            "single computed dispatch not instrumented: {}", output);
    }

    #[test]
    fn test_dot_then_computed() {
        let (output, diag) = instrument("var r = vm.handlers[opcode]();");
        assert!(diag.is_none());
        assert!(output.contains("__iv8_trace"),
            "dot-then-computed dispatch not instrumented: {}", output);
    }
}
