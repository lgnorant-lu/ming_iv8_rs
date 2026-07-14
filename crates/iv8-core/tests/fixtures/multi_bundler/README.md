# multi_bundler fixtures

> Maintained **with the tests**, not as a separate roadmap doc.  
> Policy: caller-supplied **text only** (no CI network fetch of remote sites).

## Why a README here (not a new roadmap file)

Sample inventory is **test infrastructure**. Canonical product plan stays in  
`docs/roadmap/v0.8/native-substrate/v0.8.99-scope-brief.md` +  
`v0.8.99-s7-audit-residual.md` + `docs/todo/TODO-bundler.md`.  
Avoid scattering extra `v0.8.99-*-samples.md` under roadmap.

## Layers (strict inventory)

| Layer | Purpose | In-repo? | Location |
|---|---|---|---|
| **L0 synthetic** | Fast classify/plan/exec unit tests | YES (CI) | `*_minimal.js` |
| **L1 multi-chunk synthetic** | Cross-chunk require / factory merge | YES (CI) | `webpack_multichunk_*.js`, `bdms_*.js` |
| **L2 official-shape** | Gold shapes from webpack examples | YES (CI) | `generated/*` (pinned dist + NOTICE) |
| **L3 historical / pressure** | Real site dumps used in past audits | **NO** (not CI) | `_ref/yy/*` (local only; see below) |
| **L4 deobf corpora** | Identifier recovery | OUT | Branch B only |

### Honest status (R11 audit)

- **Before this wave:** only L0 + hand-written L1. L2 was documented but **not checked in**.
- **Selection datasets that existed but were NOT fixture-CI:**
  - `_ref/yy/` — real webpackJsonp multi-chunk (runtime / Page.chunk / vendor ~679KB). Historical load notes in `TODO-bundler.md` (v0.8.79). **Not copied into fixtures** (size + site origin; use as optional local pressure).
  - `_ref/iv8-examples/js/bdms_*.js` — minified BDMS-like pressure for detection; L1 has synthetic pos/neg only.
  - Official webpack example dist — now **pinned under `generated/`**.

## L0 (always CI)

| File | Kind |
|---|---|
| `browserify_minimal.js` | Browserify |
| `rollup_iife_minimal.js` | Rollup IIFE |
| `rollup_umd_minimal.js` | UMD |
| `vite_iife_minimal.js` / `vite_esm_minimal.js` | Vite |
| `esbuild_minimal.js` | esbuild → UnknownIife |
| `unknown_iife_minimal.js` | Unknown IIFE |
| `webpack_multichunk_runtime.js` | Webpack runtime + require |
| `webpack_multichunk_vendor.js` | webpackChunk vendor table |
| `webpack_multichunk_page.js` | page modules requiring vendor |
| `bdms_positive_minified_like.js` | BDMS-like positive |
| `bdms_negative_plain_cjs.js` | BDMS false-positive guard |

## L2 official (checked in: `generated/`)

Source: webpack `examples/common-chunk-and-vendor-chunk`  
Build: webpack 5.x, `--mode production --devtool false`, named chunkIds, splitChunks vendor+commons.  
Stub vendors: `vendor1` / `vendor2` modules (example node_modules shape).

| File | Role |
|---|---|
| `vendor.js` | shared vendor chunk (`webpackChunk{uniqueName}`) |
| `commons-utility2_js.js` / `commons-utility3_js.js` | commons splits |
| `pageA.js` / `pageB.js` / `pageC.js` | multi-entry pages |
| `NOTICE` | attribution |

**Shape lesson:** real webpack 5 uses `self.webpackChunk{output.uniqueName}`, not only `webpackChunk`.  
IV8 scanners must enumerate `webpackChunk*` keys (implemented S7 continuum).

Rebuild (optional, offline once):

```text
# see NOTICE; copy dist/*.js into generated/ after local webpack build
```

## L3 historical (local `_ref/yy`, not CI)

| File | Notes |
|---|---|
| `runtime_patched.js` / `runtime~Page.*.js` | webpackJsonp runtime + `c.e` remote ensureChunk |
| `Page.chunk.*.js` | page modules |
| `vendor.chunk.*.js` | large vendor (~679KB) |
| `*_vmp*` | non-standard / error cases |

Use for manual pressure only. Default CI must not depend on `_ref/`.

## Assertions to prefer

- `collect_module_graph`: `factories_installed`, `edges`, `cycles`, `chunk_id`, **named `webpackChunk*`**
- Cross-chunk `__webpack_require__(id)` after `run_with_entry(..., chunks=[...])`
- Classify: BDMS positive minified-like; negative plain CJS
- L2: load `generated/vendor.js` + commons + `pageA.js` → non-empty graph

## Branch B (out of this fixture set)

L4 **identifier recovery** / full deobfuscation corpora — not maintained here.
