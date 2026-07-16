# iv8-rs

High-fidelity **browser-like JS runtime** for Python (V8 + Rust + PyO3).  
Target use: Web JS reverse engineering, controlled re-execution, anti-bot / fingerprint **host** simulation.

**Current**: milestone continuum through **v0.8.102** · package **0.8.12** (D-151 dual-track) — [CHANGELOG](CHANGELOG.md)  
中文版：[README.zh-CN.md](README.zh-CN.md) · API contracts：[docs/api/](docs/api/) · Coverage audit：[docs/api/COVERAGE.md](docs/api/COVERAGE.md)

## Why iv8-rs

| Approach | Gap |
|---|---|
| Pure Node / pure Python | Weak browser surface; `instanceof`, getters, workers, Intl often wrong |
| Full CDP browser only | Heavy, hard to instrument VMs offline, non-deterministic for CI |
| Thin stubs | Fail brand checks, canvas/WebGL/crypto fingerprints, DOM collections |

**iv8-rs** embeds V8 with a large native browser surface, offline ResourceBundle networking, deterministic seeds, ChaosVM/`instrument_source` path A, multi-bundler entry plane, and a **diagnostic** environment toolchain — one Python process, same-thread isolate, honest bounds (not full Chrome).

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
- WorkerNavigator / import static+dynamic paths; honest residual gaps documented in residual ledgers

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

**Stack:** prefer creating `JSContext` after `import iv8_rs` (module sets `threading.stack_size(128MB)`). Rust tests that build full kernels: `RUST_MIN_STACK=134217728`.

Optional: cargo/maturin `--target-dir` on a fast local cache path to avoid IDE contention.

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
| **[docs/api/](docs/api/)** | Stable API contracts (layered) |
| **[docs/GUIDE.md](docs/GUIDE.md)** | Long tutorials, evolution notes |
| **[CHANGELOG.md](CHANGELOG.md)** | Per-version deltas |
| **[docs/quality-harness/](docs/quality-harness/)** | Quality gate definitions |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | Commit / scope conventions |
| **[docs/PROGRESS.md](docs/PROGRESS.md)** | Internal progress (private-oriented) |

Do not treat acceptance/roadmap trees as public product API.

## Architecture

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

See CONTRIBUTING.md and AGENTS.md for commit format, stack size, and non-authorization rules (no public push / package bump without explicit request).

## License

MIT
