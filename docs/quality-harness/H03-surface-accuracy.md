# H03: Surface Accuracy (TDC Field Accuracy Rate)

> Created: 2026-06-25
> Status: candidate (L2 implemented, L1/L3/L4/L5 planned)
> Harness Charter: HARNESS-CHARTER.md

## Domain

Browser fingerprint surface value accuracy — verifies that IV8's
fingerprint values match real Chrome output across all fingerprint dimensions.

## Five-Layer Architecture

### L1 — Coverage Rate (Khronos IDL)

Parse Khronos WebGL IDL files to get the complete pname list, then verify
IV8 covers every pname. Does not verify values.

**Status**: Planned (v0.8.82+)

### L2 — Value Correctness (Chrome CDP Sampling + Diff)

Sample all fingerprint values from real Chrome via CDP, sample the same
values from IV8, diff the two.

**Implementation**:
- `scripts/sample_iv8_surface.py` — IV8 sampler
- `scripts/evaluate_surface_accuracy.py` — diff orchestrator
- `golden/chrome147_win10_rtx4060.json` — Chrome golden (needs CDP sampling)

**Sampling covers**:
- WebGL1 getParameter (all Khronos pname) + getShaderPrecisionFormat (12 combos)
- WebGL2 getParameter (all WebGL2 pname)
- getSupportedExtensions (WebGL1 + WebGL2)
- navigator.* (20+ properties)
- screen.* (8 properties)
- AudioContext (sampleRate/baseLatency/outputLatency)
- matchMedia (26 media feature queries)

**CDP sampling method**: Pure Python CDP (no Playwright contamination):
1. Launch Chrome with `--remote-debugging-port=9222`
2. WebSocket connect → `Target.createTarget` → `Target.attachToTarget(flatten:true)`
3. `Runtime.evaluate` (do NOT call `Runtime.enable`)
4. Collect fingerprint JSON → save as golden

**Playwright fallback**: Only if CDP proves unworkable (note: injects
`__playwright__binding__` which contaminates fingerprint).

**Thresholds**:
- WebGL pname: 100% match (exact integer/string comparison)
- Shader precision: 100% match (12/12 combos)
- Navigator: 100% match
- Screen: 100% match
- Audio: 100% match (baseLatency/outputLatency)
- Media: 100% match (boolean match per feature)

### L3 — Consistency (Self-built Ruleset)

Cross-field consistency verification. Self-built (NOT using
fingerprint-coherence PyPI package — GitHub 404, supply chain risk).

**Status**: Planned (v0.8.82+)

**Rules** (reference fingerprint-coherence 11 rules + extensions):
- E001: UA ↔ platform
- E002: Screen ↔ form factor
- E003: Timezone ↔ language
- E004-E006: Client Hints ↔ UA
- E007: WebGL renderer ↔ OS
- E008: Screen avail ≤ screen
- E009: Touch points ↔ form factor
- E010: DPR plausibility
- E011: UA validity
- GPU coherence: GPU model ↔ MAX_TEXTURE_SIZE/extensions/precision
- Canvas hash stability: same seed → same hash
- Font list ↔ OS consistency

### L4 — Cross-Context (main vs iframe vs Worker)

Verify fingerprint consistency across execution contexts.

**Status**: Planned (v0.8.82+)

**Checks** (inspired by fpscanner):
- webdriver: main vs iframe vs Worker
- platform: main vs iframe vs Worker
- WebGL renderer: main vs Worker
- navigator.userAgent: main vs Worker

### L5 — Detection Page Scoring

Run CreepJS/BotD/neoprint against IV8 and extract scores.

**Status**: Planned (v0.9+)

**Tools**: CreepJS (primary, 40+ collectors, 50+ lie checks)
+ self-built noise detection (Canvas 5x + Audio 3x variance).

## Known Limitations

- L2 golden data requires Chrome CDP environment (not yet collected)
- L3 ruleset is self-built (no external dependency)
- L4 requires iframe/Worker execution in IV8 (partially supported)
- L5 requires full page rendering (v0.9+ for complete CreepJS execution)
- Golden data is per-GPU-profile (RTX 4060 ≠ Intel UHD)
