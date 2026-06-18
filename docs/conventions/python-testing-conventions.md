# Python Testing Conventions

> Created: 2026-06-18
> Status: accepted
> Scope: All Python test files under `tests/`
> Parent: `docs/conventions/testing-conventions.md` §8

## Purpose

Define how Python tests are written, organized, and maintained in the IV8
project. Complements the main `testing-conventions.md` which covers Rust.
Python tests are the primary end-to-end validation layer — they exercise
the full `iv8_rs` package through PyO3 bindings.

---

## 1. Test File Naming

```text
tests/test_<module>.py

<module>:  matches a source module in python/iv8_rs/
           或 capability area (probe, environment, crypto, deobf, etc.)
```

Examples:

```text
tests/test_cfg.py                      → python/iv8_rs/cfg.py
tests/test_taint.py                    → python/iv8_rs/taint.py
tests/test_isolation.py                → python/iv8_rs/isolation.py
tests/test_environment_toolchain_candidates.py → python/iv8_rs/environment_toolchain_candidate_mapping.py
tests/test_crypto_detection.py         → python/iv8_rs/patterns.py (crypto detection engine)
```

**Prohibited**: `test_v0850_*` version-tagged names, `test_` prefix for non-test scripts.

## 2. Test Function Naming

```text
def test_<what_under_test>_<expected_behavior>():
```

Examples:

```python
def test_parse_trace_handles_3_field_format():
def test_cfg_from_trace_detects_loops():
def test_ctx_eval_returns_integer():
def test_fingerprint_webdriver_is_false():
```

**Prohibited**: generic names like `test_1()`, `test_run()`, or mirroring class names without behavior description.

## 3. Import Safety

All test files that import `iv8_rs` at module level MUST use `pytest.importorskip`:

```python
# Correct — test file can be collected even without Rust extension
iv8_rs = pytest.importorskip("iv8_rs")

# Wrong — crashes at import time if iv8_rs not built
import iv8_rs
```

`conftest.py` itself must use `importorskip` so fixture registration does not fail collection.

Files that only test `tools/` modules (no `iv8_rs` dependency) are exempt.

## 4. Fixture Patterns

### Standard Fixtures (in `conftest.py`)

All tests that create a `JSContext` MUST use shared fixtures from conftest.py.
Do NOT duplicate fixture definitions in individual test files.

```python
# conftest.py provides:
#   ctx         → JSContext() with default environment, auto-closed via yield
#   ctx_custom  → JSContext() with Chrome fingerprint, auto-closed via yield
```

### When to Create New Fixtures

Create a fixture when:
- The setup is used by 3+ test files (→ put in conftest.py)
- The setup is domain-specific to a single module (→ put in that module's test file)

### Scope

```python
# Function-scoped (default) — fresh context per test, safest
@pytest.fixture
def ctx():
    c = JSContext()
    yield c
    c.close()

# Module-scoped — shared context for speed, only when tests are read-only
@pytest.fixture(scope="module")
def shared_ctx():
    c = JSContext()
    yield c
    c.close()
```

**Rule**: default to function-scoped. Module scope is allowed only when the test file
explicitly documents that tests are read-only (no mutable state on the context).

## 5. Contract Tests

Contract tests validate that fixture JSON conforms to schema. They should NOT
exist as standalone test files with 1-2 tests each.

### Pattern

```python
import pytest
from experimental_contract_helpers import load_fixture, assert_no_strong_evidence, assert_diagnostic

CONTRACT_CASES = [
    ("deobf-registry", ["schema_version", "entries", "selection_report"]),
    ("deobf-sandbox", ["schema_version", "opt_in_level", "source_mutated"]),
    ("cff", ["schema_version", "detected", "variant", "confidence"]),
    # ... all 17 families
]

@pytest.mark.parametrize("family,required_fields", CONTRACT_CASES)
def test_contract_schema(family, required_fields):
    report = load_fixture(family)
    for field in required_fields:
        assert field in report, f"{family} missing {field}"

@pytest.mark.parametrize("family", [c[0] for c in CONTRACT_CASES])
def test_contract_no_strong_evidence(family):
    report = load_fixture(family)
    assert_no_strong_evidence(report.get("evidence", []))
```

**Rule**: All contract fixtures for a capability area should be parametrized into
a single test function or a single file per area. Do NOT create one file per contract.

## 6. Behavioral vs Contract Distinction

| Type | Runs JS via `iv8_rs`? | Tests real behavior? | Example |
|---|---|---|---|
| **Behavioral** | Yes | Yes | `test_crypto_full.py` |
| **Contract** | No | No (validates JSON schema) | `test_deobf_sandbox_contract.py` |
| **Both** | Yes | Both | `test_experimental_report_contract.py` |

Behavioral tests are the primary quality gate. Contract tests are supplementary
schema enforcement.

## 7. Assertion Patterns

```python
# Preferred: plain assert with descriptive message
assert result == expected, f"got {result}, expected {expected}"

# For exceptions:
with pytest.raises(ValueError, match="expected pattern"):
    bad_function()

# For structured comparison:
assert set(result.keys()) == {"a", "b", "c"}

# For approximate float:
assert abs(result - expected) < 0.001
```

**Prohibited**: `assert True`, `assert 1 == 1`, or any assertion that cannot fail.

## 8. Parametrize

Use `@pytest.mark.parametrize` for data-driven tests instead of loop-based assertions:

```python
# Correct
@pytest.mark.parametrize("algo", ["AES", "SHA-256", "MD5", "XTEA"])
def test_detect_constants_finds_algo(algo):
    trace = make_constant_trace(algo)
    result = detect_constants(trace, load_constants_db())
    assert any(c.algorithm == algo for c in result)

# Wrong
def test_detect_constants():
    for algo in ["AES", "SHA-256", "MD5", "XTEA"]:
        trace = make_constant_trace(algo)
        result = detect_constants(trace, load_constants_db())
        assert any(c.algorithm == algo for c in result)
```

This gives one test case per parameter value, with independent pass/fail reporting.

## 9. Hypothesis (Property-Based Testing)

Use `hypothesis` for invariant testing — properties that should hold for ANY valid input:

```python
from hypothesis import given, strategies as st, assume, settings

@given(st.lists(trace_line_strategy(), min_size=1, max_size=1000))
@settings(max_examples=500)
def test_parse_trace_is_idempotent(raw):
    parsed = parse_trace(raw)
    # parse_trace should never crash for any valid input
    assert parsed is not None
```

**When to use**:
- Parser functions (trace parsing, value conversion)
- Graph algorithms (CFG properties)
- Detection engines (no false negatives on known patterns)
- Data round-trips (serialize → deserialize → compare)

**When NOT to use**:
- V8-dependent tests (hypothesis + JSContext is too slow)
- Tests requiring exact output values (not property-based)

## 10. Organization

```text
tests/
  conftest.py                  ← Shared fixtures (ctx, ctx_custom, debugger, trace data)
  experimental_contract_helpers.py ← Contract test helpers (load_fixture, assert_no_strong_evidence)
  
  test_<module>.py             ← Per-module behavioral tests
  test_<module>_contract.py    ← Per-module contract tests (when multiple families needed)
  test_properties.py           ← Hypothesis property-based tests
  test_compat.py               ← Compatibility suite (tests/compat/)
```

### Scripts vs Tests

Files starting with `test_` are collected by pytest. Non-test scripts MUST NOT
use the `test_` prefix:

```text
# Correct
_audit_full.py              ← Script, not a test
test_crypto_detection.py    ← Test file

# Wrong
test_v0850_rs_verify.py     ← Script with test_ prefix — collected but has zero test functions
```

## 11. Prohibited Patterns

| Pattern | Reason |
|---|---|
| `import unittest` / `unittest.TestCase` | Use pytest native functions |
| `sys.path.insert(0, ...)` for imports | Use proper package structure |
| `test_` prefix on non-test scripts | Confuses pytest collection |
| Per-file `ctx`/`ctx_custom` fixture duplication | Use conftest.py |
| Single-fixture contract files (1-2 tests) | Use parametrize in shared contract file |
| `import iv8_rs` without `importorskip` at module level | Crashes collection if extension not built |
| `assert True` / `assert 1 == 1` | Not a test |
| Manual `ctx.close()` in try/finally | Use fixture with `yield` |

## 12. Coverage Targets

Per-module coverage, measured by test file existence + behavioral test presence:

| Tier | Target | Modules |
|---|---|---|
| P0 (runtime-critical) | ≥ 1 behavioral test file | trace, patterns, cfg, entry |
| P1 (analysis tools) | ≥ 1 behavioral test file | taint, isolation, vm_diff |
| P2 (environment) | ≥ 1 behavioral test file | environment, environment_toolchain_*, probe |
| P3 (reports) | ≥ 1 contract test | deobf_reports, vm_reports, ir_reports, string_array_reports |

Source modules with zero test coverage (currently 4) should be brought to P2+ level.

## 13. Review Checklist

- [ ] Test file named `test_<module>.py`, not version-tagged
- [ ] Test functions use `test_<what>_<expected>()` naming
- [ ] `importorskip("iv8_rs")` at module level (or in conftest if shared fixture)
- [ ] No per-file fixture duplication — use conftest.py
- [ ] Contract tests parametrized, not standalone thin files
- [ ] No `unittest.TestCase`, no `sys.path.insert`
- [ ] No `test_` prefix on non-test scripts
- [ ] Hypothesis used for invariant testing where applicable
- [ ] No trivial assertions (`assert True`)
