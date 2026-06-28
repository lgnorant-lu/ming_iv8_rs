# Logging Conventions

> Created: 2026-06-28
> Status: accepted
> Scope: All Rust crates (iv8-core, iv8-py, iv8-surface, iv8-undetect)
> Parent: `docs/conventions/README.md`

## Purpose

Define how structured logging and tracing is used across the IV8 project.
IV8 uses the `tracing` crate (already a workspace dependency) for all
diagnostic output. This document standardizes module hierarchy, log level
semantics, and structured field conventions.

## Module Hierarchy

The `target` (module path) follows the source tree. Each module has a
clear scope:

| Target | Scope | Key events |
|--------|-------|------------|
| `iv8_core::kernel` | Isolate lifecycle, eval, context | init, eval, close |
| `iv8_core::kernel::embedded_v8` | Kernel init phases | install_all, build_dom, chain_protos |
| `iv8_core::dom::template` | DOM template creation/merge | template built, property merged |
| `iv8_core::dom::binding` | DOM callbacks | createElement, appendChild, etc. |
| `iv8_core::shims::native_env` | Navigator/Screen getters | config resolution, value fallback |
| `iv8_core::shims::worker` | Worker lifecycle | spawn, message, terminate |
| `iv8_core::events::binding` | Event loop, timers | advance, setTimeout, rAF |
| `iv8_core::events::timers` | Timer callbacks | fire, cancel |
| `iv8_core::convert` | Rust ↔ V8 conversion | type mismatch, fallback |
| `iv8_core::safe_callback` | Panic catching | callback panic |
| `iv8_core::inspector` | DevTools protocol | connect, disconnect |
| `iv8_core::shims::console` | JS console.* | console.log/warn/error |
| `iv8_surface` | BrowserSurface install | template count, install result |
| `iv8_py::context` | Python JSContext | create, eval, close, thread check |
| `iv8_py::logging` | Logging init | enable_logging, filter |

## Log Level Semantics

### TRACE

Per-element detail inside a larger operation. Disabled in release builds
by default (`release_max_level_debug` in Cargo.toml).

Examples:
- Per-property descriptor copy in chain_dom_prototypes
- Per-member installation in codegen template creation
- Per-argument conversion in eval
- Per-timer fire in event loop

### DEBUG

Per-operation summary. The primary debugging level. Enabled via
`IV8_LOG=iv8=debug` or `enable_logging("debug")`.

Examples:
- Template created for interface X with N members
- Property merge: copied=N skipped=N failed=N for interface X
- Config resolved: key=path source=env/profile/default value=...
- Worker spawned with URL=...
- BrowserSurface installed: N interfaces across M files

### INFO

Lifecycle events visible in production. Enabled by default.

Examples:
- Kernel initialized (isolate created, context built)
- BrowserSurface installation complete (N interfaces)
- DevTools inspector connected
- Worker spawned/terminated
- GC threshold reached

### WARN

Fallback or degraded behavior. Always visible.

Examples:
- Config key not found, using default value
- Codegen mixin not found for interface
- External script load failed, skipping
- Thread stack size insufficient, using fallback

### ERROR

Failures that should be investigated. Always visible.

Examples:
- V8 callback panic (caught by safe_callback)
- Template creation failed
- BrowserSurface installation error
- JSContext dropped from wrong thread

## Structured Fields

Always use structured fields (`key = value`) instead of string
interpolation. This enables filtering and aggregation in log tools.

```rust
// Correct
tracing::debug!(
    interface = name,
    codegen_props = codegen_count,
    dom_props = dom_count,
    same_ctor = same_ctor,
    "prototype merge"
);

// Wrong
tracing::debug!("prototype merge for {}: codegen={} dom={} same={}", name, codegen_count, dom_count, same_ctor);
```

### Standard Field Names

| Field | Type | Usage |
|-------|------|-------|
| `interface` | &str | Interface name (Document, Element, etc.) |
| `count` | usize | Item count |
| `duration_ms` | u64 | Elapsed time |
| `source` | &str | Value source (env, profile, default) |
| `result` | &str | Operation result (ok, fail, skip) |
| `error` | &str | Error message |

## Spans

Use `#[instrument]` on public API functions and key internal operations.
Use `tracing::debug_span!` for sub-operations.

```rust
#[tracing::instrument(skip_all, fields(interface = %def.name.as_deref().unwrap_or("unknown")))]
pub fn generate_template_function(def: &Definition, ...) -> String {
    // ...
}
```

### Span Hierarchy

```
kernel_init (INFO span)
  install_all (DEBUG span)
    create_template:Document (DEBUG span)
    create_template:Element (DEBUG span)
  build_dom_templates (DEBUG span)
  install_dom_constructors (DEBUG span)
  chain_dom_prototypes (DEBUG span)
    merge:Document (TRACE span)
    merge:Element (TRACE span)
```

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
ctx = iv8_rs.JSContext()
```

### Rust Tests

```bash
RUST_MIN_STACK=67108864 IV8_LOG=iv8=debug cargo test -p iv8-core --lib
```

## Performance

- `tracing` with `release_max_level_debug`: TRACE calls are compiled out in release
- `tracing` with `release_max_level_info`: DEBUG and TRACE compiled out in release
- Structured fields are only evaluated if the level is enabled
- `#[instrument]` creates spans only if the level is enabled
- No measurable overhead when logging is disabled

## Migration Guide

1. Replace all `eprintln!` with `tracing::warn!` or `tracing::error!`
2. Replace all `println!` (non-build.rs) with `tracing::info!`
3. Add `#[instrument]` to public API functions
4. Add `tracing::debug!` to operations that needed manual eprintln! debugging
5. Use structured fields consistently

## Review Checklist

- [ ] No `eprintln!` or `println!` in production code (build.rs exempt)
- [ ] All tracing calls use structured fields
- [ ] Key functions have `#[instrument]`
- [ ] Module hierarchy matches source tree
- [ ] Log levels follow the semantics in this document
- [ ] IV8_LOG env var documented and tested
