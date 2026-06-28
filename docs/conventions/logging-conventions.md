# Logging Conventions

> Created: 2026-06-28
> Status: accepted
> Scope: All Rust crates (iv8-core, iv8-py, iv8-surface, iv8-undetect)
> Parent: `docs/conventions/README.md`

## Purpose

Define how structured logging is used across the IV8 project. IV8 uses the
`tracing` crate directly — no custom abstraction layer. This document
standardizes module hierarchy, log level semantics, structured field
conventions, and security considerations.

## Architecture Decision

### Why `tracing` (not `log` or custom)

| Factor | `tracing` | `log` | Custom |
|--------|-----------|-------|--------|
| Structured fields | Yes (`key = value`) | No | Manual |
| Compile-time elimination | `release_max_level_debug` | `release_max_level_*` | Manual |
| Test assertions | `tracing-test` crate | Manual | Manual |
| Already a dependency | Yes | No | No |
| `log` interop | Via `log` feature | Native | Manual |
| Overhead when disabled | Near-zero | Near-zero | Varies |

`tracing` is retained because:
1. Structured fields proved critical for debugging (interface=, proto_copied=, same_ctor=)
2. `release_max_level_debug` eliminates TRACE in release builds at compile time
3. `tracing-test` enables log assertion in tests without manual capture
4. The `log` feature flag allows interop with `log`-based consumers
5. IV8 is compiled as cdylib — owns its tracing global, no subscriber conflict

### Why no abstraction layer

The previous iteration created `telemetry.rs` with custom span helper macros
(`iv8_merge_span!`, etc.). This was **premature abstraction**:
- `tracing::debug!(interface = %name, "merge")` is as readable as a custom macro
- Custom macros add maintenance burden and indirection
- The Rust community convention is to use `tracing` directly
- Removed in favor of direct `tracing` usage

### Why no `#[instrument]` (for now)

`#[instrument]` creates spans automatically, but IV8 is single-threaded
synchronous — span context propagation adds overhead without async benefit.
Use `tracing::debug!` events for most code paths. Use `tracing::debug_span!`
only for the kernel init phase (which has a clear hierarchy worth tracking).

## Security Consideration

IV8 is an anti-detection tool. Logging to stderr in production could itself
be a detection signal if a monitoring script inspects process output.

**Mitigations** (all already in place):
1. `release_max_level_debug`: TRACE and DEBUG compiled out in release builds
2. Subscriber is NOT auto-initialized — only when user calls `enable_logging()`
3. In production (no `enable_logging()` call), zero logging output
4. INFO/WARN/ERROR events are minimal in hot paths (eval, callbacks)

**Rule**: Never log in V8 callback hot paths at INFO or above. Use DEBUG or
TRACE for callback diagnostics, which are compiled out in release.

## Module Hierarchy

The `target` (module path) follows the source tree automatically via
`module_path!()`. No manual target specification needed.

| Target | Scope | Key events |
|--------|-------|------------|
| `iv8_core::kernel::embedded_v8` | Kernel init phases | install_all, build_dom, chain_protos |
| `iv8_core::dom::template` | DOM template creation/merge | template built, property merged |
| `iv8_core::dom::binding` | DOM callbacks | createElement, appendChild |
| `iv8_core::shims::native_env` | Navigator/Screen getters | config resolution, value fallback |
| `iv8_core::shims::worker` | Worker lifecycle | spawn, message, terminate |
| `iv8_core::events::binding` | Event loop, timers | advance, setTimeout, rAF |
| `iv8_core::convert` | Rust <-> V8 conversion | type mismatch, fallback |
| `iv8_core::safe_callback` | Panic catching | callback panic |
| `iv8_core::inspector` | DevTools protocol | connect, disconnect |
| `iv8_core::shims::console` | JS console.* | console.log/warn/error |
| `iv8_surface` | BrowserSurface install | template count, install result |
| `iv8_py::context` | Python JSContext | create, eval, close, thread check |
| `iv8_py::logging` | Logging init | enable_logging, filter |

## Log Level Semantics

### TRACE

Per-element detail inside a larger operation. Compiled out in release builds.

Examples:
- Per-property descriptor copy in chain_dom_prototypes
- Per-member installation in codegen template creation
- Per-argument conversion in eval

### DEBUG

Per-operation summary. The primary debugging level. Enabled via
`IV8_LOG=iv8=debug` or `enable_logging("debug")`.

Examples:
- `chain_dom_prototypes start interfaces=39`
- `prototype property merge interface=Document proto_copied=191 same_ctor=false`
- `dom templates built`
- `BrowserSurface installation complete interfaces=1287`

### INFO

Lifecycle events visible in production. Enabled by default.

Examples:
- Kernel initialized
- DevTools inspector connected
- Worker spawned/terminated

### WARN

Fallback or degraded behavior. Always visible.

Examples:
- `dom constructor equals codegen; override may have failed`
- Config key not found, using default value
- External script load failed, skipping

### ERROR

Failures requiring investigation. Always visible.

Examples:
- V8 callback panic (caught by safe_callback)
- Template creation failed
- JSContext dropped from wrong thread

## Structured Fields

Always use structured fields (`key = value`) instead of string interpolation.

```rust
// Correct
tracing::debug!(
    interface = %name,
    proto_copied = copied,
    same_ctor = same_ctor,
    "prototype property merge"
);

// Wrong
tracing::debug!("merge for {}: copied={} same={}", name, copied, same_ctor);
```

### Standard Field Names

| Field | Type | Usage |
|-------|------|-------|
| `interface` | &str | Interface name (Document, Element, etc.) |
| `count` | usize | Item count |
| `source` | &str | Value source (env, profile, default) |
| `result` | &str | Operation result (ok, fail, skip) |
| `error` | &str | Error message |
| `same_ctor` | bool | Constructor identity check result |

## Init Phase Spans

The kernel init phase uses `tracing::debug_span!` for hierarchical context:

```
kernel_init
  install_all (1287 templates)
  build_dom_templates (39 templates)
  install_dom_constructors
  chain_dom_prototypes
    merge:Document (191 properties)
    merge:Element (119 properties)
```

Other phases (eval, callbacks, config resolution) use events only, not spans.

## Runtime Control

### Environment Variable

```bash
# All modules at debug
IV8_LOG=iv8=debug

# Specific module at trace
IV8_LOG=iv8_core::dom::template=trace

# Multiple modules
IV8_LOG=iv8_core::kernel=debug,iv8_core::shims=info

# Production (warnings only)
IV8_LOG=warn
```

### Python API

```python
import iv8_rs
iv8_rs.enable_logging("debug")  # or "trace", "info", "warn", "error"
```

### Rust Tests

```bash
RUST_MIN_STACK=67108864 IV8_LOG=iv8=debug cargo test -p iv8-core --lib
```

## Test Log Assertions

Use `tracing-test` crate to assert that expected log events occur:

```rust
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_kernel_init_produces_debug_logs() {
    let mut kernel = common::make_kernel();
    assert!(logs_contain("chain_dom_prototypes"));
    assert!(logs_contain("dom templates built"));
}
```

This provides **logging coverage** — assertions that key operations produce
expected tracing events. Not a formal harness (H<NN>), but a test-level
convention that can be formalized later if valuable.

## Performance

- `release_max_level_debug`: TRACE calls compiled out in release
- Structured fields evaluated only if level is enabled (tracing lazy eval)
- No spans in hot paths (eval, callbacks) — events only
- No measurable overhead when logging is disabled (no subscriber set)

## Migration Rules

1. **No `eprintln!` or `println!`** in production code (build.rs exempt)
2. Use `tracing::debug!` / `tracing::warn!` / `tracing::error!` directly
3. Use structured fields, not string interpolation
4. Use `tracing::debug_span!` only for init phase hierarchy
5. Do not create custom logging abstraction layers

## Review Checklist

- [ ] No `eprintln!` or `println!` in production code (build.rs exempt)
- [ ] All tracing calls use structured fields
- [ ] Init phase has `tracing::debug!` events at each stage
- [ ] Log levels follow the semantics in this document
- [ ] No logging in V8 callback hot paths at INFO or above
- [ ] `IV8_LOG` env var documented and tested
- [ ] Key tests have `#[traced_test]` log assertions
