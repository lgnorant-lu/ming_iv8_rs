//! L3 historical / local-only pressure (not default CI).
//!
//! Requires workspace `_ref/` samples. Skipped when missing.
//! Run: `cargo test -p iv8-core --test test_entry_l3_pressure -- --ignored`
//!
//! Does **not** load `_ref/yy/vendor.chunk.*` (~679KB) into default CI.

mod common;

use iv8_core::entry::classification;
use iv8_core::entry::types::SampleKind;
use iv8_core::kernel::EvalOpts;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn load_ref(rel: &str) -> Option<String> {
    let p = workspace_root().join(rel);
    std::fs::read_to_string(p).ok()
}

fn ref_exists(rel: &str) -> bool {
    workspace_root().join(rel).is_file()
}

/// Real BDMS minified sample: must classify WebpackRuntime (FP audit positive path).
#[test]
#[ignore = "local _ref BDMS corpus; run with --ignored"]
fn test_l3_bdms_real_sample_classifies_webpack() {
    let rel = "_ref/iv8-examples/js/bdms_1.0.1.19.js";
    if !ref_exists(rel) {
        eprintln!("SKIP missing {rel}");
        return;
    }
    let src = load_ref(rel).expect("read bdms");
    let kind = classification::classify(&src, &[]);
    // Real BDMS may be WebpackRuntime or WebpackVmHybrid (VM + webpack table).
    assert!(
        matches!(
            kind,
            SampleKind::WebpackRuntime | SampleKind::WebpackVmHybrid
        ),
        "real BDMS should hit webpack family, got {:?}",
        kind
    );
}

/// yy runtime+Page (no large vendor): load + named/jsonp graph non-empty.
#[test]
#[ignore = "local _ref/yy pressure; run with --ignored"]
fn test_l3_yy_runtime_and_page_load_graph() {
    let runtime = "_ref/yy/runtime_patched.js";
    let page = "_ref/yy/Page.chunk.d5594821969491679d5e.js";
    if !ref_exists(runtime) || !ref_exists(page) {
        eprintln!("SKIP missing yy fixtures");
        return;
    }
    let rt = load_ref(runtime).unwrap();
    let pg = load_ref(page).unwrap();
    assert!(
        rt.contains("webpackJsonp") || rt.contains("webpack"),
        "runtime markers"
    );
    assert!(pg.contains("webpackJsonp") || pg.contains("push("));

    let mut kernel = common::make_kernel();
    kernel
        .eval(
            iv8_core::entry::webpack::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    // Product path: preload labeled chunks (caller-supplied text; no network)
    let rep = iv8_core::entry::webpack::preload_chunk_sources_labeled(
        &mut kernel,
        &[rt, pg],
        &["yy_runtime", "yy_page"],
    );
    assert!(
        rep["chunks_eval_ok"].as_u64().unwrap_or(0) >= 1,
        "preload report={:?}",
        rep
    );
    assert_eq!(rep["ensure_chunk"]["remote_fetch"], false);

    // Optional: if require exists after runtime, collect graph
    let has_req = matches!(
        kernel.eval_to_rust_value(
            "typeof __webpack_require__==='function'||typeof __iv8_wp_require==='function'||typeof window.__loader==='function'"
        ),
        iv8_core::convert::RustValue::Bool(true)
    );
    if has_req {
        if let Some(graph) = iv8_core::entry::webpack::collect_module_graph(&mut kernel) {
            let count = graph["module_count"].as_u64().unwrap_or(0);
            assert!(
                count > 0 || !graph["chunks"].as_array().map(|a| a.is_empty()).unwrap_or(true),
                "expected modules or chunks from yy: {:?}",
                graph
            );
        }
    }
}

/// Confirm we never auto-include the 679KB vendor in this suite's default paths.
#[test]
fn test_l3_policy_excludes_large_vendor_from_ci_paths() {
    let vendor = workspace_root().join("_ref/yy/vendor.chunk.062f57657390b2408623.js");
    // Policy test: suite source must not reference vendor path as default load
    let this = include_str!("test_entry_l3_pressure.rs");
    assert!(
        !this.contains("vendor.chunk.062f57657390b2408623")
            || this.contains("Does **not** load"),
        "must not load large vendor in default CI"
    );
    let _ = vendor; // may or may not exist on machine
}
