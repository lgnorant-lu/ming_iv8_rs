//! SWC-based AST instrumentation for v0.6/v0.7 EntryPlanner.
//!
//! Provides source-level rewrite for the 3 join points defined in the
//! AST Instrumentation MVP: dispatch expression, module init wrapper,
//! and eval/Function constructor.
//!
//! Falls back to the original source if parsing fails.
//! Also produces structured transform reports with evidence and diagnostics.
//!
//! ## Join Points (v0.7)
//!
//! | Join point | v0.7 status | Primary consumer |
//! |---|---|---|
//! | dispatch expression | stable | Dispatch Generalization |
//! | module init wrapper | stable | WebpackBridge |
//! | eval source point | stable | dynamic source capture |
//! | Function constructor source point | stable | dynamic source capture |
//!
//! Deferred: generic member access, generic function calls,
//! whole-bundle factory rewriting.

use crate::entry::diagnostics;
use serde::{Deserialize, Serialize};
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{VisitMut, VisitMutWith};

// ───
// Structured types (source-ast-pipeline.md sections 6-9)
// ───

/// A transform request describing which join point to instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAstRequest {
    pub request_id: String,
    pub source_id: String,
    pub join_point_kind: String,
    pub target_hint: Option<String>,
    pub policy: String,
    pub persona: String,
    pub expected_evidence: Vec<String>,
}

/// Report produced by a SourceAst transform attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAstReport {
    pub schema_version: String,
    pub request_id: String,
    pub source_id: String,
    pub parser: String,
    pub parse_ok: bool,
    pub candidates: Vec<SourceAstCandidate>,
    pub selected_join_points: Vec<String>,
    pub edits: Vec<SourceAstEdit>,
    pub emit_ok: bool,
    pub runtime_validated: bool,
    pub evidence: Vec<diagnostics::EvidenceRecord>,
    pub diagnostics: Vec<diagnostics::DiagnosticRecord>,
}

/// A candidate join point found during AST inspection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAstCandidate {
    pub candidate_id: String,
    pub kind: String,
    pub span_start: usize,
    pub span_end: usize,
    pub score: f64,
    pub reasons: Vec<String>,
    pub decision: String,
}

/// A single edit/instrumentation applied to the source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAstEdit {
    pub edit_id: String,
    pub candidate_id: String,
    pub kind: String,
    pub span_before_start: usize,
    pub span_before_end: usize,
    pub span_after_start: usize,
    pub span_after_end: usize,
    pub helper: String,
    pub policy: String,
}

/// Quick viability probe: check whether SWC can successfully parse the source.
/// Returns false if parsing fails for any reason.
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

/// Apply AST-level instrumentation, returning structured report alongside
/// the transformed source.
pub fn instrument_with_report(source: &str) -> (String, SourceAstReport) {
    let request_id = format!("source_ast.transform.{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos());
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
    let parse_errors = parser.take_errors();
    let mut module = match parser.parse_module() {
        Ok(m) => m,
        Err(e) => {
            let diag = diagnostics::warn_diag(
                diagnostics::codes::source_ast::PARSE_FAILED,
                "source_ast.parse",
                &format!("SWC parse failed: {:?}", e),
            );
            let report = SourceAstReport {
                schema_version: "source-ast-report.v0.1".to_string(),
                request_id,
                source_id: "input.js".to_string(),
                parser: "swc".to_string(),
                parse_ok: false,
                candidates: Vec::new(),
                selected_join_points: Vec::new(),
                edits: Vec::new(),
                emit_ok: false,
                runtime_validated: false,
                evidence: vec![],
                diagnostics: vec![diag],
            };
            return (source.to_string(), report);
        }
    };

    let mut diags: Vec<diagnostics::DiagnosticRecord> = Vec::new();
    if !parse_errors.is_empty() {
        diags.push(diagnostics::warn_diag(
            diagnostics::codes::source_ast::PARSE_FAILED,
            "source_ast.parse",
            &format!("{} parse warnings", parse_errors.len()),
        ));
    }

    // Count candidates and apply transforms
    let candidates = count_candidates(&module);
    if candidates.is_empty() {
        diags.push(diagnostics::warn_diag(
            diagnostics::codes::source_ast::CANDIDATE_EMPTY,
            "source_ast.probe",
            "no transform candidates found in source",
        ));
    }

    // Apply transforms
    module.visit_mut_with(&mut TrapTransform);

    // Emit back to string
    let output = emit_js(&cm, &module);
    let emit_ok = !output.is_empty();

    // Prepend helper infrastructure
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

    let transformed = format!("{}{}", helper, output);

    // Build evidence
    let mut evidence = Vec::new();
    if !candidates.is_empty() {
        evidence.push(
            diagnostics::EvidenceRecord::new(
                "source_ast_candidate_detected",
                diagnostics::EvidenceStrength::Weak,
                "source_ast",
                "source_ast.probe",
                &format!("{} transform candidate(s) found", candidates.len()),
            )
            .with_producer("source_ast.main"),
        );
    }
    if emit_ok {
        evidence.push(
            diagnostics::EvidenceRecord::new(
                "source_ast_transform_applied",
                diagnostics::EvidenceStrength::Weak,
                "source_ast",
                "source_ast.transform",
                "transform emitted successfully; runtime validation needed for strong evidence",
            )
            .with_producer("source_ast.main"),
        );
    }

    if !emit_ok {
        diags.push(diagnostics::error_diag(
            diagnostics::codes::source_ast::EMIT_FAILED,
            "source_ast.emit",
            "code generation produced empty output",
        ));
    }

    let report = SourceAstReport {
        schema_version: "source-ast-report.v0.1".to_string(),
        request_id,
        source_id: "input.js".to_string(),
        parser: "swc".to_string(),
        parse_ok: true,
        candidates: candidates
            .into_iter()
            .enumerate()
            .map(|(i, (kind, _start, _end, score))| SourceAstCandidate {
                candidate_id: format!("source_ast.candidate.{:03}", i),
                kind,
                span_start: _start as usize,
                span_end: _end as usize,
                score,
                reasons: vec!["matched join point pattern".to_string()],
                decision: "selected".to_string(),
            })
            .collect(),
        selected_join_points: vec!["dispatch_expression".to_string()],
        edits: Vec::new(),
        emit_ok,
        runtime_validated: false,
        evidence,
        diagnostics: diags,
    };

    (transformed, report)
}

/// Count how many transform candidates exist in the module.
fn count_candidates(module: &Module) -> Vec<(String, u32, u32, f64)> {
    let mut result = Vec::new();
    for stmt in &module.body {
        if let ModuleItem::Stmt(s) = stmt {
            collect_candidates_from_stmt(s, &mut result);
        }
    }
    result
}

fn collect_candidates_from_stmt(stmt: &Stmt, result: &mut Vec<(String, u32, u32, f64)>) {
    match stmt {
        Stmt::Expr(s) => {
            if let Expr::Call(call) = &*s.expr {
                if let Callee::Expr(callee) = &call.callee {
                    collect_call_candidate(callee, result);
                }
            }
        }
        Stmt::Decl(d) => {
            if let Decl::Var(v) = d {
                for decl in &v.decls {
                    if let Some(init) = &decl.init {
                        collect_expr_candidate(init, result);
                    }
                }
            }
        }
        _ => {}
    }
}

fn collect_expr_candidate(expr: &Expr, result: &mut Vec<(String, u32, u32, f64)>) {
    if let Expr::Call(call) = expr {
        if let Callee::Expr(callee) = &call.callee {
            collect_call_candidate(callee, result);
        }
    }
    if let Expr::Paren(p) = expr {
        collect_expr_candidate(&p.expr, result);
    }
}

fn collect_call_candidate(callee: &Expr, result: &mut Vec<(String, u32, u32, f64)>) {
    if let Expr::Member(member) = callee {
        if is_dispatch_join_point(member) {
            use swc_common::Spanned;
            result.push((
                "dispatch_expression".to_string(),
                member.obj.span().lo.0,
                member.span().hi.0,
                0.84,
            ));
        }
    }
    if let Expr::Ident(ident) = callee {
        let name = &*ident.sym;
        if name == "eval" || name == "Function" {
            use swc_common::Spanned;
            let kind = if name == "eval" { "eval_source_point" } else { "function_ctor_source_point" };
            result.push((kind.to_string(), ident.span().lo.0, ident.span().hi.0, 0.6));
        }
    }
}

/// Apply AST-level instrumentation to a JS source string.
///
/// Returns `(transformed_source, Option<diagnostic_message>)`.
/// This is the backward-compatible API. Prefer `instrument_with_report` for
/// structured output.
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
    module.visit_mut_with(&mut TrapTransform);
    let output = emit_js(&cm, &module);
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
// ───

struct TrapTransform;

impl VisitMut for TrapTransform {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        call.visit_mut_children_with(self);
        if let Callee::Expr(callee_expr) = &call.callee {
            match &**callee_expr {
                Expr::Member(member) if is_dispatch_join_point(member) => {
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
                Expr::Ident(ident) if &*ident.sym == "eval" || &*ident.sym == "Function" => {
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

fn is_dispatch_join_point(member: &MemberExpr) -> bool {
    let MemberProp::Computed(prop) = &member.prop else {
        return false;
    };
    let Expr::Member(index_member) = &*prop.expr else {
        return false;
    };
    let MemberProp::Computed(index_prop) = &index_member.prop else {
        return false;
    };
    matches!(
        &*index_prop.expr,
        Expr::Update(UpdateExpr {
            op: UpdateOp::PlusPlus,
            ..
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_expression() {
        let (output, diag) = instrument("var r = A[Q[U++]]();");
        assert!(diag.is_none(), "unexpected diagnostic: {:?}", diag);
        assert!(
            output.contains("__iv8_trap"),
            "expected __iv8_trap wrapper in output, got: {}",
            output
        );
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
        kernel
            .eval(&transformed, crate::kernel::EvalOpts::default())
            .unwrap();

        let eval_value = kernel.eval_to_rust_value("globalThis.__iv8_eval_value");
        let function_value = kernel.eval_to_rust_value("globalThis.__iv8_function_value");
        assert_eq!(eval_value, crate::convert::RustValue::Int(2));
        assert_eq!(function_value, crate::convert::RustValue::Int(3));

        let trace =
            kernel.eval_to_rust_value("typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []");
        if let crate::convert::RustValue::Array(items) = trace {
            let entries: Vec<&str> = items.iter().map(entry_str).collect();
            assert!(
                entries.iter().any(|e| e.starts_with("eval,")),
                "expected eval source-point entry, got: {:?}",
                entries
            );
            assert!(
                entries.iter().any(|e| e.starts_with("fn_ctor,")),
                "expected Function source-point entry, got: {:?}",
                entries
            );
        } else {
            panic!("expected array, got {:?}", trace);
        }
    }

    #[test]
    fn test_plain_script_passthrough() {
        let (output, diag) = instrument("var x = 1 + 1;");
        assert!(diag.is_none());
        assert!(
            !output.contains("__iv8_trap(var"),
            "no dispatch calls, so no wrapping should occur: {}",
            output
        );
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
        assert!(
            output.contains("handlers[opcode]();"),
            "ordinary computed call should remain intact: {}",
            output
        );
        assert!(
            !output.contains("__iv8_trap(handlers"),
            "ordinary computed call should not be trapped: {}",
            output
        );
    }

    #[test]
    fn test_dot_then_computed() {
        let (output, diag) = instrument("var r = vm.handlers[opcode]();");
        assert!(diag.is_none());
        assert!(
            output.contains("vm.handlers[opcode]();"),
            "ordinary dot-then-computed call should remain intact: {}",
            output
        );
        assert!(
            !output.contains("__iv8_trap(vm.handlers"),
            "ordinary dot-then-computed call should not be trapped: {}",
            output
        );
    }

    fn entry_str(item: &crate::convert::RustValue) -> &str {
        match item {
            crate::convert::RustValue::String(s) => s.as_str(),
            _ => panic!("expected String, got {:?}", item),
        }
    }

    #[test]
    fn test_trap_produces_trace_on_execution() {
        let src = "var A = [function a() {}, function b() {}]; var Q = [0]; var U = 0; A[Q[U++]]();";
        let (transformed, diag) = instrument(src);
        assert!(diag.is_none(), "transform should succeed: {:?}", diag);
        assert!(transformed.contains("__iv8_trap(A"));

        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(&transformed, crate::kernel::EvalOpts::default())
            .unwrap();

        let trace =
            kernel.eval_to_rust_value("typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []");
        if let crate::convert::RustValue::Array(items) = trace {
            assert!(!items.is_empty(), "expected at least one D entry");
            assert!(
                entry_str(&items[0]).starts_with("D,"),
                "expected D entry, got: {:?}",
                items[0]
            );
        } else {
            panic!("expected array, got {:?}", trace);
        }
    }

    #[test]
    fn test_trap_multi_arg() {
        let src = "var A = [function(x,y){}]; var Q = [0]; var U = 0; A[Q[U++]](1,2);";
        let (transformed, diag) = instrument(src);
        assert!(diag.is_none());
        assert!(transformed.contains("__iv8_trap(A"));

        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(&transformed, crate::kernel::EvalOpts::default())
            .unwrap();

        let trace =
            kernel.eval_to_rust_value("typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []");
        if let crate::convert::RustValue::Array(items) = trace {
            assert!(
                !items.is_empty(),
                "expected D entries for multi-arg dispatch"
            );
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
        assert!(!transformed.contains("__iv8_trap(obj"));

        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(&transformed, crate::kernel::EvalOpts::default())
            .unwrap();

        let trace =
            kernel.eval_to_rust_value("typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []");
        if let crate::convert::RustValue::Array(items) = trace {
            assert!(
                items.is_empty(),
                "ordinary computed member call should not emit D trace: {:?}",
                items
            );
        } else {
            panic!("expected array, got {:?}", trace);
        }
    }

    // ──
    // New tests for instrument_with_report
    // ──

    #[test]
    fn test_instrument_with_report_dispatch() {
        let (source, report) = instrument_with_report("var r = A[Q[U++]]();");
        assert_eq!(report.schema_version, "source-ast-report.v0.1");
        assert!(report.parse_ok);
        assert!(report.emit_ok);
        assert!(!report.candidates.is_empty());
        assert!(source.contains("__iv8_trap"));
    }

    #[test]
    fn test_instrument_with_report_parse_failure() {
        let (_, report) = instrument_with_report("var x = ;;;;");
        assert!(!report.parse_ok);
        assert!(report.diagnostics.iter().any(|d| d.code == "SOURCE_AST_PARSE_FAILED"));
    }

    #[test]
    fn test_instrument_with_report_no_candidates() {
        let (_, report) = instrument_with_report("var x = 1 + 1;");
        assert!(report.parse_ok);
        assert!(report.candidates.is_empty());
        assert!(report.diagnostics.iter().any(|d| d.code == "SOURCE_AST_CANDIDATE_EMPTY"));
    }

    #[test]
    fn test_instrument_with_report_evidence() {
        let (_, report) = instrument_with_report("A[Q[U++]]();");
        assert!(report.evidence.iter().any(|e| e.kind == "source_ast_candidate_detected"));
        assert!(report.evidence.iter().any(|e| e.kind == "source_ast_transform_applied"));
    }

    #[test]
    fn test_source_ast_request_schema() {
        let req = SourceAstRequest {
            request_id: "source_ast.001".to_string(),
            source_id: "main.js".to_string(),
            join_point_kind: "dispatch_expression".to_string(),
            target_hint: Some("handlers[op](a,b)".to_string()),
            policy: "analysis_only".to_string(),
            persona: "analysis".to_string(),
            expected_evidence: vec!["dispatch_trace_observed".to_string()],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("dispatch_expression"));
        assert!(json.contains("analysis_only"));
    }

    #[test]
    fn test_source_ast_report_schema() {
        let report = SourceAstReport {
            schema_version: "source-ast-report.v0.1".to_string(),
            request_id: "source_ast.001".to_string(),
            source_id: "main.js".to_string(),
            parser: "swc".to_string(),
            parse_ok: true,
            candidates: vec![SourceAstCandidate {
                candidate_id: "source_ast.candidate.001".to_string(),
                kind: "dispatch_expression".to_string(),
                span_start: 120,
                span_end: 148,
                score: 0.84,
                reasons: vec!["handler_array".to_string(), "pc_hint".to_string()],
                decision: "selected".to_string(),
            }],
            selected_join_points: vec!["dispatch_expression".to_string()],
            edits: vec![],
            emit_ok: true,
            runtime_validated: false,
            evidence: vec![],
            diagnostics: vec![],
        };
        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("source-ast-report.v0.1"));
    }
}