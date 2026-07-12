# H06 — Cross-Context Consistency Audit Harness

> Charter document for the H06 quality harness.
> Created: 2026-07-12 (v0.8.90)
> Status: ACTIVE
> Source: HARNESS-CHARTER §7 registry

## 1. Problem Statement

Fingerprinting scripts compare property values across execution contexts
(main window vs iframe vs worker) to detect inconsistencies. If
`navigator.userAgent` in the main window differs from
`iframe.contentWindow.navigator.userAgent`, the environment is detected
as non-browser.

H06 verifies that key properties return **identical values** across
contexts.

## 2. Scope

### 2.1 Sub-layers

| Sub-layer | Audit Target | Priority | Version |
|---|---|---|---|
| **H06a** | Window vs iframe.contentWindow consistency | P0 | v0.8.90 |
| **H06b** | Window vs Worker consistency | P1 | v0.8.91+ |

### 2.2 In Scope (v0.8.91)

- H06a: 43 properties across Navigator/Screen/Window/Crypto/Misc
- H06b: Deferred (requires Worker support)

### 2.3 Out of Scope

- H06c ServiceWorker/ShadowRealm (v0.9+)
- Cross-origin iframe consistency (security model)

## 3. Gold Standard

**Source**: Browser spec behavior — per [HTML §7.1.4](
https://html.spec.whatwg.org/#creating-windows), nested browsing
contexts (iframes) share the same `navigator`, `screen`, and platform
properties as the parent. Values must be identical.

**Expected behavior**:
- `mainWindow.navigator.userAgent === iframe.contentWindow.navigator.userAgent`
- `mainWindow.screen.width === iframe.contentWindow.screen.width`
- `mainWindow.devicePixelRatio === iframe.contentWindow.devicePixelRatio`
- typeof must also match (e.g., both "string", not "string" vs "object")

## 4. Test Data Generation

Properties are derived from:
1. **WebIDL [Global] attributes** — all properties exposed on Window
2. **Navigator attributes** — from `Navigator` interface in IDL
3. **Screen attributes** — from `Screen` interface in IDL
4. **Window dimension properties** — innerWidth/innerHeight/etc.

Not hand-picked — all [Global] + Navigator + Screen attributes with
available values are tested.

## 5. Classification

| Classification | Condition |
|---|---|
| **PASS** | Same value AND same typeof in both contexts |
| **VALUE_DIFF** | typeof matches but values differ |
| **TYPE_DIFF** | typeof differs between contexts |
| **THROW** | Property access threw in one or both contexts |
| **SKIP** | Property not available in iframe |

## 6. Quality Gate (per HARNESS-CHARTER §3)

| Category | Metric | Threshold | Status |
|---|---|---|---|
| A (Data Integrity) | VALUE_DIFF + TYPE_DIFF + THROW | max 0 | **Mandatory** |
| C (False Positive) | Property consistency (same value) | 100% must PASS | **Mandatory** |
| D (Coverage) | % of key properties tested | ≥80% | Optional |

**Category C negative test**: properties that should be identical
must be identical. Any difference is a detection vector. There are no
"acceptable" differences — if a property differs, it's a bug.

## 7. Architecture

```
[1. Create iframe] — document.createElement('iframe')
        |
        v
[2. Enumerate properties] — Navigator/Screen/Window/Crypto from IDL
        |
        v
[3. Read main window] — eval property in window context
        |
        v
[4. Read iframe window] — eval property in iframe.contentWindow context
        |
        v
[5. Compare] — value + typeof match
```

## 8. Current Baseline

- v0.8.91: 43/43 PASS, 0 VALUE_DIFF, 0 TYPE_DIFF, 0 THROW, OVERALL PASS
- Evaluator: `scripts/evaluate_h06_window_iframe.py`
- Status file: `status/h06a-window-iframe.json`

## 9. Success Criteria

- [x] H06 charter document exists (this file)
- [x] `scripts/evaluate_h06_window_iframe.py` evaluator script exists
- [x] H06a initial run completed (43/43 PASS)
- [x] No regression in existing harnesses

## 10. Non-Goals

- H06b Worker consistency (v0.8.91+, depends on Worker support)
- H06c ServiceWorker/ShadowRealm (v0.9+)
- Cross-origin iframe (security model, not applicable)
