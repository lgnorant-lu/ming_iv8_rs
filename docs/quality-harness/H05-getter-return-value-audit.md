# H05 — API Behavior Audit Harness (Coverage Layer L2)

> Charter document for the H05 quality harness.
> Created: 2026-07-12 (v0.8.89)
> Status: ACTIVE
> Source: docs/roadmap/v0.8/analysis/wpt-coverage-gap-analysis.md §5, §7.2
> TODO: docs/todo/TODO-native.md §H05 (lines 2413-2481)

## 1. Problem Statement

idlharness (H04/S8) verifies **shape conformance**: attribute exists on
the correct prototype, getter/setter presence, `instanceof` checks. It
does **not** verify **return value correctness** — the gap between
"type correct" and "value semantically correct".

**Example**: `navigator.connection.rtt` exists and is type `number`
(idlharness PASS), but IV8 returns `0` while Chrome returns `50`. This
discrepancy is invisible to idlharness but detectable by fingerprinting
libraries.

H05 fills this gap by systematically invoking every IDL-declared
attribute getter and comparing the return value against Chrome via CDP.

## 2. Scope

### 2.1 Sub-layers

| Sub-layer | Audit Target | Priority | Version |
|---|---|---|---|
| **H05a** | Getter return values (all IDL attributes) | P0 (基建) | v0.8.89 |
| **H05b** | Setter side effects (reflected attributes) | P1 (家具) | v0.8.90 |
| **H05c** | Method return values (key methods) | P1 (家具) | v0.8.90 |
| **H05d** | Constructor behavior (instance type, illegal constructor) | P2 (简装) | v0.8.91 |
| **H05e** | Exception type/message (error paths) | P2 (简装) | v0.8.91 |
| **H05f** | toString/toJSON/Symbol.toStringTag | P0 (基建) | v0.8.89 |

### 2.2 In Scope (v0.8.89)

- H05a: Getter return value audit (Phase 1: top 50 interfaces)
- H05f: toString/Symbol.toStringTag audit (full 1284 interfaces)
- Evaluator script architecture + Chrome CDP diff pipeline
- Status file format + initial baseline

### 2.3 Out of Scope (v0.8.89)

- H05b-e implementation (v0.8.90-91)
- H06 cross-context consistency (v0.8.90)
- WPT functional tests (v0.8.91+)
- Multi-browser consensus (beyond Chrome 151)

## 3. Architecture

### 3.1 Pipeline (5 steps)

```
unified_ir.json
       |
       v
[1. IDL Enumeration] — extract all attributes from 1284 interfaces
       |
       v
[2. Instance Creation] — create or acquire an instance of each interface
       |
       v
[3. Getter Invocation] — call getter, record value/typeof/exception
       |
       v
[4. Chrome CDP Diff] — same getter in Chrome via Runtime.evaluate
       |
       v
[5. Classification] — PASS / TYPE_FAIL / VALUE_FAIL / THROW / SKIP
```

### 3.2 IDL Enumeration (Step 1)

**Input**: `tools/idl/output/unified_ir.json`

**Extraction logic**:
```python
for definition in ir["definitions"]:
    if definition["kind"] != "interface":
        continue
    iface_name = definition["name"]
    for member in definition["members"]:
        if member["kind"] != "attribute":
            continue
        attr_name = member["name"]
        idl_type = member["idl_type"]  # e.g., "DOMString", "long", "boolean"
        readonly = member.get("readonly", False)
        nullable = "?" in idl_type
        yield (iface_name, attr_name, idl_type, readonly, nullable)
```

**Estimated attribute count**: ~5000+ across 1284 interfaces.

### 3.3 Instance Creation (Step 2)

Not all interfaces can be instantiated. The evaluator uses a tiered
strategy:

| Tier | Strategy | Examples | Count (est.) |
|---|---|---|---|
| A | Global singleton | `window`, `navigator`, `document`, `screen`, `history`, `location` | ~15 |
| B | `document.createElement(tag)` | `HTMLDivElement`, `HTMLSpanElement`, `HTMLInputElement`, ... | ~130 |
| C | Constructor `new Interface()` | `Headers`, `Request`, `Response`, `Blob`, `FormData`, `URL`, `URLSearchParams`, `AbortController`, `Event`, `CustomEvent`, `MouseEvent`, `MessageChannel`, `Worker`, `Map`, `Set`, `Promise`, `Date`, `RegExp` | ~50 |
| D | Factory method | `document.createEvent(...)`, `canvas.getContext('2d')`, `crypto.subtle`, `performance`, `new OfflineAudioContext()` | ~30 |
| E | Skip (no instance available) | Abstract interfaces, deprecated, experimental | ~1059 |

**Tier A-C are the primary coverage targets** (~195 interfaces).
Tier D is incremental. Tier E is SKIP.

### 3.4 Getter Invocation (Step 3)

**IV8 side**: Use `JSContext.eval("interface_name.attribute_name")` for
globals, or create instance then access attribute.

**Chrome side**: Use CDP `Runtime.evaluate` with `returnByValue: true`:
```json
{
  "method": "Runtime.evaluate",
  "params": {
    "expression": "navigator.userAgent",
    "returnByValue": true,
    "awaitPromise": false
  }
}
```

**Recorded data per attribute**:
```json
{
  "interface": "Navigator",
  "attribute": "userAgent",
  "idl_type": "DOMString",
  "iv8": {
    "value": "Mozilla/5.0 ...",
    "typeof": "string",
    "threw": false
  },
  "chrome": {
    "value": "Mozilla/5.0 ...",
    "typeof": "string",
    "threw": false
  }
}
```

### 3.5 Chrome CDP Diff (Step 4)

**CDP connection**: Reuse `WSClient` pattern from
`scripts/sample_surface_values.py`.

**Chrome launch**: `chrome --headless=new --remote-debugging-port=9222
--no-first-run --no-default-browser-check`

**Comparison rules**:

| IDL Type | Comparison Method |
|---|---|
| `DOMString`, `USVString` | Exact string match |
| `boolean` | Exact boolean match |
| `byte`, `octet`, `short`, `long`, `long long`, unsigned variants | Exact numeric match |
| `float`, `double` | Numeric match with tolerance (0.001) |
| `DOMString[]`, `FrozenArray<T>` | Array length + element-wise match |
| `object`, `any` | typeof match only (deep comparison impractical) |
| `Promise<T>` | Skip (async, not comparable at this layer) |
| `Function` | typeof === "function" |
| Interface types (e.g., `Document`, `Element`) | typeof === "object" + constructor.name match |
| `undefined` / `void` | typeof === "undefined" |
| Nullable (`T?`) | Both null or both non-null with inner type match |

### 3.6 Classification (Step 5)

| Classification | Condition | Action |
|---|---|---|
| **PASS** | typeof matches IDL type, value matches Chrome | None |
| **TYPE_FAIL** | typeof wrong (e.g., `object` for `DOMString`) | Bug — codegen type conversion |
| **VALUE_FAIL** | typeof correct but value differs from Chrome | Bug — shim/profile/config |
| **THROW** | Getter threw unexpected exception | Bug — missing implementation |
| **SKIP_NO_INSTANCE** | No instance available (Tier E) | Expected — document gap |
| **SKIP_CHROME_UNAVAILABLE** | Chrome CDP connection failed | Infrastructure — retry |
| **SKIP_ASYNC** | Attribute returns Promise | Expected — defer to H05c |

## 4. H05a Implementation Plan

### 4.1 Evaluator Script: `scripts/evaluate_h05_getter.py`

**Architecture** (estimated ~600 lines):

```
evaluate_h05_getter.py
├── IDLEnumerator        — parse unified_ir.json, yield (iface, attr, type)
├── InstanceFactory      — tiered instance creation (A-E)
├── IV8Probe             — JSContext getter invocation
├── ChromeCDPProbe       — CDP Runtime.evaluate getter invocation
├── ValueComparator      — type-aware comparison (§3.5 rules)
├── ReportBuilder        — JSON report + classification
└── main()               — CLI entry, single-command-single-exit-code
```

**Reuse from existing code**:
- `scripts/sample_surface_values.py` → `WSClient` class (CDP WebSocket)
- `scripts/descriptor_diff.py` → value comparison patterns
- `scripts/evaluate_surface_accuracy.py` → numeric tolerance, flatten logic
- `scripts/idl_coverage_audit.py` → unified_ir.json parsing

### 4.2 Output Format

**Primary**: `status/h05-getter-values.json`
```json
{
  "schema_version": "h05-getter-values.v0.1",
  "chrome_version": "151",
  "iv8_version": "0.8.89",
  "timestamp": "2026-07-12T...",
  "summary": {
    "total_attributes": 5234,
    "evaluated": 1856,
    "pass": 1723,
    "type_fail": 45,
    "value_fail": 62,
    "throw": 26,
    "skip": 378
  },
  "results": [
    {
      "interface": "Navigator",
      "attribute": "userAgent",
      "idl_type": "DOMString",
      "classification": "PASS",
      "iv8": { "value": "Mozilla/5.0 ...", "typeof": "string" },
      "chrome": { "value": "Mozilla/5.0 ...", "typeof": "string" }
    }
  ]
}
```

**Secondary**: Console summary + exit code (0 if no TYPE_FAIL/THROW, 1 otherwise).

### 4.3 Coverage Phases

| Phase | Scope | Target | When |
|---|---|---|---|
| Phase 1 | Top 50 interfaces (by WPT idlharness count) | ~800 attributes | v0.8.89 |
| Phase 2 | Top 200 interfaces | ~2000 attributes | v0.8.90 |
| Phase 3 | All 1284 interfaces | ~5000+ attributes | v0.8.91 |

### 4.4 Quality Gate

Per HARNESS-CHARTER §3:
- **Category A (Data Integrity)**: N/A (no mutation)
- **Category B (Recall)**: % of IDL attributes evaluated (target: 35% Phase 1)
- **Category C (False Positive)**: N/A (diagnostic, non-blocking)
- **Category D (Coverage)**: Interface coverage (target: 50/1284 Phase 1)
- **Category E (Robustness)**: Chrome disconnect handling (graceful SKIP)

**Gate binding**: Non-blocking (diagnostic-only) for v0.8.89. Promote to
blocking in v0.8.91 after Phase 2 baseline is established.

## 5. H05f: toString/Symbol.toStringTag Audit

### 5.1 Scope

For all 1284 interfaces:
1. `Object.prototype.toString.call(instance)` returns `"[object InterfaceName]"`
2. `Interface.prototype[Symbol.toStringTag]` === `"InterfaceName"` (writable=false, enumerable=false, configurable=true)
3. `Interface.prototype.toString()` does NOT return `"[object InterfaceNamePrototype]"`

### 5.2 Special Cases

- **[Global] interfaces**: toStringTag on prototype, not on global object
- **WindowProperties**: Not in webref IDL — manually created
- **Anonymous interfaces**: Skip (no interface object)
- **Partial interfaces**: Merge attributes from all partials

### 5.3 Implementation

H05f can reuse the H05a evaluator infrastructure with a different
invocation expression:
- H05a: `instance.attribute`
- H05f: `Object.prototype.toString.call(instance)` + `Interface.prototype[Symbol.toStringTag]`

The H05f audit does NOT require Chrome CDP — the expected values are
fully determined by the WebIDL spec (toStringTag = interface name). This
makes H05f a **spec-conformance check**, not a Chrome-diff check.

## 6. Relationship to Existing Harnesses

| Harness | Layer | Overlap | Distinction |
|---|---|---|---|
| H01 (Crypto Detection) | L1 | None | Different domain (crypto API) |
| H02 (Env Consistency) | L1 | Navigator/Screen values | H02 checks cross-field consistency, H05 checks Chrome diff |
| H03 (Surface Accuracy) | L1 | Chrome golden JSON | H03 uses curated golden, H05 is IDL-driven full enumeration |
| H04 (Surface Integrity) | L2 | idlharness parsing | H04 classifies idlharness FAILs, H05 finds FAILs idlharness can't |
| **H05** (this) | **L2** | **None** | **Value correctness (not shape)** |

H05 is complementary to H04. H04 finds shape conformance failures
(392 FAILs in current WPT run). H05 finds value conformance failures
that idlharness cannot detect (estimated 100+ issues).

## 7. Existing Infrastructure Reuse

| Asset | Location | Reuse |
|---|---|---|
| `WSClient` (CDP WebSocket) | `scripts/sample_surface_values.py:20-304` | Chrome CDP probe |
| `unified_ir.json` parser | `scripts/idl_coverage_audit.py` | IDL enumeration |
| Value comparison patterns | `scripts/descriptor_diff.py` | Type-aware comparison |
| Numeric tolerance | `scripts/evaluate_surface_accuracy.py` | float comparison |
| `COLLECTOR_JS` pattern | `scripts/sample_surface_values.py:48-109` | Getter invocation JS |
| Harness charter template | `docs/quality-harness/HARNESS-CHARTER.md` | Document structure |
| `evaluate_*.py` pattern | `scripts/evaluate_h04_*.py` | CLI + exit code |

## 8. Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Chrome not installed locally | Can't run CDP diff | Graceful SKIP_CHROME_UNAVAILABLE; fall back to spec-conformance where possible |
| Getter side effects | Some getters modify state | Run each attribute in fresh context; document side-effecting getters |
| Worker-only attributes | Can't access in Window context | Skip in Window mode; run separately in Worker mode (v0.8.90) |
| Chrome version drift | Values change between Chrome versions | Pin Chrome version in report; re-baseline quarterly |
| Instance creation failures | Some interfaces can't be instantiated | Tier E SKIP; document gap for future shim work |

## 9. Non-Goals

- H05b-e implementation (v0.8.90-91)
- H06 cross-context consistency (v0.8.90)
- WPT functional test execution (v0.8.91+)
- Multi-browser consensus (Chrome-only for v0.8.89)
- Performance benchmarking
- Fuzzing / property-based testing
- Visual rendering comparison

## 10. Success Criteria

- [ ] H05 charter document exists (this file)
- [ ] `scripts/evaluate_h05_getter.py` evaluator script exists
- [ ] Phase 1 (top 50 interfaces) initial run completed
- [ ] `status/h05-getter-values.json` baseline file committed
- [ ] H05f toString audit completed for all 1284 interfaces
- [ ] No regression in existing harnesses (H01-H04)
