# multi_bundler fixtures

> Maintained **with the tests**, not as a separate roadmap doc.  
> Policy: caller-supplied **text only** (no CI network fetch of remote sites).

## Why a README here (not a new roadmap file)

Sample inventory is **test infrastructure**. Canonical product plan stays in  
`docs/roadmap/v0.8/native-substrate/v0.8.99-scope-brief.md` +  
`v0.8.99-s7-audit-residual.md` + `docs/todo/TODO-bundler.md`.  
Avoid scattering extra `v0.8.99-*-samples.md` under roadmap.

## Layers

| Layer | Purpose | Location |
|---|---|---|
| **L0 synthetic** | Fast classify/plan/exec unit tests | `*_minimal.js` (checked in) |
| **L1 multi-chunk synthetic** | Cross-chunk require / factory merge | `webpack_multichunk_*.js` (checked in) |
| **L2 official-shape** | Gold shapes from webpack examples (optional build) | `generated/` or script-built; see below |
| **L3 pressure** | Large/minified (optional, may be `#[ignore]`) | not required for default CI |

## L0 (current, always CI)

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
| `bdms_negative_plain_cjs.js` | BDMS false-positive guard |

## L2 gold sources (external, do not scrape live sites)

Prefer **webpack official examples** (build offline, pin webpack version):

1. `webpack/webpack` → `examples/common-chunk-and-vendor-chunk`  
2. `webpack/webpack` → `examples/scope-hoisting` (lazy chunk + `webpackChunk`)  
3. Optional: `examples/many-pages` / `two-explicit-vendor-chunks`

**How to add (manual, not default CI network):**

```text
# on a machine with network, once:
git clone --depth 1 https://github.com/webpack/webpack
cd webpack/examples/common-chunk-and-vendor-chunk
npm i && npx webpack --mode production
# copy dist/*.js into fixtures/multi_bundler/generated/ with LICENSE note
```

License: webpack examples follow webpack repo license; keep attribution in  
`generated/NOTICE` if vendoring dist.

## Assertions to prefer

- `collect_module_graph`: `factories_installed`, `edges`, `cycles`, `chunk_id` on nodes  
- Cross-chunk `__webpack_require__(id)` after `run_with_entry(..., chunks=[...])`  
- Classify: BDMS positive minified-like; negative plain CJS  

## Branch B (out of this fixture set)

L4 **identifier recovery** / full deobfuscation corpora — not maintained here.
