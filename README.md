# iv8-rs

High-fidelity browser runtime Python extension, built on V8 + Rust.
For Web JS reverse engineering / anti-bot environment simulation.

**Current**: v0.8.75 (M7 gate audit, v0.9 OPEN) — [Progress](docs/PROGRESS.md)

## Features

- **Broad browser-like surface**: navigator / screen / window / document / location / history / performance
- **BrowserProfile**: 40-field centralized browser identity struct with runtime injection (v0.8.55/0.8.57)
- **Layer5 Window/Screen/DPR consistency**: native `global_template` accessors for window dimensions and `devicePixelRatio`, profile/env fallback, and Chrome-compatible descriptors (v0.8.65)
- **Codegen stability and bounded surface audit**: `iv8-surface-codegen` golden snapshots, 1284/1284 current IR templates generated+installed, Image/Option/Audio named constructor aliases, and explicit non-claim boundary for Chrome/Web Platform parity (v0.8.67)
- **Bundler refinement (M5)**: Parcel detection + bridge (`$parcel$`/`parcelRequire`); Vite ESM G5-G8 minimal support (import.meta, dynamic import, TLA); Browserify bridge quality fixes (string-key deps, span fallback) (v0.8.68)
- **Infrastructure convergence**: CI Rust toolchain pinned to 1.96.0; env_inject skips 26 direct keys already covered by native accessors (v0.8.69)
- **Browser surface coverage baseline**: curated priority probe matrix (30 tests, 9 dimensions); tiered T1-T5 coverage statistics; timeOrigin fix (no longer hardcoded 0) (v0.8.71)
- **Stateful runtime substrate**: localStorage cross-kernel persistence + Drop flush; cookie security attribute parsing/filtering (Path prefix-match, Secure context); Headers comma-join duplicates + constructor array init; structured ProbeRecord JSON schema with collect_probe_records (v0.8.72)
- **Debug / automation / geometry closure**: inspector step_out wiring (kernel cdp_step_out + Python binding); Python inventory auto-update generator; layout/geometry basic model (Rust callback reads this.__iv8Rect__; __iv8SetElementRect fixture hook); bridge vocabulary glossary (16 terms) (v0.8.73)
- **Substrate debt sweep**: codegen warning cleanup (67→4 + heck removal); ledger sweep (VERSION_SCOPE_MAP mid-range, CAPABILITY_INDEX v0.8.45-47, stale decision register entries); metadata policy decision (keep 0.8.11 frozen); crypto/env_inject audit (paths confirmed unified/complete); CI/config hygiene (rust-toolchain 1.96.0, .gitignore cleanup) (v0.8.74)
- **M7 gate audit**: TODO recount (grep-verified 316 items); detection surface reclassification (11+2 categories, ~92% avg); residual risk register (21 items R01-R21); v0.9 entry gate OPEN with conditions (v0.8 精装≈100%; ~8 v0.8.76 patch items identified) (v0.8.75)
- **Navigator consistency + WorkerNavigator**: 10 new integration tests for Navigator cross-property coherence and WorkerNavigator runtime shape; WorkerNavigator illegal_constructor fix (v0.8.70)
- **Generated Navigator skeletons**: 46 IDL properties via codegen + native template unification (v0.8.58/0.8.60)
- **Native Navigator stubs**: connection, getBattery, sendBeacon, geolocation, clipboard, credentials (v0.8.55); getGamepads, requestMediaKeySystemAccess, requestMIDIAccess (v0.8.61)
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
- **L2 Analyze Depth M1 (v0.8.39)**: enriched MAPE-K Analyze/Plan with gap taxonomy, severity weighting, and cross-source correlation consumption; 1454 Python PASS.
- **Diagnostic-to-Substrate Bridge M1 (v0.8.40)**: repair ticket schema, knowledge-to-ticket projection, L3 owner routing table, and evidence referencing; 1464 Python PASS.
- **Diagnostic-to-Substrate Bridge M2 (v0.8.41)**: before/after delta contract and repair candidate ledger; 1469 Python PASS.
- **Runtime Repair Harness M1 (v0.8.42)**: repair brief, evidence bundle manifest, validation plan, and readiness classification; 1481 Python PASS.
- **L3 P0 Navigator/Profile Runtime Batch M1 (v0.8.43)**: first evidence-driven Rust runtime mutation; Navigator value projection from v0.8.42 repair briefs with before/after delta validation; 1490 Python PASS.
Current `0.8.11` package release scope. v0.8.12-v0.8.73 are local milestones; metadata remains `0.8.11`. v0.9 holding track blocked pending M7 gate audit.

## Install

```bash
# From source (requires Rust toolchain + Python 3.13+)
git clone <repo>
cd iv8-rs

# Local development (fast: ~10s per file change)
uv run maturin develop --target-dir target-maturin --strip --profile dev

# Distribution build (slow: LTO optimization, 5-10 min)
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
- **Conventions**: [docs/conventions/](docs/conventions/) (execution protocol, naming, commit, testing, harness charter)
- **Progress**: [docs/PROGRESS.md](docs/PROGRESS.md)
- **v0.6 Baseline**: [docs/baseline/v0.6-dev-baseline.md](docs/baseline/v0.6-dev-baseline.md)
- **Architecture Philosophy**: [docs/_legacy/early-design/IV8_RUST_ARCHITECTURE_PHILOSOPHY.md](docs/_legacy/early-design/IV8_RUST_ARCHITECTURE_PHILOSOPHY.md)
- **Post-v0.6.0 Roadmap**: [docs/_legacy/early-design/POST_V0.6_ROADMAP.md](docs/_legacy/early-design/POST_V0.6_ROADMAP.md)
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
- **v0.6 Stabilization Plan**: [docs/_legacy/early-design/V0.6_STABILIZATION_PLAN.md](docs/_legacy/early-design/V0.6_STABILIZATION_PLAN.md)
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
├── tests/            # Python integration tests, environment_toolchain/, helpers/
└── docs/             # Design docs + research + quality-harness
```

## License

MIT
