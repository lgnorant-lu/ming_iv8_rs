# Logging Conventions

> Created: 2026-06-28
> Status: accepted
> Scope: All Rust crates (iv8-core, iv8-py, iv8-surface, iv8-undetect)
> Parent: `docs/conventions/README.md`

## Purpose

Define how structured logging is used across the IV8 project. IV8 uses the
`tracing` crate with a **typed log event catalog** (`telemetry.rs`) that
provides category-based filtering, schema enforcement, and test coverage.

## Architecture

### Design sources

The catalog design synthesizes patterns from:

| Source | Pattern borrowed |
|--------|-----------------|
| V8 `TRACE_EVENT` | Category-based filtering (`v8`, `v8.runtime`, `v8.gc`) |
| OpenTelemetry Events | Event name = schema identity; same name = same fields |
| Node.js `trace_events` | Programmatic category enable/disable |
| tracing `Targets` | Prefix-based target filtering (`iv8.dom` matches `iv8.dom.template`) |
| `tracing-test` | `#[traced_test]` for log assertion in tests |

### Why `tracing` (not `log`)

- Structured fields proved critical for debugging (`interface=`, `proto_copied=`, `same_ctor=`)
- `release_max_level_debug` eliminates TRACE at compile time
- `tracing-test` enables log assertion in tests
- `log` feature flag allows interop with `log`-based consumers
- IV8 is cdylib — owns its tracing global, no subscriber conflict

### Why typed catalog (not direct tracing calls)

| Direct `tracing::debug!(...)` | Typed catalog `telemetry::init_proto_merge(...)` |
|-------------------------------|--------------------------------------------------|
| No schema enforcement | Function signature = schema (can't forget a field) |
| Inconsistent field names | Consistent (same function = same fields) |
| No discoverability | IDE autocomplete shows all events |
| No test target | Each function is a testable unit |
| No introspection | `catalog()` returns all event specs |
| No safety annotation | Each event has Safety::Safe/Diagnostic/Sensitive |

### Why not `#[instrument]`

IV8 is single-threaded synchronous. Span context propagation adds overhead
without async benefit. Use events (not spans) for most code paths.

## Categories

Categories are hierarchical strings used as tracing `target` overrides.
EnvFilter matches by prefix.

| Category | Scope | Example events |
|----------|-------|----------------|
| `iv8.init` | Kernel init phases | proto_merge, dom_templates_built, phase_skipped |
| `iv8.dom` | DOM template/binding | dom_template_created, dom_binding_panic |
| `iv8.config` | Config resolution, state | state_created, state_dropped, state_error |
| `iv8.worker` | Worker lifecycle | worker_script_error |
| `iv8.callback` | V8 callback execution | callback_panic, convert_error |
| `iv8.eval` | JS evaluation | eval_complete, eval_error |
| `iv8.console` | JS console.* | console_message |
| `iv8.inspector` | Inspector lifecycle | inspector_listening, inspector_connected |
| `iv8.shim` | Shim installation | shim_installed |
| `iv8.canvas` | Canvas/WebGL | canvas_fingerprint_warning |

## Log Event Catalog

The catalog is defined in `crates/iv8-core/src/telemetry.rs`. Each event is
a typed function. The function list IS the documentation.

### Current events

| Function | Category | Level | Safety | Fields |
|----------|----------|-------|--------|--------|
| `init_browser_surface_installed` | iv8.init | INFO | Safe | interface_count |
| `init_codegen_prototypes_captured` | iv8.init | DEBUG | Safe | count |
| `init_dom_templates_built` | iv8.init | DEBUG | Safe | (none) |
| `init_dom_constructors_installed` | iv8.init | DEBUG | Safe | (none) |
| `init_proto_merge_start` | iv8.init | DEBUG | Safe | interface_count |
| `init_proto_merge` | iv8.init | DEBUG | Safe | interface, proto_copied, proto_skipped, ctor_copied, same_ctor |
| `init_proto_merge_complete` | iv8.init | DEBUG | Safe | (none) |
| `init_same_ctor_warning` | iv8.init | WARN | Diagnostic | interface |
| `init_phase_start` | iv8.init | INFO | Safe | phase |
| `init_phase_complete` | iv8.init | INFO | Safe | phase, duration_ms |
| `init_phase_failed` | iv8.init | ERROR | Diagnostic | phase, error |
| `init_phase_skipped` | iv8.init | DEBUG | Safe | phase, reason |
| `dom_template_created` | iv8.dom | DEBUG | Safe | interface |
| `dom_binding_panic` | iv8.dom | ERROR | Diagnostic | operation |
| `state_created` | iv8.config | INFO | Safe | strict_compat, time_mode, js_api_name, env_entries |
| `state_dropped` | iv8.config | INFO | Safe | eval_count |
| `state_error` | iv8.config | WARN | Diagnostic | error |
| `worker_script_error` | iv8.worker | ERROR | Diagnostic | error |
| `worker_import_script_not_found` | iv8.worker | WARN | Diagnostic | url |
| `callback_panic` | iv8.callback | ERROR | Diagnostic | callback, panic_msg |
| `convert_error` | iv8.callback | WARN | Diagnostic | type_name |
| `v8_fatal_error` | iv8.callback | ERROR | Diagnostic | file, line, message |
| `v8_oom` | iv8.callback | ERROR | Safe | location, is_heap_oom |
| `v8_uncaught_exception` | iv8.eval | ERROR | Diagnostic | message |
| `rust_panic` | iv8.callback | ERROR | Diagnostic | msg |
| `eval_complete` | iv8.eval | DEBUG | Safe | success, duration_ms |
| `eval_error` | iv8.eval | WARN | Diagnostic | message |
| `console_message` | iv8.console | DEBUG | Sensitive | method, message |
| `inspector_listening` | iv8.inspector | INFO | Safe | port |
| `inspector_connected` | iv8.inspector | INFO | Safe | port |
| `inspector_disconnected` | iv8.inspector | INFO | Safe | (none) |
| `inspector_accept_error` | iv8.inspector | WARN | Diagnostic | error |
| `shim_installed` | iv8.shim | DEBUG | Safe | name |
| `canvas_fingerprint_warning` | iv8.canvas | WARN | Diagnostic | parameter, renderer, forbidden |

### Adding a new event

1. Add a function to `telemetry.rs` with typed parameters
2. Add an `EventSpec` entry to the `CATALOG` const array
3. Add a `#[test]` if the event has non-trivial logic
4. Call the function from the appropriate code path

## Safety Levels

| Level | Meaning | Example |
|-------|---------|---------|
| Safe | No sensitive data, safe in production | interface name, count |
| Diagnostic | May contain internal state, debug-only | same_ctor, error message |
| Sensitive | May contain user data, never log in production | config value, UA string |

**Rule**: Never log actual fingerprint values, cookie contents, or user data.
Use `Safety::Sensitive` and only emit at TRACE level (compiled out in release).

## Runtime Control

### Environment Variable

```bash
# All IV8 categories at debug
IV8_LOG=iv8=debug

# Specific category at trace
IV8_LOG=iv8.init=trace

# Multiple categories
IV8_LOG=iv8.init=debug,iv8.worker=trace

# Production (warnings only)
IV8_LOG=warn
```

### Python API

```python
import iv8_rs
iv8_rs.enable_logging("debug")  # all categories
```

### Rust Tests

```bash
RUST_MIN_STACK=67108864 IV8_LOG=iv8.init=debug cargo test -p iv8-core --lib
```

## Test Log Assertions

Use `tracing-test` crate to assert expected events:

```rust
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_init_emits_proto_merge_events() {
    let _ = common::make_kernel();
    assert!(logs_contain("iv8.init"));
    assert!(logs_contain("proto_merge"));
}
```

## Performance

- `release_max_level_debug`: TRACE compiled out in release
- No subscriber = zero output, near-zero overhead (atomic load)
- Catalog functions are inlined (zero function call overhead)
- Structured fields evaluated only if level is enabled

## Security

IV8 is an anti-detection tool. Logging could be a detection signal.

**Mitigations**:
1. Subscriber NOT auto-initialized — only via explicit `enable_logging()`
2. `release_max_level_debug`: DEBUG/TRACE compiled out in release
3. No logging in V8 callback hot paths at INFO+ (DEBUG only)
4. Safety annotations prevent sensitive data in log fields

## Review Checklist

- [ ] All log events go through `telemetry.rs` catalog functions
- [ ] No `eprintln!` or `println!` in production code (build.rs exempt)
- [ ] No direct `tracing::debug!` calls outside `telemetry.rs`
- [ ] Each catalog event has an `EventSpec` entry
- [ ] Safety level set for each event
- [ ] `IV8_LOG` filtering tested for at least one category
- [ ] Key tests have `#[traced_test]` log assertions
- [ ] `test_coverage_matrix_satisfied` passes (category x level coverage)
- [ ] `test_no_direct_tracing_outside_telemetry` passes

## Coverage Matrix

The `COVERAGE_MATRIX` const in `telemetry.rs` defines expected coverage:
each category must have at least one event at each listed level.

| Category | ERROR | WARN | INFO | DEBUG | TRACE |
|----------|-------|------|------|-------|-------|
| iv8.init | x | x | x | x | |
| iv8.dom | x | | | x | |
| iv8.config | | x | x | | |
| iv8.worker | x | x | | | |
| iv8.callback | x | x | | | |
| iv8.eval | | x | | x | |
| iv8.console | | | | x | |
| iv8.inspector | | x | x | | |
| iv8.shim | | | | x | |
| iv8.canvas | | x | | | |

`test_coverage_matrix_satisfied` validates this matrix at test time.

### Adding coverage

1. Identify the category and level needed
2. Add an `EventSpec` to the `CATALOG` const
3. Add a typed function
4. If adding a new level to an existing category, update `COVERAGE_MATRIX`
5. The test will fail until coverage is satisfied
