# Changelog

All notable changes to iv8-rs are documented here.
This project adheres to [Semantic Versioning](https://semver.org/) and
[Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

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
