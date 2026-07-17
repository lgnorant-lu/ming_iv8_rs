# pkg-0.8.12 — first public wheels (draft)

> Package track **0.8.12** · Product **iv8-rs** · PyPI **`ming_iv8_rs`** · import **`iv8_rs`**  
> Continuum context: through **v0.8.102** (D-151 dual-track; package number != milestone tag)  
> Status: **release candidate notes** (fill publish date / URLs after upload)

## What this is

First **public installable binary** release of the package track:

- High-fidelity browser-like JS host (V8 + Rust/PyO3)
- Instrumentation spine (`instrument_source`, unified trace)
- Dual-engine oracle posture vs reference iv8 0.1.x (related lineage, not identity)

This is **not** product 1.0: fingerprint thickness, full Chromium layout, and some site shells remain bounded.

## Install

```bash
pip install ming_iv8_rs==0.8.12
python -c "import iv8_rs; c=iv8_rs.JSContext(); print(c.eval('1+1')); c.close()"
```

Requires **Python 3.13+**. ICU 77 data ships in the wheel (`icudtl.dat`).

## Wheels

| Platform | Typical tag | Notes |
|---|---|---|
| Windows x86_64 | `win_amd64` | Primary desktop |
| Linux x86_64 | `manylinux_2_34_x86_64` | V8 **150.2** crates.io prebuilt (cdylib TLS-safe) |
| macOS Apple Silicon | `macosx_*_arm64` | arm64 only |

**Not provided:** macOS Intel x86_64, Linux aarch64 (source build).

## Engineering notes (0.8.12 packaging)

- Linux: denoland rusty_v8 **#1706** / PR **#1911** — use V8 **150+** prebuilt for shared-library-safe TLS; 147.4 TPOFF32 on `.so` link
- Default CI matrix: no `macos-13` (GHA Intel Mac queue stalls whole runs)
- Build: public `Build Wheels` / `build-wheels.yml`; PyPI via OIDC env **`pypi`**

## Verification (pre-publish)

| Gate | Result |
|---|---|
| Public Build Wheels (win / linux prebuilt / mac arm) | PASS (`29578126623`) |
| Windows clean-venv L0–L2 (d1 / TDC / ctrip load) | PASS |
| Intl + TZ smoke on wheel | PASS |
| `cargo test -p iv8-core --lib` | 520 substantive; parallel isolate serial flake known |

## Tag policy

| Tag | Role |
|---|---|
| **`pkg-0.8.12`** | Package binary / GH Release (this document) |
| `v0.8.100`–`v0.8.102` | Continuum milestones (capability closeouts) |
| historical `v0.8.12` | Early milestone pointer — **do not move / reinterpret as PyPI** |

## Boundaries (honesty)

- Not a full Chromium; not a site-bypass product
- Sample Path A / hybrid / pure-calc split remains
- Fingerprint thickness residual vs browser
- Default environment toolchain is diagnostic-first

## Links

- Public repo: https://github.com/lgnorant-lu/ming_iv8_rs  
- PyPI (after publish): https://pypi.org/project/ming_iv8_rs/  
- Governance: `docs/roadmap/v0.8/shared/v0.8-release-and-tag-governance-closeout.md`  
- Opensource ledger: `docs/todo/TODO-opensource.md`
