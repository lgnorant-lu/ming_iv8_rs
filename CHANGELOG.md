# Changelog

All notable changes to iv8-rs are documented here.
This project adheres to [Semantic Versioning](https://semver.org/) and
[Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

## [0.8.46] - 2026-06-15

> Local milestone accepted. v0.8.46 starts P3 Timing/Performance with a boundary
> slice: owner routing, brief selection, performance.now profile-backed coherence
> gate, and timing surface classification. Package metadata remains `0.8.11`.

### Added

- **Timing owner path**: `TIMING_OWNER_PATH = "iv8-core/events/"` with
  `TIMING_RUNTIME_VECTORS = {"V067"}` for performance.now brief selection.
- **performance.now Rust test**: `timing_performance_now_boundary_v046` verifies
  `typeof`/`>=0`/monotonic with profile injection.

### Quality Gates

- Bridge + convergence focused: 85/85 PASS
- Python: 1504/1504 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff: PASS

### Deferred to P3 Follow-Up

- `performance.timeOrigin` exact value, `performance.timing` offset fidelity,
  `PerformanceObserver`, navigation timing level 2, sub-ms precision.

### Non-Goals Preserved

- No timing precision modification, no timeOrigin/navigation mutation,
  no BCR/UA mutation, no Chromium zero-diff claim.

## [0.8.45] - 2026-06-15

> Local milestone accepted. v0.8.45 starts P2 NavigatorUAData / Client Hints with
> a low-entropy boundary slice: owner routing, brief selection, profile-backed
> platform/mobile/brands-shape, and async inventory. Package metadata remains
> `0.8.11`.

### Added

- **UAData owner path**: `NAVIGATOR_UADATA_OWNER_PATH = "iv8-core/user_agent_data.rs"`
  with `_OWNER_ALIASES` accepting `"iv8-core/native_env.rs"` for backward compat.
- **UAData runtime vectors**: `UADATA_RUNTIME_VECTORS = {"V014"}` — only V014
  selected as low-entropy scope.
- **UAData Rust test**: `uadata_low_entropy_boundary_v045` verifies
  `platform`/`mobile` profile projection and `brands` array/key shape.

### Quality Gates

- Bridge + convergence focused: 81/81 PASS
- Python: 1500/1500 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff: PASS

### Deferred to P2 Follow-Up

- `getHighEntropyValues()` Promise behavior (needs async shape design doc)
- `fullVersionList`, brands value/order/version, UA string coherence

### Non-Goals Preserved

- No getHighEntropyValues behavior repair.
- No fullVersionList or high-entropy value verification.
- No brands value/order/version or UA string coherence claim.
- No BCR/UA mutation.

## [0.8.44] - 2026-06-15

> Local milestone tag pending. v0.8.44 completes L3 P1 Screen/Window
> Coherence Batch by extending multi-owner runtime brief selection to
> Screen/Window vectors. Package metadata remains `0.8.11`.

### Added

- **Multi-owner brief selection**: `select_runtime_briefs()` and
  `validate_runtime_brief()` now accept `owner_path` parameter;
  `SCREEN_WINDOW_OWNER_PATH = "iv8-surface"` and `SCREEN_RUNTIME_VECTORS`
  added.
- **Screen runtime test**: `screen_profile_runtime_batch_v044` covers 7
  screen/display fields via `KernelConfig::with_profile_matrix()`.

### Quality Gates

- Bridge + convergence focused: 75/75 PASS
- Python: 1494/1494 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff: PASS

### Non-Goals Preserved

- No NavigatorUAData, Timing, Descriptor, or layout parity claim.
- No BCR parameterization activation.

## [0.8.43] - 2026-06-15

> Local milestone tagged. v0.8.43 completes L3 P0 Navigator/Profile
> Runtime Batch M1, the first evidence-driven Rust runtime mutation version.
> Package metadata remains `0.8.11`.

### Added

- **Runtime brief selection** (`tools/diagnostic_bridge/`): `select_runtime_briefs()`
  filters v0.8.42 repair briefs by readiness, Navigator/Profile owner path,
  value_mismatch gap class, and ≤medium risk.
- **Runtime validation**: `validate_runtime_brief()` verifies source_vector is
  in the Navigator runtime set and all gating conditions are met.
- **Navigator runtime test** (`crates/iv8-core/`): `navigator_profile_runtime_batch_v043`
  covers all six target Navigator fields (language, languages, platform,
  webdriver, hardwareConcurrency, deviceMemory) via `KernelConfig::with_profile_matrix()`.

### Quality Gates

- Bridge + convergence focused: 71/71 PASS
- Python: 1490/1490 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff: PASS

### Non-Goals Preserved

- No Screen/Window, NavigatorUAData, Timing, or Descriptor repair.
- No BCR parameterization activation.
- No userAgent construction change.
- No profile, baseline, or probe-pack writes.

## [0.8.42] - 2026-06-15

> Local milestone tagged. v0.8.42 completes Runtime Repair Harness M1 by
> projecting repair candidates into repair briefs, evidence bundle manifests,
> validation plans, and readiness classifications. Package metadata remains
> `0.8.11`.

### Added

- **Repair brief schema** (`iv8-repair-brief.v0.1`): `build_repair_brief()`
  creates deterministic, diagnostic-only repair instructions from candidate and
  ticket data.
- **Evidence bundle manifest** (`iv8-repair-evidence-bundle.v0.1`):
  `build_evidence_bundle_manifest()` records source report refs, delta refs,
  cross-source refs, knowledge refs, and explicit missing evidence without file
  reads or writes.
- **Validation plan schema** (`iv8-repair-validation-plan.v0.1`):
  `build_validation_plan()` returns command strings, acceptance predicates, and
  negative gates without executing commands.
- **Readiness classification**: `classify_repair_readiness()` reports
  `ready`, `incomplete`, or `blocked` with reasons while preserving
  `diagnostic_only`.

### Quality Gates

- Bridge + convergence focused: 57/57 PASS
- Python: 1476/1476 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff: PASS

### Non-Goals Preserved

- No Rust runtime repair or BCR parameterization.
- No auto-apply, validation command execution, or patch generation.
- No profile, baseline, probe-pack, corpus, source, or manifest writes.

## [0.8.41] - 2026-06-15

> Local milestone tag pending. v0.8.41 completes Diagnostic-to-Substrate
> Bridge M2 with before/after delta contract and repair candidate ledger.
> Package metadata remains `0.8.11`.

### Added

- **Delta contract** (`tools/diagnostic_bridge/`): `build_delta_contract()`
  binds a repair ticket to base and current convergence snapshots.
  `check_gap_resolved()` detects when a gap's status transitions from
  mismatched/missing/errored to matched.
- **Candidate ledger**: `build_candidate_ledger()` produces prioritized
  ticket queue sorted by risk level and gap class severity.
- **Bridge M2 gates**: 5 tests covering delta contract, gap resolution,
  snapshot immutability, priority ordering, and lifecycle state.

### Quality Gates

- Bridge + convergence + crossref + feedback focused: 85/85 PASS
- Python: 1469/1469 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff: PASS

### Non-Goals Preserved

- No runtime repair, harness, or BCR parameterization.
- No Rust runtime changes.

## [0.8.40] - 2026-06-15

> Local milestone tag pending. v0.8.40 completes Diagnostic-to-Substrate
> Bridge M1 by creating the repair ticket schema, knowledge-to-ticket
> projection, L3 owner routing table, and evidence referencing layer.
> Package metadata remains `0.8.11`.

### Added

- **Diagnostic bridge** (`tools/diagnostic_bridge/`): new module between L2
  diagnostics and future L3 repair.
- **Repair ticket schema** (`iv8-repair-ticket.v0.1`): `RepairTicket` dataclass
  with deterministic `ticket_id`, `source_vector`, `gap_class`, `evidence_refs`,
  `l3_owner_module`, `risk_level`.
- **Knowledge projection**: `project_tickets_from_knowledge_index()` projects
  convergence `known_gaps` into structured tickets.
- **L3 owner routing**: `OWNER_ROUTING_TABLE` (~70 vectors → 6 crate targets)
  with `route_ticket_to_owner()`.
- **Evidence referencing**: tickets bundle `source_event_ids` from convergence.
- **Bridge gates**: 9 tests covering schema, determinism, projection,
  routing, no-mutation, no-write.

### Quality Gates

- Bridge + convergence + crossref + feedback focused: 80/80 PASS
- Python: 1464/1464 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff: PASS

### Non-Goals Preserved

- No runtime repair, BCR parameterization, or delta contract.
- No Rust runtime changes.
- No file writes.

## [0.8.39] - 2026-06-15

> Local milestone tag pending. v0.8.39 completes L2 Analyze Depth M1 by
> enriching MAPE-K Analyze/Plan with gap taxonomy, severity weighting, and
> cross-source correlation consumption. Package metadata remains `0.8.11`.

### Added

- **Enriched `analyze()`**: Per-group gap_class distribution and severity
  summary. Optional `cross_source_report` parameter for cross-reference
  consumption.
- **Enriched `plan()`**: PlanItem metadata now includes `gap_class`,
  `severity`, and `cross_classification`. More specific reason text.
- **Map depth gates**: 9 tests covering gap taxonomy, severity, backward
  compatibility, crossref input, no-mutation, enriched PlanItems, and
  report-only invariants.

### Quality Gates

- Feedback + convergence + crossref focused: 70/70 PASS
- Python: 1454/1454 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff: PASS

### Non-Goals Preserved

- No repair ticket schema or L3 owner routing.
- No Rust runtime vector fixes.
- No BCR parameterization activation.
- No profile, baseline, probe-pack writes.

## [0.8.38] - 2026-06-15

> Local milestone tag pending. v0.8.38 completes L2 Signal Completion M1 by
> filling handled coverage visibility, adding in-memory profile auto-fill, and
> conservatively expanding the constructor allowlist. Package metadata remains
> `0.8.11`.

### Added

- **Coverage map completion** (`tools/convergence/`): ~27 new entries in
  `_VECTOR_COVERAGE_MAP` for handled-but-unmapped screen, window, element
  box-model, and DOM shape surfaces; map grows from 68 to 99.
- **Profile auto-fill** (`tools/idl_probe/generate_probe_pack.py`):
  `build_profile_values_from_env()` projects flat environment data into the
  `profile_values` dot-path shape, skipping sensitive standard-IDL surfaces.
- **Constructor allowlist expansion**: 9 live-global constructors added to
  `_CONSTRUCTOR_AVAILABLE` (`CustomEvent`, `DOMMatrix`, `DOMPoint`, `DOMParser`,
  `DOMRectReadOnly`, `File`, `KeyboardEvent`, `MessageChannel`, `MouseEvent`).

### Quality Gates

- Focused convergence + IDL + compat: 111/111 PASS
- IDL + compatibility focused: 69/69 PASS
- Convergence focused: 36/36 PASS
- Python: 1445/1445 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff for implementation files: PASS (3 pre-existing issues only)

### Non-Goals Preserved

- No Rust runtime vector fixes or L3 behavior changes.
- No Analyze/Plan depth or repair ticket schema.
- No BCR parameterization activation.
- No profile, baseline, probe-pack, manifest, corpus, sample, or source writes.

## [0.8.37] - 2026-06-15

> Local milestone tagged. v0.8.37 completes L2 Data-Flow Depth M1 by
> repairing Navigator/NavigatorUAData probe data depth and adding a report-only
> Probe/Witness cross-source correlation layer. Package metadata remains
> `0.8.11`.

### Added

- **Navigator supplementary IR repair** (`tools/idl_probe/`): in-memory
  supplementary attributes add probes for 9 classic Navigator fingerprint paths:
  `userAgent`, `platform`, `vendor`, `language`, `languages`,
  `hardwareConcurrency`, `deviceMemory`, `webdriver`, and `cookieEnabled`.
- **NavigatorUAData supplementary attributes**: adds diagnostic probes for
  `architecture`, `bitness`, `model`, `platformVersion`, `wow64`, and
  `fullVersionList`.
- **Probe/Witness cross-reference** (`tools/cross_reference/`): static exact
  `iv8-cross-source-map.v0.1` plus report-only
  `iv8-cross-source-correlation.v0.1` with `consistent`, `divergent`,
  `one_sided`, and `missing_both` classifications.
- **Focused tests** (`tests/test_cross_reference.py` and IDL probe gates): cover
  supplementary probes, profile overlay activation, sensitive split access,
  deterministic mapping, no-mutation behavior, and report-only invariants.

### Changed

- Default 51-interface generated probe count increases from 1,125 to 1,155 while
  preserving the original 1,125 probe IDs and relative order.
- `navigator.cookieEnabled` is treated as a sensitive standard IDL surface and
  uses split property access in generated JS.
- R2 review hardening records supplementary IR provenance in generated probe
  metadata and separates `missing_both` from one-sided cross-source results.

### Quality Gates

- R2 focused IDL + compatibility + cross-reference: 62/62 PASS
- Python: 1427/1427 PASS (1 skipped)
- L3 Phase C: 81/81 PASS
- Scoped ruff for implementation files and cross-reference tests: PASS

### Non-Goals Preserved

- No Rust runtime vector fixes or L3 behavior changes.
- No profile auto-fill from `flat_env`.
- No constructor allowlist expansion.
- No MAPE-K Analyze depth enhancement.
- No coverage map unmapped vector fill.
- No profile, baseline, probe-pack, manifest, corpus, sample, or source writes.
- No full 1284-interface probe generation.

## [0.8.36] - 2026-06-15

> Local milestone tag. v0.8.36 completes L2 Data-Flow Connectivity M1 by
> connecting profile-derived expectations, IDL probes, witness reports, MAPE-K,
> convergence snapshots, and 105-vector coverage mapping. Package metadata
> remains `0.8.11`.

### Added

- **Profile-aware IDL probes** (`tools/idl_probe/`): optional keyword-only
  `profile_values` overlay adds in-memory expected-value checks for generated
  value probes while keeping type/shape guards first.
- **Constructor allowlist checks**: conservative live-global `instanceof`
  checks for audited constructors, with `source_ir.type_check_strength`
  metadata for constructor, explicit-map, V8 built-in, and weak fallback cases.
- **Witness report routing** (`tools/feedback_loop/`): optional keyword-only
  `witness_reports` input routes source reports into convergence snapshots
  without turning them into fake probe results or monitor observations.
- **Coverage map data-fill** (`tools/convergence/`): expanded representative
  105-vector cross-reference mapping across identity, rendering, locale,
  behavioral, JSVM, and protocol surfaces.

### Changed

- Profile overlay is path-agnostic for generated probe targets and remains
  sensitive-surface-aware; blocked IDL surfaces such as `Document.cookie` stay
  type-only.
- Coverage reporting observes more vector paths but does not promote any vector
  to fixed, handled, or browser-equivalent status.
- v0.8.36 acceptance and post-implementation review now record QoderWork
  independent audit observations and tag readiness.

### Quality Gates

- Focused convergence + IDL + compat + feedback: 100/100 PASS
- IDL focused: 44/44 PASS
- Feedback focused: 19/19 PASS
- Convergence focused: 31/31 PASS
- Python: 1415/1415 PASS (1 skipped)
- L3 Phase C: 81/81 PASS

### Non-Goals Preserved

- No profile, baseline, probe-pack, manifest, corpus, sample, or source writes.
- No auto apply; Execute remains dry-run/report-only.
- No Rust `iv8-feedback` crate or mutable Knowledge DB.
- No full 1284-interface probe generation.
- No Rust runtime vector fixes, L3 behavior changes, or instance-aware descriptor
  execution.
- No Chromium zero-diff or live-network acceptance.

## [0.8.35] - 2026-06-14

> Local milestone tag. v0.8.35 completes L2 Probe Coverage Expansion M1 by
> expanding the IDL probe compiler from a narrow MVP to a curated 51-interface,
> 1,125-probe diagnostic surface. Package metadata remains `0.8.11`.

### Added

- **Probe type dictionary expansion** (`tools/idl_probe/`): type entries expanded
  from 14 to 31 with additional primitive aliases, typed arrays, callbacks, and
  nullable handling.
- **IDL generic and union handlers**: generic sequences and selected unions now
  produce diagnostic checks instead of being skipped.
- **Descriptor and prototype-chain probes**: generated probes now include
  existence, value, descriptor, and inheritance/prototype-chain layers.
- **Curated interface batch**: default generation expanded from 4 to 51 verified
  interfaces while preserving deterministic output and no-write behavior.
- **Coverage gap report** (`tools/convergence/`): diagnostic-only report for
  generated probe coverage versus the 105-vector audit.

### Changed

- WebIDL interface types use weak object fallback by default; v0.8.35 does not
  add general `instanceof` checks.
- Sensitive surfaces and runtime accessibility metadata were hardened in R2 for
  `Document.cookie`, `Document.domain`, and generated JS access paths.

### Quality Gates

- Focused v0.8.35 gates: 85/85 PASS
- Python: 1400/1400 PASS (1 skipped)
- L3 Phase C: 81/81 PASS

### Non-Goals Preserved

- No full 1284-interface generation.
- No Profile-to-Probe data-flow connection.
- No AFL/TDD automatic feedback loop.
- No mutable Knowledge DB or Rust feedback crate.
- No L3 runtime behavior changes or runtime vector fixes.

## [0.8.34] - 2026-06-14

> Local milestone tag closeout ready after final strict review. v0.8.34 completes
> convergence event normalization and reproducible feedback snapshots. Package
> metadata remains `0.8.11`.

### Added

- **Convergence tooling** (`tools/convergence/`): pure-function helpers for
  deterministic `iv8-convergence-event.v0.1`, reproducible
  `iv8-convergence-snapshot.v0.1`, `iv8-convergence-delta.v0.1`, and derived
  `iv8-feedback-knowledge-index.v0.1` artifacts.
- **Source report adapters**: normalize BrowserSurface, undetectable, BCR,
  feedback monitor/loop, profile report, and convergence checker reports into
  diagnostic-only convergence events.
- **Feedback integration**: additive
  `tools.feedback_loop.run_mapek_cycle_with_snapshot()` attaches convergence
  events, snapshot, and read-only knowledge index while preserving dry-run
  Execute semantics.
- **Focused tests** (`tests/test_convergence.py`): deterministic IDs, blocked
  target-flow key filtering and string redaction, source ceiling preservation,
  BCR error handling, error-result precedence, snapshot no-write enforcement,
  snapshot delta lifecycle, expected-divergence knowledge filtering, and feedback
  delta integration.

### Changed

- Stable subject key design excludes `gap_class` for snapshot deltas so
  fail-to-pass transitions classify as lifecycle changes for the same subject.
- v0.8.34 governance wording now explicitly says derived read-only knowledge
  index, not mutable Knowledge DB.

### Quality Gates

- Python: 1367/1367 PASS (1 skipped)
- Focused convergence + feedback: 31/31 PASS
- v0.8.33/v0.8.34 focused gates: 62/62 PASS
- L3 Phase C: 81/81 PASS

### Non-Goals Preserved

- No mutable Knowledge DB.
- No profile, baseline, probe-pack, manifest, or corpus writes.
- No auto apply.
- No Rust `iv8-feedback` crate.
- No full 1284-interface probe generation.
- No L3 runtime behavior changes.
- No Chromium zero-diff or live-network acceptance.

## [0.8.33] - 2026-06-14

> Local milestone tag. v0.8.33 completes the L2 IDL Probe Automation and
> report-only Python MAPE-K phases. Package metadata remains `0.8.11`.

### Added

- **IDL probe compiler** (`tools/idl_probe/`): generates ProbePack JSON from
  `unified_ir.json` for Window, Navigator, Screen, and Location interfaces.
  Produces 43 probe definitions with deterministic output and schema-validated
  shape. All probes include `source_ir` provenance metadata.
- **L3 runtime witness reports** (`tools/witness_reports/`):
  - BCR dispatch structural report — source-code analysis of
    `install_behavior_via_bcr` calls in active path (15 dispatch / 1 direct).
  - BrowserSurface expression matrix report — 26 runtime probes over
    typeof/instanceof/navigator/screen/crypto/WebGL surfaces.
  - Native undetectable semantics report — 7 MarkAsUndetectable checks
    (typeof, loose/strict equality, boolean, key enumeration).
- **Python MAPE-K feedback loop** (`tools/feedback_loop/`):
  Monitor→Analyze→Plan→Execute phases as report-only functions.
  Execute runs in dry-run mode with no mutation.
  Knowledge base provides read-only schema access.
  All reports carry `writes=[]` and `evidence_ceiling=diagnostic_only`.
- **L3 deep refinement backlog** (`l3-deep-refinement-backlog-from-iv8-0.1.3-comparison.md`):
  three-tier gap classification from direct IV8 0.1.3 vs IV8 Rust comparison
  (deferred, not a v0.8.33 gate).

### Design Documents

- `v0.8.33-scope.md`, `v0.8.33-foundation-audit.md`,
  `v0.8.33-idl-probe-runtime-design.md`,
  `v0.8.33-feedback-loop-boundary-matrix.md`,
  `v0.8.33-negative-gate-plan.md`,
  `v0.8.33-implementation-task-plan.md`,
  `v0.8.33-acceptance.md`.

### Quality Gates

- Python: 1349/1349 PASS (1 skipped)
- Focused IDL probe + reports + feedback: 44/44 PASS
- L3 Phase C: 81/81 PASS
- v0.8.30 BCR 15/15 dispatch hub: unchanged
- v0.8.31 use_old_chain retirement: unchanged
- v0.8.32 ProfileMatrix certified path: unchanged

### Non-Goals Preserved

- No Chromium zero-diff claim.
- No L3 broad semantic rewrite.
- No full 1284-interface probe generation (only 4-interface subset).
- No mutation of existing hand-written probe packs, profiles, or baselines.
- No Rust `iv8-feedback` crate created.
- No BCR registry or dispatch architecture changes.

## [0.8.32] - 2026-06-14

> Local milestone tag. v0.8.32 completes the L2 Profile Injection Verification
> Foundation as a profile-to-environment certified path, not as full
> profile-to-BCR native parameterization. Package metadata remains `0.8.11`.

### Added

- **iv8-profile crate implementation**: `ProfileSource`, `ProfileMatrix`, strict validation, deterministic materialization, environment projection, `BehaviorConfig`, `ProfileManifest`, and `ProfileReport`.
- **Certified runtime path**: `KernelConfig::with_profile_matrix()` projects a materialized profile into `RuntimeState.environment` through `environment_overrides`.
- **Runtime E2E coverage**: profile-derived navigator, screen, window DPR, location, UA-CH, WebGL vendor/renderer, and timer projection are verified through Rust tests.
- **Python convergence checker**: report-only expected-vs-actual verification with separate expected and runtime environments.
- **IDL output readiness**: `tools/idl/output/unified_ir.json` verified locally as generated output; directory remains intentionally gitignored.
- **Final audit**: v0.8.32 completion boundary recorded in `v0.8.32-final-audit.md`.

### Changed

- **v0.8.32 scope corrected**: certified path is now profile -> environment -> runtime observations. BehaviorConfig/BCR remains scaffold for v0.8.33+.
- **Profile validation hardened**: unknown modes, zero timing fps, and invalid extended permission states are errors.
- **Timer projection**: `timing.fps` now derives `timers.raf_interval_ms`.
- **Noise seeds**: per-surface `sub_seed` overrides now materialize into deterministic seed domains.
- **Report verdicts**: empty reports now finalize to `no_data`, not `equivalent`.

### Quality Gates

- Rust lib: 289/289 PASS
- Python: 1305/1305 PASS (1 skipped)
- v0.8.32 checker negative case: mismatched runtime UA fails as material failure

### Non-Goals Preserved

- No Chromium zero-diff claim.
- No full 105-vector claim.
- No full IDL-to-probe compiler implementation.
- No full MAPE-K/AFL feedback loop.
- No full BehaviorConfig/BCR native installer parameterization claim.

## [0.8.31] - 2026-06-13

> Local milestone tag. v0.8.31 finalizes the pre-v0.6 native substrate mainline
> with use_old_chain retirement and L2 architecture foundation design.

### Removed

- **KernelConfig.use_old_chain**: field removed from struct, Default impl, iv8-py context.rs. The pre-v0.8.26 init chain (install_environment → undetect_shims → dom_templates) is no longer reachable. All initialization follows the new chain path (install_browser_surface_init → install_undetect_shims(true)).
- **make_old_chain_kernel()**: removed from test_init_chain_comparison.rs. All 81 Phase C tests converted to new-chain-only assertions.
- **assert_both_eval_equal() / assert_both_truthy()**: removed alongside dual-kernel comparison framework.

### Changed

- **EmbeddedV8Kernel::new()**: simplified from if/else branch to unconditional new-chain-only path.
- **Phase C tests (81)**: all dual-kernel comparison assertions converted to single-kernel direct value assertions. Zero cross-chain comparison remains.
- **Probe packs**: fingerprint.m1.json and descriptor.m1.json archived to `probe_packs/_archive/`. Still loadable via existing `load_probe_pack()` API. Replacement by IDL-generated probes planned for v0.8.32.

### Added

- **8 diagnostic codes**: ENV_TOOLCHAIN_PROBE_RUN_STARTED, ENV_TOOLCHAIN_DRY_RUN_STARTED, ENV_TOOLCHAIN_DRY_RUN_COMPLETED, ENV_TOOLCHAIN_COMPARISON_REPORT_BUILT, PATCH_POLICY_RUNTIME_SAFE, PATCH_POLICY_ANALYSIS_ONLY, PATCH_POLICY_UNSAFE_HOOK, PATCH_POLICY_DRY_RUN_SKIPPED. Total: 10/10 diagnostic codes emitted by environment_report_builder.py.
- **5 L2 architecture design documents**:
  - Profile model design (AD-1): 5-layer serde schema, deterministic noise, fluent builder, crates/iv8-profile/ skeleton
  - IDL-to-probe compiler design (AD-2): unified_ir.json → auto-generated ProbePack JSON, 5 probe categories, CLI
  - Feedback architecture design (AD-3): MAPE-K+TDD+AFL tri-level nested loop replacing linear pipeline
  - 105-Vector gap matrix audit: full inventory 7 categories, 55% handled, V8-hard-limit analysis
  - BCR fluent builder API spec (AD-4): BehaviorConfig 36-field model, per-context closure switching

### Quality Gates

- Rust lib: 255/255 PASS
- Phase C: 81/81 PASS (new-chain-only)
- Python: 1296/1296 PASS (1 skipped)
- zero `use_old_chain` / `old_chain` references in Rust codebase
- 12/12 acceptance dimensions verified PASS

## [0.8.30] - 2026-06-13

> Local milestone (L3 100% closure). BCR dispatch hub complete —
> 15/15 install_X modules via BCR, zero direct calls remaining.
> Package metadata and lock metadata remain `0.8.11`.

### Added

- **Batch 1**: console (17 methods) + location (2 methods) via BCR dispatch.
- **Batch 2**: event_loop (9) + timers (6) + page_api (1) + input_api (3)
  via BCR dispatch. Timers dependency on event_loop verified.
- **Batch 3**: crypto_random (2) + subtle_crypto (12) + canvas_bindings
  (4+JS) + webgl_stubs (3+JS) + xhr (1+JS) + date_interceptor (3+JS)
  via BCR dispatch.
- **Batch 4**: native_env (30+ native getters) via BCR dispatch.

### Changed

- **15/15 modules via BCR**: install_browser_surface_init now routes all
  behavior installation through install_behavior_via_bcr. Zero direct
  `install_X(scope, global)` calls remain.
- **BCR interface hardened**: `_bcr` → `bcr` parameter activated, documented
  for v0.8.31 L2-native candidate injection path.

### Known Issues

- **use_old_chain flag remains** (5 refs across 4 files). 23-sample regression
  now at 23/23 PASS (v0.8.28: 9/23), meeting the >=15 threshold for removal.
  Removal candidate for v0.8.31.
- **JS shims retained**: 15 JS shim eval calls preserved. BCR dispatch replaces
  Rust-side install_X calls only; JS prototype overrides (e.g. Canvas2D,
  WebGL, XHR, Date) remain as JS-layer behavior.
- **SPIKE-1 constraint**: ObjectTemplate.set() does not support overriding
  existing prototype methods. Template-level injection requires codegen
  modification — optional for v0.8.31+, not blocking L3 100%.

### Quality Gates

- `cargo build`: PASS
- `cargo test --workspace --lib`: 255/255 PASS (183 core + 30 surface + 42 undetect)
- `cargo test --test test_init_chain_comparison`: 81/81 PASS
- `uv run pytest tests/ -q`: 1296 passed, 1 skipped
- 23-sample regression: 23/23 PASS (baseline for v0.8.31 use_old_chain removal)

## [0.8.29] - 2026-06-12

> Local milestone (BCR Step B dispatch hub + L2 Stage 2 MVP). BCR
> BehaviorInstaller type + 15 installer fields + Tier 1 dispatch (atob_btoa,
> fetch), canvas_2d_gradient fill, SPIKE-1 (prototype.set no-overwrite),
> L2 probe runner + dry-run engine + report builder + 5 guardrail tests.
> Package metadata and lock metadata remain `0.8.11`.

### Added

- **BCR Step B dispatch hub**: BehaviorInstaller type + 15 named installer
  fields in BehaviorCallbackRegistry. `install_behavior_via_bcr()` dispatch
  hook replaces direct install_X calls with BCR-mediated dispatch (dual-path:
  fallback to direct call if BCR field is None).
- **Tier 1 BCR migration**: atob_btoa (2 methods) and fetch (1 method)
  dispatched through BCR. Timers installer populated (retained direct call
  pending event_loop dependency resolution).
- **canvas_2d_gradient**: GradientFactory field populated with
  createLinearGradient-style callback (CanvasGradient stub with addColorStop,
  x0/y0/x1/y1 properties).
- **SPIKE-1**: FunctionTemplate late-bound callback discovery.
  Result: ObjectTemplate.set() does NOT override existing prototype methods.
  First set wins; subsequent sets with same key silently ignored.
  Implication: v0.8.30+ template-level BCR injection requires codegen mods.
- **L2 Stage 2 MVP**:
  - S1 Probe Runner: fingerprint.m1 + descriptor.m1 execution in JSContext,
    gap classification (missing/mismatch/present), IIFE wrapping for
    return-statement probes.
  - S3 Dry-Run Engine: fresh EmbeddedV8Kernel per candidate, JS eval apply,
    before/after ComparisonReport with gap delta.
  - S4 Report Builder: Environment Plane Report v0.1 JSON
    (l2-stage2.v0.1 schema), diagnostic emission for gap detection and
    candidate selection.
  - 5 guardrail tests (G1-G5): no profile/manifest/corpus/probe/candidate
    file mutation. 12/12 tests PASS.

### Changed

- **Comment fix**: install_browser_surface_init native behavior count
  14 → 15 (verified against actual call sites).
- **BCR installer registration**: Tier 1 closures populated in
  install_browser_surface_init before install_browser_surface call.

### Quality Gates

- `cargo build`: PASS
- `cargo test --workspace --lib`: 255/255 PASS (183 core + 30 surface + 42 undetect)
- `cargo test --test test_init_chain_comparison`: 81/81 PASS
- `uv run pytest tests/ -q`: 1296 passed, 1 skipped
- L2 Stage 2 MVP: 12/12 PASS

### Known Issues

- **Timers BCR dispatch not completed**: Scope declares Tier 1 includes timers
  (9 callbacks total), but only atob_btoa + fetch (3 callbacks) are dispatched
  through BCR. Timers installer field is populated but never queried by dispatch
  hook. Deferred to v0.8.30.
- **BCR 12/15 installer fields still None**: Only 3 of 15 BehaviorInstaller
  fields registered (atob_btoa, fetch, timers). 12 fields await Tier 2-4
  migration. Deferred to v0.8.30.
- **SPIKE-1 codegen debt**: ObjectTemplate.set() does NOT support overriding
  existing prototype methods. v0.8.30+ template-level BCR injection requires
  modifying the iv8-surface-codegen pipeline to pass BCR callbacks at
  FunctionTemplate creation time.
- **23-sample regression not re-executed**: v0.8.28 Known Issues recorded
  9/23 PASS. v0.8.29 BCR dispatch changes did not trigger a re-verification
  of the full 23-sample baseline.
- **use_old_chain flag remains**: Inherited from v0.8.28. Maintenance burden
  of ~5 lines across 4 files. Removal deferred to v0.8.30+.

## [0.8.28] - 2026-06-12

> Local milestone (verification closure + BCR Step A). Phase C side-by-side
> old-vs-new chain comparison, 1284 Python tests on new chain, coverage gate,
> BCR Canvas2D/WebGL callback injection (7/10 fields).
> Package metadata and lock metadata remain `0.8.11`.

### Added

- **Phase C side-by-side comparison**: 80 tests comparing old chain
  (install_environment → undetect_shims(false) → dom_templates) vs new chain
  (install_browser_surface_init default path). 76/76 expression pairs match;
  4 documented expected differences (WebGL global visibility, Canvas context
  prototype, toDataURL canvas back-ref, window property count 222 vs 1391).
- **KernelConfig.use_old_chain**: flag to select pre-v0.8.26 init chain for
  regression comparison. Default false (no behavior change).
- **Coverage gate**: window property count >= 95% Chrome 147 (new chain 1391
  props > 1380 threshold PASS).
- **BCR Step A**: BehaviorCallbackRegistry 7/10 fields populated with working
  callbacks:
  - Canvas2D: canvas_2d_factory (v8-bound), toDataURL/getImageData/setSize (send-safe)
  - WebGL: webgl_factory/getParameter/getExtension (v8-bound)
  - iv8-surface install_browser_surface wires all callbacks before template installation
- **WebGL instance creation**: create_webgl_rendering_context_instance with 60+ method
  stubs, 76 GL parameters (7 types: string/int/float/boolean/int-array/float-array/null),
  24 extensions, getParameter/getExtension dispatch.
- **L2 Stage 2 MVP implementation spec**: 4-step controlled adaptation loop
  (observe→propose→apply→compare), 83% reuse from existing Python toolchain,
  7-component gap analysis, 5 mutation guardrails, dry-run design.
- **type_conv helpers**: make_float32_array + make_int32_array for GL typed array returns.

### Changed

- **Python test suite**: 1284 passed, 1 skipped on new chain (v0.8.27 baseline:
  not executed). 6 test expectation updates for new-chain enhancements:
  Request now IDL FT global, AudioContext state "suspended" (spec-compliant).
- **BCR injection**: register_canvas_2d_callbacks placeholder replaced with
  actual CanvasRenderingContext2D factory closure. register_webgl_callbacks
  and register_canvas_send_safe_callbacks added.

### Quality Gates

- `cargo build`: PASS
- `cargo test --workspace --lib`: 255/255 PASS (183 core + 30 surface + 42 undetect)
- `cargo test --test test_init_chain_comparison`: 81/81 PASS (80 side-by-side + 1 coverage gate)
- `uv run pytest tests/ -q`: 1284 passed, 1 skipped
- Real samples: 9 of 23 automated samples PASS (test_v07_real_samples.py covers 9; remaining 14 require external assets and are not in automated scope)
- Coverage: window props 1391 (> 1380 = 95% Chrome 147)

### Known Issues

- **23-sample coverage**: test_v07_real_samples.py covers 9 of 23 samples.
  Remaining 14 samples (abogus, h5st, zp_stoken, rui-shu variants, tdc, etc.)
  require external JS files or network requests not in automated scope.
- **BCR Step A is registration, not replacement**: 7/10 BCR fields are populated
  but no runtime consumer invokes them. Actual behavior continues via direct
  `install_X(scope, global)` calls. BCR-mediated dispatch deferred to v0.8.29.
- **KernelConfig.use_old_chain**: flag exists solely for regression comparison.
  Adds ~15 lines of maintenance burden in embedded_v8.rs. Consider removal in
  v0.9+ after verification is deemed sufficient.

## [0.8.27] - 2026-06-12

> Local milestone (closure/completion). Phase C validation, archive
> tier1_stubs+browser_apis+legacy snapshot, scope_validation extension.
> Package metadata and lock metadata remain `0.8.11`.

### Added

- **Phase C new-chain validation**: 4 tests confirming install_browser_surface_init
  default path creates a working JS environment (create/eval/shim globals/multi-kernel).
  11 JS shim global checks pass.
- **scope_validation extension**: 500 templates in 5 batches, inherit chain
  (EventTarget→Node→Element→HTMLElement→HTMLDivElement) across 5 batch boundaries.
  Both tests use heap_limits(512MB,4GB) matching production config.

### Changed

- **Archive**: tier1_stubs.js/rs → _archive/shims/ (716 empty constructors,
  replaced by 1284 IDL templates). browser_apis.js → _archive/dom/
  (API existence stubs, replaced by 1284 IDL templates).
- **embedded_v8.rs**: steps 16/16b (tier1_stubs + browser_apis eval) removed,
  replaced with explanatory comments.
- **embedded_v8.rs legacy snapshot**: 1761 lines extracted from v0.8.24 tag
  to _archive/kernel/.

### Known Issues

- Phase C scope reduced: side-by-side old-vs-new chain comparison
  (scope.md Track 1) not executed. Current tests are new-chain-only
  validation. Deferred to v0.8.28.
- Python 1285-test regression on new chain not executed. Deferred.
- 23-sample regression on new chain not executed. Deferred.
- env_inject.rs + geometry.rs not archived: still provide 393 real values
  and getBoundingClientRect behavior respectively. Deferred.

### Quality Gates

- `cargo build`: PASS
- `cargo test --workspace --lib`: 255/255 PASS (183 core + 30 surface + 42 undetect)
- Phase C: 4/4 PASS (new-chain-only)
- scope_validation: 3/3 PASS (200 + 500 + inherit chain)

## [0.8.26] - 2026-06-12

> Local milestone. BREAKTHROUGH: V8 GC heap_limits fix enables
> default path switch to install_browser_surface_init.
> Package metadata and lock metadata remain `0.8.11`.

### Changed

- **V8 GC fix**: 根因确认为 heap pressure（非 HandleScope capacity）。
  `CreateParams::heap_limits(512MB initial, 4GB max)` 使 1284 模板
  全量安装成功。3 版本阻塞（v0.8.24-v0.8.26）解除。
- **Codegen 重写**: generate_install_all 产出 Global-handle HashMap +
  v8::scope! 批次（100模板/批，26 scope blocks）。`_parent` →
  `parent` 参数改名引入 624 个 `tmpl.inherit(p)` 原型链改进。
- **install_browser_surface_init**: 成为默认 init 路径。1284 IDL
  模板 + 14 原生行为模块 + 38 DomTemplate 构造函数。
- **install_undetect_shims 参数化**: `skip_native_behaviors: bool`
  控制 Step 5 14 个原生行为是否安装。
- **域文件附带重生成**: 33 文件变更（install_all.rs + 31 域 +
  mod.rs），624 inherit 调用现已正确连接。

### Known Issues

- 6 个旧 shim 文件未归档（post-switch cleanup, deferred v0.8.27）
- Phase C 新旧链对比回归未执行（deferred v0.8.27）
- install_browser_surface_init 未安装 JS shim（Canvas2D/XHR/WebGL
  等仍通过 install_undetect_shims 的 JS eval 步骤提供）
- 域文件附带重生成超出 scope 声明范围（仅 install_all.rs），
  但这是 codegen 全量重生成的自然结果，`_parent`→`parent` 改进为正向

### Quality Gates

- `cargo build`: PASS
- `cargo test --workspace --lib`: 255/255 PASS
- `cargo clippy` (新增警告): 0

## [0.8.25] - 2026-06-12

> Local milestone (partial completion). BehaviorCallbackRegistry upgrade,
> install_all Global-handle transform, behavior wiring to install_browser_surface_init.
> Package metadata and lock metadata remain `0.8.11`.

### Changed

- **BehaviorCallbackRegistry 签名升级**: 10/10 回调字段从 Box<dyn Fn()>
  占位符升级为真实 V8 签名（for<'s> HRTB + scope/local 参数）。
  clippy::type_complexity 预期抑制。
- **install_all Global-handle 转换**: HashMap 从 Local<FunctionTemplate>
  改为 Global<FunctionTemplate>，1284 模板创建后 Global 句柄存活。
  Scope-break 分批未实施（HandleScope::new 返回 ScopeStorage
  需 Pin::new + init，简单模式不激活实际嵌套 scope）。
- **install_browser_surface_init 行为接线**: 6 个旧链行为模块
  （Canvas2D/WebGL/Fetch/XHR/SubtleCrypto/Navigator）通过直接调用
  旧链 install_X(scope, global) 函数接入，与 1284 IDL 模板在同一
  scope 安装。BCR 回调注入路径（register_canvas_2d_callbacks）
  仅签名就位，内部逻辑为空壳。
- **LEGACY_CHAIN 注释块物理删除**: 按 D-031 在 v0.8.25 执行，
  embedded_v8.rs 中 LEGACY_CHAIN_START/END 块已删除。

### Known Issues

- V8 GC IsOnCentralStack 崩溃未修复。install_all 7573 行仍在单一
  HandleScope 内。HandleScope::new(scope) 返回 ScopeStorage
  需 Pin::new + init 才能激活实际嵌套 scope。
  需 v0.8.26 调查 EscapableHandleScope 或 codegen 批量安装策略。
- 默认 init 路径仍为 install_dom_templates()（31 模板）。
- 旧 shim 文件归档未执行（依赖默认路径切换）。
- BCR 回调注入为空壳（register_canvas_2d_callbacks 等仅 let _ = factory）。

### Quality Gates

- `cargo build`: PASS
- `cargo test --workspace --lib`: 255/255 PASS (183 core + 30 surface + 42 undetect)
- `cargo clippy` (新增警告): 0
- 运行时验证: 阻塞（V8 GC 崩溃，新链 install_browser_surface_init 无法执行）

## [0.8.24] - 2026-06-12

> Local milestone (partial completion). Feature Flag removal,
> install_user_overrides, V8 GC scope investigation.
> Package metadata and lock metadata remain `0.8.11`.

### Changed

- **Feature Flag 移除**: `native-surface` feature 定义及全部 7 处
  `#[cfg(feature = "native-surface")]` 条件编译已移除，零残留。
  iv8-surface 变为无条件依赖。LEGACY_CHAIN_START/END 标记旧 cfg 结构。
- **install_user_overrides**: UserOverrides 结构体 + OverrideValue 枚举 +
  install_user_overrides() V8 安装函数。空段和原型链路径过滤。
  INIT 链 Step 8 位置正确（最高优先级）。
- **install_browser_surface_init**: 转为公开 API（可选入口）。

### Known Issues

- V8 GC IsOnCentralStack 崩溃：install_all 创建 ~850+ 模板时触发。
  v0.8.25 尝试 scope-break (HandleScope::new) 未成功——发现
  ScopeStorage 需 Pin::new + init 才能激活。推迟到 v0.8.26。
  见 D-032/D-033。
- install_browser_surface() 未成为默认初始化路径。
  当前默认 install_dom_templates（31 模板）。依赖 V8 GC 修复，
  推迟到 v0.8.26。
- 旧 shim 文件归档延迟到 v0.8.26（6 文件：tier1_stubs.js/rs、
  env_inject.rs、browser_apis.js、geometry.rs、embedded_v8.rs 旧链）。
  依赖默认路径切换完成。
- Python API user_overrides 参数未暴露（仅 Rust KernelConfig 字段就绪）。

### Quality Gates

- `cargo build`（单路径，无 feature flag）: PASS
- `cargo test --workspace --lib`: 255/255 PASS (183 core + 30 surface + 42 undetect)
- `cargo test --workspace`: 306/306 PASS
- `cargo clippy`（新增警告）: 0

## [0.8.23] - 2026-06-12

> Local milestone. Infrastructure optimization: node_cache v8::Weak migration,
> dead code cleanup, document_props.rs modularization.
> Package metadata and lock metadata remain `0.8.11`.

### Changed

- **node_cache v8::Weak 迁移**: HashMap<NodeId, v8::Global<Object>> 改为
  HashMap<NodeId, v8::Weak<Object>>。V8 GC 可回收未使用的 DOM 对象，
  5000 节点场景内存从 ~9MB 降至 ~1.5MB。
- **create_node_object / node_to_v8_object_plain**: 缓存读写逻辑改为
  Weak::to_local + is_empty 检查，miss 时被动清扫失效条目。
- **Lazy Sweep**: 新增 bump_and_maybe_sweep 混合清扫策略，每 500 次
  缓存操作触发全量 retain 清理。

### Removed

- **死代码归档**: dom_prototypes.rs / element_prototypes.rs / navigation.rs
  移动到 _archive/ 目录并附带记录 README.md。
- **document_props.rs 死代码**: 删除被覆盖的 document.title 定义和
  冗余的 document.documentURI 定义。

### Refactored

- **document_props.rs 模块化**: 833 行巨型文件拆分为 254 行核心 +
  audio_context.rs (280 行) + window_extras.rs (280 行)。
  AudioContext 子系统、Window 属性、全局构造函数、structuredClone、
  Blob、performance.timing 独立模块化。

### Quality Gates

- cargo check 零错误
- cargo test -p iv8-core --lib 180/180 通过
- cargo test -p iv8-surface --lib 30/30 通过
- cargo check --features native-surface 通过

## [0.8.22] - 2026-06-11

> Local milestone. P1 deep stubs: Document/createElement/classList/style/Fetch
> FunctionTemplate migration. Package metadata and lock metadata remain `0.8.11`.

### Added

- **Document.createElement 深桩**: tagName 统一 to_ascii_lowercase()。
  HTMLUnknownElement FunctionTemplate 作为未知标签 fallback。
  template_for_tag 扩展到 75+ 标签名覆盖。
- **NodeList FunctionTemplate + querySelectorAll 接线**: 全部集合返回方法
  （querySelectorAll/getElementsByTagName/getElementsByClassName）返回
  NodeList FT 实例（含 item()/length）+ indexed properties。
- **DOMTokenList classList 深桩**: 从 plain Object 迁移到 DOMTokenList
  FunctionTemplate。__nodeId__ 从 DONT_ENUM JS 属性迁移到 internal field
  External。新增 replace/forEach/entries/keys/values 方法。
  toggle 支持 force 参数（truthy/falsy 判断）。Symbol.toStringTag 正确设置。
- **CSSStyleDeclaration style 深桩**: CSSStyleDeclaration FunctionTemplate
  创建（setProperty/getPropertyValue/removeProperty/item/cssText/length）。
  NodeData::Element 新增 style_map 字段实现 per-node 状态持久化。
  camelCase/kebab-case 双向映射。style_cache 实现 element.style===element.style。
- **Headers/Response/Request FunctionTemplate**: 三个 Fetch API FT 及完整
  prototype 方法/accessor 集。build_response_object 使用 Response FT 创建实例。
  clone() 深拷贝 headers。heap_registry 注册 Box 分配供 RuntimeState drop 释放。
- **AudioContext 去重**: element_prototypes.rs 死代码删除。BaseAudioContext.state
  初始值修正为 'suspended'。

### Changed

- install_browser_surface_init() 先 build_dom_templates 再 install_all
  再 install_dom_constructors 覆写，确保 createElement instanceof 闭合。

### Quality Gates

- cargo test -p iv8-core --lib: 179/179 PASS
- cargo test -p iv8-surface --lib: 30/30 PASS
- cargo check --features native-surface: zero errors

## [0.8.21] - 2026-06-10

> Local milestone. P0 deep stubs: Canvas2D data / WebGL data / Location URL
> parser / Navigator getters. Package metadata and lock metadata remain `0.8.11`.

### Added

- **Canvas2D 深桩数据骨架**: 24 属性默认值 + 38 方法 FunctionTemplate 注册。
  create_canvas_2d_context_instance() 工厂函数（留待 v0.8.22 回调签名升级后
  接入 getContext('2d')）。
- **WebGL 深桩数据骨架**: 36 pname → default-value 参数映射表、28 extensions、
  76 constants。全部与 Chrome 147 对齐。
- **Location URL 解析器**: LocationState 数据结构 + rebuild_href()。
  install_browser_surface() 中 debug_assert! 验证 + 严格断言。
- **Navigator 22 属性验证**: 全部 getter 名称确认。debug_assert! 验证返回值。

### Quality Gates

- cargo test -p iv8-surface --lib: 30/30 PASS
- cargo test -p iv8-core --lib: 176/176 PASS
- cargo clippy --features native-surface: clean

## [0.8.20] - 2026-06-10

> Local milestone. BrowserSurface integration + Feature Flag architecture.
> Package metadata and lock metadata remain `0.8.11`.

### Added

- **native-surface Feature Flag** (Cargo.toml): 默认关闭。使用 dep: 语法精准
  控制 iv8-surface 可选依赖。
- **BehaviorCallbackRegistry**: 10 字段双分组（6 V8-bound !Send + 4 Send-safe）。
  Canvas 2D factory/gradient、WebGL factory/getParameter/getExtension、
  Audio context factory 回调槽位。
- **BrowserSurfaceRegistry + SurfaceInstallError**: 安装结果类型。
- **RuntimeState 双字段**: surface_registry + behavior_callbacks（cfg-gated）。

### Changed

- embedded_v8.rs: cfg 分支——install_browser_surface_init()（native-surface）
  vs install_dom_templates()（default）。旧链行为不变。
- cargo check 双模式通过。176/176 old-chain regression PASS。

## [0.8.19] - 2026-06-10

> Local milestone. Rust codegen + iv8-surface crate with 1284 interfaces.
> Package metadata and lock metadata remain `0.8.11`.

### Added

- **Rust 代码生成器** (tools/iv8-surface-codegen/): 6 源文件（ir/topo/
  type_mapper/ea_handler/codegen/main），1182 行。从 unified_ir.json 生成
  Rust FunctionTemplate 构建代码。
- **iv8-surface crate**: 31 域文件 + install_all.rs + mod.rs，~128K 行生成代码。
  1284 interfaces 全覆盖。
- **install_all() 两阶段拓扑注册**: 660 无父 + 624 有父通过 HashMap 查找 parent。
- toStringTag 1284/1284、setter 2528、method 2020、DONT_ENUM 1238、
  tmpl.inherit 624。

### Changed

- empty_constructor 从 19 重复项合并为 generated/mod.rs 单一定义。
- classify_domain() 死代码清理。
- cargo check -p iv8-surface: zero errors。14/14 codegen tests PASS。

## [0.8.18] - 2026-06-09

> Local milestone. IDL preprocessing toolchain → unified_ir.json.
> Package metadata and lock metadata remain `0.8.11`.

### Added

- **Node.js IDL 管线** (tools/idl/): fetch-webref.js → normalize-ast.js →
  merge-tool.js → type-mapper.js → validate.js → generate-ir.js。
- **unified_ir.json**: 5,645 KB，2913 定义，1284 interfaces，覆盖度 91%。
- **Go Gate**: 13/13 PASS。幂等性验证通过。Chrome extensions IR: 161/150 门槛。

### Quality Gates

- Go Gate: 13/13 PASS
- 幂等性验证: 通过（仅 generated_at 时间戳不同）

## [0.8.17] - 2026-06-09

> Local milestone. Navigator/Screen FunctionTemplate migration —
> prototype chain correct，instanceof checks pass.
> Package metadata and lock metadata remain `0.8.11`.

### Added

- **Navigator FunctionTemplate**: 原型链 Navigator.prototype → Object.prototype。
  illegal_constructor 抛 TypeError。instance_template 绕过 new 构造。
- **Screen FunctionTemplate**: 原型链 Screen.prototype → Object.prototype。
- **descriptor.m1**: 184/184 PASS（23 样本 × 8 探针）。
- **fingerprint.m1**: 322/322 PASS。

### Changed

- native_env.rs: 388→434 行。1 处测试更新。5 Python 测试修正。
  4 compat fixture 修正。
- delete navigator.userAgent: 从抛异常改为返回 true（与 Chrome 行为匹配）。

## [0.8.16] - 2026-06-09

> Local milestone. Focused final audit complete. Package metadata and lock
> metadata remain `0.8.11`.

### Added

- **Environment pressure adaptation attempt model**: added
  `environment_toolchain_pressure_adaptation.py` with attempt status vocabulary
  (8 statuses), evaluator status vocabulary (7 statuses), eligibility checks,
  and candidate query helpers.
- **Pressure plan item candidate query**: maps eligible `candidate_pack_review`
  pressure plan items to synthetic `EnvironmentGap` objects for delegate to
  existing `map_gaps_to_candidates()`.
- **Attempt execution harness**: fresh-context before/after probe pack delta,
  with stop_reason and evaluator_status projection.
- **Evidence ceiling unification**: `bridge_contract.ALLOWED_EVIDENCE_CEILINGS`
  expanded to `{"diagnostic_only", "weak"}`.

### Quality Gates

- Focused v0.8.16 gate: 45 passed.
- Combined focused gate: 106 passed.
- Full Python tests: 1284 passed, 1 skipped.
- Scoped v0.8.16 ruff: passed.

## [0.8.15] - 2026-06-09

> Local milestone tag for planning work only. Package metadata and lock metadata
> remain `0.8.11`.

### Closed / Deferred

- Bridge vocabulary cleanup (BRIDGE_CAPABILITIES single-owner consolidation)
  deferred to later hygiene pass. Low-risk; existing tests prevent silent
  divergence.
- Implementation paused. v0.8.16 pressure-aware controlled adaptation
  integration planning supersedes this cleanup.

### Quality Gates

- Documentation review and indexing only; no implementation or release metadata
  changes.

## [0.8.14] - 2026-06-09

> Local milestone tag. Package metadata and lock metadata remain `0.8.11`.

### Added

- **Environment bridge contract helpers**: added
  `environment_toolchain_bridge_contract.py` with bridge levels, observation
  statuses, evidence-ceiling validation, package-neutral validation, no-write
  contract validation, and target-flow term detection.
- **Pressure route bridge context**: pressure-to-plan diagnostics now include
  diagnostic-only `bridge_level`, `observation_status`, `bridge_level_counts`,
  and `observation_status_counts`.
- **Bridge boundary regressions**: added tests proving bridge planning does not
  weaken no-apply, no-write, no-source_ref-read, no-adapter, and no-strong/PASS
  boundaries.

### Changed

- Pressure planning now reuses bridge contract target-flow detection through
  `check_target_flow_terms`, replacing the local `_blocked_payload_keys` scan.
- Pressure planning now reuses bridge contract package-specific vocabulary,
  adding detection coverage for `playwright` and `cdp` without authorizing
  adapters.

### Quality Gates

- Focused v0.8.14 gate: 152 passed.
- Full Python tests: 1239 passed, 1 skipped.
- Scoped v0.8.14 ruff: passed.

## [0.8.13] - 2026-06-09

> Local milestone tag. Package metadata and lock metadata remain `0.8.11`.

### Added

- **Environment probe taxonomy helper**: added
  `environment_toolchain_probe_taxonomy.py` with probe roles, route owners,
  bridge capabilities, future placeholder probe-pack names, and pressure-kind
  route alignment helpers.
- **Pressure planning taxonomy fields**: pressure-to-plan item details now carry
  review-only `probe_role` and `route_owner` fields.
- **Boundary regression tests**: added coverage proving future placeholder probe
  packs are not loadable built-ins, custom probe provenance remains redacted,
  and pressure `source_ref` metadata cannot invoke corpus reads.

### Changed

- `fingerprint.m1` remains classified as `baseline_surface` and `descriptor.m1`
  remains classified as `descriptor_surface`; neither was expanded into a full
  browser or fingerprint model.

### Quality Gates

- Focused final v0.8.13 gate: 115 passed.
- Slice 3 focused gate: 43 passed.
- Slice 4 focused gate: 83 passed.
- Scoped v0.8.13 ruff: passed.

## [0.8.12] - 2026-06-09

> Local milestone tag for documentation/audit work only. Package metadata and
> lock metadata remain `0.8.11`.

### Added

- **Environment mainline continuity audit**: recorded how v0.8.12 continues the
  long-running Environment runtime-readiness mainline.
- **Early-lineage debt audit**: classified v0.1-v0.6 residual debt as absorbed,
  active Environment work, downstream research, or blocked evidence risk.
- **Environment debt taxonomy**: grouped debt into abstraction, keep-small,
  bridge, orchestration, promotion, and blocked-artifact categories.
- **Probe and bridge boundary plan**: clarified `fingerprint.m1` limits,
  bridge levels, and v0.8.13+ handoff candidates.
- **External ecosystem reference notes** and **real-sample evidence boundary**:
  kept external JS environment ecosystems as method references and real samples
  as pressure seeds rather than compatibility targets.

### Quality Gates

- Documentation review and roadmap indexing only; no implementation or release
  metadata changes.

## [0.8.11] - 2026-06-09

### Added

- **Environment pressure-to-plan bridge diagnostics**: `run_environment_toolchain()`
  now emits `ENV_TOOLCHAIN_PRESSURE_PLAN_SUMMARY` and
  `ENV_TOOLCHAIN_PRESSURE_PLAN_ITEM` when both `dry_run_planning=True` and
  `pressure_harness=True` are explicitly enabled.
- **Pressure planning helper module**: added
  `environment_toolchain_pressure_planning.py` for package-agnostic, review-only
  pressure report to plan item mapping.
- **Bridge-not-absorb governance**: added a durable principle that IV8 bridges
  external JS environment ecosystems through capability boundaries instead of
  absorbing or rewriting them by default.
- **Continuous execution protocol**: added v0.8.x execution rules for when
  accepted plans can proceed continuously and when explicit confirmation is still
  required.

### Changed

- v0.8.11 keeps pressure-to-plan diagnostic-only: plan items carry
  `apply_authorized=false`, `writes=[]`, and `diagnostic_only` evidence ceilings.
- v0.8.11 reuses existing explicit flags instead of adding a public API flag for
  pressure planning.
- Package-specific bridge adapters remain blocked; route recommendations use
  capability names such as `dom_fixture_runtime`, `network_shape_stub`, and
  `native_substrate_candidate`.

### Quality Gates

- Full Python tests: 1182 passed, 1 skipped.
- Rust workspace tests: passed.
- Rust clippy with `-D warnings`: passed.
- Focused pressure-to-plan gate: 87 passed.
- Pressure-to-plan focused tests: 12 passed.
- Scoped Environment lint: `ruff check` passed.

## [0.8.10] - 2026-06-09

### Added

- **Environment candidate mapping boundary**: extracted `map_gaps_to_candidates()`
  into `environment_toolchain_candidate_mapping.py` while preserving runtime
  re-export compatibility.

### Changed

- Candidate mapping remains behavior-preserving: disabled candidate packs,
  explicit environment precedence, gap-class filtering, boundary validation,
  patch id deduplication, and first-match order are unchanged.
- v0.8.10 does not change report-only behavior, runtime-safe apply behavior,
  iterative adaptation, pressure harness behavior, writes, or report schema.

### Quality Gates

- Full Python tests: 1170 passed, 1 skipped.
- Rust workspace tests: passed.
- Rust clippy with `-D warnings`: passed.
- Focused v0.8.10 gate: 94 passed.
- Scoped Environment lint: `ruff check` passed.

## [0.8.9] - 2026-06-09

### Added

- **Environment boundary helper module**: extracted boundary validation helpers to
  `environment_toolchain_boundary.py`.
- **Environment asset model module**: extracted asset dataclasses to
  `environment_toolchain_asset_models.py`.
- **Environment asset loading module**: extracted asset loading and provenance to
  `environment_toolchain_asset_loading.py`.

### Changed

- Runtime public re-exports remain stable.
- The JSON asset package `iv8_rs.environment_toolchain_assets` remains unchanged
  and is not shadowed by a Python module.
- v0.8.9 does not extract candidate mapping, local overlay, runner,
  orchestration, or iterative adaptation.

### Quality Gates

- Full Python tests: 1168 passed, 1 skipped.
- Rust workspace tests: passed.
- Rust clippy with `-D warnings`: passed.
- Focused asset/boundary gate: 31 passed.

## [0.8.8] - 2026-06-09

### Added

- **Environment diagnostic builder decomposition**: extracted diagnostic record
  builders to `environment_toolchain_diagnostics.py` with a one-way import
  boundary from runtime to diagnostics.
- **Synthetic pressure manifest smoke fixture**: added non-target-specific
  coverage for pressure manifest summaries, `source_ref` basename redaction, and
  diagnostic-only bridge output.
- **v0.8 Environment governance ledger**: recorded durable planning, naming, and
  commit discipline rules for Environment readiness work.

### Changed

- Diagnostic output shape, codes, severities, ordering, evidence, writes, runner
  behavior, and pressure harness semantics are preserved.
- v0.8.8 remains decomposition-first and does not expand pressure harness or
  automatic adaptation behavior.

### Quality Gates

- Full Python tests: 1159 passed, 1 skipped.
- Rust workspace tests: passed.
- Rust clippy with `-D warnings`: passed.
- Combined v0.8.8 focused gate: 146 passed.

## [0.8.7] - 2026-06-09

### Added

- **Environment Pressure Observation Harness**: added `environment-pressure.v0.1`
  report schema, pressure signal and promotion decision models, and no-write
  typed report helpers.
- **Pressure taxonomy**: added input kind, execution mode, failure kind, pressure
  kind, and promotion level classifiers.
- **Default-off toolchain pressure bridge**: added `pressure_harness=True` to
  capture entry failures as diagnostic-only pressure reports.
- **In-memory pressure batch and manifest helpers**: added batch summaries,
  source-ref basename redaction, and toolchain-compatible diagnostics without
  reading `source_ref` files.
- **Initial Environment Toolchain decomposition**: extracted static data and
  low-risk models to `environment_toolchain_static.py` and
  `environment_toolchain_models.py`.

### Changed

- v0.8.7 treats real samples as pressure seeds, not compatibility targets.
- `pressure_harness=True` with `adapt_runtime_safe=True` remains blocked.
- Pressure output does not authorize apply, writes, profile promotion, substrate
  promotion, or pass-rate claims.

### Quality Gates

- Full Python tests: 1157 passed, 1 skipped.
- Rust workspace tests: passed.
- Rust clippy with `-D warnings`: passed.

## [0.8.6] - 2026-06-08

### Added

- **Environment substrate coverage diagnostics**: `run_environment_toolchain()`
  can now emit explicit report-only substrate inventory diagnostics through
  `substrate_coverage=True`.
- **Environment scaffold gap diagnostics**: added `scaffold_gaps=True` to expose
  a selected core projection of scaffold gaps across substrate, probe,
  candidate, policy, evidence, rollback, and negative-gate classes.
- **Dry-run planning diagnostics**: added `dry_run_planning=True` with
  `ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY` and `ENV_TOOLCHAIN_DRY_RUN_PLAN_ITEM`
  diagnostics that preserve `apply_authorized=false`, `writes=[]`, and
  `diagnostic_only` evidence ceilings.
- **Rollback diagnostics**: added `rollback_diagnostics=True` with rollback
  summary/record diagnostics for context-only and ephemeral-report review scopes.

### Changed

- Candidate packs now accept bounded optional metadata for coherence groups,
  substrate family, dependency kind, expected probe delta, evidence ceiling,
  planning status, rollback scope/hint, boundary status, and blocked reasons.
- v0.8.6 keeps Environment automation report-only by default: no default
  AutoFly, no default apply, no profile/manifest/baseline/corpus/sample/source
  writes, no local-overlay runtime apply, no family-pressure apply, and no
  Rust/native substrate hardening.
- Scaffold gap diagnostics intentionally expose a selected core projection while
  the larger documentation matrix remains review backlog for explicit future
  promotions.
- The public typing stub now mirrors the current `run_environment_toolchain()`
  call shape, including candidate packs, bounded adaptation, local overlays, and
  v0.8.6 report-only diagnostic switches.

### Fixed

- Removed a duplicate `environment_family_pressure_analyzed` diagnostic-only
  evidence record from the single-pass Environment Toolchain path.

### Quality Gates

- Full Python tests: 1100 passed, 1 skipped.
- Targeted v0.8.6/API audit tests: 84 passed.
- Rust workspace tests: passed.
- Rust clippy with `-D warnings`: passed.
- `uv lock --check` passed.
- v0.8.6 audit-scoped `ruff check` passed.
- `git diff --check` passed.
- Full `uv run ruff check python tests` still reports pre-existing legacy test
  lint debt outside the v0.8.6 Environment slice.

## [0.8.5] - 2026-06-08

### Added

- **Environment coherence expansion diagnostics**: `run_environment_toolchain()` now
  includes conservative diagnostic-only coherence groups for `ua_platform`,
  `network_info`, and `timezone_locale`.
- **UA/platform coherence checks**: bounded checks compare generic
  `navigator.userAgent`, `navigator.platform`, `navigator.userAgentData.platform`,
  and `navigator.userAgentData.mobile` shapes without full UA parsing or device
  inference.
- **Network information coherence checks**: bounded checks classify
  `navigator.connection.effectiveType`, `downlink`, `rtt`, `saveData`, and `type`
  value-shape inconsistencies without modeling real network, IP, TLS, HTTP, or
  request flows.
- **Timezone/locale coherence checks**: bounded checks classify `config.timezone`,
  runtime `timezone`, `navigator.language`, and `navigator.languages` shape
  inconsistencies without country-to-timezone mapping, DST modeling, or locale
  database equivalence.
- **Native substrate review diagnostics**: reports include
  `ENV_TOOLCHAIN_NATIVE_SUBSTRATE_REVIEW` to summarize review-gated candidate
  areas and explicitly blocked actions without authorizing runtime or Rust/native
  hardening.

### Changed

- v0.8.5 keeps Environment automation diagnostic-only by default: no profile
  writes, no manifest writes, no corpus/source mutation, no local overlay runtime
  apply, no profile-group auto-apply, no family-pressure apply, and no native
  substrate hardening.
- v0.8.5 records coherence/family/native review signals through diagnostics and
  `diagnostic_only` evidence while preserving `environment-toolchain.v0.1`.
- v0.8.5 keeps target-specific bypass material outside core facts: no domain,
  endpoint, cookie, token, signature, nonce, request body, authorization header,
  request sequence, site-specific hook, or ordered protected-flow recipe is
  generated or applied.

### Quality Gates

- Focused Environment coherence/family/native/adaptation tests: 37 passed.
- Focused Environment compatibility tests: 77 passed.
- Scoped Environment lint: `ruff check` passed.
- Full Python tests: 1061 passed, 1 skipped.
- `git diff --check` passed.

## [0.8.4] - 2026-06-08

### Added

- **Environment profile coherence diagnostics**: `run_environment_toolchain()` now
  emits diagnostic-only coherence groups for `language` and `screen_window`.
- **Coherence diagnostics projection**: reports include
  `ENV_TOOLCHAIN_PROFILE_COHERENCE_SUMMARY` and
  `ENV_TOOLCHAIN_PROFILE_COHERENCE_GROUP` while preserving
  `environment-toolchain.v0.1`.
- **Local overlay boundary**: added explicit `local_overlay` report-only input for
  coherence analysis with dict/path support, basename `redacted_ref` path
  provenance, non-generic key rejection, and blocked vocabulary validation.
- **Local overlay diagnostics**: reports include
  `ENV_TOOLCHAIN_LOCAL_OVERLAY_PROVENANCE` or
  `ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED` without applying overlay values to
  runtime state.
- **Family pressure taxonomy diagnostics**: existing probe gaps are projected into
  generic `FamilyPressure` categories and target families through
  `ENV_TOOLCHAIN_FAMILY_PRESSURE_SUMMARY`.

### Changed

- v0.8.4 keeps Environment automation diagnostic-only by default: no profile
  writes, no manifest writes, no corpus/source mutation, no local overlay runtime
  apply, and no family pressure apply.
- v0.8.4 records `environment_profile_coherence_analyzed` and
  `environment_family_pressure_analyzed` as `diagnostic_only` evidence.
- v0.8.4 preserves the Python/Rust boundary; Rust/native substrate hardening
  remains review-gated and was not changed in this release.

### Quality Gates

- Focused profile coherence tests: 12 passed.
- Focused family pressure tests: 4 passed.
- Focused iterative adaptation tests: 7 passed.
- Focused Environment compatibility tests: 77 passed.
- Scoped Environment lint: `ruff check` passed.
- Full Python tests: 1047 passed, 1 skipped.
- `git diff --check` passed.

## [0.8.3] - 2026-06-08

### Added

- **Environment iterative adaptation scaffold**: `run_environment_toolchain()` now
  supports explicit `adapt_runtime_safe=True` bounded runtime-safe iteration.
- **Adaptation controls**: added `max_iterations` and `stop_on_regression` to
  control safe iteration budget and regression stopping.
- **Machine-readable stop reasons**: reports expose adaptation termination through
  `ENV_TOOLCHAIN_ADAPTATION_SUMMARY` diagnostics.
- **Per-iteration diagnostics**: reports expose before/after coverage, delta,
  matched/applied patch IDs, and stop reason through
  `ENV_TOOLCHAIN_ADAPTATION_ITERATION` diagnostics.
- **Iterative adaptation tests**: added focused synthetic tests for completed,
  budget exhausted, no candidate, no progress, regression, evidence ceiling, and
  invalid iteration budget behavior.

### Changed

- v0.8.3 preserves `environment-toolchain.v0.1` report schema and projects
  adaptation graph data through `ExperimentalDiagnosticRecord.details`.
- v0.8.3 records the Python/Rust implementation boundary: Python owns
  orchestration, assets, policy, iteration, stop reasons, and reports; Rust owns
  `JSContext`, `EnvironmentMap`, `env_inject`, `native_env`, deterministic runtime,
  and DOM/network/timing/browser substrate.

### Quality Gates

- Focused iterative tests: 7 passed.
- Focused Environment compatibility tests: 77 passed.
- Scoped Environment lint: `ruff check` passed.
- Full Python tests: 1031 passed, 1 skipped.
- `git diff --check` passed.

## [0.8.2] - 2026-06-08

### Added

- **Environment custom probe packs**: `run_environment_toolchain()` can now accept
  built-in IDs, `ProbePack` objects, dicts, and JSON paths for probe packs.
- **Environment custom candidate packs**: candidate mapping can now use built-in
  IDs, `CandidatePack` objects, dicts, JSON paths, or `None` to disable mapping.
- **Asset validation**: custom probe/candidate packs are schema-validated and
  bypass-boundary checked before JavaScript execution or candidate mapping.
- **Asset provenance diagnostics**: reports expose built-in, object, custom dict,
  custom path, and disabled asset origins through diagnostics with redacted path
  references.
- **Descriptor probe pack**: added built-in `descriptor.m1` with diagnostic-only
  navigator/screen descriptor and prototype probes.
- **Custom asset fixtures**: added valid, blocked, and malformed custom
  probe/candidate fixtures for boundary and validation gates.

### Changed

- `ExperimentalDiagnosticRecord` now preserves optional `details` fields while
  keeping existing code/severity-only diagnostics compatible.
- `available_probe_packs()` now lists `descriptor.m1` and `fingerprint.m1`.

### Quality Gates

- Targeted Environment tests: 77 passed.
- Scoped Environment lint: `ruff check` passed.
- `git diff --check` passed.

## [0.8.1] - 2026-06-08

### Added

- **Environment Toolchain runtime foundation**: `run_environment_toolchain()` now
  runs bounded Environment probe packs and returns an `EnvironmentToolchainReport`.
- **Probe pack runtime model**: added `fingerprint.m1` probe execution, gap
  classification, coverage snapshots, diagnostics, and root API smoke coverage.
- **Runtime-safe candidate mapping**: generic gaps can map to reviewed
  `runtime_safe` browser-like value candidates.
- **Explicit safe rerun**: `apply_runtime_safe=True` applies only reviewed
  `runtime_safe` candidates in a fresh rerun context.
- **Profile suggestions**: the runner emits review-only profile suggestions while
  keeping `writes=[]`.
- **Package data assets**: built-in probe and candidate definitions moved into
  package JSON assets under `environment_toolchain_assets/`.

### Changed

- v0.8.1 scope is now recorded as Environment Toolchain Runtime Foundation, not
  multi-branch runtime promotion.
- Environment policy style was modernized for the scoped release lint gate without
  changing policy behavior.

### Documentation

- `docs/acceptance/v0.8.1-environment-runtime-acceptance.md` records accepted
  behavior, non-capabilities, real-sample pressure boundaries, and gates.
- `docs/roadmap/post-v0.6/v0.8.2-environment-hybrid-runtime-strategy.md` records
  the next hybrid strategy: native substrate plus data assets plus adaptation
  scaffold plus IV8 policy/report governance.
- Roadmap indexes and the v0.8 decision register now reference the v0.8.1
  acceptance closure and v0.8.2+ hybrid strategy decisions.

### Quality Gates

- Targeted Environment tests: 63 passed.
- Scoped Environment lint: `ruff check` passed.
- Root API smoke: `run_environment_toolchain('', probe_pack='fingerprint.m1', profile=None)` reports `present=14`, `missing=0`, `mismatch=0`, `writes=[]`.
- `git diff --check` passed.

## [0.8.0] - 2026-06-07

### Added

- **Experimental report model APIs**: Python report carriers for Environment
  Toolchain, Deobf Registry / Validation, String Array, VM Analysis / Handler,
  and IR Node reports.
- **Shared report envelope**: `ExperimentalReport`,
  `ExperimentalEvidenceRecord`, `ExperimentalDiagnosticRecord`, schema version
  registry helpers, and typed `from_dict()` / `to_dict()` round-trips.
- **Runtime report model gates**: fixture-backed tests validate v0.8 schema
  compatibility, root exports, type stubs, and typed model conversions.
- **Naming conventions**: public Python APIs use capability-first names instead
  of roadmap labels such as `v08` / `V08`.

### Changed

- v0.8 contract helper naming was promoted from roadmap-specific helpers to
  `experimental_contract_helpers.py`.
- Root README and roadmap workspace now describe v0.8.0 as a schema-backed
  report carrier layer, while v0.7 entry/runtime remains the real-sample
  execution surface.
- Rust release lint gate now uses `cargo clippy --workspace --all-targets -- -D warnings`.

### Documentation

- `docs/roadmap/post-v0.6/runtime-report-api-guide.md` documents public imports,
  conversion helpers, examples, and non-capabilities.
- `docs/acceptance/v0.8.0-runtime-api-acceptance.md` records demonstrated API
  behavior, verification commands, and real-sample regression scope.
- `docs/roadmap/post-v0.6/v0.8.0-implementation-task-plan.md` records the
  release scope, non-scope, acceptance gates, and runtime review boundary.
- `docs/roadmap/post-v0.6/naming-conventions.md` records roadmap label and
  public API naming rules.

### Quality Gates

- Python: 947 passed, 1 skipped.
- Rust: `cargo test --workspace` passed.
- Rust lint: `cargo clippy --workspace --all-targets -- -D warnings` is the
  release lint gate.
- Scoped Python lint: v0.8 report model APIs and tests pass `ruff check`.

## [0.7.0] - 2026-06-07

### Added

- **共享诊断/证据/回退结构化类型**：Rust 端 `crates/iv8-core/src/entry/diagnostics.rs`
  定义 `EvidenceRecord`、`DiagnosticRecord`、`FallbackAttempt` 等类型，
  对齐 Python `iv8_rs/diagnostics.py`，全部 serde 序列化。
- **诊断代码常量库**：common / webpack / dispatch / source-ast / corpus /
  policy 六类共 50+ 诊断代码常量。
- **Webpack module graph 运行时证据填充**：
  - `runtime_flavor` 运行时探测（webpack4/webpack5/unknown）
  - 节点 `executed` 元数据从 `__webpack_require__.c` 获取
  - `module_cache_captured` / `entry_module_executed` / `chunk_event_observed` evidence
  - WEBPACK_REQUIRE_CAPTURE_LATE / WEBPACK_RUNTIME_FLAVOR_UNKNOWN / WEBPACK_MODULE_CACHE_EMPTY
    / WEBPACK_CHUNK_UNSUPPORTED / WEBPACK_EVIDENCE_WEAK 诊断
  - require 运行时回退捕获（处理 IIFE 内 define 的场景）
- **Dispatch detection 结构化输出**：
  - `DispatchCandidate` schema（candidate_id, static_score, risk_level, decision）
  - `to_candidate()` / `to_diagnostic_records()` / `to_evidence_records()`
  - 扩展 trace 格式 `D,pc,opcode,stack_depth,handler_count,argc`
  - `extract_argc()` 从调用表达式提取参数计数
  - `is_overbroad_trap()` 过宽陷阱护栏（需 2+ VM 指示器）
  - `handler_array_captured` / `multi_arg_dispatch_observed` / `switchvm_marker_only` evidence
- **SourceAst transform report**：
  - `SourceAstRequest` / `SourceAstReport` / `SourceAstCandidate` / `SourceAstEdit` 结构体
  - `instrument_with_report()` 返回结构化 report + 转换后源码
  - `source_ast_candidate_detected` / `source_ast_transform_applied` evidence
- **Corpus Runner 稳定化**：
  - CLI 入口 `main()` 支持 `--manifest` / `--out` / `--sample` / `--dry-run` / `--strict`
  - exit codes 0-4 遵循 corpus-runner-contract.md section 17
  - 样本报告嵌入 `module_graph` / `environment_report` 片段
  - `observed_evidence` 从 EntryResult 获取取代空列表
- **v0.7 验收基础设施**：
  - 18 个 fixture JS 文件（webpack/dispatch/source-ast）
  - `docs/acceptance/v0.7-real-sample-manifest.md`
  - `tests/test_v07_acceptance.py` 14 项质量门禁
  - `tests/test_webpack_bridge_solidification.py` 6 项 webpack 合约测试

### Changed

- `EntryResult` 结构：移除 `errors`/`warnings`，改用 `diagnostic_records`/`observed_evidence`
- `Diagnostics` 结构：`fallback_attempts` 从字符串列表升级为结构化 `FallbackAttempt`
- `ExecutedStrategy.diagnostics` 类型改为 `Vec<DiagnosticRecord>`
- `run_with_entry` API 返回字段同步更新
- `SOURCE_AST_RUNTIME_VALIDATION_FAILED` 降级为 warn 级别
- `collect_module_graph` 在读取 prelude log 前执行运行时 require 捕获

### Documentation

- `docs/PROGRESS.md` — 新增 v0.6.x/v0.7.0 章节
- `docs/roadmap/post-v0.6/CAPABILITY_INDEX.md` — webpack/dispatch/source-ast 三项 accepted
- `docs/roadmap/post-v0.6/V0.7_ENTRY_SOLIDIFICATION.md` — v0.7 exit gates 定义
- `docs/acceptance/v0.7-real-sample-manifest.md` — 18 fixture 条目

### Quality Gates

- Rust: 176 tests passed, 0 failed
- Python: 884 passed, 1 skipped
- v0.7 acceptance: 20 tests (14 v0.7 + 6 webpack bridge)
- Corpus Runner: stable report emitted, evidence embedding, CLI exit codes

## [0.6.2] - 2026-06-07

### Added

- **Trace / Evidence / Diagnostics 共享契约**：`python/iv8_rs/diagnostics.py`
  提供 `TraceEvent` 结构化事件信封、`EvidenceGateResult` 统一 PASS/WARN/FAIL、
  `evaluate_evidence_gate()`、`build_trace_events()`、`TRACE_PARSE_PARTIAL`
  已知前缀异常负载检测、`DIAGNOSTIC_CATALOG` 初始 14 代码。
- **API Contract gate 加强**：context manager、post-close rejection、
  expose callback error propagation、specialized API 存在性测试。
- **Environment Patch Policy 策略引擎**：`runtime_safe` / `analysis_only` /
  `unsafe_hook` 三级决策，`decide_patch_policy()` 全决策树，8 种补丁种类
  最低策略映射，`block_mutation()` 5 种变异目标封锁。
- **Environment Plane Automation**：`run_environment_plane()` 完整
  probe → patch → rerun 工作流。报告含 `patch_candidates` / `applied_patches` /
  `rejected_patches` / `coverage` / `evidence` / `diagnostics`。
- **Corpus Runner**：`CorpusManifestItem` / `load_manifest()` Markdown
  表格解析、`build_corpus_report()` corpus-report.v0.1 信封、
  `default_executor()` 通过 Entry Plane 执行样本、`_decide_eligibility()`
  9 种 skip reason、fixture 执行路径。

### Changed

- **Environment Policy 诊断代码细化**：`PATCH_POLICY_OPT_IN_MISSING`、
  `PATCH_POLICY_PERSONA_MISMATCH`、`PATCH_POLICY_RECLASSIFIED` 专用代码。
- **Environment Plane 诊断扩展**：`ENVIRONMENT_PATCH_CANDIDATE`、
  `ENVIRONMENT_PATCH_CONFLICT`、`ENVIRONMENT_RERUN_NO_CHANGE`、
  `ENVIRONMENT_RERUN_REGRESSED`、`ENVIRONMENT_PROFILE_WRITE_BLOCKED`。
- **Environment Plane evidence 扩展**：`environment_gap_observed`、
  `environment_patch_candidate`、`environment_patch_rejected`、
  `environment_coverage_regressed`。
- **Corpus Runner 可选字段**：`CorpusManifestItem` 新增 `fixtures`、
  `policy_overrides`。artifact 记录支持。

### Documentation

- `docs/roadmap/post-v0.6/corpus-runner-contract.md` — Status: In Review
- `docs/roadmap/post-v0.6/environment-patch-policy.md` — Status: In Review
- `docs/roadmap/post-v0.6/environment-plane-automation.md` — Status: In Review
- `docs/roadmap/post-v0.6/CAPABILITY_INDEX.md` — 三项 v0.6.2 标记 in_review
- `docs/GUIDE.md` — 新增 4 节：Trace/Evidence/Diagnostics、
  Environment Patch Policy、Environment Plane Automation、Corpus Runner

### Fixed

- 已知前缀异常负载现触发 `TRACE_PARSE_PARTIAL` 诊断
- 回退 outcome 同时接受 `skip`（spec）与 `skipped`（runner）

## [0.6.1] - 2026-06-07

### Added

- **Shared Trace / Evidence / Diagnostics contracts**: `python/iv8_rs/diagnostics.py`
  provides `EvidenceRecord`, `DiagnosticRecord`, `FallbackAttempt` dataclasses
  with validation, dict roundtrip, and shared prefix registry
  `TRACE_PREFIX_REGISTRY`. `evidence_satisfies()` / `confidence_from_evidence()`
  implement marker-only guard and confidence calculation.
- **Structured trace event envelope**: `TraceEvent` dataclass with `from_raw()`
  converts raw trace lines to normalized `version/kind/prefix/stage/strategy_id/
  sample_kind/payload/source/confidence` shape per spec.
- **Evidence gate evaluator**: `EvidenceGateResult` and `evaluate_evidence_gate()`
  unify PASS/WARN/FAIL decision: policy blocked → FAIL, marker-only → WARN,
  strong evidence → PASS. Covers missing expected evidence, diagnostic output,
  and confidence calculation.
- **DIAGNOSTIC_CATALOG**: 14 initial diagnostic codes from the shared spec,
  each with severity and stage classification.
- **Trace diagnostics enhancement**: `build_trace_diagnostics()` now detects
  known-prefix malformed payloads and emits `TRACE_PARSE_PARTIAL` in addition
  to existing empty/unknown detection.
- **API Contract gate**: `tests/test_api_contract.py` covers constructor/profile,
  eval/Promise, page_load/resource bundle, network handler, inspector safe APIs,
  Entry/Environment wrapper dataclass shape, context manager lifecycle,
  post-close rejection for all stable public methods, expose callback
  success/error/non-callable, and specialized stable API existence checks.
- **Corpus Runner contract**: `python/iv8_rs/corpus.py` with Markdown manifest
  parsing (`load_manifest` / `CorpusManifestItem` / `CorpusRunOptions`),
  eligibility classification, `build_corpus_report()` with `corpus-report.v0.1`
  envelope. Default behavior: skip undeployed/external/manual/blocking samples,
  no automatic file mutation.
- **Environment Patch Policy contract**: `python/iv8_rs/environment_policy.py`
  with `runtime_safe` / `analysis_only` / `unsafe_hook` policy levels,
  `PatchPolicyDecision`, conflict detection, kind risk reclassification,
  mutation guard (`block_mutation()`).
- **Environment Plane Automation report extended**: `EnvironmentPlaneReport`
  gains `schema_version`, `patch_candidates`, `applied_patches`,
  `rejected_patches`, `coverage`, `evidence`, `diagnostics`.
- **Dispatch detection evidence levels**: `DispatchEvidenceLevel` enum
  (`StrongStatic` / `MarkerOnly` / `DiagnosticOnly`) in `dispatch.rs`.
  Multi-argument handler-array classification supported. SwitchVM marked as
  marker-only. Ordinary `obj[key]()` flagged as diagnostic-only.
- **Webpack module graph schema**: `collect_module_graph()` now returns
  `module-graph.v0.1` with `schema_version`, `runtime_family`,
  `runtime_flavor`, `nodes`, `edges`, `chunks`, `evidence`, `diagnostics`.
  Fallback to global `__webpack_require__` for require reference retention.
- **SourceAst dispatch join point narrowing**: Only nested computed member
  with `UpdateOp::PlusPlus` (`A[Q[U++]]()`) triggers `__iv8_trap` instrumentation.
  Ordinary computed calls (`handlers[op]()`, `obj[key]()`) remain intact.

### Changed

- `tests/test_api_contract.py` strengthened: context manager lifecycle,
  post-close rejection for page_load / add_resource / set_network_handler /
  expose, expose callback error propagation, specialized API existence checks.
- `tests/test_trace_evidence_diagnostics.py` strengthened: diagnostic catalog
  coverage, missing expected evidence / marker-only / policy-blocked diagnostics,
  fallback `skip` outcome roundtrip, TraceEvent roundtrip, malformed prefix
  detection, evidence gate evaluator tests.

### Performance

- SourceAst transform no longer wraps all computed member calls, reducing
  false-positive `D,` trace entries and runtime overhead from overbroad trapping.

### Documentation

- `docs/roadmap/post-v0.6/trace-evidence-diagnostics.md` — Status: In Review,
  review checklist complete
- `docs/roadmap/post-v0.6/api-contract.md` — Status: In Review,
  review checklist complete
- `docs/roadmap/post-v0.6/CAPABILITY_INDEX.md` — Two v0.6.1 capabilities
  marked `in_review`

### Fixed

- Fallback attempt outcome now accepts both `skip` (spec) and `skipped`
  (runner) via `FALLBACK_OUTCOMES`.

## [0.6.0] - 2026-06-06

### Added

- **v0.6 baseline**: `docs/baseline/v0.6-dev-baseline.md` records
  the current targeted validation commands, verified outputs, partial
  strategy status, and known stabilization blockers.
- **Environment Plane workflow helper**: `run_environment_plane()` now provides
  a conservative Python-level probe -> patch -> rerun workflow with
  `EnvironmentPatch` and `EnvironmentPlaneReport` result types.
- **SourceAst dynamic source points**: `eval(...)` and `Function(...)` calls are
  captured by AST strategy traps and recorded as `eval,` / `fn_ctor,` trace
  entries.
- **v0.6 real sample manifest**: `docs/acceptance/v0.6-real-sample-manifest.md`
  records current corpus candidates, path status, automation status, and
  validation status without treating missing or historical samples as verified.
- **Architecture philosophy and stabilization plan**: v0.6.0 documents the
  dual mission of Environment Runtime and Runtime Analysis, plus the staged
  stabilization path for v0.6.1 / v0.7.

### Changed

- Clarified release-facing documentation so historical test counts are not
  presented as the current `0.6.0` release baseline unless rerun and recorded.
- Annotated the research index and legacy acceptance criteria as historical
  records with known placeholder corruption, preserving their original content
  while preventing damaged status data from being treated as current truth.
- WebpackBridge runtime glue was moved out of the executor into the webpack
  entry module, with module graph collection behind a dedicated helper.
- Handler-array dispatch detection now recognizes multi-argument calls such as
  `A[Q[U++]](stack, ctx)` and whitespace before the call parentheses.

### Fixed

- `JSContext.close()` and public API lifecycle handling now reject use after
  close and avoid unsafe non-owner-thread V8 teardown.
- `eval_promise()` now surfaces Promise rejection and pending timeout instead
  of silently returning `None`.
- `page_load(base_url=...)` now uses JSON string encoding, and `js_api` names
  are validated before being embedded into generated JavaScript.
- DevTools inspector bind failures are reported synchronously to Python.
- Python callback metadata created by `expose()` is owned by the context and
  released on owner-thread close/drop.
- WebpackBridge no longer clears `__iv8_wp_require` after capturing a global
  require candidate.

### Known limitations

- `0.6.0` EntryPlane is a working skeleton, not a completed expansion:
  `SourceRegex` is pass-through, SwitchVM dispatch only records a detection
  marker, Environment Plane is a conservative workflow rather than a complete
  automatic patch/rerun loop, and real-sample corpus re-validation is still
  pending.
- Current network runtime remains `ResourceBundle -> Python callback ->
  NetworkError`; real HTTP via `reqwest`, async network callbacks, and typed
  `NetworkRequest` / `NetworkResponse` are deferred design items.

## [0.5.0] - 2026-06-03

### Added

- **M25 StructuredTrace**: `parse_trace` / `parse_trace_stream` / `compress_trace` /
  `StructuredTrace` with typed access (dispatches/reads/calls/writes), filtering
  (`filter`/`between`), statistics (`summary`), export (`to_jsonl`/`to_dataframe`),
  sequence extraction (`pc_sequence`/`value_sequence`/`unique_pcs`),
  indexing (`index_by_pc`/`index_by_target`), and `CompressedTrace`.
- **M25b CFG Construction**: `CFG.from_trace` builds directed graph from D entry
  PC sequence. Loop detection (`find_loops`, back edge), module boundary detection
  (`find_modules`, PC gap), cyclomatic complexity, DOT/JSON/DataFrame export,
  basic block collapsing (`collapse_to_blocks`).
- **M26 Taint Tracking**: `TaintEngine` value-matching propagation engine.
  Tracks source values through D entry stack values to W entry outputs.
  `TaintReport` with sources/sinks/flows/unreached_sources/stack_hits.
- **M27 Crypto Detection (4-layer)**:
  - L1: `detect_constants` — 216 constants across 51 algorithms, min_value filtering
  - L2: `detect_sequences` — 24 known tables (AES S-box, SHA-256 K, MD5 T, etc.)
  - L3: `detect_patterns` — behavior pattern matching via opcode_map contract
  - L4: `detect_all` — cross-validation, confidence boost, ambiguity annotation
  - Loop/hotspot detection (`detect_loops`/`detect_hotspots`)
- **M28 Cross-version VM Diff**: `compare_vm_versions` extracts handler arrays
  from two JS VM sources and diffs via SequenceMatcher. `DiffReport` with
  new/removed/modified/unchanged handlers and similarity score.
- **M29 Module Isolation**: `exec_vm_handler` runs a single VM handler in
  controlled conditions with specified stack input, PC setting, and env mocking.
- **M30 CDP Scope API**: `cdp_get_scope_properties(object_id, own_properties)`
  retrieves JS object properties via CDP Runtime.getProperties protocol.
- **M31 Environment Probe**: `probe_environment(js_source, ...)` auto-detects VM
  patterns, instruments or records, executes, and produces structured report
  with reads/calls/writes/missing/issues/coverage/vm_info.
- **TDC dispatch trace**: Deferred stack capture via `__iv8i_cap__` switch,
  instrument_source D entries extended with stack values (comma-separated).
- **Quality Harness**: `docs/quality-harness/HARNESS-CHARTER.md` (10 principles),
  `H01-crypto-detection.md` (A-E categories, thresholded), `evaluate_detection.py`,
  `verify_crypto_data_integrity.py` (7 sections, 4149 checks), `audit_false_positives.py`,
  `audit_m27_realworld.py`, `check_coverage.py`.

### Changed

- **detect_patterns (L3)**: Now requires explicit `opcode_map` parameter.
  Without opcode→semantic token mapping, returns `[]` (was silently non-functional).
- **instrument_source D entries**: Extended from `D,pc,opcode,stack_depth` to
  `D,pc,opcode,depth,val1,val2,val3` for stack value capture.
- **Python test count**: 552 → 681 (Phase 1) → 775 passed, 1 skipped (current v0.5 tree).
- `_parse_entry` now correctly handles 3-field (TYPE,target,value) vs 4-field
  (TYPE,pc,target,value) trace formats.
- pyproject.toml version aligned to 0.5.0 for release.

### Fixed

- **3-field trace parse bug**: `start_recording` 3-field format was parsed as
  4-field, causing probe (non-VM mode) to silently produce garbage.
- **False positives in crypto detection**: min_value filter + removed dangerous
  small-int sequences + raised min_match thresholds. 6 adversarial trace types
  produce zero false matches.

### Quality

- Crypto detection: 51 patterns / 216 constants / 24 sequences / 104 tests.
- H01 harness: OVERALL PASS (A 4149 checks/0 errors, B recall 100%+L3 8/8 fire,
  C false positives 0, D coverage 100%, E robustness 100%).
- Ground truth tests (`test_ground_truth.py`): real V8 execution pipeline verification.
- Real-world adversarial audit (`scripts/audit_m27_realworld.py`).

## [0.4.0] - 2026-06-01

### Added

- **M20 NavigatorUAData API**: `navigator.userAgentData` with brands/mobile/platform
  (synchronous), `getHighEntropyValues(hints)` (async Promise), `toJSON()`.
  Values from environment config. Default brands include Chrome 147 GREASE.
- **M21 Profile System**: `iv8_rs.load_profile(path)` + `JSContext(profile=...)`.
  Three-layer override: environment > profile > defaults. Built-in default
  profile (Chrome 147 Win10). Browser export script (`scripts/export_browser_fingerprint.js`).
- **M22 Diff Analysis Framework**: `iv8_rs.diff_analysis(js_source, eval_expr,
  base_env, test_variables, ...)`. Multi-threaded, deterministic, structured report.
- **Browser API stubs** (`browser_apis.js`): navigator properties (requestMIDIAccess,
  bluetooth, usb, credentials, clipboard, storage, wakeLock, locks, share,
  getBattery, getGamepads, vibrate, sendBeacon, connection), window.customElements,
  window.matchMedia, CSS.supports, window.getComputedStyle (Proxy stub).
- **DOM captureStream**: HTMLVideoElement/HTMLCanvasElement templates gain
  captureStream/mozCaptureStream/webkitCaptureStream methods.
- **navigator.plugins/mimeTypes**: 5 PDF Viewer plugins + 2 MIME types (Chrome standard).
- `document.location` getter (= window.location, was undefined).

### Changed

- `__init__.py` refactored: JSContext is now a factory function (fixes infinite
  recursion OOM from monkey-patching `__new__`). Profile parameter handled in Python layer.

### Fixed

- `document.location` was undefined (ChaosVM uses it for URL → modules failed).

### Verified (TDC real-world)

- cd[] field correctness: 37/38 (97.4%)
- Bitmask cd[7]: 256 → 295 (5/10 bits, API stub ceiling reached)
- Remaining 5 bits: DOM rendering checks (structural limitation, not fixable via stubs)
- 80% of tdc.js versions produce 20+ fields (up from 65% in v0.3.2)

### Fixed

- **[Critical] MarkAsUndetectable integration**: `typeof __iv8__` was returning
  `"object"` instead of `"undefined"`. v0.2 implemented the C++ binding and unit
  tests but never called `mark_as_undetectable` in the kernel initialization path.
  Now `__iv8__` has full `[[IsHTMLDDA]]` semantics: `typeof === "undefined"`,
  `== null`, `Boolean() === false`, while properties remain accessible.
  Real-world impact: TDC ChaosVM collect dropped from 2400 chars to 1088 chars
  (matching browser output) because anti-automation detection modules no longer trigger.
- **WebGL callLog not installed**: The `__iv8__.gl.callLog` array was never created
  because the installation code used `if (!iv8) return` — which always short-circuits
  on an undetectable (falsy) object. Fixed to use `'__iv8__' in globalThis` check.
- **XHR netLog not recording**: Same falsy-check pattern as WebGL. XHR requests
  were silently skipped from `__iv8__.netLog.entries`. Fixed with `'in'` operator.
- **insertAdjacentHTML("beforebegin")**: Was incorrectly appending to the target
  node itself. Now correctly inserts as previous sibling via `insert_before` on parent.
- **WebGL test environment key**: Test used `webgl.renderer` but implementation
  reads `webgl.UNMASKED_RENDERER_WEBGL`. Aligned test with actual API.

### Changed

- **Default fingerprint migrated from Chrome 124 to Chrome 147**: `navigator.userAgent`,
  `navigator.appVersion`, `config.features.profile` updated. Aligns with V8 crate
  version (`v8 = "147"`), eliminating engine/UA version inconsistency.
- `iv8-defaults.json` restored to git tracking (was accidentally untracked in
  a previous cleanup commit). Added `.gitignore` negation rule.
- `Cargo.lock` now tracked for reproducible builds.

### Added

- `docs/GUIDE.md`: 19-section comprehensive usage guide (runtime + analysis).
- Python type stubs (`.pyi`) fully cover v0.3 API surface (CDP, trace, deterministic,
  VM instrumentation, recording, profiler, coverage, instrument_source, trace_diff).
- `__init__.py` exports `instrument_source` and `trace_diff` at module level.
- `README.md` rewritten with v0.3 observability section and documentation links.

### Chore

- Resolved clippy warnings: unused imports, dead code, unreachable pattern, unused variables.
- Version numbers unified to 0.3.2 across Cargo.toml and pyproject.toml.

## [0.3.1] - 2026-05-31

### Fixed

- `instrument_source`: Rewrite injection strategy to dispatch expression replacement.
  Previous approach (source-head only) missed recursive ChaosVM calls. New approach
  replaces `A[Q[U++]]()` with `(log_push, A[Q[U++]]())` — captures EVERY iteration.
- DOM `clientWidth`/`clientHeight` now reads environment fallback chain
  (`document.body.clientWidth` -> `window.innerWidth` -> default).
- WebGL `getParameter(37446)` now reads `webgl.UNMASKED_RENDERER_WEBGL` from
  environment (was reading wrong path `webgl.vendor`).

## [0.3.0] - 2026-05-30

### Added

- **M15 Python CDP API**: `cdp_set_breakpoint`, `cdp_remove_breakpoint`,
  `cdp_evaluate_on_frame`, `cdp_resume`, `cdp_step_over`, `cdp_step_into`,
  `cdp_get_call_frames`, `cdp_process_events`. Full programmatic V8 Inspector
  control without Chrome DevTools.
- **M16 Trace mode**: `set_trace_point`, `remove_trace_point`, `get_trace_log`,
  `clear_trace_log`, `set_trace_limit`. Non-pausing execution tracing via CDP
  conditional breakpoints with side-effect expressions.
- **M17 Deterministic mode**: `random_seed` (Math.random), `crypto_seed`
  (crypto.getRandomValues), `time_freeze` (Date.now/performance.now). Same seed
  produces identical sequences across runs.
- **M18 VM-aware helper**: `detect_chaosvm_vars`, `instrument_chaosvm`,
  `get_vm_trace`, `clear_vm_trace`, `uninstrument_chaosvm`, `detect_vm_dispatch`,
  `trace_vm`. High-performance JSVMP tracing via Proxy (~0.5s for 50000 instructions).
- **M19 Deep trace**:
  - `instrument_source()`: Unified source injection (dispatch replacement + env Proxy).
  - `start_recording()` / `stop_recording()`: Global object Proxy recording.
  - `start_profiler()` / `stop_profiler()`: V8 CPU Profile.
  - `start_coverage()` / `stop_coverage()`: Precise code coverage.
  - `get_unified_trace()` / `clear_unified_trace()`: Unified D/R/C/W trace log.
  - `iv8_rs.trace_diff(trace_a, trace_b)`: Find first divergence between traces.

### Changed

- `with_devtools()` gains `wait` parameter (default True). Set `wait=False` for
  programmatic CDP use without waiting for external DevTools client.
- M19 deep trace validated on TDC ChaosVM: 50000+ bytecode instructions traced,
  58 unique opcodes, TYPE_B source located at PC=26588 in 5 seconds.

## [0.2.0] - 2026-05-30

### Added

- `iv8_core::v8_extra` module providing `MarkAsUndetectable` and
  `SetCallAsFunctionHandler` bindings via cc crate, enabling real V8
  `[[IsHTMLDDA]]` semantics without requiring a forked v8 crate.
- 3 new integration tests for v8_extra (typeof/==/Boolean/if, callable,
  document.all combined pattern).
- `RustValue` enum gains four variants (`BigInt`, `DateTime`, `Map`, `Set`)
  produced when `strict_compat=False`. They map to Python `int` (any size),
  `datetime.datetime` (UTC), `dict` (insertion order), `set` respectively.
- `iv8_py::value_convert` helper module centralizes the new conversions and
  also handles round-trip back to V8 (`int -> BigInt`, `datetime -> Date`,
  `dict -> Map`, `set -> Set`).
- `RuntimeState::has` helper that returns `false` when no state is installed
  (used by conversion code that may run before/without a RuntimeState).

### Changed

- `__iv8__` tool object now has full `[[IsHTMLDDA]]` semantics:
  `typeof __iv8__ === 'undefined'`, `__iv8__ == null`, `Boolean(__iv8__) === false`.
  Property access (`__iv8__.page.load` etc.) remains unchanged.
- `document.all` now uses real `MarkAsUndetectable` (was JS-level workaround).
- `document` is now a real `EventTarget`: `addEventListener`, `removeEventListener`,
  and `dispatchEvent` are wired to the central `EventListenerRegistry` via the
  DOM tree's root `NodeId`. Listeners on `DOMContentLoaded`, `click`, etc. now
  fire correctly. Events from child nodes with `bubbles: true` reach `document`.
- `fetch()` requests are now recorded to `__iv8__.netLog.entries` alongside
  XHR. Same entry shape: `{ method, url, headers, body }`. Header names are
  lowercased; method is uppercased to match XHR semantics.
- When `strict_compat=False`, type conversion produces richer Python values:
  `BigInt -> int`, `Date -> datetime.datetime`, `Map -> dict`, `Set -> set`,
  `TypedArray -> list[int|float]` (11 typed array subtypes preserved).
  Previously these all degraded to strings, `None`, or raw bytes.
  `strict_compat=True` (default) is unchanged for v0.1 compatibility.
- `set_network_handler` is now documented as always-on regardless of
  `strict_compat`. The Python handler runs as the second tier of a three-layer
  fallback chain (ResourceBundle -> handler -> NetworkError) for both `fetch`
  and synchronous XHR. (No code change in v0.2 — this was already the case
  in v0.1; v0.2 just documents and tests the existing behavior explicitly.)
- Resolves L-01, L-03, L-04, L-09, L-10 known limitations from v0.1.

### Build

- iv8-core gains a `build.rs` that compiles `cxx/iv8_v8_extra.cc` via cc crate.
  Auto-locates V8 headers from cargo registry cache; override with
  `IV8_V8_CRATE_DIR` env var if needed.
- Requires C++20 compiler. On MSVC `/Zc:__cplusplus` is added so V8 headers
  detect the standard version correctly.

## [0.1.0] - 2026-05-30

### Added

- V8 147 kernel with eval pipeline, TryCatch, and strict_compat mode (default True)
- RuntimeState per-isolate container with slot-based storage
- IV8Error enum with 5 Python exception classes (IV8Error/EvalError/TypeError/TimeoutError/InternalError)
- safe_callback macro for catch_unwind in V8 callbacks
- 393 environment defaults injection via dot-path notation
- Type conversion matrix (D-3): JS primitives, objects, arrays, BigInt(->None), Date(->'[object Date]')
- GIL release for source >= 256 bytes
- Multiple JSContext coexistence with LIFO drop ordering

- MarkAsUndetectable JS shim for document.all (typeof -> "undefined" via shim)
- wrapNative: function.toString() -> "function name() { [native code] }"
- hookNative: dot-path function interception with Python callable
- window.chrome object (app/csi/loadTimes/runtime with connect/sendMessage error format)
- navigator.webdriver = false (strict_compat iv8 0.1.2 behavior)
- navigator/screen native getter (ObjectTemplate, getter.toString() -> [native code])
- __iv8__ DontEnum (Object.keys invisible)

- ego-tree DOM tree with html5ever HTML5 parsing
- selectors crate CSS Selector Level 4 engine
- FunctionTemplate prototype chain (31 element types)
- Node identity cache (same NodeId -> same V8 object)
- DOM query APIs: getElementById, querySelector, querySelectorAll, getElementsByTagName, getElementsByClassName
- DOM mutation APIs: appendChild, insertBefore, removeChild, replaceChild, cloneNode
- DOM attribute APIs: getAttribute, setAttribute, removeAttribute, classList, dataset
- DOM navigation: parentNode, childNodes, firstChild, lastChild, nextSibling, previousSibling, children
- innerHTML/outerHTML getter and setter with id index maintenance
- document.documentElement, document.body, document.head native getters

- EventLoop with microsecond precision (advance/sleep/tick/drain/getTime/reset)
- setTimeout, setInterval, clearTimeout, clearInterval, requestAnimationFrame, queueMicrotask
- DateInterceptor: Date.now() = EPOCH + eventLoop.getTime()
- EventTarget three-phase dispatch (capture -> target -> bubble)
- stopPropagation, stopImmediatePropagation, preventDefault, once option
- Event/CustomEvent/MouseEvent/KeyboardEvent/PointerEvent constructors

- page.load with HTML parsing, inline script execution, external script execution
- page.load snapshot API ({baseURL, html, resources})
- location object (href/origin/protocol/host/pathname/search/hash + assign/replace/reload)
- document.cookie read/write, document.referrer, document.hidden, document.visibilityState
- document.readyState lifecycle (loading -> interactive -> complete)

- SubtleCrypto: all 12 methods (digest/importKey/exportKey/generateKey/sign/verify/encrypt/decrypt/deriveBits/deriveKey/wrapKey/unwrapKey)
- Algorithms: SHA-1/256/384/512, HMAC, AES-GCM, AES-CBC, PBKDF2, HKDF, RSA-OAEP, RSA-PSS, ECDSA(P-256/P-384), ECDH(P-256/P-384)
- Key formats: raw, spki, pkcs8, jwk
- crypto.getRandomValues (BCryptGenRandom on Windows, getrandom elsewhere)
- crypto.randomUUID

- Canvas2D with tiny-skia real rendering (fillRect/strokeRect/clearRect/fillText/arc/path/transform)
- Canvas toDataURL with PNG encoding, deterministic noise (LCG seed), fixed fingerprint fallback
- Canvas save/restore state stack
- WebGL parameter table with environment-configured values and callLog

- fetch() with ResourceBundle -> Python network_handler -> NetworkError fallback
- XMLHttpRequest (sync + async modes) with network_handler fallback
- NetLog (XHR request recording via __iv8__.netLog.entries)
- eval_promise() for Promise/async function awaiting

- V8 Inspector with CDP WebSocket server (hand-rolled WebSocket with SHA1/base64)
- with_devtools(port, watch_apis) Python API
- vdebugger statement support
- Debugger class: trace_api/trace_apis/watch_property/eval_traced/snapshot/get_call_log/get_call_summary

- PyO3 binding: JSContext class with eval/eval_promise/expose/page.load/add_resource
- Python type stubs (_iv8.pyi + __init__.pyi)
- JSContext context manager (with statement)
- enable_logging() API (tracing subscriber, IV8_LOG env var)
- expose(callable, name) and expose(data, name) for Python interop

- atob/btoa, URL/URLSearchParams, TextEncoder/TextDecoder
- MessageChannel, localStorage/sessionStorage
- navigator.mimeTypes/plugins/permissions/mediaDevices
- history, AudioContext/OfflineAudioContext
- MutationObserver/IntersectionObserver/ResizeObserver (stubs)
- Blob, structuredClone, AbortController
- getComputedStyle, getBoundingClientRect (geometry from environment)
- console.log/warn/error with message capture + Python get_console_messages API

- GitHub Actions CI: lint + rust-test + python-test (Linux/macOS/Windows)
- cibuildwheel: 5 platforms x 2 wheels = 10 wheels
- criterion benchmark suite (context_lifecycle/eval/browser_api/dom/crypto/throughput)
- 198 diff-test fixtures across 19 categories
- 119 CreepJS/FingerprintJS anti-detection tests
- Memory stability tests (100-round long-run, <= 5MB drift)

### Known Limitations

See docs/PROGRESS.md section 7 (L-01..L-10) and docs/adr/001-mark-as-undetectable-deferred.md.

Key items:
- L-01: typeof __iv8__ === 'object' (MarkAsUndetectable not exposed in v8 crate)
- L-05: DOM wrapper without cppgc GC integration
- L-08: Windows context lifecycle ~9ms (Linux ~4.6ms)
