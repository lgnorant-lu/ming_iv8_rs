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

Current `0.6.0` status is tracked in `docs/baseline/v0.6-dev-baseline.md`.
v0.6.0 is a stabilized Entry / Environment / Evidence baseline, not the final
deobfuscation expansion. Some strategies remain intentionally partial.

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
- **v0.6 Stabilization Plan**: [docs/design/V0.6_STABILIZATION_PLAN.md](docs/design/V0.6_STABILIZATION_PLAN.md)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **TDC Testing Guide**: [docs/tdc-testing-guide.md](docs/tdc-testing-guide.md)
- **Quality Harness**: [docs/quality-harness/HARNESS-CHARTER.md](docs/quality-harness/HARNESS-CHARTER.md)
- **Research**: [docs/research/](docs/research/) (28 documents)

## Tests

```bash
# Current v0.6.0 release gate is recorded in docs/baseline/.
uv run python -m pytest tests -q

# Rust tests
cargo test --workspace

# Release warning gate
cargo clippy --workspace -- -D warnings

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
├── python/iv8_rs/    # Python package (analysis/cfg/taint/patterns/probe/trace/vm_diff/isolation + type stubs)
├── tests/            # Python integration tests and v0.6 acceptance tests
└── docs/             # Design docs + research + quality-harness
```

## License

MIT
