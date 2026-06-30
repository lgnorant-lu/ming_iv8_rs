# Authoritative Data Sources

> Created: 2026-06-29
> Status: accepted
> Scope: All browser spec, Web IDL, fingerprinting, and V8 binding references
> Parent: `docs/conventions/README.md`

## Purpose

Define the authoritative data sources for IV8 development. Each source is
tiered by authority level and mapped to its usage context. This document
prevents ad-hoc web searches from substituting for normative references.

When in doubt: **spec first, implementation second, experience third.**

---

## Tier 1 — Normative Specifications

These define "what should happen." They are the highest authority for
correctness claims.

| Source | URL | Coverage | IV8 Usage |
|---|---|---|---|
| Web IDL | https://webidl.spec.whatwg.org/ | Interface object, attribute, operation, constant, constructor, stringifier, iterable, named constructor semantics | codegen spec basis; idlharness test basis; receiver check semantics (R3 audit) |
| HTML Living Standard | https://html.spec.whatwg.org/ | Window, Navigator, Document, Location, Storage, Worker, EventTarget, CustomElements, etc. | Window/Navigator/Document behavior; event handler semantics |
| DOM Standard | https://dom.spec.whatwg.org/ | Node, Element, Event, EventTarget, MutationObserver, Range, TreeWalker, etc. | DOM tree, event dispatch, Range, NodeIterator |
| ECMAScript (ECMA-262) | https://tc39.es/ecma262/ | Object, Function, Proxy, Symbol, Promise, Array, TypedArray, etc. | V8 internal behavior; descriptor shape; prototype chain; toString |
| W3C Fingerprinting Guidance | https://www.w3.org/TR/fingerprinting-guidance/ | Fingerprinting risk classification, mitigation patterns for spec authors | Detection surface taxonomy; understanding WHY APIs are fingerprint vectors |
| CSSOM View | https://drafts.csswg.org/cssom-view/ | Screen, Window scroll/inner, Element BCR, ResizeObserver | Screen properties; BCR; scroll behavior |
| Fetch Standard | https://fetch.spec.whatwg.org/ | Request, Response, Headers, fetch() | Network layer; fetch/XHR |
| URL Standard | https://url.spec.whatwg.org/ | URL parsing, URLSearchParams | Location; URL shim |
| Web Crypto API | https://w3c.github.io/webcrypto/ | SubtleCrypto, CryptoKey | Crypto module |

### Usage Rule

Tier 1 is the **first stop** for any "should X be writable/configurable/what
type" question. Codegen property attributes, descriptor shapes, and TypeError
conditions must trace to a Tier 1 spec clause.

---

## Tier 2 — Reference Implementations

These define "how Chrome actually does it." They are the highest authority for
implementation-detail questions not covered by spec.

| Source | URL | Coverage | IV8 Usage |
|---|---|---|---|
| Chromium Blink-V8 Bindings | https://chromium.googlesource.com/chromium/src/+/refs/heads/main/third_party/blink/renderer/bindings/ | V8 binding code generator, attribute/operation template, receiver check (GetCompatibleReceiver) | R3 receiver check audit; codegen design reference |
| V8 Binding Design | https://chromium.googlesource.com/chromium/src/+/master/third_party/blink/renderer/bindings/core/v8/V8BindingDesign.md | Context stack, exception handling, wrapper lifecycle, scope management | V8 callback architecture; scope/error handling |
| Blink IDL Extended Attributes | https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/renderer/bindings/IDLExtendedAttributes.md | [RuntimeEnabled], [Unforgeable], [LegacyUnforgeable], [Exposed], [Constructor], etc. | codegen ext_attr handling; [Exposed] filtering; [Constructor] detection |
| Chromium IDL Files | https://source.chromium.org/chromium/src/third_party/blink/renderer/ | Navigator.idl, Element.idl, etc. — the actual IDL Chrome uses | Cross-validation against webref IDL; [RuntimeEnabled] flag discovery |
| V8 Source (rusty-v8) | https://github.com/denoland/rusty-v8 | Rusty V8 API surface: FunctionTemplate, ObjectTemplate, PropertyCallbackInfo, etc. | API availability audit; codegen API constraints |
| V8 Source (C++) | https://chromium.googlesource.com/v8/v8/ | builtins-api.cc, properties.cc, objects.cc | Internal V8 behavior; GetCompatibleReceiver; ReadOnlyPrototype semantics |

### Usage Rule

Tier 2 is the **second stop** when Tier 1 is ambiguous or when implementation
details matter (e.g., V8 FunctionTemplate::inherit only sets
prototype.\_\_proto\_\_, not constructor.\_\_proto\_\_). Always cite the
specific file and function.

---

## Tier 3 — Test Suites

These are executable validation tools that check spec conformance.

| Source | URL | Coverage | IV8 Usage |
|---|---|---|---|
| WPT (Web Platform Tests) | https://github.com/web-platform-tests/wpt | Cross-browser test suite for all web specs | idlharness source; testharness.js source |
| idlharness.js | https://web-platform-tests.org/writing-tests/idlharness.html | WebIDL conformance: interface object, prototype, constants, attributes, operations, inheritance, stringifier, iterable | Primary idlharness runner (D-100); 10222 tests across 5 P0 interfaces |
| bcd-collector | https://github.com/foolip/bcd-collector | Browser API existence + feature detection across Chrome/Firefox/Safari | H04 S1 data source; P3 interface list generation; L0 existence baseline |
| @webref/idl | https://www.npmjs.com/package/@webref/idl | W3C/WHATWG IDL fragment extraction (authoritative, from spec sources) | codegen IR input; interface/attribute/operation enumeration |
| wptrunner | https://web-platform-tests.org/tools/wptrunner/docs/design.html | WPT official test runner framework, custom product plugin system | Architecture reference for IV8 WPT integration |
| wpt.fyi API | https://wpt.fyi/api/search | Chrome/Firefox/Safari WPT results archive | Chrome baseline comparison (9481/9640 Chrome 151.0.7921.0, 2026-06-30) |
| Node.js WPTRunner | https://github.com/nodejs/node/blob/main/test/wpt/README.md | Non-browser WPT reuse pattern (V8 isolate + eval) | IV8 WPT integration design reference (selected model) |
| Deno wpt.ts | https://github.com/denoland/deno/blob/main/tests/wpt/wpt.ts | Non-browser WPT reuse pattern (spawn binary + wpt serve) | Alternative considered; rejected (needs Python server) |
| Ladybird WPT.sh | https://github.com/LadybirdBrowser/ladybird/blob/master/Meta/WPT.sh | Non-mainstream engine WPT integration (WebDriver + import) | Alternative considered; rejected (needs browser binary) |
| crawlex.net runtime fingerprinting | https://blog.crawlex.net/blog/javascript-runtime-fingerprinting/ | Complete JS runtime probe taxonomy (stack, toString, enum, timing, CDP) | L14/L15/L16 layer justification; detection surface taxonomy |

### Usage Rule

Tier 3 provides **executable evidence**. A claim like "IV8's
Element.prototype.getAttribute has correct descriptor" must be backed by a
passing idlharness test, not just a spec citation.

---

## Tier 4 — Fingerprinting and Anti-Detection Research

These define "what detectors look for" and "what inconsistencies reveal
spoofing."

| Source | URL | Coverage | IV8 Usage |
|---|---|---|---|
| CreepJS | https://github.com/abrahamjuliot/creepjs | Lie detection: prototype tampering, toString inconsistency, function .name mismatch, getter/setter anomalies | H04 S5; L4-L7 validation rules; CreepJS lies = 0 target |
| FP-Inconsistent | https://arxiv.org/abs/2406.07647 | 390 cross-field inconsistency rules (UA<->platform<->CH, screen<->DPR, etc.) | H04 S6; L2 consistency rules; 206 applicable rules extracted |
| FP-Scanner | https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-vastel.pdf | Fingerprint inconsistency detection via attribute restoration | L2 cross-validation; complementary to FP-Inconsistent |
| Byte by Byte (2025) | https://dl.acm.org/doi/10.1145/3719027.3765158 | Function-level fingerprinting detection precision | Understanding granular detection beyond property level |
| Castle fingerprinting guide | https://blog.castle.io/roll-your-own-bot-detection-fingerprinting-javascript-part-1 | Bot detection probe classes, tamper detection | Detection surface taxonomy; probe classification |
| nullpt.rs RE browser | https://nullpt.rs/reverse-engineering-browser | Runtime hook detectability, native toString markers | L4 toString validation; hook detection avoidance |

### Usage Rule

Tier 4 defines **what to defend against**. Each CreepJS lie type and
FP-Inconsistent rule should map to an H04 matrix cell. IV8's target: 0/38
CreepJS lies, 0 FP-Inconsistent violations.

---

## Tier 5 — Network-Layer References

These cover detection surfaces outside JS (TLS, HTTP, IP). IV8 does not
implement these but must ensure JS-layer values are consistent with them.

| Source | URL | Coverage | IV8 Usage |
|---|---|---|---|
| JA3/JA4 TLS Fingerprinting | https://blog.cloudflare.com/ja4-signals/ | TLS ClientHello fingerprinting, cipher suite ordering | Documentation only; JS values must not contradict TLS claims |
| HTTP/2 Fingerprinting | https://fetch.spec.whatwg.org/ (H2 settings) | HTTP/2 SETTINGS frame, stream priorities | Documentation only; UA must match H2 behavior |
| W3C Fingerprinting Guidance | https://www.w3.org/TR/fingerprinting-guidance/ | Fingerprinting risk levels, mitigation patterns | Detection surface taxonomy; Cat 1-11 classification |

### Usage Rule

Tier 5 is **documentation only** — IV8 does not implement TLS or HTTP/2
layers. But JS-layer values (navigator.userAgent, Client Hints) must be
consistent with what the network layer claims. H04 L2 (value consistency)
documents this as out-of-scope but noted.

---

## Tier 6 — Practical References

These are non-normative but high-quality practical references.

| Source | URL | Coverage | IV8 Usage |
|---|---|---|---|
| MDN Web Docs | https://developer.mozilla.org/ | Cross-browser API behavior, browser differences, historical context | Quick lookup; browser-specific behavior; known quirks |
| Can I Use | https://caniuse.com/ | Feature availability across browsers/versions | Feature gating; [RuntimeEnabled] discovery |
| Chrome Platform Status | https://chromestatus.com/ | Chrome feature rollout, flag status, milestone tracking | [RuntimeEnabled] flag version mapping |
| webidl2.js | https://github.com/w3c/webidl2.js | WebIDL parser (used by idlharness) | Understanding idlharness internals; IR validation |

### Usage Rule

Tier 6 is for **quick lookups and cross-validation**. Never cite MDN as the
sole authority for a correctness claim — always trace back to Tier 1 spec.
MDN's value is in documenting browser differences and historical quirks that
specs don't cover.

---

## Usage Priority

```
Question arises →
  1. Check Tier 1 (spec defines it?) → cite spec clause
  2. Check Tier 2 (implementation defines it?) → cite source file:line
  3. Check Tier 3 (test suite validates it?) → cite passing test
  4. Check Tier 4 (detector probes it?) → cite lie type / rule
  5. Check Tier 6 (MDN documents it?) → use as cross-reference
  6. General web search → last resort, must be cross-validated
```

## Cross-Validation Principle (D-109)

Any verification conclusion must be based on >=2 independent sources from
different tiers. Ideal: Tier 1 (spec) + Tier 3 (test) + Tier 4 (detector).

Prohibited:
- Claiming "correct" based on single MDN page
- Claiming "undetectable" based on single CreepJS run
- Claiming "spec-compliant" without citing the spec clause

## Citation Format

In code comments, commit messages, and analysis docs:

```
// Per Web IDL §3.2.2: interface object .name is the interface name
// Verified: idlharness "interface object name" PASS
// Cross-ref: Blink IDL ExtendedAttributes.md [Constructor]
```

In H04 matrix cells:

```
L3[Navigator, userAgent, S1+S4] = PASS
  Spec: Web IDL §3.2.5 (attribute getter)
  Evidence: idlharness PASS + CDP sample match
  Sources: S1 (bcd-collector), S4 (CDP)
```

## Review Checklist

- [x] Tier 1 covers all normative specs relevant to IV8
- [x] Tier 2 covers Chromium/V8 implementation references
- [x] Tier 3 covers executable test suites
- [x] Tier 4 covers fingerprinting/anti-detection research
- [x] Tier 5 documents network-layer out-of-scope surfaces
- [x] Tier 6 covers practical quick-reference sources
- [x] Usage priority defines lookup order
- [x] Cross-validation principle requires >=2 sources
- [x] Citation format defined for code and docs
