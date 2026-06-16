# Testing Conventions

> Created: 2026-06-16
> Status: accepted
> Scope: All Rust integration tests, Rust inline unit tests, Python tests
> Parent: `docs/conventions/README.md`

## Purpose

Define how tests are written, organized, and maintained in the IV8 project.
This covers Rust (primary), with references to Python conventions where they
differ.

---

## 1. Test Layer Model

```text
Layer 1: Inline Unit Tests    src/<module>.rs  #[cfg(test)] mod tests
  → Pure logic only. No V8 dependency. Fast, deterministic.
  → Examples: data structure validation, conversion rules, event loop math.

Layer 2: Integration Tests    tests/test_<layer>_<module>.rs
  → Full kernel lifecycle. Create Isolate → eval JS → assert.
  → Primary test layer for IV8. Most tests live here.

Layer 3: End-to-End Tests     tests/*.py (Python/pytest)
  → Cross-layer validation. Entry pipeline, diagnostic bridge, real samples.
  → Existing conventions (pytest, hypothesis) are preserved.
```

---

## 2. Test File Naming

Integration test files follow the pattern:

```text
tests/test_<layer>_<module>.rs

<layer>:   surface | dom | network | events | crypto | shims | kernel
<module>:  navigator | element | fetch | timers | subtle | location | embedded_v8
```

Examples:

```text
tests/test_surface_navigator.rs    → src/shims/native_env.rs (navigator)
tests/test_surface_location.rs     → src/shims/location.rs
tests/test_surface_cookie.rs       → src/shims/document_props.rs (cookie)
tests/test_dom_element.rs          → src/dom/template.rs (Element)
tests/test_network_fetch.rs        → src/network/fetch.rs
tests/test_events_timers.rs        → src/events/timers.rs
tests/test_kernel_init.rs          → src/kernel/embedded_v8.rs (init chain)
```

Existing files are renamed to this pattern over time. v0.8.51 completed the
normalization (31/31 files conforming); later versions maintain it.

Test function naming:

```text
fn test_<what_under_test>_<expected_behavior>()
fn test_<what_under_test>_<edge_case>()

Examples:
  fn test_navigator_user_agent_returns_string()
  fn test_cookie_set_get_roundtrip()
  fn test_location_href_setter_persists()
  fn test_body_used_after_text_call()
  fn test_xhr_ready_state_transitions()
```

---

## 3. Test File Organization

```text
tests/
  common/
    mod.rs                  ← Shared harness helpers
  test_surface_navigator.rs
  test_surface_location.rs
  test_surface_location_url.rs
  test_surface_cookie.rs
  test_surface_window_extras.rs
  test_surface_atob_btoa.rs
  test_surface_url.rs
  test_dom_binding.rs
  test_dom_navigation.rs
  test_dom_inner_html.rs
  test_dom_inner_html_setter.rs
  test_dom_geometry.rs
  test_network_fetch.rs
  test_network_fetch_netlog.rs
  test_network_netlog.rs
  test_network_xhr.rs
  test_events_timers.rs
  test_events_event_loop.rs
  test_events_clock.rs
  test_events_target.rs
  test_events_target_dispatch.rs
  test_events_constructors.rs
  test_events_document_target.rs
  test_events_message_channel.rs
  test_events_page_load.rs
  test_crypto_basic.rs
  test_crypto_subtle.rs
  test_canvas_webgl.rs
  test_kernel_init.rs
  test_kernel_edge_cases.rs
  test_kernel_v8_extra.rs
  test_kernel_v8_hello.rs
```

`tests/common/mod.rs` provides:

```rust
use iv8_core::kernel::embedded_v8::EmbeddedV8Kernel;
use iv8_core::kernel::KernelConfig;
use iv8_core::convert::RustValue;

/// Create a kernel with default configuration.
pub fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

/// Create a kernel with a fixed random seed for deterministic tests.
pub fn make_kernel_seeded(seed: u64) -> EmbeddedV8Kernel {
    let mut cfg = KernelConfig::default();
    cfg.random_seed = Some(seed);
    EmbeddedV8Kernel::new(cfg).unwrap()
}

/// Extract a Rust string from a RustValue for assertion comparison.
pub fn to_str(v: &RustValue) -> String {
    match v {
        RustValue::String(s) => s.clone(),
        RustValue::Null => "null".to_string(),
        RustValue::Bool(b) => b.to_string(),
        RustValue::Int(n) => n.to_string(),
        RustValue::Float(f) => f.to_string(),
        RustValue::JsObject(s) => s.clone(),
        other => format!("{:?}", other),
    }
}

/// Assert that a JS expression evaluates to the expected Rust string.
pub fn assert_js_str(kernel: &mut EmbeddedV8Kernel, js: &str, expected: &str) {
    let val = to_str(&kernel.eval_to_rust_value(js));
    assert_eq!(val, expected, "for expr: {}", js);
}

/// Assert that a JS expression evaluates to the expected RustValue.
pub fn assert_js_val(kernel: &mut EmbeddedV8Kernel, js: &str, expected: RustValue) {
    let val = kernel.eval_to_rust_value(js);
    assert_eq!(val, expected, "for expr: {}", js);
}

/// Assert that a JS expression throws (returns Null on evaluation failure).
pub fn assert_js_error(kernel: &mut EmbeddedV8Kernel, js: &str) {
    let result = kernel.eval_to_rust_value(js);
    assert_eq!(result, RustValue::Null, "expected error for expr: {}, got: {:?}", js, result);
}
```

---

## 4. Assertion Patterns

**Rule**: Use the shared `common::` harness. No local copies of `make_kernel()`
or `to_str()` in individual test files.

**Preferred**: `common::assert_js_str(k, "expr", "expected")` for simple
type/value checks.

**Acceptable**: `common::to_str(&k.eval_to_rust_value("expr"))` + `assert_eq!`
for complex assertions (multi-value comparison, pattern matching, numeric
ranges). This is the primary pattern in `test_kernel_init.rs` (94 tests, 131
assertions) where each test checks a specific JS expression against a specific
expected value — wrapping every one in a helper function would reduce
readability without adding safety.

**Prohibited**: Local `fn make_kernel()` / `fn to_str()` definitions that duplicate
`common::`. Raw `use iv8_core::RustValue;` in test files that could use `common::to_str`.

---

## 5. V8 Initialization

**Rule**: Reuse the existing `EmbeddedV8Kernel::new()` path. Do not create a
separate lightweight V8 init.

Rationale:
- `EmbeddedV8Kernel::new()` internally calls `ensure_v8_initialized()`, which
  uses `std::sync::Once` for thread-safe single initialization.
- A parallel init path creates maintenance risk: two paths that must remain
  synchronized.
- The test harness wrapper (`make_kernel()`) is a convenience constructor,
  not an alternative init path.

---

## 6. Coverage Targets

Coverage is measured along multiple dimensions. Line coverage alone is
insufficient.

| Dimension | Target | Measurement |
|---|---|---|
| Core module coverage | 100% of P0/P1 modules have ≥1 integration test | Per-module test file existence |
| Behavior coverage | Each public API has ≥1 positive and ≥1 error test | Per-function assertion count |
| Edge case coverage | Each module has ≥1 boundary/null/empty input test | Manual review per test file |
| Overall line coverage | ≥80% (baseline target) | cargo-tarpaulin |

P0 modules (补環境-critical): `native_env.rs`, `document_props.rs`, `location.rs`,
`target.rs`, `event_loop.rs`, `timers.rs`.

P1 modules (execution-critical): `binding.rs`, `template.rs`, `fetch.rs`,
`xhr.rs`, `date_interceptor.rs`.

---

## 7. Determinism

**Rule**: Tests that depend on `Math.random()` or `Date.now()` must use a
fixed seed (`make_kernel_seeded()`). Tests that depend on event loop timing
must use `eventLoop.advance()` with explicit millisecond values.

---

## 8. Python Tests

Python tests (pytest) have their own mature conventions and are not
restructured in this pass. Rust test conventions do not apply to Python.

The Python test layer is referenced here for completeness:
- Layer: End-to-End (cross-layer validation)
- Framework: pytest + hypothesis
- Location: `tests/*.py`
- Naming: `test_<capability>.py` (per naming-conventions.md)

---

## 9. Prohibited Patterns

| Pattern | Reason |
|---|---|
| Macros for assertion wrapping | Functions are sufficient, macros add unnecessary complexity |
| Separate lightweight V8 init | Creates dual-path maintenance risk |
| Raw `assert_eq!` with inline `to_str()` in test bodies | Use `assert_js_str` from common/mod.rs |
| `_`-prefixed test files committed to git | Violates .gitignore convention |
| Test names with version labels (`test_v0850_*`) | Use capability-based naming per naming-conventions.md |
| Tests without `DONT_ENUM`-level property checks | Descriptor shape matters for补環境 correctness |

---

## 10. Review Checklist

- [x] Test layers are defined (unit / integration / e2e)
- [x] File naming convention is specified (`test_<layer>_<module>.rs`)
- [x] Function naming convention is specified
- [x] Common harness helpers are defined in `tests/common/mod.rs`
- [x] Assertion pattern is function encapsulation (no macros)
- [x] V8 init reuses existing `EmbeddedV8Kernel::new()`
- [x] Coverage targets are multi-dimensional (not line-only)
- [x] Determinism requirements are specified
- [x] Python tests are explicitly out of scope for restructuring
- [x] Prohibited patterns are listed
