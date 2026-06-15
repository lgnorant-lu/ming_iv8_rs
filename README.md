# iv8-rs

High-fidelity browser runtime Python extension, built on V8 + Rust.
For Web JS reverse engineering / anti-bot environment simulation.

## Features

- **Broad browser-like surface**: navigator / screen / window / document / location / history / performance
- **DOM**: html5ever parsing + ego-tree + selectors CSS Level 4 + EventTarget 3-phase dispatch
- **SubtleCrypto**: AES-GCM/CBC, RSA-OAEP/PSS, ECDSA/ECDH(P-256/P-384), HMAC, HKDF, PBKDF2
- **Canvas 2D**: tiny-skia real rendering + deterministic noise + fixed fingerprint fallback
- **WebGL**: 49 environment-configurable parameters + `__iv8__.gl.callLog`
- **Anti-detection**: wrapNative / hookNative / window.chrome / MarkAsUndetectable
- **CDP Inspector**: V8Inspector + WebSocket server + programmatic Python API
- **Observability (v0.3)**: trace mode / deterministic mode / VM instrumentation / recording / profiler / coverage / instrument_source / trace_diff
- **Environment Precision (v0.4)**: NavigatorUAData / Profile System / Diff Analysis / browser API stubs
- **Analysis (v0.5)**: StructuredTrace / CFG reconstruction / Taint Tracking / 4-layer Crypto Detection / VM handler diff / Module Isolation / CDP Scope API / Environment Probe / Quality Harness
- **Network**: ResourceBundle -> Python callback -> NetworkError (3-layer fallback)
- **Event loop**: logical / system dual time mode, advance / sleep / tick / drain
- **Entry Plane Solidification (v0.7)**: WebpackBridge (flavor detection, require/module table capture), Dispatch Generalization (zero-arg, multi-arg, switch loop, indirect handler map), SourceAst Pipeline (transform join points, transform report), and Corpus Runner CLI
- **Runtime Report Models (v0.8.0)**: schema-backed experimental report carriers for Environment Toolchain, Deobf Registry / Validation, String Array, VM Analysis / Handler, and IR Node reports
- **Environment Toolchain Runtime Foundation (v0.8.1)**: bounded `fingerprint.m1` probe runner, generic gap classification, reviewed `runtime_safe` candidate mapping, explicit safe rerun, profile suggestions, and no-write typed reports
- **Environment Custom Asset Foundation (v0.8.2)**: custom probe/candidate packs, schema and bypass-boundary validation, provenance diagnostics, and diagnostic-only `descriptor.m1` probes
- **Environment Iterative Adaptation (v0.8.3)**: explicit bounded `runtime_safe` adaptation with fresh-context reruns, stop reasons, regression/no-progress handling, and adaptation diagnostics
- **Environment Profile Coherence (v0.8.4)**: diagnostic-only profile coherence groups, local overlay boundary diagnostics, and generic family pressure taxonomy summaries
- **Environment Coherence Expansion (v0.8.5)**: diagnostic-only `ua_platform`, `network_info`, `timezone_locale`, and native-substrate review diagnostics
- **Environment Substrate Scaffolding (v0.8.6)**: report-only substrate coverage, scaffold gap, dry-run planning, rollback, and candidate metadata diagnostics with expanded negative gates
- **Environment Pressure Harness (v0.8.7)**: diagnostic-only pressure taxonomy, in-memory pressure manifest summaries, default-off toolchain pressure capture, and staged Environment Toolchain decomposition
- **Environment Toolchain Decomposition (v0.8.8-v0.8.10)**: diagnostic builder, boundary, asset model/loading, and candidate mapping modules split from the runtime while preserving public behavior
- **Environment Pressure-To-Plan Bridge (v0.8.11)**: diagnostic-only pressure plan summaries/items that connect explicit pressure harness output to review-only dry-run planning routes
- **Environment Mainline Audits And Governance (v0.8.12-v0.8.15)**: continuity audit, debt taxonomy, probe/bridge boundaries, external ecosystem reference, evidence boundary, bridge contract helpers, pressure route bridge context, stage map, governance closeout, and bridge vocabulary cleanup (deferred)
- **Environment Pressure-Aware Adaptation (v0.8.16)**: diagnostic-only pressure adaptation attempt model, candidate query bridge, and fresh-context attempt execution harness; single reusable module; no apply / no writes / no adapters
- **Native Substrate Mainline (v0.8.17-v0.8.31)**: Navigator/Screen FT migration, IDL toolchain, codegen + iv8-surface (1284 interfaces), P0/P1 deep stubs, infrastructure optimization, Feature Flag removal (v0.8.24), BCR upgrade (v0.8.25), GC fix (v0.8.26), Phase C + archive (v0.8.27), verification + BCR Step A (v0.8.28), BCR Step B + L2 MVP (v0.8.29), L3 100% BCR 15/15 dispatch hub (v0.8.30), use_old_chain retirement + L2 architecture foundation (v0.8.31). 255 lib + 81 Phase C + 1296 Python + 23/23 samples PASS.
- **L2 Profile-To-Environment Verification Foundation (v0.8.32)**: `iv8-profile` crate, `KernelConfig::with_profile_matrix()` certified runtime path, strict Python convergence checker, manifest/report schemas; 289 Rust lib + 1305 Python PASS.
- **L2 IDL Probe Automation + Report-Only MAPE-K (v0.8.33)**: IDL probe compiler (4 interfaces, 43 probes), L3 witness reports (BCR/BrowserSurface/undetectable), Python MAPE-K phases (Monitor/Analyze/Plan/Execute/Knowledge, report-only, dry-run); 1349 Python PASS.
- **L2 Convergence Event + Reproducible Snapshot (v0.8.34)**: `tools/convergence/` event/snapshot/delta/knowledge-index helpers, source report adapters, additive MAPE-K snapshot/delta integration; final strict audit pass; 1367 Python PASS.
- **L2 Probe Coverage Expansion M1 (v0.8.35)**: IDL probe compiler expands to 51 curated interfaces and 1,125 diagnostic probes with descriptor/prototype layers and coverage gap reporting; 1400 Python PASS.
- **L2 Data-Flow Connectivity M1 (v0.8.36)**: profile-aware probe expectations, audited constructor allowlist, witness-to-convergence routing, and expanded 105-vector coverage map; 1415 Python PASS.
- **L2 Data-Flow Depth M1 (v0.8.37)**: Navigator/NavigatorUAData supplementary IR probes and report-only Probe/Witness cross-source correlation with R2 hardening; 1427 Python PASS.
- **L2 Signal Completion M1 (v0.8.38)**: coverage map completion, in-memory profile auto-fill, and conservative constructor allowlist expansion; 1445 Python PASS.
Current `0.8.11` package release scope. v0.8.12-v0.8.38 are local milestones; v0.8.39 is planned; metadata remains `0.8.11`. v0.9 holding track blocked.

## Install

```bash
# From source (requires Rust toolchain + Python 3.13+)
git clone <repo>
cd iv8-rs
uv run maturin develop --release
```

## Quick Start

```python
import iv8_rs

# Basic eval
ctx = iv8_rs.JSContext()
print(ctx.eval("navigator.userAgent"))  # Mozilla/5.0 ...
ctx.close()

# Context manager
with iv8_rs.JSContext() as ctx:
    result = ctx.eval("1 + 1")  # 2

# Custom environment (fingerprint)
ctx = iv8_rs.JSContext(environment={
    "navigator.userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ...",
    "navigator.platform": "Win32",
    "screen.width": 1920,
    "screen.height": 1080,
})
```

## Core API

```python
ctx = iv8_rs.JSContext(
    environment=None,       # Browser fingerprint overrides
    config=None,            # Framework config (timezone, locale)
    time_mode="logical",    # "logical" (virtual clock) | "system" (real clock)
    js_api="__iv8__",       # Tool object name
    strict_compat=True,     # True = align with iv8 0.1.2 behavior
    random_seed=None,       # Deterministic Math.random (v0.3)
    crypto_seed=None,       # Deterministic crypto.getRandomValues (v0.3)
    time_freeze=None,       # Frozen Date.now() timestamp in ms (v0.3)
)

ctx.eval("1 + 1")                                    # Execute JS
ctx.page_load("<html>...</html>", base_url="...")     # Load HTML page
ctx.add_resource("https://...", body, status=200)     # Register offline resource
ctx.set_network_handler(handler)                      # Python network callback
ctx.expose("myFunc", lambda x: x * 2)                # Expose Python to JS
```

## Observability (v0.3)

```python
# Deterministic mode
ctx = iv8_rs.JSContext(random_seed=42, time_freeze=1700000000000)

# VM instrumentation (ChaosVM / JSVMP)
patched, info = iv8_rs.instrument_source(tdc_js)
ctx.eval(patched)
trace = ctx.get_unified_trace()  # D/R/C/W entries with PC

# Trace diff
diff = iv8_rs.trace_diff(trace_a, trace_b)

# CDP programmatic control
ctx.with_devtools(port=9229, wait=False)
bp = ctx.cdp_set_breakpoint("script.js", 100)
ctx.set_trace_point("script.js", 100, expression="JSON.stringify({pc:pc})")
```

## Documentation

- **Usage Guide**: [docs/GUIDE.md](docs/GUIDE.md) (comprehensive guide)
- **Progress**: [docs/PROGRESS.md](docs/PROGRESS.md)
- **v0.6 Baseline**: [docs/baseline/v0.6-dev-baseline.md](docs/baseline/v0.6-dev-baseline.md)
- **Architecture Philosophy**: [docs/design/IV8_RUST_ARCHITECTURE_PHILOSOPHY.md](docs/design/IV8_RUST_ARCHITECTURE_PHILOSOPHY.md)
- **Post-v0.6.0 Roadmap**: [docs/design/POST_V0.6_ROADMAP.md](docs/design/POST_V0.6_ROADMAP.md)
- **Roadmap Workspace**: [docs/roadmap/post-v0.6/](docs/roadmap/post-v0.6/)
- **v0.8 Runtime Report API Guide**: [docs/roadmap/post-v0.6/runtime-report-api-guide.md](docs/roadmap/post-v0.6/runtime-report-api-guide.md)
- **v0.8 Runtime API Acceptance**: [docs/acceptance/v0.8.0-runtime-api-acceptance.md](docs/acceptance/v0.8.0-runtime-api-acceptance.md)
- **v0.8.1 Environment Runtime Acceptance**: [docs/acceptance/v0.8.1-environment-runtime-acceptance.md](docs/acceptance/v0.8.1-environment-runtime-acceptance.md)
- **v0.8.2 Environment Runtime Acceptance**: [docs/acceptance/v0.8.2-environment-runtime-acceptance.md](docs/acceptance/v0.8.2-environment-runtime-acceptance.md)
- **v0.8.3 Environment Runtime Acceptance**: [docs/acceptance/v0.8.3-environment-runtime-acceptance.md](docs/acceptance/v0.8.3-environment-runtime-acceptance.md)
- **v0.8.4 Environment Runtime Acceptance**: [docs/acceptance/v0.8.4-environment-runtime-acceptance.md](docs/acceptance/v0.8.4-environment-runtime-acceptance.md)
- **v0.8.5 Environment Runtime Acceptance**: [docs/acceptance/v0.8.5-environment-runtime-acceptance.md](docs/acceptance/v0.8.5-environment-runtime-acceptance.md)
- **v0.8.6 Environment Runtime Acceptance**: [docs/acceptance/v0.8.6-environment-runtime-acceptance.md](docs/acceptance/v0.8.6-environment-runtime-acceptance.md)
- **v0.8.7 Environment Pressure Harness Acceptance**: [docs/acceptance/v0.8.7-environment-pressure-harness-acceptance.md](docs/acceptance/v0.8.7-environment-pressure-harness-acceptance.md)
- **v0.8.8 Environment Decomposition Acceptance**: [docs/acceptance/v0.8.8-environment-toolchain-decomposition-acceptance.md](docs/acceptance/v0.8.8-environment-toolchain-decomposition-acceptance.md)
- **v0.8.9 Environment Asset Boundary Acceptance**: [docs/acceptance/v0.8.9-environment-toolchain-asset-boundary-acceptance.md](docs/acceptance/v0.8.9-environment-toolchain-asset-boundary-acceptance.md)
- **v0.8.10 Environment Candidate Mapping Acceptance**: [docs/acceptance/v0.8.10-environment-toolchain-candidate-mapping-acceptance.md](docs/acceptance/v0.8.10-environment-toolchain-candidate-mapping-acceptance.md)
- **v0.8.11 Environment Pressure-To-Plan Acceptance**: [docs/acceptance/v0.8.11-environment-pressure-to-plan-bridge-acceptance.md](docs/acceptance/v0.8.11-environment-pressure-to-plan-bridge-acceptance.md)
- **v0.6 Stabilization Plan**: [docs/design/V0.6_STABILIZATION_PLAN.md](docs/design/V0.6_STABILIZATION_PLAN.md)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **TDC Testing Guide**: [docs/tdc-testing-guide.md](docs/tdc-testing-guide.md)
- **Quality Harness**: [docs/quality-harness/HARNESS-CHARTER.md](docs/quality-harness/HARNESS-CHARTER.md)
- **Research**: [docs/research/](docs/research/) (28 documents)

## Tests

```bash
# Current Python release gate:
uv run python -m pytest tests -q

# Rust tests
cargo test --workspace

# Rust release lint gate
cargo clippy --workspace --all-targets -- -D warnings

# Benchmark
cargo bench --bench iv8_bench
```

## Architecture

```
iv8-rs/
├── crates/
│   ├── iv8-core/     # Rust core (V8 + DOM + Crypto + Canvas + Network + Inspector)
│   ├── iv8-undetect/ # Anti-detection (wrapNative / hookNative / window.chrome)
│   └── iv8-py/       # PyO3 Python binding
├── python/iv8_rs/    # Python package (runtime APIs, analysis helpers, report models + type stubs)
├── tests/            # Python integration tests and v0.6 acceptance tests
└── docs/             # Design docs + research + quality-harness
```

## License

MIT
