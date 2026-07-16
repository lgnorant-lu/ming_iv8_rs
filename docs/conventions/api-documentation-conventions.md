# API Documentation Conventions (iv8-rs)

> Status: **accepted working standard** (2026-07-16)  
> Scope: public Python surface (`iv8_rs` / `_iv8`) and `docs/api/`  
> Supersedes informal claims in `docs/api/COVERAGE.md` where they conflict:  
> **this file defines metrics; COVERAGE.md only reports measurements against them.**

## 0. Honest admission (why this doc exists)

Earlier session claims such as “完整性 PASS 144/144” used an **operational definition
chosen by the author of that audit**:

| Claimed dimension | Actual measurement used | Is that an industry standard? |
|---|---|---|
| “完整 / 全面” | symbol name appears as substring under `docs/api/**` | **No** — only a **name inventory** check |
| “正确” | mostly not measured; some signatures cross-read from Rust | **No** as a full correctness score |
| “精确” | qualitative judgment | **No** — subjective unless defined |

That inventory check is **useful** and **reproducible**, but it is **not**
completeness/correctness/precision of API documentation in the sense of
Google API reference rules, numpydoc, or OpenAPI contract practice.

**Rule:** Never report PASS on a quality dimension without (1) a named metric
from this document, (2) a measurement command or procedure, (3) a numeric result.

---

## 1. Product type → documentation model

iv8-rs is a **native Python library** (PyO3 + large browser host), not an HTTP API.

| Model | Fit for iv8-rs | Notes |
|---|---|---|
| OpenAPI / Swagger | Poor as primary | No HTTP surface; keep only if we later expose HTTP |
| **Python reference (Google / numpydoc)** | **Primary** | Per-public-symbol: purpose, params, returns, raises, bounds |
| **Layered handbook** (`docs/api/` + GUIDE) | **Primary companion** | Contracts + tutorials; not one mega-file |
| Contract-first OpenAPI CI | Partial analogy | “Executable contract” idea applies; mechanism differs |
| Auto-generated from source only | Partial | `_iv8.pyi` + docstrings can generate; **hand contracts** still needed for host bounds |

**Industry anchors (references, not cargo-cult):**

1. [Google API reference comments](https://developers.google.com/style/api-reference-comments):  
   every public class/method/constant; **each parameter, return, exceptions**;
   short sample on major pages; present tense; defaults documented.
2. [numpydoc](https://numpydoc.readthedocs.io/en/stable/format.html) / Google Python Style:  
   Parameters / Returns / Raises / Notes / Examples; Examples strongly encouraged;
   doctest is for **illustration check**, not full unit-test replacement.
3. OpenAPI best practice (analogy): **single source of truth** + CI drift detection
   (for us: `__all__` / live `dir` / pyi / docs tiers stay consistent).
4. Project already: `docs/conventions/testing-conventions.md` —
   public API should have behavioral tests (separate from doc text).

---

## 2. Documentation layers (what goes where)

| Layer | Artifact | Responsibility | Must not |
|---|---|---|---|
| L-Product | `README.md` / `README.zh-CN.md` | Positioning, capabilities by domain, install, 1 minimal example, doc map | Version waterfall; full method tables |
| L-Contract | `docs/api/**` | Stable **calling contracts**: params, returns, raises, thread/stack/ICU, honesty bounds | Paste GUIDE novels; sample adapters |
| L-Tutorial | `docs/GUIDE.md` | Long narrative, evolution, multi-step recipes | Be the only place a public symbol is named |
| L-Machine | `python/iv8_rs/_iv8.pyi` (+ optional docstrings) | Types for IDE/typecheck; signatures SoT for **shape** | Alone claim semantic bounds |
| L-Change | `CHANGELOG.md` | Version deltas | Product capability map |
| L-Private | `docs/roadmap/**`, `docs/todo/**` | Planning / debt | Public keep set by default |

**Single source of truth (by concern):**

| Concern | SoT |
|---|---|
| Exists on public package | `iv8_rs.__all__` + live import |
| Method exists on instance | live `dir(JSContext())` / `dir(Debugger(...))` |
| Parameter names/types (shape) | `crates/iv8-py` signatures → `_iv8.pyi` |
| Semantic contract + bounds | `docs/api/**` (hand-maintained) |
| How-to story | GUIDE |

If pyi and Rust disagree, **Rust wins**; fix pyi.  
If docs and Rust disagree, **Rust wins**; fix docs.

---

## 3. Granularity: three contract tiers (mandatory classification)

Every public export is classified into exactly one tier. Tier is about
**required contract depth**, not importance of the feature.

### Tier C — Catalog (index)

**Who:** pure data carriers, schema version constants, `*_from_dict` / `*_to_dict`
helpers, enum-like constant sets, re-export aliases.

**Required:**

- Appears in a `docs/api/**` index table with **one-line role**
- Listed in `__all__` (if public)
- Type available via import / stubs where applicable

**Not required:** full param prose, examples, raises matrix.

### Tier B — Behavioral module API

**Who:** analysis helpers (`parse_trace`, `detect_patterns`, …), entry helpers,
environment plane runners, corpus helpers, most non-kernel callables.

**Required (per symbol or tight symbol group on one page):**

- Purpose (1–3 sentences)
- Signature (names + types or “see pyi”)
- Returns (shape summary)
- Raises (relevant interface exceptions only)
- Honesty bounds / non-goals if any
- Link to tests or GUIDE section if complex

**Encouraged:** minimal example (5–15 lines).

### Tier A — Kernel / host surface

**Who:** `JSContext`, `Debugger`, module-level `instrument_source`,
`prepare_entry` / `run_with_entry` / `plan_multi_entry`, `enable_logging`,
`load_profile`, exception types, `JSContext` ctor factory (`profile=`).

**Required (Google-reference level):**

- Full public method inventory (no silent omissions)
- For each method: purpose, parameters (incl. defaults), returns, raises
- Global host contracts: thread affinity, stack, ICU, time modes, network chain
- At least one **runnable** example per major group (eval, page_load, instrument, entry)
- Explicit non-goals (not full Chrome, offline chunks, diagnostic toolchain, …)

**Worker-internal methods** (e.g. `set_worker_prototype`): document as Tier A
**with** “internal / Worker path” label — still listed, not hidden.

---

## 4. Measurable quality dimensions (definitions)

These replace vague “完整/正确/精确/全面”.

### D1 — Inventory completeness (name-level)

| ID | Metric | Pass rule | Measurement |
|---|---|---|---|
| D1a | Export coverage | every `n in __all__` appears in `docs/api/**` with a role line (not bare string spam) | script: export vs docs index |
| D1b | JSContext method coverage | every live public method named on `runtime/jscontext.md` | live `dir` vs doc inventory section |
| D1c | Debugger method coverage | same for Debugger | live `dir` vs `debugger.md` |
| D1d | pyi coverage | every live public method has `def` in `_iv8.pyi` class block | live vs pyi parse |

**Current (2026-07-16, measured):** D1a/b/c/d **PASS** at name-level  
(see `docs/api/COVERAGE.md`).  
**This is only D1.**

### D2 — Signature fidelity (shape-level)

| ID | Metric | Pass rule | Measurement |
|---|---|---|---|
| D2a | pyi vs Rust | parameter names + defaults match `#[pyo3(signature)]` / `fn` | manual or generated diff; CI later |
| D2b | docs vs pyi (Tier A) | docs list same params as pyi for Tier A methods | checklist audit |
| D2c | factory extras | `profile=` documented as factory-only | docs + `__init__.py` |

**Pass rule for “signature correct”:** D2a + D2b for Tier A; D2a for Tier B if pyi exists.

**Current:** D2a **partially** fixed this pack (several pyi gaps closed);  
**not** fully machine-verified for all 50 methods.  
**Do not claim D2 PASS globally until a full matrix is stored.**

### D3 — Semantic contract depth (Tier-weighted)

| ID | Metric | Pass rule |
|---|---|---|
| D3a | Tier A methods meet §3 Tier A required fields | checklist per method: purpose/params/returns/raises/bounds |
| D3b | Tier B symbols meet §3 Tier B | checklist per symbol/group |
| D3c | Tier C symbols meet §3 Tier C | index + one-liner |

**Scoring (objective):**

```text
D3_score = weighted_fraction of symbols meeting their tier checklist
weight: Tier A = 3, Tier B = 2, Tier C = 1
```

**Current:** D3 **not fully measured**. Rough expert estimate only if labeled **estimate**:
Tier A ~ checklist partial (tables + inventory; many Raises incomplete);
Tier B mixed; Tier C largely OK after reports index fill.

**Forbidden:** reporting D3 as PASS without a filled checklist file.

### D4 — Example validity

| ID | Metric | Pass rule | Measurement |
|---|---|---|---|
| D4a | README quick start runs | exit 0 on supported env | pytest or script |
| D4b | Tier A group examples run | each marked example in docs/api Tier A pages | pytest `tests/test_api_doc_examples.py` (to add) |
| D4c | doctest optional | if used, CI runs doctest for those files | pytest --doctest-modules selective |

**Current:** D4 **FAIL / not automated** (no doc-example gate).

### D5 — Behavioral test coverage of public API (product quality, not doc text)

From `testing-conventions.md`: public API should have positive + error tests.  
This is **orthogonal** to docs: docs can be perfect while tests lag, and vice versa.

| ID | Metric | Measurement |
|---|---|---|
| D5a | Public callable has ≥1 test referencing it | inventory vs tests (rg / harness) |

**Current:** partial (many contract tests exist; not proven 100% of 144).

### D6 — Drift / single source of truth

| ID | Metric | Pass rule |
|---|---|---|
| D6a | No public symbol only in GUIDE | every public name also in docs/api or pyi+index |
| D6b | No docs-only fake API | every docs-claimed method exists live |
| D6c | Tier map exists | `docs/api/TIER_MAP.md` or section lists A/B/C |

**Current:** D6b strengthened after removing false `assert_thread` public claim;  
D6c **missing** until tier map is written.

---

## 5. What should we do? (priority for this product)

### Must do (product integrity)

| Priority | Work | Why |
|---|---|---|
| P0 | Keep D1 green in CI (inventory scripts) | Prevents silent API/doc drift |
| P0 | Finish D2 for **Tier A** (pyi = Rust, docs = pyi) | Callers depend on shapes |
| P0 | Tier A semantic fills: Raises + host bounds already partly done; complete Gaps | Tool users hit thread/ICU/network first |
| P1 | D4a + D4b: runnable examples for Tier A groups | Stops “docs lie” class failures |
| P1 | `TIER_MAP.md` classifying all 144 exports | Makes D3 measurable |
| P2 | Tier B checklist fill (analysis / entry / env) | Heavy use in reverse workflows |
| P3 | Tier C one-liners only (already mostly OK) | Avoid over-documenting DTOs |
| P3 | Full Sphinx/numpydoc site | Optional; not required if `docs/api/` is SoT |

### Should **not** do (by default)

| Anti-goal | Reason |
|---|---|
| Full Google-level prose for every of 144 symbols in one pass | Cost >> value; Tier C is index by design |
| OpenAPI for entire library | Wrong shape |
| Doctest as sole test suite | numpydoc: examples illustrate; tests/ owns correctness |
| Claiming “API docs complete” after D1 only | Misleading (this is the failure mode we hit) |
| Documenting private Rust helpers as public | False API surface |

### “每个符号完整参数/返回/异常/长契约/示例/自动化”

| Scope | Verdict |
|---|---|
| Every **Tier A** method | **Yes — should** |
| Every **Tier B** callable | **Yes — should** (can group siblings) |
| Every **Tier C** type + serde helper | **No** — catalog + link to schema version is enough |
| Automated run of all examples | **Yes for Tier A examples**; optional for B |
| Long narrative per symbol | **No** — put in GUIDE; contract stays scannable |

---

## 6. Production workflow (how to produce, not feel)

```text
1. Classify export → Tier A/B/C (TIER_MAP)
2. Shape SoT: Rust signature → update _iv8.pyi
3. Contract: update docs/api page for that tier checklist
4. If Tier A: add/adjust runnable example + test_api_doc_examples
5. Measure D1–D4 scripts; store numbers in COVERAGE.md
6. Only then claim the dimensions that passed
```

**PR gate (recommended):**

- [ ] D1 scripts green  
- [ ] Tier A changes update jscontext/module-level/instrumentation/entry  
- [ ] pyi updated if signature changed  
- [ ] No new public export without tier classification  

---

## 7. Language policy

| Artifact | Language |
|---|---|
| `README.md` | English |
| `README.zh-CN.md` | Chinese (capability-level parity with EN, not necessarily line-identical) |
| `docs/api/**` | English symbols + English or bilingual prose; **prefer English contracts** for public keep |
| Private roadmap | Chinese OK |

---

## 8. Relation to prior COVERAGE.md

| File | Role |
|---|---|
| **This convention** | Defines metrics, tiers, pass rules, workflow |
| `docs/api/COVERAGE.md` | Latest **measurements** only; must cite metric IDs (D1a, …) |

If COVERAGE says “PASS” without metric ID, treat as **invalid claim**.

---

## 9. Immediate next engineering tasks (ordered)

1. ~~Add `docs/api/TIER_MAP.md`~~ **DONE** (A=15 B=42 C=87).  
2. ~~`scripts/_api_doc_inventory.py` for D1a–D1d + D6c~~ **DONE**.  
3. ~~D2 matrix seed~~ **DONE** (`docs/api/D2_TIER_A_MATRIX.md`) — fill Y cells next.  
4. ~~`tests/test_api_doc_examples.py` (D4 smoke)~~ **DONE** (10 passed; large-stack).  
5. **NEXT:** Fill Raises/defaults gaps on Tier A pages; promote D2 rows P→Y.  
6. **NEXT:** D3 checklist file (per Tier A method required fields).  
7. Optional: wire inventory script into CI / quality harness.

---

## 10. One-sentence standard

**For iv8-rs: public API docs are tiered contracts with measurable inventory,
signature, and example gates — not a feeling, and not “name appears in markdown”.**
