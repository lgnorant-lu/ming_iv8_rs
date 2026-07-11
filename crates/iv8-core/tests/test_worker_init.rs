//! Worker mode initialization tests.
//!
//! Validates that in worker_mode:
//! - WorkerGlobalScope and DedicatedWorkerGlobalScope are visible
//! - WorkerGlobalScope.prototype has operations (btoa, setTimeout, etc.)
//! - freeze_all_prototypes does NOT delete worker interfaces in worker mode
//! - WorkerGlobalScope is recognized as [Global] interface

mod common;

use common::*;

#[test]
fn test_worker_globalscope_visible_in_worker_mode() {
    let mut k = make_kernel_worker();
    assert_js_str(&mut k, "typeof WorkerGlobalScope", "function");
}

#[test]
fn test_dedicated_worker_globalscope_visible_in_worker_mode() {
    let mut k = make_kernel_worker();
    assert_js_str(&mut k, "typeof DedicatedWorkerGlobalScope", "function");
}

#[test]
fn test_worker_globalscope_prototype_has_operations() {
    let mut k = make_kernel_worker();
    assert_js_str(&mut k, "typeof WorkerGlobalScope.prototype.btoa", "function");
    assert_js_str(&mut k, "typeof WorkerGlobalScope.prototype.atob", "function");
    assert_js_str(&mut k, "typeof WorkerGlobalScope.prototype.setTimeout", "function");
    assert_js_str(&mut k, "typeof WorkerGlobalScope.prototype.clearTimeout", "function");
}

#[test]
fn test_worker_globalscope_attributes_on_instance() {
    let mut k = make_kernel_worker();
    // [Global] interface attributes should be on globalThis (instance),
    // NOT on WorkerGlobalScope.prototype (per codegen [Global] split).
    assert_js_str(&mut k, "typeof self", "object");
    assert_js_str(&mut k, "typeof location", "object");
    assert_js_str(&mut k, "typeof navigator", "object");
}

#[test]
fn test_worker_globalscope_not_visible_in_window_mode() {
    let mut k = make_kernel();
    let val = to_str(&k.eval_to_rust_value("typeof WorkerGlobalScope"));
    assert_eq!(val, "undefined", "WorkerGlobalScope should NOT be visible in window mode");
}

#[test]
fn test_worker_location_visible_in_worker_mode() {
    let mut k = make_kernel_worker();
    assert_js_str(&mut k, "typeof WorkerLocation", "function");
}

#[test]
fn test_worker_navigator_visible_in_worker_mode() {
    let mut k = make_kernel_worker();
    assert_js_str(&mut k, "typeof WorkerNavigator", "function");
}
