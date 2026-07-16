# iv8-rs

High-fidelity **browser-like JS runtime** for Python (V8 + Rust + PyO3).  
Built for Web JS reverse engineering, controlled re-execution, anti-bot / fingerprint **host** simulation, and ChaosVM-class instrumentation / analysis.

**Current**: milestone continuum through **v0.8.102** · package **0.8.12** (D-151 dual-track) — [CHANGELOG](CHANGELOG.md)  
**PyPI name (planned):** `ming_iv8_rs` · **import:** `import iv8_rs` (module name unchanged for now)  
中文版：[README.zh-CN.md](README.zh-CN.md) · API contracts：[docs/api/](docs/api/) · Coverage audit：[docs/api/COVERAGE.md](docs/api/COVERAGE.md)

## Origin: why iv8-rs exists

We first met **[iv8](https://github.com/jofpin/iv8)** (the PyPI `iv8` 0.1.x line): V8 inside Python, environment-driven browser surface — a shape that fits Web reverse work. We wanted **the same product category**, dug one layer deeper:

- **Sturdier host surface** — not only “can eval”, but brand checks, getters, Workers, Intl, DOM collections under high-signal probes;
- **Reproducible runs** — offline ResourceBundle, deterministic seeds, logical clocks for CI and dual-engine compare;
- **Execution + observation together** — run the script in a host while `instrument_source` / CDP / unified traces open ChaosVM / TDC-style paths. A bit “left foot on right foot”, but the intent is real: **runtime host + analysis plane** on one stack.

Product intuition borrows from iv8 (Python-friendly, injectable environment, offline-first). The kernel choice is **Rust + PyO3 + large codegen/native browser surface** for performance, type boundaries, and long-term maintenance.  
**No need to dunk on peers.** This repo and PyPI `iv8` 0.1.x are **related lineage / dual-engine oracle**. The product name here is **iv8-rs** — not “replace X”, but “same spark, deeper dig”.

## Design philosophy: two branches, one loop

iv8-rs is not “only a browser stub” and not “only a decompiler”. The core bet is a **dual-branch product**:

| Branch | Name | Question it answers |
|---|---|---|
| **A** | **Runtime host** | Can this script **run** under a controlled, browser-like host? |
| **B** | **Runtime analysis** | Can we **observe, structure, and reason about** what that run did? |

They are meant to **feed each other**: a better host produces cleaner traces and fewer Illegal-invocation lies; better analysis points back at missing host surface, wrong brands, or incomplete network. Informally: **left foot on right foot** — host fidelity and analysis depth climb together, not as two disconnected tools.

What ships today is a **strong Branch A** plus a **usable Branch B spine** (instrumentation, unified trace, entry plane, diagnostics). Deeper IR / SSA / full decompile pipelines are an **explicit desired state** on Branch B — not vaporware branding, not “everything unfinished,” but a north-star map of where analysis is going while Branch A remains the production weight.

### Branch A — runtime environment (host)

The script must believe it lives in something like a page: same-thread V8 isolate, large browser surface, offline-first I/O, deterministic knobs for CI.

| Layer | Includes (illustrative, not exhaustive) |
|---|---|
| **Kernel** | V8 isolate, stack/thread affinity, ICU/Intl + timezone redetect, logical vs system time, seeds |
| **Browser surface** | Window / Navigator / Screen / Location / DOM parse & query, events, collections, Workers |
| **Media & crypto** | Canvas, WebGL params, Audio paths, SubtleCrypto |
| **Network** | ResourceBundle → optional Python handler → error (no silent open crawl) |
| **Identity** | Profiles, flat environment overrides, storage, cookies/headers helpers |
| **Anti-detect building blocks** | wrap/hook natives, chrome object, toString / brand hygiene (fidelity, not “pass all detectors”) |
| **Product honesty** | Explicit non-goals: not full Chromium, not layout luxury, not one-click bypass |

Branch A is what makes **controlled re-execution** possible: same inputs, same seeds, offline chunks, dual-engine compare.

### Branch B — runtime analysis (observe & structure)

Once the host can run the payload, Branch B turns the run into **evidence**: traces, diffs, entry plans, diagnostic reports — and, as the desired state deepens, richer intermediate forms.

| Layer | Today (shipping spine) | Desired state (north star, not a claim of full completion) |
|---|---|---|
| **Instrumentation** | `instrument_source` / ChaosVM path A, global `instrument_chaosvm`, env Proxies | Broader VM families, lower false Illegal-invocation, optional expose_handlers policies |
| **Trace plane** | Unified D/R/C/W lines, VM trace, `trace_diff`, recording / profiler / coverage | Stable schemas, streaming, multi-run corpora as first-class |
| **Debugger / CDP** | DevTools attach, breakpoints, step, `Debugger` helpers | Deeper scope/object UX, CI-friendly programmatic sessions |
| **Entry / bundlers** | `prepare_entry` / `run_with_entry` / multi-entry plans; caller-owned chunks | Richer bundler graphs without silent network fetch |
| **Structure & reasoning** | CFG / taint / pattern / crypto detectors, report models, environment plane (diagnostic) | **IR / SSA-style** views, opcode/handler IR, string-array & deobf reports as a coherent pipeline, cross-version VM diff as default workflow |
| **Formats & adapters** | Schema-versioned reports, corpus manifests | More intermediate formats, dual-oracle packs, export for external RE tools |

Branch B must stay **honest**: default environment toolchain is **report-only**; analysis is not a “site bypass kit.” Deeper IR/SSA language in the table is **design intent** — pieces exist or are scaffolded; the **full pipeline** remains the climb, not a checkbox of “already done.”

### How A and B reinforce each other

```text
        +------------------+
        |  JS under test   |
        +--------+---------+
                 |
        +--------v---------+
        |  Branch A: host  |  run / freeze / offline / brand
        +--------+---------+
                 |  traces, events, gaps
        +--------v---------+
        |  Branch B: analyze |  instrument, CFG, plan, report
        +--------+---------+
                 |  missing surface, wrong values, entry fixes
                 +-----> back into Branch A (profiles, shims, network)
```

1. **A → B:** Without a faithful host, traces are noise and Illegal invocation masks real logic.  
2. **B → A:** Diffs and probes show which getters, plugins, or network edges still lie.  
3. **Loop:** Each milestone should either harden the host, deepen observation, or tighten the contract between them.

That is the product thesis: **one Python process, two complementary branches**, not a pile of unrelated scripts.

## Why this technical path

| Approach | Common gap |
|---|---|
| Pure Node / pure Python | Thin browser surface; `instanceof`, getters, workers, Intl often wrong |
| Full CDP browser only | Heavy, hard to instrument VMs offline, non-deterministic for CI |
| Thin stubs | Fail brand checks, canvas/WebGL/crypto fingerprints, DOM collections |

**iv8-rs** embeds V8 with a large native browser surface, offline ResourceBundle networking, deterministic seeds, ChaosVM / `instrument_source` path A, multi-bundler entry plane, and a **diagnostic** environment toolchain — one Python process, same-thread isolate, honest bounds (not full Chrome).

## Capabilities

Organized by **domain**, not version waterfall. Version deltas live in [CHANGELOG](CHANGELOG.md). Contracts: [docs/api/](docs/api/).

### Runtime host

- `JSContext` factory with `environment` / `profile` / defaults merge; context manager; `close` / dispose
- Same-thread isolate affinity; 128MB Python thread stack at import (mixin-scale templates)
- `time_mode` `logical` | `system`; `random_seed` / `crypto_seed` / `time_freeze`
- `config`: `timezone`, `locale`, `storage_path`; TZ → process `TZ` + V8 Redetect (ICU 77 data)
- Dual-track versioning: milestone tags vs package number ([docs/api/versioning.md](docs/api/versioning.md))

### Browser surface & DOM

- Window / Navigator / Screen / Location / History / Performance / document APIs (codegen + native)
- html5ever parse, ego-tree, CSS Level 4 selectors, EventTarget 3-phase dispatch
- `page_load` / `page_load_with_headers`; NodeList iterable; `getElementsByTagName('*')`
- Collections / plugins / HTMLAll / Options structure; Worker + WorkerNavigator paths
- Profile-driven identity (Chrome-line default profile; flat dot-path environment)

### Crypto / Canvas / WebGL / Audio

- SubtleCrypto: AES-GCM/CBC/CTR, RSA-OAEP/PSS, ECDSA/ECDH, HMAC, HKDF, PBKDF2, X25519/AES-KW paths
- Canvas 2D (tiny-skia + deterministic noise / fixed fingerprint modes)
- WebGL parameter surface + environment UNMASKED_* / call log hooks
- AudioContext / OfflineAudio completion paths; font metrics from profile where wired

### Network & event loop

- 3-layer network: ResourceBundle → Python `set_network_handler` → error (no silent open-web crawl)
- `add_resource` offline bodies; XHR / fetch / WebSocket lifecycle surfaces
- Logical vs system timers; advance / sleep / tick / drain patterns (see GUIDE)
- Cookie / Headers / storage persist-load helpers on context

### Anti-detection primitives

- wrapNative / hookNative / `window.chrome` / MarkAsUndetectable paths
- Function toString / toStringTag / prototype brand hygiene (ongoing fidelity work)
- High-signal Illegal invocation fixes on brand-sensitive APIs
- Not a “pass every detector” product guarantee — host fidelity + explicit bounds

### Instrumentation & observability

- Module `instrument_source` (ChaosVM path A, closure-scoped handlers, e.g. TDC)
- Instance `instrument_chaosvm` for **global** handler tables only
- Unified / VM traces; `trace_diff`; trace points; recording / profiler / coverage
- CDP Inspector: `with_devtools`, breakpoints, step, scope, programmatic API
- `Debugger` class: `trace_api`, `watch_property`, `eval_traced`, snapshots

### Entry / multi-bundler

- `prepare_entry` / `run_with_entry` / `plan_multi_entry`
- Webpack / Parcel / Browserify / Vite-adjacent bridges; chunk supply is **caller-owned**
- Corpus runner CLI helpers for offline multi-case runs

### Environment toolchain (diagnostic plane)

- Probe / gap / candidate / pressure / MAPE-K-style reports
- **Report-only / no automatic apply / no silent profile write** by default
- Not a one-click site bypass kit; sample adapters are separate and non-API

### Workers

- Dedicated isolate + OS thread + structured clone (方案 A)
- WorkerNavigator / import static+dynamic paths; residual gaps in residual ledgers

## Non-goals / honest bounds

| Not claimed | Reality |
|---|---|
| Full Chromium / Blink | Large IDL + intentional stubs; parity is continuous work |
| Auto-fetch all bundler chunks | Offline-first; you supply chunk text |
| Silent live network product | ResourceBundle-first |
| Environment toolchain auto-fixes hosts | Diagnostic reports only unless you authorize mutation elsewhere |
| Identity with PyPI package `iv8` 0.1.x | Related lineage / dual-engine oracle; **this product is iv8-rs** |
| Full layout engine / deep fingerprint luxury | Deferred (v0.9+ / residual) |

Global bounds: [docs/api/overview.md](docs/api/overview.md).

## Install

Requires **Rust toolchain**, **Python 3.13+**, and ICU **77** data (`icudtl.dat` ships with the package; override with `IV8_ICUDTL_PATH`).

```bash
git clone <repo>
cd iv8-rs

# Local development (fast iteration)
uv run maturin develop --target-dir target-maturin --strip --profile dev

# Distribution build (LTO, slow)
uv run maturin develop --release
```

**Stack:** create `JSContext` after `import iv8_rs` (module sets `threading.stack_size(128MB)`). Full-kernel Rust tests: `RUST_MIN_STACK=134217728`.

Optional: cargo/maturin `--target-dir` on a fast local cache path to avoid IDE contention on `target/`.

## Quick start

```python
import iv8_rs

with iv8_rs.JSContext() as ctx:
    print(ctx.eval("navigator.userAgent"))

# Profile + overrides (environment wins over profile)
ctx = iv8_rs.JSContext(
    profile="default",
    environment={
        "timezone": "Asia/Shanghai",
        "navigator.userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ...",
        "screen.width": 1920,
        "screen.height": 1080,
    },
    time_mode="system",
    config={"timezone": "Asia/Shanghai"},
)
print(ctx.eval("Intl.DateTimeFormat().resolvedOptions().timeZone"))
ctx.close()

# Offline resource + eval
ctx = iv8_rs.JSContext()
ctx.add_resource("https://example.com/app.js", "window.__ok = 1", status=200)
ctx.page_load("<html><body></body></html>", base_url="https://example.com/")
# ChaosVM / TDC-style: prefer instrument_source, then eval patched source
# patched, info = iv8_rs.instrument_source(source)
# ctx.eval(patched)
ctx.close()
```

Full method surface: [docs/api/runtime/jscontext.md](docs/api/runtime/jscontext.md).  
Instrumentation: [docs/api/instrumentation/](docs/api/instrumentation/).

## Documentation map

| Doc | Role |
|---|---|
| **[docs/api/](docs/api/)** | Stable API contracts (layered; public-oriented) |
| **[docs/GUIDE.public.md](docs/GUIDE.public.md)** | Public tutorial cut (§1–16); full GUIDE is private-oriented |
| **[CHANGELOG.md](CHANGELOG.md)** | Per-version deltas |
| **[docs/quality-harness/](docs/quality-harness/)** | Quality gate definitions |
| **[docs/conventions/](docs/conventions/)** | Naming / testing / docs / docstring standards |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | Commit / scope conventions |
| **[docs/PROGRESS.md](docs/PROGRESS.md)** | Internal progress (private-oriented) |

Do not treat acceptance/roadmap trees as public product API.

## Architecture (bird's-eye)

```text
Python (iv8_rs)
    │  PyO3
    ▼
iv8-py  ──►  iv8-core (V8 isolate, DOM, crypto, canvas, network, inspector)
                │
                ├── iv8-undetect (wrap/hook/chrome primitives)
                ├── iv8-surface / codegen (IDL templates)
                └── iv8-profile (profile matrix helpers)
```

```text
iv8-rs/
├── crates/          # Rust workspace (core, py, undetect, surface, …)
├── python/iv8_rs/   # Package surface, profiles, analysis, toolchain
├── tests/           # Python integration
└── docs/            # GUIDE, api/, quality-harness, roadmap (mixed public/private)
```

## Development

```bash
# Rust
cargo test --workspace
cargo test -p iv8-core --lib
cargo clippy --workspace --all-targets -- -D warnings

# Python release-style gate
uv run python -m pytest tests -q
```

See CONTRIBUTING.md and in-repo agent notes for commit format, stack size, and non-authorization rules (no public push / package bump without explicit request).

## Acknowledgments

- **[iv8](https://github.com/jofpin/iv8)** — major inspiration and dual-engine reference lineage for Python↔V8 hosting  
- Upstream: V8, PyO3, maturin, html5ever, and the wider ecosystem

## Disclaimer and terms of use (read carefully)

This software is provided for **research, education, interoperability testing,
debugging, and legitimate software engineering** only.

**No warranty.** The software is provided **AS IS**, without warranty of any
kind, express or implied, including but not limited to merchantability, fitness
for a particular purpose, non-infringement, accuracy of browser emulation, or
undetectability. See the [Apache License 2.0](LICENSE).

**You are solely responsible** for how you use this software and for compliance
with all applicable laws, regulations, website/service terms of use, and third-
party rights in your jurisdiction.

**Prohibited / out of scope (non-exhaustive):**

- Unauthorized access, fraud, credential stuffing, account takeover
- Bypassing security, anti-bot, CAPTCHA, or access controls **without permission**
- Targeting production systems or users in ways that violate law or terms of service
- Distributing malware, phishing kits, or “one-click site bypass” packs built on this engine
- Misrepresenting this project as a full Chromium browser or as guaranteed anti-detect

**Not a bypass product.** Environment tooling is **diagnostic / report-oriented**
by design. Sample-specific reverse notes and site overlays are **not** product
API and are not provided as attack recipes.

**Fingerprint / anti-bot.** Any anti-detection primitives are **host-fidelity
building blocks**, not a promise to pass every detector or to evade any named
vendor.

**Dual-engine / lineage.** Mentions of related packages (e.g. PyPI `iv8` 0.1.x)
are for technical comparison only. This product is **iv8-rs** / **ming_iv8_rs**.

**Export and dual-repo.** Public trees are path-filtered views of a private
development history. Commit messages may still reflect development narrative;
do not treat the public git log as a complete private process record.

**Indemnity.** To the maximum extent permitted by law, authors and contributors
shall not be liable for any claim, damages, or other liability arising from use
or inability to use the software, including legal costs arising from your misuse.

By cloning, installing, or using this software, you acknowledge this disclaimer.

## License

[Apache License 2.0](LICENSE)
