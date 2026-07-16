# Documentation toolchain selection (iv8-rs)

> Status: **accepted** (2026-07-17)  
> Question: Sphinx vs MkDocs vs pdoc — is the current stack optimal for this product?

## Decision

| Layer | Choice | Role |
|---|---|---|
| Machine reference | **Sphinx + autodoc + Napoleon** | Extract Rust/`///` and Python docstrings from the **built** extension |
| Human contracts | **`docs/api/**` (Markdown)** | Host bounds, tiers, honesty, offline network rules |
| Tutorials | **`docs/GUIDE.md`** (+ optional public cut) | Long narrative |
| Types for IDE | **`_iv8.pyi`** | Signature shape SoT for static tools |

**Do not switch the stack mid-flight** unless a hard blocker appears. Current stack matches PyO3-native library practice.

## Why not replace Sphinx with MkDocs / pdoc

| Option | Fit for iv8-rs | Verdict |
|---|---|---|
| **Sphinx autodoc + Napoleon** | Industry default for library API ref; proven with PyO3 (`///` → `__doc__`); RTD-friendly | **Keep** |
| **MkDocs + Material + mkdocstrings** | Excellent for prose-first product docs; mkdocstrings weaker on native/PyO3 edge cases and intersphinx depth | Optional **second** site for marketing/guides later — not a replacement for native ref |
| **pdoc / pdoc3** | Fast pure-Python; weak/historically awkward with extension submodules | **Reject** as primary for this codebase |
| **OpenAPI** | HTTP APIs | **N/A** (Python library) |
| **pyi-only** | Types only; no host bounds / network honesty | **Insufficient alone** |

References (industry, not cargo-cult):

- Sphinx remains the library-reference default when autodoc is the point (NumPy/scientific lineage; 2025–2026 Sphinx vs MkDocs comparisons).
- PyO3 projects document via Rust docstrings + Sphinx Napoleon (e.g. extension-doc writeups; SO/PyO3 guidance: document in Rust, autodoc like Python).
- MkDocs Material is preferred when **Markdown velocity** beats **reference depth**; Material entered maintenance-mode discussions in 2026 — still fine for prose, not a reason to abandon Sphinx for native API.

## Dual track is intentional (not a mistake)

```text
docs/api/*.md     = calling contracts + product honesty (read first)
Sphinx HTML       = generated reference from live __doc__ (rebuild after maturin)
_iv8.pyi          = types for editors / typecheck
```

Readers: **contracts first**, Sphinx for browsing signatures/docstrings after install.

## When to re-open the decision

Re-evaluate only if:

1. Sphinx cannot import the extension in CI for a full release matrix; or  
2. Maintainers refuse reST forever and MyST still cannot cover needed directives; or  
3. A PyO3-first doc generator becomes clearly superior (not the case as of this write-up).

Until then: **invest in content quality**, not another generator migration.
