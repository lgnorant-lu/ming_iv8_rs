# Docstring Conventions

> Status: **active working standard** (2026-07-16)  
> Scope: All Rust `///` and Python `"""` docstrings in the iv8-rs project  
> Supersedes: informal one-liner docstrings; see `api-documentation-conventions.md` for API layer metrics

## 1. Purpose

Docstrings are the **single source of truth** for API documentation in iv8-rs.
Sphinx autodoc + Napoleon extracts them at build time. Hand-written `docs/api/`
contracts supplement host bounds, thread safety, and ICU notes — they do not
repeat parameter tables.

## 2. Format: Google-style (Napoleon-compatible)

Use **Google-style** docstrings (not numpydoc). Google-style is more compact
and Napoleon parses both.

```rust
/// Brief one-line summary (imperative verb, period at end).
///
/// Extended description when needed. Blank line before sections.
///
/// Args:
///     param1: Description of the first parameter.
///     param2: Description of the second parameter. Default: ``None``.
///     param3 (optional): Description. Default: ``True``.
///
/// Returns:
///     Description of the return value. May include shape info.
///
/// Raises:
///     RuntimeError: When context is closed / wrong thread.
///     ValueError: When argument is out of valid range.
///
/// Example:
///     ```python
///     with iv8_rs.JSContext() as ctx:
///         result = ctx.eval("1 + 1")
///     ```
///
/// Note:
///     Additional notes about host bounds or known limitations.
#[pyfunction]
pub fn my_api(...) -> PyResult<...> { ... }
```

### 2.1 Required sections by tier

| Tier | Sections required | Applies to |
|---|---|---|
| A (kernel / host) | Summary + Args + Returns + Raises + Example | `JSContext`, `Debugger`, `instrument_source`, entry, etc. |
| B (behavioral) | Summary + Args + Returns | Analysis helpers, env runners, etc. |
| C (catalog) | Summary only | DTOs, serde helpers, schema constants |

Tier definitions: `docs/conventions/api-documentation-conventions.md`.

### 2.2 Summary line rules

- **Imperative verb**: "Evaluate JavaScript..." NOT "Evaluates JavaScript..."
- Ends with period.
- Describes the action, not the internal state.
- Max ~72 chars for first line.

### 2.3 Parameter conventions

```rust
/// Args:
///     source: JavaScript source string to evaluate.
///     url: Resource URL. Must be absolute (https://...).
///     handler: Callable Python function. Signature: ``(url, method) -> (status, body) | None``.
///     strict_compat: If ``True`` (default), replicate iv8 0.1.x conversion.
///                    If ``False``, enable enhanced types (BigInt→int, Date→datetime).
```

- Use **indented** format (3 spaces after `///` + 4 spaces indent).
- Mention default value where applicable: "Default: ``True``."
- Mention type where not obvious from signature.
- Separate boolean branches with examples.

### 2.4 Return conventions

```rust
/// Returns:
///     Tuple of ``(patched_source, info_dict)``.
///     ``info_dict`` keys: ``mode``, ``handler_array``, ``pc_var``, …
///     See ``docs/api/instrumentation/`` for full field reference.
```

- Describe shape, not just "dict" or "str".
- Cross-reference hand-written docs where exhaustive detail would bloat the docstring.

### 2.5 Raises conventions

```rust
/// Raises:
///     JSError: JavaScript runtime threw.
///     JSCompileError: Syntax error in source.
///     RuntimeError: Context already closed or wrong thread.
///     ValueError: ``time_mode`` not ``"logical"`` or ``"system"``.
```

- Only **interface-relevant** exceptions (caller can catch).
- Do NOT document internal implementation errors the caller cannot handle.
- Exception names without module prefix if they are re-exported at `iv8_rs.*`.

### 2.6 Example conventions

```rust
/// Example:
///     ```python
///     import iv8_rs
///     with iv8_rs.JSContext(time_mode="logical") as ctx:
///         result = ctx.eval("JSON.stringify(navigator.userAgent)")
///         print(result)
///     ```
```

- Use Python code blocks (`` ```python ``).
- Must be **runnable** (will be gated by D4 tests).
- One example is enough for Tier A; zero for Tier C.

### 2.7 Cross-reference conventions

Use Sphinx/autodoc cross-references where helpful:

```rust
/// See :py:meth:`iv8_rs.JSContext.eval` for evaluation details.
/// See :py:class:`iv8_rs.Debugger` for API-level tracing.
/// See :doc:`../api/instrumentation/README` for path A vs path B.
```

Use backticks for in-code references:
```rust
/// Replaces ``A[Q[U++]]()`` with a logging wrapper.
```

## 3. Language

- All docstrings in **English**.
- Exception: private notes in `//` comments may use Chinese.
- First person ("we", "our") discouraged. Use "the caller", "the user", or passive.

## 4. Validation

Docstring format is validated post-build via Sphinx warnings.
Additional checks (planned):

| Check | Tool | When |
|---|---|---|
| Args match signature | Sphinx napoleon (PR01 equivalent) | Build time |
| Summary present | Sphinx + custom | Build time |
| Tier A has Example | Manual / script | Review |
| Raises documented for interface errors | Manual / script | Review |

See `docs/api/D3_TIER_A_CHECKLIST.md` for the manual audit checklist.

## 5. Migration from existing docstrings

Existing one-liner docstrings marked as ``/** brief */`` should be expanded
to Google-style when that function is touched for any change.

## 6. Relationship to other docs

| Doc | Relationship |
|---|---|
| `api-documentation-conventions.md` | Tier definitions, metrics D1–D6 |
| `docs/api/COVERAGE.md` | Measurements against metric definitions |
| `docs/api/TIER_MAP.md` | Tier A/B/C assignment for every export |
| `docs/api/D2_TIER_A_MATRIX.md` | Cross-check Rust vs pyi vs docs signatures |
| `docs/api/D3_TIER_A_CHECKLIST.md` | Manual semantic checklist |
| `docs/quality-harness/` | Test gates (D4 runs examples; D5 test coverage) |

## 7. Enforcement

- This document governs **docstring content only**.
- Hand-written `docs/api/` pages retain their own style guidelines
  (English prose; Chinese README.zh-CN.md allowed).
- Agents and reviewers: flag violations inline.
