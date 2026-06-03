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

    // Apply the combined transform
    module.visit_mut_with(&mut CombinedTransform);

    // Emit back to string
    let output = emit_js(&cm, &module);
    (output, diag)
}

// ───
// Visitor
// ───

struct CombinedTransform;

impl VisitMut for CombinedTransform {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        call.visit_mut_children_with(self);

        if let Callee::Expr(expr) = &call.callee {
            match &**expr {
                // 1. Dispatch expression: X[Y[Z++]]()
                Expr::Member(member) if is_computed_member(member) => {
                    call.args.insert(0, marker_arg("__iv8_dispatch"));
                }
                // 2. Module init: __webpack_require__(...)
                Expr::Ident(ident) if &*ident.sym == "__webpack_require__" => {
                    call.args.insert(0, marker_arg("__iv8_module_init"));
                }
                // 3. Eval / Function: eval(...), Function(...)
                Expr::Ident(ident) if &*ident.sym == "eval" || &*ident.sym == "Function" => {
                    call.args.insert(0, marker_arg("__iv8_eval_source"));
                }
                _ => {}
            }
        }
    }
}

/// Check if a MemberExpr has nested computed properties (dispatch pattern).
fn is_computed_member(member: &MemberExpr) -> bool {
    matches!(&member.prop, MemberProp::Computed(c)
        if matches!(&*c.expr, Expr::Member(inner)
            if matches!(&inner.prop, MemberProp::Computed(_))))
}

fn marker_arg(value: &str) -> ExprOrSpread {
    ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: swc_common::DUMMY_SP,
            value: value.into(),
            raw: None,
        }))),
    }
}

// ───
// Codegen
// ───

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
        assert!(output.contains("__iv8_dispatch"),
            "expected dispatch marker in output, got: {}", output);
    }

    #[test]
    fn test_webpack_require() {
        let (output, diag) = instrument("__webpack_require__(42);");
        assert!(diag.is_none());
        assert!(output.contains("__iv8_module_init"),
            "expected module_init marker in output, got: {}", output);
    }

    #[test]
    fn test_eval_function() {
        let (output, diag) = instrument("eval('1+1'); Function('return 1');");
        assert!(diag.is_none());
        assert!(output.contains("__iv8_eval_source"));
    }

    #[test]
    fn test_plain_script_passthrough() {
        let (output, diag) = instrument("var x = 1 + 1;");
        assert!(diag.is_none());
        // Should not contain any markers
        assert!(!output.contains("__iv8_"));
    }

    #[test]
    fn test_fallback_on_invalid_syntax() {
        let (output, diag) = instrument("var x = ;;;;");
        // Should fall back to original source
        assert_eq!(output, "var x = ;;;;");
        assert!(diag.is_some());
        assert!(diag.unwrap().contains("parse"));
    }

    #[test]
    fn test_combined_transform() {
        let src = "A[Q[U++]](); __webpack_require__(7); eval(code);";
        let (output, _) = instrument(src);
        assert!(output.contains("__iv8_dispatch"));
        assert!(output.contains("__iv8_module_init"));
        assert!(output.contains("__iv8_eval_source"));
    }
}
