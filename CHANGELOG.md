# Changelog

All notable changes to iv8-rs are documented here.
This project adheres to [Semantic Versioning](https://semver.org/) and
[Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

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
