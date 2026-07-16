# H04 — Surface Integrity Matrix

> 第四个 quality harness 实例。遵循 `HARNESS-CHARTER.md` 元规范。
> H02/H03 的超集，统一浏览器检测面完整性验证到单一三维矩阵框架。
> 决策依据：D-102 (H04 立项), D-096 (数据源), D-100 (idlharness V8 内运行),
> D-101 (Metamorphic Testing), D-103 (行为探真 D1-D6)。
> Created: 2026-06-26
> Status: candidate (spec defined, phased implementation)

| 字段 | 值 |
|---|---|
| 编号 | H04 |
| 领域 | 浏览器检测面完整性矩阵 (Surface Integrity Matrix) |
| 实现 | `scripts/evaluate_surface_integrity.py` (Phase 1+) |
| 门禁 | `.kiro/steering/surface-integrity-quality-gate.md` |
| 被测模块 | codegen 全部 1284 接口 + ~20 手写 native 接口; `native_env.rs`; `wrap_native.rs`; `env_inject.rs` |
| 前置 harness | H02 (env-consistency), H03 (surface-accuracy) |
| 决策来源 | `docs/roadmap/v0.8/analysis/property-vs-behavior-truth.md` §9 发现 5 |

---

## 0. 摘要

H04 将浏览器检测面验证统一为一个 **三维稀疏矩阵**：

```
Matrix[interface, layer, source] -> Cell{status, evidence[], confidence}
```

- **轴 1 — 接口**: P0/P1/P2/P3 四优先级，~80 个浏览器接口
- **轴 2 — 检测层**: L0-L16 十七层（属性探真，spec-grounded + CreepJS-validated）+ D1-D6 六维度（行为探真）
- **轴 3 — 数据源**: bcd-collector / Chromium IDL / webref / CDP 采样 / CreepJS

矩阵采用**稀疏策略**：高优先级接口做属性级全层多源深查，低优先级接口做接口级单源浅查。
每个 PASS 判定需要 >=2 个独立数据源确认（公正性保证）。

H02 的跨字段一致性检查和 H03 的五层架构作为 H04 的子矩阵被吸收合并。

---

## 1. 命名与定位

### 1.1 编号

- **H04** — Surface Integrity Matrix
- 编号分配遵循 `HARNESS-CHARTER.md` §5（按创建顺序，永不复用）

### 1.2 与 H01/H02/H03 的关系

| Harness | 领域 | 与 H04 的关系 | 合并计划 |
|---|---|---|---|
| H01 | Crypto Detection | **独立** — H01 验证 trace 后处理算法识别，与检测面无关 | 不合并 |
| H02 | Env Consistency | **子集** — H02 的 16 项跨字段一致性检查 = H04 矩阵的 L2 子空间 | Phase 4 合并 |
| H03 | Surface Accuracy | **子集** — H03 的 L1-L5 五层 = H04 矩阵的 L0/L1/L3/L4/L6 行 | Phase 4 合并 |

**H04 是 H02/H03 的超集**：

- H02 覆盖 L2（值一致性）的一个子集（16 项配置一致性规则） -> H04 将其扩展为
  全接口 x L2 x 多源
- H03 覆盖 L0/L1（存在性/值正确性）+ L3/L4（一致性/跨上下文） -> H04 将其纳入
  统一矩阵并增加 L3-L8 全层 + D1-D6 行为维度
- H02 的已知缺陷（D-098: 验证 defaults.rs 非 runtime 输出；B 类假测试；
  C03 单一矛盾场景）在 H04 中通过 runtime 接入和多源确认根治

### 1.3 合并路径

```
Phase 1-3: H04 独立开发，H02/H03 维持运行
Phase 4 (v0.8.85+):
  - H02 evaluate_env_consistency.py -> H04 L2 子矩阵的 runtime 模式
  - H03 evaluate_surface_accuracy.py -> H04 L0/L1 子矩阵的 CDP 模式
  - H02/H03 spec 标记 DEPRECATED，指向 H04
  - H02/H03 gate 合并入 H04 gate
  - H02/H03 不再独立维护，但 spec 保留作为历史参考
```

### 1.4 HARNESS-CHARTER 注册

H04 注册于 `HARNESS-CHARTER.md` §7 Harness 注册表：

| 编号 | 领域 | spec | impl | gate | 状态 |
|---|---|---|---|---|---|
| H01 | Crypto Detection | `H01-crypto-detection.md` | `scripts/evaluate_detection.py` | `crypto-detection-quality-gate.md` | 基线 PASS |
| H02 | Env Consistency | `H02-env-consistency.md` | `scripts/evaluate_env_consistency.py` | `env-consistency-quality-gate.md` | candidate (Phase 4 合并入 H04) |
| H03 | Surface Accuracy | `H03-surface-accuracy.md` | `scripts/evaluate_surface_accuracy.py` | `surface-accuracy-quality-gate.md` | candidate (Phase 4 合并入 H04) |
| H04 | Surface Integrity Matrix | `H04-surface-integrity-matrix.md` | `scripts/evaluate_surface_integrity.py` | `surface-integrity-quality-gate.md` | candidate (Phase 1 待实施) |

---

## 2. 三维矩阵定义

### 2.1 轴 1 — 接口（Interface Axis）

接口按检测重要性分四级。分级依据：检测器实际查询频率 + IV8 实现深度。

| 优先级 | 接口数 | 接口列表 | 验证粒度 |
|---|---|---|---|
| **P0** | 4 | Navigator, Window, Document, Screen | 属性级（每个属性/方法独立单元格） |
| **P1** | 5 | WebGLRenderingContext, WebGL2RenderingContext, CanvasRenderingContext2D, AudioContext, Permissions | 属性级 |
| **P2** | 5 | WorkerGlobalScope, BroadcastChannel, ServiceWorker, Crypto, Storage | 接口级（接口存在 + 关键属性抽检） |
| **P3** | ~66 | HTMLMediaElement, RTCPeerConnection, WebSocket, MediaDevices, FontFace, BatteryManager, ... | 接口级（仅接口存在性） |

> P3 接口完整列表从 bcd-collector 的 `api.*` 记录中程序化生成，不手工维护。

### 2.2 轴 2 — 检测层（Layer Axis）

检测层分两大维度：属性探真（L0-L8）和行为探真（D1-D6）。

#### 2.2.1 属性探真 L0-L13

来源：`detection-surface-taxonomy.md` §4 + Web IDL spec §3.7 (authoritative-data-sources.md Tier 1)。

| 层 | 名称 | 验证内容 | Web IDL spec 条款 | IV8 完成度 |
|---|---|---|---|---|
| L0 | 存在性 | interface object 在 global 上? prototype 存在? 属性/方法在 prototype 上? (`in`, `hasOwnProperty`); **自动化标记存在性: $cdc_/, __playwright/, window.chrome 成员** | §3.7 interface on global; §3.7.3 prototype exists; CreepJS `failed undefined properties`; crawlex.net feature matrix | 95% |
| L1 | 值正确性 | 返回值是正确类型和值吗? (类型 + 值匹配真实 Chrome); **Math 引擎特性值: Math.tan(-1e308), 1/-0, Math.acos(1.0001)**; **navigator.connection.rtt/downlink**; **chrome.loadTimes().firstPaintAfterLoadTime** | §3.2 type mapping; §3.7.6 getter steps; crawlex.net Math quirks + connection probes | 60% |
| L2 | 值一致性 | 值与其他信号一致吗? (UA<->platform<->Client Hints 等跨字段) | (非 spec 层 — 检测器经验) | 40% |
| L3 | 描述符正确性 | attribute accessor: get.name="get X", get.length=0, set.name="set X", set.length=1; operation data: writable=true, enumerable=true, configurable=true, .name=id, .length=shortest overload; constant data: writable=false, enumerable=true, configurable=false; prototype.constructor: writable=true, enumerable=false, configurable=true; **function own keys = ['length','name'] (无 'arguments'/'caller'/'prototype')**; **getter 不应有 'prototype' 属性**; **accessor descriptor.value 必须 undefined**; **Object.getOwnPropertyNames(fn).sort() = 'length,name'**; **Error.stack 描述符: data on instance (V8) vs accessor on prototype (SpiderMonkey)**; **navigator.webdriver 描述符形状** | §3.7.5 const; §3.7.6 attr; §3.7.7 op; §3.7.3 constructor; CreepJS `failed descriptor`/`failed own property`/`failed descriptor keys`/`failed own property names`/`failed own keys names`/`failed prototype in function`; crawlex.net stack descriptor; FP-Scanner webdriver descriptor | 90% |
| L4 | toString 完整性 | `getter.toString()` -> `function get X() { [native code] }`? operation `function X() { [native code] }`? **Function.prototype.toString.call(fn) 精确格式匹配 (6 种变体)**? **toString.toString() 本身也是 native**? **eval.toString().length (Chrome=33, Firefox=37)**? **函数字符串化长度**? | §3.7.6/3.7.7 CreateBuiltinFunction name; ECMA-262 §22.1.3; CreepJS `failed toString` + `hasKnownToString` 6 变体; crawlex.net eval.length; FP-Scanner override detection | 95% |
| L5 | 递归 toString | `Function.prototype.toString.toString()` 递归检测 | ECMA-262 §22.1.3 | 100% |
| L6 | TypeError 行为 | constructor called as function throws? non-constructable called/constructed throws? attribute getter on wrong receiver throws? operation on wrong receiver throws? **Object.create(fn).toString() throws? fn.arguments/fn.caller throws? class extends fn throws? Object.setPrototypeOf(fn, null).toString() throws?** | §3.7.1 constructor behavior; §3.7.6 attr receiver check; §3.7.7 op receiver check; CreepJS `failed illegal error`/`failed call interface`/`failed apply interface`/`failed new instance`/`failed class extends`/`failed null conversion`/`failed object toString error`/`failed at incompatible proxy` | 70% |
| L7 | 原型链 + Proxy 检测 | interface object .__proto__ = parent interface object? prototype .__proto__ = parent prototype? instance .__proto__ = interface prototype? **prototype cycle: setPrototypeOf(fn, Object.create(fn)).toString() throws?** **Proxy 创建后递归检测? Reflect.setPrototypeOf 行为?** | §3.7.1 constructorProto; §3.7.3 proto = parent prototype; CreepJS `failed at too much recursion error` + Proxy detection | 85% |
| L8 | 跨上下文 | Worker vs Window navigator 一致? **iframe vs main: webdriver/platform/WebGL 一致?** **Worker: webdriver/platform/WebGL 一致?** | (非 spec 层 — 实现一致性); FP-Scanner cross-context validation; fp-scanner GitHub iframe/worker checks | 20% |
| L9 | 接口对象属性 | .name = interface id? .length = constructor shortest overload? .prototype = proto object {W:F,E:F,C:F}? constants on interface object {W:F,E:T,C:F}? | §3.7.1 CreateBuiltinFunction(steps, length, id); "prototype" {W:F,E:F,C:F}; §3.7.5 const on interface object | 80% |
| L10 | 命名构造函数 | LegacyFactoryFunction: .name = factory id? .length = shortest overload? .prototype = interface prototype object? exists on global? | §3.7.2 legacy factory function; §3.4.1 [LegacyFactoryFunction] | 0% |
| L11 | 静态操作 | static operation exists on interface object? .name=id? .length=shortest overload? {W:T,E:T,C:T}? | §3.7.7 static operations on interface object; op descriptor {W:T,E:T,C:T} | 0% |
| L12 | Stringifier | "toString" property on prototype? .name="toString"? .length=0? {W:B,E:T,C:B}? returns correct string? | §3.7.8 stringifier; CreateBuiltinFunction(steps, 0, "toString") | 0% |
| L13 | Iterable/Setlike/Maplike | value iterator: Symbol.iterator=Array.prototype.values, entries/keys/values/forEach from Array.prototype? pair iterator: custom entries/keys/values/forEach, .name correct? setlike: size/add/delete/has/entries/keys/values/forEach/clear? maplike: size/get/set/has/delete/entries/keys/values/forEach/clear? | §3.7.9 iterable; §3.7.11 maplike; §3.7.12 setlike | 0% |
| L14 | Stack trace shape | Error.stack 行模式匹配? AT_FUNCTION/AT_OBJECT regex? stack 长度? 错误消息文本? **Error.stack 描述符: V8 data on instance vs SpiderMonkey accessor on prototype**? **Error.stackTraceLimit 存在性 (V8-only)**? | CreepJS `src/lies/index.ts` hasValidStack; crawlex.net stack format taxonomy (V8 "at" vs SM "@" vs JSC "global code"); TC39 Error Stacks proposal (impl-defined format) | 0% |
| L15 | Enumeration order | Object.keys() 顺序? for-in 顺序? Reflect.ownKeys() 顺序? 与 Chrome 一致? | ECMA-262 §6.1.7.1 OrdinaryOwnPropertyKeys; CreepJS enumeration order checks | 0% |
| L16 | Timing resolution | performance.now() 精度? Date.now() 末位裁剪? requestAnimationFrame 时序? | CreepJS `src/resistance/index.ts`; W3C fingerprinting-guidance §3.2 | 0% |

#### 2.2.2 行为探真 D1-D6

来源：`property-vs-behavior-truth.md` §9.2 发现 3 (D-103)。

| 维度 | 名称 | 验证内容 | 当前覆盖 |
|---|---|---|---|
| D1 | 方法返回值语义 | 返回值类型/形状正确（`clipboard.read()` -> Promise，不是 undefined） | 无 |
| D2 | Promise 语义 | resolve/reject 时机和值正确 | 无 |
| D3 | 事件触发时序 | addEventListener -> dispatch -> callback 链完整 | 无 |
| D4 | 状态转换 | WebSocket/readyState 状态机正确转换 | 无 |
| D5 | 异常行为 | TypeError/NotSupportedError/DOMException 类型正确 | CreepJS 部分 |
| D6 | 异步排序 | microtask vs macrotask 排序正确 | 无 |

#### 2.2.3 检测层与 Web IDL spec 映射

来源：`docs/conventions/authoritative-data-sources.md` Tier 1。多源验证：Web IDL spec (whatwg) + Chromium Blink-V8 bindings + WPT idlharness.js。

| 层 | Web IDL spec 条款 | 关键规范要求 | idlharness 测试对应 | IV8 codegen 实现 |
|---|---|---|---|---|
| L0 | §3.7 interface on global; §3.7.3 prototype exists | interface object 作为 global 属性存在; prototype object 存在 | "existence and properties of interface object" | install_all global registration |
| L3 | §3.7.5 const {W:F,E:T,C:F}; §3.7.6 attr getter/setter; §3.7.7 op {W:T,E:T,C:T}; §3.7.3 constructor {W:T,E:F,C:T} | const: writable=false, enumerable=true, configurable=false; attr getter: name="get X", length=0; attr setter: name="set X", length=1; op: writable=true, enumerable=true, configurable=true, name=id, length=shortest overload; constructor: writable=true, enumerable=false, configurable=true | "attribute descriptor", "getter must have name", "setter length must be 1", "constant writable", "property should be writable" | codegen set_class_name + .length() + set_with_attr + chain_dom_prototypes writable |
| L6 | §3.7.1 constructor behavior; §3.7.6 attr receiver check; §3.7.7 op receiver check | constructor called as function throws TypeError; non-constructable throws; wrong receiver throws TypeError | "Illegal invocation", "assert_throws" | codegen prototype_chain_check + illegal_constructor + construct_only |
| L7 | §3.7.1 constructorProto = parent interface object; §3.7.3 proto = parent prototype | interface object .__proto__ = parent interface object; prototype .__proto__ = parent prototype | "prototype of X is not Y" | tmpl.inherit() + manual __proto__ fix (CTOR_INHERITANCE + codegen install_all) |
| L9 | §3.7.1 CreateBuiltinFunction(steps, length, id); "prototype" {W:F,E:F,C:F}; §3.7.5 const on interface object | .name = interface id; .length = constructor shortest overload; .prototype = proto {writable:false, enumerable:false, configurable:false}; constants on interface object {writable:false, enumerable:true, configurable:false} | "interface object length", "interface object name", "constant on interface object" | codegen set_class_name + constructor_arg_count + read_only_prototype + const define_own_property |
| L10 | §3.7.2 legacy factory function; §3.4.1 [LegacyFactoryFunction] | .name = factory id (e.g. "Image"); .length = shortest overload; .prototype = interface prototype object; Audio.prototype === HTMLAudioElement.prototype | "named constructor name", "named constructor prototype property" | codegen NamedConstructor alias (incomplete — needs .name + .prototype fix) |
| L11 | §3.7.7 static operations on interface object; op descriptor {W:T,E:T,C:T} | static operation on interface object (not prototype); .name=id; .length=shortest overload; writable=true, enumerable=true, configurable=true | "static operation" | codegen (not implemented — IR has static flag but codegen ignores) |
| L12 | §3.7.8 stringifier; CreateBuiltinFunction(steps, 0, "toString") | "toString" property on prototype; .name="toString"; .length=0; {writable:B, enumerable:true, configurable:B}; returns correct string | "stringifier" | codegen (not implemented — IR stringifier detection needed) |
| L13 | §3.7.9 iterable; §3.7.11 maplike; §3.7.12 setlike | value iterator: Symbol.iterator=Array.prototype.values, entries/keys/values/forEach from Array.prototype; pair iterator: custom entries/keys/values/forEach, name="entries"/"keys"/"values", length=0; setlike: size/add/delete/has/entries/keys/values/forEach/clear/Symbol.iterator; maplike: size/get/set/has/delete/entries/keys/values/forEach/clear/Symbol.iterator | "iterable", "setlike", "entries equality" | codegen (not implemented — IR iterable/setlike detection needed) |

### 2.3 轴 3 — 数据源（Source Axis）

来源：`property-vs-behavior-truth.md` §9.1 数据源可信度矩阵 (D-096 第二轮修正)。

| 编号 | 数据源 | 评分 | 角色 | 覆盖层 | 更新频率 | 获取方式 |
|---|---|---|---|---|---|---|
| S1 | bcd-collector results | 9.5/10 | **实现真相主数据源** | L0, L1 | 每次 Chrome 大版本 | GitHub raw URL 下载 JSON |
| S2 | Chromium Blink IDL | 8.5/10 | 实现真相补充（含 [RuntimeEnabled]） | L0, L3 | 随 Chrome (6wk) | sparse checkout ~100MB |
| S3 | @webref/idl | 5/10 | 规范对照集（"应该有什么"） | L0 | 6h | NPM `@webref/idl` |
| S4 | CDP 采样 | — | 值正确性金标准 | L1, L3, L4 | 按需 | 纯 Python CDP |
| S5 | CreepJS lies | 9/10 | 行为探真规则集 | L4-L7, D5 | 活跃 | `src/lies/index.ts` |
| S6 | FP-Inconsistent 规则 | 6/10 | 一致性规则集（逻辑自行实现） | L2 | 论文(2025-01) | 提取规则，不用原始代码 |
| S7 | runtime_enabled_features.json5 | — | [RuntimeEnabled] flag 真相 | L0 (过滤) | 随 Chrome | Chromium sparse checkout 同源 |
| S8 | WPT idlharness.js | 9/10 | 描述符 + 构造函数行为 | L0, L3, L6 | 随 WPT | V8 内 shim 原版运行 (D-100) |

> S6 (FP-Inconsistent) 无 license，仅提取规则逻辑自行实现，不引用原始代码。
>
> S8 (idlharness) Phase 1 已实现 WPT 官方测试文件直接复用
> (run_wpt.py, 3 suites × 6 variants, 3758 tests, 3213 PASS 85.50%)。
> Chrome 151 基线: 9481/9640 (98.35%), wpt.fyi API (2026-06-30, run id 5155506334466048)。
> 旧 runner (run_idlharness.py, 10222 tests) 已废弃。
> R4 修复: const 描述符 + accessor .name/.length + configurable
>   + dom callback receiver check + shim .name + dom method .length
> 参见 `docs/roadmap/v0.8/analysis/wpt-integration-design.md` §12。

### 2.4 矩阵规模

```
P0: 4 接口 x ~500 属性 x 20 层(L0-L13+D1-D6) x 8 源 = ~320,000 单元格（稀疏填充）
P1: 5 接口 x ~200 属性 x 5 层(L0/L1/L3/L9/L13) x 2 源 = ~10,000 单元格
P2: 5 接口 x 1 接口级  x 2 层  x 1 源 = ~10 单元格
P3: ~66 接口 x 1 接口级 x 1 层  x 1 源 = ~66 单元格
```

实际有效单元格远小于理论上限（稀疏矩阵），多数为 N/A。

---

## 3. 稀疏矩阵策略

不同优先级的接口使用不同的填充密度策略，平衡验证深度与计算成本。

### 3.1 P0 — 属性级 x 全层 x 多源

```
粒度: 每个属性/方法独立单元格
层数: L0-L13 全部 + D1-D6 全部
源数: >=3 源交叉确认（S1+S2+S4 为主，S3/S5/S8 补充）
适用: Navigator, Window, Document, Screen
```

P0 是检测器查询最频繁的接口，必须做最深验证。每个属性在每个层都有独立单元格。

### 3.2 P1 — 属性级 x L0/L1/L3/L9/L13 x 双源

```
粒度: 每个属性/方法独立单元格
层数: L0 (存在性) + L1 (值正确性) + L3 (描述符) + L9 (接口对象) + L13 (iterable)
源数: >=2 源确认（S1+S4 或 S1+S2）
适用: WebGLRenderingContext, WebGL2RenderingContext,
      CanvasRenderingContext2D, AudioContext, Permissions
```

P1 是设备指纹核心接口，验证属性级的存在性/值/描述符，但不验证 L4-L8 高阶层。

### 3.3 P2 — 接口级 x L0/L1 x 单源

```
粒度: 接口级（接口存在 + 3-5 个关键属性抽检）
层数: L0 (存在性) + L1 (关键属性值)
源数: 1 源（S1 bcd-collector）
适用: WorkerGlobalScope, BroadcastChannel, ServiceWorker, Crypto, Storage
```

P2 接口检测器查询频率低，验证接口存在性和少量关键属性即可。

### 3.4 P3 — 接口级 x L0 x webref

```
粒度: 接口级（仅接口构造函数存在性）
层数: L0 (存在性)
源数: 1 源（S3 webref 规范对照）
适用: 其余 ~66 个接口
```

P3 仅验证接口"存在"，不验证深度。作为 coverage 基线，不是 PASS/FAIL 门禁。

### 3.5 稀疏策略汇总

| 优先级 | 粒度 | 层数 | 源数 | 门禁参与 | 预估单元格 |
|---|---|---|---|---|---|
| P0 | 属性级 | L0-L8 + D1-D6 (15) | >=3 | 是 (A-E 全类) | ~240,000 |
| P1 | 属性级 | L0/L1/L3 (3) | >=2 | 是 (A-D 类) | ~6,000 |
| P2 | 接口级 | L0/L1 (2) | 1 | 是 (A-B 类) | ~10 |
| P3 | 接口级 | L0 (1) | 1 | 否 (仅统计) | ~66 |

---

## 4. 单元格规范

### 4.1 单元格 JSON Schema

每个矩阵单元格是一个 JSON 对象：

```json
{
  "$schema": "h04-cell-v1",
  "interface": "Navigator",
  "priority": "P0",
  "property": "userAgent",
  "layer": "L1",
  "sources": ["S1:bcd-collector", "S4:CDP-sampling"],
  "status": "PASS",
  "confidence": 1.0,
  "evidence": [
    {
      "source": "S1:bcd-collector",
      "key": "api.Navigator.userAgent",
      "chrome_version": "148",
      "value_type": "string",
      "iv8_value": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ...",
      "golden_value": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ...",
      "match": true,
      "timestamp": "2026-06-26T12:00:00Z"
    },
    {
      "source": "S4:CDP-sampling",
      "chrome_version": "148",
      "cdp_expression": "navigator.userAgent",
      "iv8_expression": "navigator.userAgent",
      "match": true,
      "timestamp": "2026-06-26T12:05:00Z"
    }
  ],
  "source_count": 2,
  "required_sources": 2,
  "notes": ""
}
```

### 4.2 状态枚举

| 状态 | 含义 | CI 影响 | 颜色（热力图） |
|---|---|---|---|
| `PASS` | >=2 独立源确认值一致 | 无 | 绿色 |
| `FAIL` | >=2 源确认值不一致，或应存在的属性不存在 | **阻断** | 红色 |
| `N/A` | 该单元格不适用（如 P3 接口的 L4 层） | 无 | 灰色 |
| `CONFLICT` | 多源给出矛盾结果（源间不一致） | **阻断** | 橙色 |
| `PENDING` | 数据源未收集或验证未实现 | 无（但记录） | 黄色 |
| `BLOCKED` | 依赖项未就绪（如 L8 依赖 Worker 实现） | 无（但记录） | 蓝色 |

### 4.3 状态判定逻辑

```
IF source_count == 0:
    status = PENDING
ELIF any source reports "should exist but doesn't":
    status = FAIL
ELIF source_count >= required_sources AND all sources agree:
    status = PASS
ELIF source_count >= 2 AND sources disagree:
    status = CONFLICT
ELIF dependency_not_ready:
    status = BLOCKED
ELSE:
    status = PENDING
```

### 4.4 证据要求

每个非 N/A 单元格必须包含以下证据字段：

| 字段 | 必填 | 说明 |
|---|---|---|
| `sources` | 是 | 参与确认的数据源列表 |
| `source_count` | 是 | 实际提供证据的源数量 |
| `required_sources` | 是 | 该优先级+层要求的最低源数 |
| `evidence[].source` | 是 | 数据源标识 |
| `evidence[].match` | 是 | 该源是否确认匹配 (true/false) |
| `evidence[].timestamp` | 是 | 证据采集时间 (ISO 8601) |
| `evidence[].chrome_version` | 是（S1/S2/S4） | 对标的 Chrome 版本 |
| `confidence` | 是 | 置信度 0.0-1.0（源数/要求源数，封顶 1.0） |
| `notes` | 否 | 异常说明 |

---

## 5. 公正性保证

### 5.1 多源确认规则

**核心原则：任何 PASS 判定必须由 >=2 个独立数据源确认。**（宪法 P6 自审性的延伸）

| 优先级 | 层 | 要求最低源数 | 允许的源组合 |
|---|---|---|---|
| P0 | L0 | 3 | S1+S2+S3 或 S1+S2+S8 |
| P0 | L1 | 2 | S1+S4 或 S2+S4 |
| P0 | L2 | 2 | S6(规则)+S4(runtime采样) 或 S6+S5 |
| P0 | L3 | 2 | S4(getOwnPropertyDescriptor)+S8(idlharness) |
| P0 | L4-L5 | 2 | S5(CreepJS)+S4(toString采样) |
| P0 | L6 | 2 | S8(idlharness)+S4(构造函数采样) |
| P0 | L7 | 1 | S5(CreepJS prototype test) |
| P0 | L8 | 1 | S4(Worker scope 采样) |
| P0 | D1-D6 | 2 | S5(CreepJS)+自建行为测试 (D-101 Metamorphic) |
| P1 | L0 | 2 | S1+S2 或 S1+S4 |
| P1 | L1 | 2 | S1+S4 |
| P1 | L3 | 2 | S4+S8 |
| P2 | L0/L1 | 1 | S1 |
| P3 | L0 | 1 | S3 |

> P0 L7 和 L8 因验证方法单一（CreepJS / Worker 采样），允许单源 PASS，
> 但在报告中标注为 `confidence < 1.0` 以示降级置信。

### 5.2 冲突处理

当 >=2 个源对同一单元格给出矛盾结果时：

1. 标记为 `CONFLICT`（不是 FAIL — FAIL 是确认"错"，CONFLICT 是"不确定谁对"）
2. CONFLICT 阻断 CI，必须在报告中列出每个源的值供人工裁定
3. 裁定后：如确认 IV8 错 -> 改为 `FAIL`；如确认某源过时 -> 更新该源数据并改为 `PASS`
4. 不允许通过"删除冲突源"来降级为单源 PASS（宪法 P3 阈值显式化）

### 5.3 避免自验证

**禁止用 IV8 自身的输出作为数据源。** 具体措施：

- S4 (CDP 采样) 的 golden 值必须来自真实 Chrome（通过 CDP 协议采集），
  **不能**用 IV8 的 `sample_iv8_surface.py` 输出替代
- S1 (bcd-collector) 的数据是第三方在真实浏览器上运行的结果
- S5 (CreepJS) 的 lie 检测在 IV8 V8 isolate 内运行，但判定逻辑来自 CreepJS
  原版代码（非 IV8 自写），视为独立源
- H02 的 D-098 缺陷（验证 defaults.rs 非 runtime 输出）在 H04 中通过以下方式根治：
  - L2 单元格的 S4 证据必须来自 IV8 runtime 实际值（通过 `IV8.eval()` 采集），
    不能来自 `defaults.rs` 配置文件

---

## 6. 评估类别（A-E）

遵循 `HARNESS-CHARTER.md` §3 评估类别模板。A、C 为强制（宪法规定）。
**总体 PASS 当且仅当全部参与类别 PASS**（宪法 P4）。

### A. Data Integrity（数据完整性）— 强制

**问题**：golden 数据源本身有没有错误？

| 指标 | 定义 | 阈值 |
|---|---|---|
| `golden_errors` | golden 数据源（S1/S2/S3）内部一致性错误数 | **= 0** |
| `golden_freshness_days` | 最旧 golden 数据源的采集天数 | **<= 42** (6wk Chrome 周期) |

验证内容：
- S1 (bcd-collector): JSON 可解析、BCD key 格式合法、Chrome 版本号一致
- S2 (Chromium IDL): IDL 可被 webidl2.js 解析、继承链无缺失
- S3 (webref): IDL 可解析、与 S2 无结构性矛盾（同名接口成员数差异 < 5%）
- S4 (CDP golden): 采样 JSON 字段完整、无 null 值（除非 Chrome 也返回 null）

金标准来源（宪法 P10）：bcd-collector (实测)、Chromium 源码 (实现真相)、
w3c/webref (规范)、WPT idlharness.js (W3C 官方验证工具)。

### B. Coverage Recall（覆盖召回率）

**问题**：IV8 codegen 覆盖了 golden 数据源定义的全部接口/属性吗？

| 指标 | 定义 | 阈值 |
|---|---|---|
| `p0_coverage_pct` | P0 接口属性在 IV8 中的存在率 (L0) | **>= 95%** |
| `p1_coverage_pct` | P1 接口属性在 IV8 中的存在率 (L0) | **>= 90%** |
| `p2_coverage_pct` | P2 接口在 IV8 中的存在率 (L0 接口级) | **>= 100%** |
| `p3_coverage_pct` | P3 接口在 IV8 中的存在率 (L0 接口级) | **>= 80%** |

验证方法：解析 S1 (bcd-collector) 的 `api.*` 记录 -> 提取接口+属性列表 ->
对照 IV8 codegen 的 `install_*` 函数 -> 计算覆盖率。

> P0 阈值 95% 而非 100%：允许少量 [RuntimeEnabled] flag 关闭的属性不覆盖
> （由 S7 runtime_enabled_features.json5 判定）。

### C. False Positive Resistance（假阳性抵抗）— 强制

**问题**：Chrome 自测会不会误报？（IV8 的实现是否在真实 Chrome 上也 PASS）

| 指标 | 定义 | 阈值 |
|---|---|---|
| `chrome_self_test_pass_pct` | 将 IV8 验证逻辑跑在真实 Chrome 上的 PASS 率 | **= 100%** |
| `creepjs_lies_on_chrome` | CreepJS 在真实 Chrome 上报告的 lie 数 | **= 0** |

验证方法：
1. 在真实 Chrome（通过 CDP）运行与 IV8 相同的验证表达式
2. 所有在 Chrome 上应为 PASS 的单元格，在 Chrome 上确实 PASS
3. 如果某单元格在 Chrome 上 FAIL -> 说明验证逻辑有 bug（harness 自身缺陷，P6）

> 这是宪法 P2（证伪优先）的体现：不只测"IV8 能 PASS"，还测"Chrome 也能 PASS"。
> 如果 Chrome 都过不了，说明验证逻辑本身错了。

### D. Multi-Source Consistency（多源一致性）

**问题**：不同数据源之间是否一致？

| 指标 | 定义 | 阈值 |
|---|---|---|
| `dual_source_pass_rate` | 双源确认的 PASS 单元格 / 全部 PASS 单元格 | **>= 80%** |
| `conflict_count` | CONFLICT 状态的单元格数 | **= 0** |
| `single_source_pass_count` | 仅单源确认的 PASS 单元格数 | 记录（不阻断） |

> 80% 阈值而非 100%：L7/L8 和部分 P0 D1-D6 维度允许单源 PASS（见 §5.1）。
> 但 conflict_count 必须为 0 — 冲突不可接受。

### E. Robustness（鲁棒性）

**问题**：重复采样结果确定吗？

| 指标 | 定义 | 阈值 |
|---|---|---|
| `determinism_pct` | 同一 IV8 实例两次采样结果一致率 | **= 100%** |
| `cdp_reproducibility_pct` | CDP 两次采样 Chrome 结果一致率 | **>= 99%** |
| `metamorphic_invariant_pass` | Metamorphic Testing 蜕变关系满足率 (D-101) | **= 100%** |

验证方法：
- `determinism_pct`: 固定随机种子，跑两次 `sample_iv8_surface.py`，diff 结果
- `metamorphic_invariant_pass`: 蜕变关系示例 —
  "同一 profile 下 `navigator.platform` 和 `navigator.userAgentData.platform` 必须一致"
  （不需要 golden 值，只需验证关系成立，D-101）

---

## 7. 阈值定义与 CI Gate

### 7.1 阈值集中定义

所有阈值在 `scripts/evaluate_surface_integrity.py` 顶部 `THRESHOLDS` 字典中集中定义
（宪法 P3）。修改阈值必须在 commit body 写明理由。

```python
THRESHOLDS = {
    # A: Data Integrity
    "golden_errors": 0,
    "golden_freshness_days": 42,
    # B: Coverage Recall
    "p0_coverage_pct": 95.0,
    "p1_coverage_pct": 90.0,
    "p2_coverage_pct": 100.0,
    "p3_coverage_pct": 80.0,
    # C: False Positive Resistance
    "chrome_self_test_pass_pct": 100.0,
    "creepjs_lies_on_chrome": 0,
    # D: Multi-Source Consistency
    "dual_source_pass_rate": 80.0,
    "conflict_count": 0,
    # E: Robustness
    "determinism_pct": 100.0,
    "cdp_reproducibility_pct": 99.0,
    "metamorphic_invariant_pass": 100.0,
}
```

### 7.2 CI Gate 退出码逻辑

```
exit_code = 0

IF golden_errors > 0:                          exit_code |= 0x01  # A fail
IF golden_freshness_days > 42:                 exit_code |= 0x01
IF p0_coverage_pct < 95.0:                     exit_code |= 0x02  # B fail
IF p1_coverage_pct < 90.0:                     exit_code |= 0x02
IF p2_coverage_pct < 100.0:                    exit_code |= 0x02
IF p3_coverage_pct < 80.0:                     exit_code |= 0x02
IF chrome_self_test_pass_pct < 100.0:          exit_code |= 0x04  # C fail
IF creepjs_lies_on_chrome > 0:                 exit_code |= 0x04
IF dual_source_pass_rate < 80.0:               exit_code |= 0x08  # D fail
IF conflict_count > 0:                         exit_code |= 0x08
IF determinism_pct < 100.0:                    exit_code |= 0x10  # E fail
IF cdp_reproducibility_pct < 99.0:             exit_code |= 0x10
IF metamorphic_invariant_pass < 100.0:         exit_code |= 0x10

# exit_code 0 = OVERALL PASS
# 非零值的 bit 位指示哪些类别失败
```

退出码位映射：

| Bit | 类别 | 值 |
|---|---|---|
| 0x01 | A (Data Integrity) | 1 |
| 0x02 | B (Coverage Recall) | 2 |
| 0x04 | C (False Positive) | 4 |
| 0x08 | D (Multi-Source) | 8 |
| 0x10 | E (Robustness) | 16 |

CI 配置：`exit_code != 0` -> 阻断合并。

### 7.3 运行方式

```bash
# 完整评估（声称完成前必跑，宪法 P7）
python scripts/evaluate_surface_integrity.py
echo $?   # 0 = PASS, 非零 = FAIL（bit 位指示类别）

# 按优先级过滤
python scripts/evaluate_surface_integrity.py --priority P0
python scripts/evaluate_surface_integrity.py --priority P0,P1

# 按层过滤
python scripts/evaluate_surface_integrity.py --layers L0,L1,L3

# 仅生成报告不阻断
python scripts/evaluate_surface_integrity.py --report-only

# 诊断：查看特定单元格
python scripts/evaluate_surface_integrity.py --cell Navigator.userAgent.L1
```

---

## 8. 更新策略

### 8.1 数据源更新频率

| 数据源 | 更新触发 | 频率 | 自动化 | 版本管理 |
|---|---|---|---|---|
| S1 bcd-collector | Chrome 新版本发布 | ~6 周 | 自动下载 | 文件名含 Chrome 版本号 |
| S2 Chromium IDL | Chrome 新版本发布 | ~6 周 | sparse checkout by tag | git tag |
| S3 @webref/idl | 规范更新 | 6 小时 | NPM 拉取 | NPM 版本号 |
| S4 CDP 采样 | 手动触发 / Chrome 大版本 | 按需 | private CDP sampler (not public keep) | golden/chrome{ver}_{os}_{gpu}.json |
| S5 CreepJS | 仓库更新 | 活跃 | git submodule | git commit hash |
| S6 FP-Inconsistent 规则 | 一次性提取 | 论文(2025-01) | 手动提取逻辑 | 固定规则集 |
| S7 runtime_enabled_features.json5 | Chrome 新版本发布 | ~6 周 | 随 S2 sparse checkout | git tag |
| S8 WPT idlharness.js | WPT 更新 | 随 WPT | git submodule | git commit hash |

### 8.2 金标准版本管理

```
golden/
  bcd-collector/
    chrome-148-windows.json          # S1: Chrome 148 Win BCD
    chrome-148-windows.timestamp     # 采集时间戳
  chromium-idl/
    chrome-148/                      # S2+S7: sparse checkout by tag
      core_idl_files/
      modules_idl_files/
      runtime_enabled_features.json5
  webref/
    @webref/idl@x.y.z/               # S3: NPM 版本
  cdp-sampling/
    chrome148_win10_rtx4060.json     # S4: CDP 采样金标准
    chrome148_win10_rtx4060.meta     # 采样元数据（Chrome 路径/flags/时间）
  creepjs/
    creepjs@<commit-hash>/           # S5: CreepJS submodule
  wpt/
    wpt@<commit-hash>/               # S8: WPT submodule
      resources/idlharness.js
      resources/testharness.js
```

### 8.3 Codegen 变更触发

每次 commit 中 codegen 相关文件变更时（通过 gate 文件触发）：

1. 重新运行 L0 覆盖率审计（`scripts/idl_coverage_audit.py`）
2. 重新运行受影响优先级的单元格验证
3. 输出 scorecard，与基线对比
4. 如覆盖率下降 -> 阻断 commit

受触发文件列表（在 gate 文件中定义）：
- `src/codegen/**/*.rs`
- `src/native_*.rs`
- `src/wrap_native.rs`
- `src/env_inject.rs`
- `codegen/templates/**/*`

---

## 9. 报告格式

H04 输出三种报告格式，面向不同消费者。

### 9.1 热力图（概览）

面向人类快速浏览。以 HTML 表格渲染，颜色映射见 §4.2。

```
              L0    L1    L2    L3    L4    L5    L6    L7    L8   D1-D6
Navigator    [###] [###] [## ] [###] [###] [###] [## ] [## ] [   ] [   ]
Window       [###] [###] [## ] [###] [###] [###] [## ] [## ] [   ] [   ]
Document     [###] [## ] [## ] [###] [###] [###] [## ] [## ] [   ] [   ]
Screen       [###] [###] [## ] [###] [###] [###] [## ] [## ] [   ] [   ]
WebGL1       [###] [## ] [   ] [## ] [   ] [   ] [   ] [   ] [   ] [   ]
WebGL2       [## ] [## ] [   ] [## ] [   ] [   ] [   ] [   ] [   ] [   ]
Canvas2D     [###] [## ] [   ] [## ] [   ] [   ] [   ] [   ] [   ] [   ]
AudioCtx     [## ] [## ] [   ] [## ] [   ] [   ] [   ] [   ] [   ] [   ]
Permissions  [## ] [## ] [   ] [## ] [   ] [   ] [   ] [   ] [   ] [   ]
WorkerGS     [## ] [   ] [   ] [   ] [   ] [   ] [   ] [   ] [   ] [   ]
...P3...     [## ] [   ] [   ] [   ] [   ] [   ] [   ] [   ] [   ] [   ]

图例: [###] PASS(>=2源)  [## ] PASS(单源)  [   ] PENDING/N/A
      [!! ] FAIL          [?? ] CONFLICT    [~~ ] BLOCKED
```

每个格子内显示 `pass_count / total_count`，鼠标悬停显示详细单元格。
报告底部附 scorecard 汇总。

### 9.2 表格（详细 FAIL/CONFLICT）

面向开发者定位问题。仅输出非 PASS 单元格。

```
+-----------+------------+------+---------+----------+----------------------------------+
| Interface | Property   | Layer| Status  | Sources  | Detail                           |
+-----------+------------+------+---------+----------+----------------------------------+
| Navigator | bluetooth  | L0   | FAIL    | S1,S2    | S1: api.Navigator.bluetooth     |
|           |            |      |         |          |   exists in Chrome 148           |
|           |            |      |         |          | S2: exists in Blink IDL          |
|           |            |      |         |          | IV8: NOT in Navigator.prototype  |
+-----------+------------+------+---------+----------+----------------------------------+
| Navigator | userAgent  | L1   | CONFLICT| S1,S4    | S1: "Mozilla/5.0 ... Chrome/148" |
|           |            |      |         |          | S4: "Mozilla/5.0 ... Chrome/147" |
|           |            |      |         |          | Source version mismatch           |
+-----------+------------+------+---------+----------+----------------------------------+
| Window    | crypto     | L8   | BLOCKED | --       | Dependency: Worker not impl.     |
+-----------+------------+------+---------+----------+----------------------------------+
```

### 9.3 JSON（CI 消费）

面向 CI 和自动化工具。完整矩阵的机器可读格式。

```json
{
  "$schema": "h04-report-v1",
  "timestamp": "2026-06-26T12:00:00Z",
  "iv8_version": "0.8.82",
  "chrome_version": "148",
  "scorecard": {
    "A": {"status": "PASS", "golden_errors": 0, "golden_freshness_days": 7},
    "B": {"status": "FAIL", "p0_coverage_pct": 92.3, "p1_coverage_pct": 88.1,
          "p2_coverage_pct": 100.0, "p3_coverage_pct": 76.5},
    "C": {"status": "PASS", "chrome_self_test_pass_pct": 100.0,
          "creepjs_lies_on_chrome": 0},
    "D": {"status": "PASS", "dual_source_pass_rate": 84.2,
          "conflict_count": 0, "single_source_pass_count": 47},
    "E": {"status": "PASS", "determinism_pct": 100.0,
          "cdp_reproducibility_pct": 99.8,
          "metamorphic_invariant_pass": 100.0}
  },
  "overall": "FAIL",
  "exit_code": 2,
  "summary": {
    "total_cells": 15234,
    "pass": 13801,
    "fail": 23,
    "conflict": 0,
    "pending": 1410,
    "blocked": 0,
    "na": 0
  },
  "cells": [
    {
      "interface": "Navigator",
      "priority": "P0",
      "property": "bluetooth",
      "layer": "L0",
      "status": "FAIL",
      "sources": ["S1:bcd-collector", "S2:chromium-idl"],
      "evidence": ["..."]
    }
  ]
}
```

CI 从 `overall` 和 `exit_code` 字段判定通过/阻断。

---

## 10. 实施路径

### Phase 1 (v0.8.82): L0 全量验证

**目标**：P0-P3 全接口的 L0 存在性验证，建立矩阵骨架。

| 任务 | 产出 | 依赖 |
|---|---|---|
| 集成 bcd-collector 数据下载 | `scripts/fetch_bcd_collector.py` | S1 |
| 集成 Chromium Blink IDL sparse checkout | `scripts/fetch_chromium_idl.py` | S2, S7 |
| 集成 @webref/idl NPM 拉取 | `scripts/fetch_webref.py` | S3 |
| 编写 IDL 覆盖率审计脚本 | `scripts/idl_coverage_audit.py` | S1+S2+S3 |
| 编写 H04 orchestrator 骨架 | `scripts/evaluate_surface_integrity.py` | 上述脚本 |
| 实现 L0 单元格填充逻辑 | P0-P3 x L0 单元格 | orchestrator |
| 实现 A/B 类评估 | golden_errors + coverage_pct | L0 单元格 |
| 生成首份热力图报告 | HTML + JSON | 上述全部 |

**Phase 1 退出标准**：
- A 类 PASS (golden_errors = 0)
- B 类 P0 coverage >= 95%
- JSON 报告可被 CI 消费
- 缺失属性完整列表产出

### Phase 2 (v0.8.83): L1+L3 全量验证

**目标**：P0/P1 接口的 L1 值正确性和 L3 描述符正确性验证。

| 任务 | 产出 | 依赖 |
|---|---|---|
| 扩展 Chrome CDP 采样脚本 | private CDP sampler (not public keep) | S4 (Chrome 环境) |
| 扩展 IV8 采样脚本 | `scripts/sample_iv8_surface.py` (升级) | IV8 runtime |
| 编写值 diff 脚本 | `scripts/value_diff.py` | CDP golden + IV8 sample |
| 编写描述符 diff 脚本 | `scripts/descriptor_diff.py` | CDP golden + IV8 sample |
| 集成 WPT idlharness.js (V8 内 shim) | `scripts/run_idlharness.py` | S8 (D-100) |
| 实现 L1/L3 单元格填充 | P0/P1 x L1/L3 单元格 | 上述脚本 |
| 实现 C 类评估 (Chrome 自测) | chrome_self_test_pass_pct | CDP 采样反向验证 |
| 实现 D 类评估 (多源一致性) | dual_source_pass_rate | L0+L1+L3 单元格 |

**Phase 2 退出标准**：
- C 类 PASS (chrome_self_test = 100%)
- D 类 PASS (conflict_count = 0, dual_source >= 80%)
- H02 D-098 缺陷修复验证（L2 runtime 接入试运行）

### Phase 3 (v0.8.84): L2/L6/L8 + D1-D6

**目标**：行为探真验证，填补 H03 L3/L4 未实现部分。

| 任务 | 产出 | 依赖 |
|---|---|---|
| 实现 FP-Inconsistent 规则集（逻辑自写） | `scripts/fp_inconsistent_rules.py` | S6 (规则提取) |
| 实现 L2 一致性单元格填充 | P0 x L2 单元格 | FP规则 + runtime 采样 |
| 集成 CreepJS lies 检测 | `scripts/run_creepjs_lies.py` | S5 (CreepJS submodule) |
| 实现 L4-L7 单元格填充 | P0 x L4-L7 单元格 | CreepJS + toString 采样 |
| 实现 L6 TypeError 行为验证 | P0 x L6 单元格 | S8 (idlharness constructor) |
| 实现 Worker 跨上下文采样 | `scripts/sample_worker_surface.py` | IV8 Worker 支持 (L8) |
| 实现 L8 单元格填充 | P0 x L8 单元格 (部分 BLOCKED 可接受) | Worker 采样 |
| 实现 D1-D6 行为测试框架 | `scripts/behavior_tests/` | D-101 Metamorphic |
| 实现 E 类评估 (鲁棒性) | determinism + metamorphic | 上述全部 |

**Phase 3 退出标准**：
- E 类 PASS (determinism = 100%, metamorphic = 100%)
- L2 单元格 P0 覆盖率 >= 80%
- L4-L7 单元格 P0 覆盖率 >= 90%
- L8 允许 BLOCKED（Worker 未实现部分）

### Phase 4 (v0.8.85+): 诊断层 + CI 集成 + H02/H03 合并

**目标**：全量 CI 集成，合并 H02/H03，诊断工具上线。

| 任务 | 产出 | 依赖 |
|---|---|---|
| 实现 V8 NamedPropertyHandlerConfiguration miss counter | Rust 侧 `missing_properties` tracker | D-097 方案 B+D |
| 集成 miss counter 到 H04 诊断报告 | `--diagnose-miss` 模式 | miss counter |
| 跑 RS VMP / DataDome / Akamai 样本收集 miss 数据 | `golden/miss-tracking/*.log` | 样本环境 |
| H02 合并：L2 runtime 模式替代 defaults.rs 模式 | H02 spec DEPRECATED 标记 | L2 单元格稳定 |
| H03 合并：L0/L1 CDP 模式替代独立脚本 | H03 spec DEPRECATED 标记 | L0/L1 单元格稳定 |
| H02/H03 gate 合并入 H04 gate | `.kiro/steering/surface-integrity-quality-gate.md` | 合并完成 |
| CI 集成：codegen 变更自动触发 H04 | CI pipeline 配置 | 全部类别 PASS |
| Mutation Testing 审计 (D-105) | `cargo-mutants` 周期审计 | E 类 PASS |

**Phase 4 退出标准**：
- H02/H03 标记 DEPRECATED，H04 成为唯一 surface 质量门禁
- CI 自动触发 H04 全量运行
- miss counter 诊断报告产出
- 全部 A-E 类别 PASS

---

## 11. 已知局限（诚实记录，宪法 P8）

### 11.1 Canvas/WebGL 像素级伪造

IV8 无真实 GPU，Canvas 2D 和 WebGL 渲染结果是软件模拟或预生成 hash。
检测器（如 DataDome Picasso）通过服务端下发随机 seed -> N 次渲染 -> 服务端验证
像素 hash 的方式，可以识别无真实 GPU 的环境。

- **H04 能验证**：WebGL getParameter 值正确性 (L1)、扩展列表完整性 (L0)
- **H04 不能验证**：像素级渲染一致性（需要真实 GPU 渲染对比）
- **影响范围**：P1 的 CanvasRenderingContext2D / WebGLRenderingContext 的 L1 值
  对 getParameter 类属性有效，但对 toDataURL / renderOutput 无效
- **缓解**：L1 单元格对 canvas/webgl 像素类属性标记为 `PENDING`（需 v0.9+ GPU 层）

### 11.2 TLS 指纹

IV8 不做网络层，TLS JA4+ 指纹由代理/网络层负责。但检测器会将 TLS 指纹与 JS 层
声称的 Chrome 版本交叉验证（`detection-surface-taxonomy.md` Category 4）。

- **H04 能验证**：JS 层的 userAgent / userAgentData 值正确性和一致性 (L1/L2)
- **H04 不能验证**：TLS JA4+ 指纹与 JS 层声称的一致性（跨层验证需要网络层接入）
- **影响范围**：H04 的 L2（值一致性）无法覆盖 TLS <-> JS 跨层一致性
- **缓解**：在 L2 单元格中记录"TLS 跨层一致性不在 H04 范围"，由独立网络层测试覆盖

### 11.3 行为层（真实用户输入）

IV8 不产生真实用户输入（鼠标轨迹、键盘时序、滚动速度）。检测器（Akamai sensor_data、
DataDome 行为遥测）通过行为生物特征识别自动化工具。

- **H04 能验证**：D1-D6 行为探真维度的 API 语义正确性（方法返回类型、Promise 语义、
  事件触发链、状态转换、异常类型、异步排序）
- **H04 不能验证**：真实用户行为模拟的不可区分性（这超出 IV8 的责任边界）
- **影响范围**：D1-D6 验证的是 API 行为语义，不是行为生物特征
- **缓解**：在 spec 中明确标注行为生物特征不在 H04 范围

### 11.4 其他局限

| 局限 | 说明 | 影响 |
|---|---|---|
| Golden 数据 per-GPU-profile | CDP 采样金标准绑定特定 GPU (如 RTX 4060) | 不同 GPU profile 需重新采样 |
| Chrome 版本漂移 | bcd-collector / Chromium IDL 每 6 周更新 | A 类 freshness 阈值 42 天 |
| Worker 实现不完整 | IV8 无真实 Worker，L8 大量 BLOCKED | Phase 3 L8 允许 BLOCKED |
| CreepJS 版本依赖 | CreepJS 更新可能引入新 lie 类型 | S5 submodule 锁定版本 |
| idlharness.js shim 风险 | V8 内 shim 可能与 WPT 原版行为有微小差异 | C 类 Chrome 自测作为兜底 |
| FP-Inconsistent 规则停更 | 论文 2025-01 后无更新，390 条规则中 206 适用 | S6 规则集固定，不跟踪上游 |
| P3 接口列表维护 | 从 bcd-collector 程序化生成，依赖上游数据质量 | P3 仅做 coverage 统计，不阻断 |

### 11.5 不在 H04 范围内的检测面

以下检测面由其他层负责，H04 不覆盖（见 `detection-surface-taxonomy.md` §5）：

| Category | 检测面 | 负责层 |
|---|---|---|
| Cat 1 | HTTP Headers (UA/CH/Accept-Language) | 代理层（IV8 仅约束 JS 值匹配） |
| Cat 4 | TLS JA4+ fingerprint | 代理/网络层 |
| Cat 5 | TCP/IP p0f | 网络层 |
| Cat 6 | IP/Geo/ASN reputation | 代理层 |
| Cat 7 | Behavioral biometrics | 行为脚本层 |
| Cat 10 | WASM/SIMD timing | 待评估（v0.9+） |
| Cat 11 | ML anomaly detection | 不负责 |
| Cat 12 | CDP 侧信道 (console.debug getter trap, Proxy ownKeys trap) | IV8 无 CDP — 天然免疫 |

H04 覆盖：Cat 2 (Navigator/Screen/Window/Document)、Cat 3 (Canvas/WebGL/Audio/Font)、
Cat 9 (webdriver/plugins/chrome stubs)、Cat 8 (XHR/cookie 行为)。

---

## 12. 当前基线

> Phase 1 实施中。以下为 `scripts/evaluate_surface_integrity.py` + 全量审计实测值。
> 数据源: `data/idlharness-report.json` (7876/10222 PASS, 77.05%)
> 全量审计: 10222/10222 idlharness 测试 100% 映射到 11 层 (L0/L1/L3/L4/L6/L7/L9-L13)
> 多源验证: 6 独立来源 (Web IDL spec + CreepJS + FP-Inconsistent + FP-Scanner + crawlex.net + fp-scanner GitHub)

### 12.1 idlharness 覆盖层 (11/23 层, 3758 测试 — WPT 官方 runner)

> WPT 官方 runner (run_wpt.py): 3758 测试, 3213 PASS (85.50%)。
> WPT 官方 Chrome 151 基线: 9640 测试, 9481 PASS (98.35%)。
> wpt.fyi API (2026-06-30, Chrome 151.0.7921.0, run id 5155506334466048)。
> 旧 runner (run_idlharness.py, 10222 测试) 已废弃。
> R4 修复: const 描述符 + accessor .name/.length + configurable
>   + dom callback receiver check + shim .name + dom method .length (+125 PASS)
> 参见 `docs/roadmap/v0.8/analysis/wpt-integration-design.md`。

| Layer | Pass | Fail | Total | Rate | 状态 |
|---|---|---|---|---|---|
| L0 | 1604 | 780 | 2384 | 67.3% | PARTIAL |
| L1 | 4208 | 126 | 4334 | 97.1% | GOOD |
| L3 | 0 | 440 | 440 | 0.0% | BLOCKED |
| L4 | 0 | 1 | 1 | 0.0% | 1 FAIL |
| L6 | 16 | 235 | 251 | 6.4% | PARTIAL |
| L7 | 125 | 541 | 666 | 18.8% | PARTIAL |
| L9 | 1253 | 7 | 1260 | 99.4% | GOOD |
| L10 | 30 | 24 | 54 | 55.6% | PARTIAL |
| L11 | 0 | 95 | 95 | 0.0% | NOT IMPL |
| L12 | 0 | 13 | 13 | 0.0% | NOT IMPL |
| L13 | 0 | 7 | 7 | 0.0% | NOT IMPL |
| other | 640 | 77 | 717 | 89.3% | 待细化 |

### 12.2 独立脚本覆盖层 (9/23 层, 已有脚本待接入)

| Layer | 名称 | 现有脚本 | 接入状态 |
|---|---|---|---|
| L2 | 值一致性 | `scripts/evaluate_env_consistency.py` | 待接入 evaluator |
| L5 | 递归 toString | `scripts/probe-iv8-wrapnative.py` (部分) | 需专用脚本 |
| L8 | 跨上下文 | `scripts/_metamorphic.py` (MR-CTX-001~007) | 待接入 evaluator |
| D1 | 方法返回值语义 | `scripts/_d1_d5_behavior.py` | 待接入 evaluator |
| D2 | Promise 语义 | `scripts/_d1_d5_behavior.py` | 待接入 evaluator |
| D3 | 事件触发时序 | `scripts/_d1_d5_behavior.py` | 待接入 evaluator |
| D4 | 状态转换 | `scripts/_d1_d5_behavior.py` | 待接入 evaluator |
| D5 | 异常行为 | `scripts/run_creepjs_lies.py` | 待接入 evaluator |
| D6 | 异步排序 | `scripts/_d1_d5_behavior.py` | 待接入 evaluator |

### 12.3 缺失脚本层 (3/23 层, 需新建)

| Layer | 名称 | 检测内容 | 参考来源 | 优先级 |
|---|---|---|---|---|
| L14 | Stack trace shape | Error.stack 格式 (V8 "at" vs SM "@"), 描述符 (data vs accessor), stackTraceLimit | crawlex.net, CreepJS hasValidStack, TC39 Error Stacks proposal | P1 |
| L15 | Enumeration order | Object.keys(navigator) 顺序, DOM 属性顺序, host object key order | crawlex.net, FP-Scanner, ECMA-262 §6.1.7.1 | P2 |
| L16 | Timing resolution | performance.now() 精度, Date.now() 末位裁剪, rAF 时序 | CreepJS resistance, crawlex.net, W3C fingerprinting-guidance §3.2 | P2 |

---

## 附录 A: 与调研文档的交叉引用

| 调研结论 | H04 对应章节 | 决策编号 |
|---|---|---|
| H04 应立项为 H02/H03 超集 | §1.2, §1.3 | D-102 |
| bcd-collector 9.5/10 主数据源 | §2.3 S1 | D-096 (修正) |
| webref 降级为规范对照 | §2.3 S3 | D-096 (修正) |
| idlharness V8 内 shim 运行 | §2.3 S8, Phase 2 | D-100 |
| Metamorphic Testing 更优 (L3/L4) | §6 E 类, Phase 3 | D-101 |
| D1-D6 行为探真六维度 | §2.2.2 | D-103 |
| Proxy 代理器不实现 JS Proxy | §11, Phase 4 miss counter | D-097 |
| H02 D-098 runtime 接入缺陷 | §5.3, Phase 2 | D-098 |
| 接口深度三层分类 | §2.1 P0-P3 | D-099 |
| PBT + Mutation Testing | Phase 4 | D-105 |

## 附录 B: 术语表

| 术语 | 定义 |
|---|---|
| 属性探真 | 属性的静态正确性 (L0/L1/L3/L4-L7/L9-L13) |
| 行为探真 | 方法的动态正确性 (L2/L6/L8/D1-D6) |
| 金标准 | 来自权威来源的可追溯测试数据 (宪法 P10) |
| 单元格 | 矩阵中一个 [interface, layer, source] 三元组对应的验证结果 |
| 稀疏矩阵 | 大部分单元格为 N/A，仅在有效组合处填充 |
| 蜕变关系 | Metamorphic Testing 中不需 gold standard 的输入-输出不变式 |
| miss counter | V8 NamedPropertyHandlerConfiguration 拦截未定义属性访问的计数器 |

## 附录 C: Web IDL spec 条款速查

> 多源验证: Web IDL Living Standard (whatwg) + Chromium Blink-V8 bindings + WPT idlharness.js
> 参见 `docs/conventions/authoritative-data-sources.md`

### C.1 Interface object (§3.7.1)

```
CreateBuiltinFunction(steps, length, id, « [[Unforgeables]] », realm, constructorProto)
  length = shortest argument list in constructor overload set (0 if no constructor)
  id = interface identifier (becomes .name)
  constructorProto = interface object of parent P (becomes .__proto__)

"prototype" property: {Writable: false, Enumerable: false, Configurable: false, Value: proto}
Constants on interface object: {Writable: false, Enumerable: true, Configurable: false}
Static operations on interface object: {Writable: true, Enumerable: true, Configurable: true}
```

### C.2 Legacy factory function / Named constructor (§3.7.2, §3.4.1)

```
CreateBuiltinFunction(steps, length, id, « », realm)
  length = shortest argument list in legacy factory overload set
  id = [LegacyFactoryFunction] identifier (e.g., "Image", "Audio", "Option")

"prototype" property: {Writable: false, Enumerable: false, Configurable: false, Value: interface prototype}
Audio.prototype === HTMLAudioElement.prototype (spec: §3.7.2)
```

### C.3 Interface prototype object (§3.7.3)

```
proto = parent interface prototype (or Object.prototype)

"constructor" property: {Writable: true, Enumerable: false, Configurable: true, Value: interface object}
Constants: {Writable: false, Enumerable: true, Configurable: false, Value: value}
Regular attributes: accessor property (get/set)
Regular operations: {Writable: true, Enumerable: true, Configurable: true, Value: function}
```

### C.4 Constants (§3.7.5)

```
PropertyDescriptor: {Writable: false, Enumerable: true, Configurable: false, Value: value}
Defined on BOTH interface object AND interface prototype object
```

### C.5 Attributes (§3.7.6)

```
Getter: CreateBuiltinFunction(getterSteps, 0, "get " + id, « », realm)
  .name = "get " + attribute identifier
  .length = 0

Setter (non-readonly): CreateBuiltinFunction(setterSteps, 1, "set " + id, « », realm)
  .name = "set " + attribute identifier
  .length = 1

Installed as accessor property on prototype (or instance for [Global])
```

### C.6 Operations (§3.7.7)

```
CreateBuiltinFunction(steps, length, id, « », realm)
  length = shortest argument list in overload set
  id = operation identifier (becomes .name)

PropertyDescriptor: {Writable: modifiable, Enumerable: true, Configurable: modifiable}
  modifiable = false if unforgeable, true otherwise

Static operations on interface object, regular on prototype
```

### C.7 Stringifier (§3.7.8)

```
Property name: "toString"
PropertyDescriptor: {Writable: B, Enumerable: true, Configurable: B}
  B = false if unforgeable, true otherwise

Value: CreateBuiltinFunction(toStringSteps, 0, "toString", « », realm)
  .name = "toString"
  .length = 0
```

### C.8 Iterable declarations (§3.7.9)

```
Value iterator:
  Symbol.iterator = %Array.prototype.values%
  entries = %Array.prototype.entries%
  keys = %Array.prototype.keys%
  values = %Array.prototype.values%
  forEach = %Array.prototype.forEach%

Pair iterator:
  Symbol.iterator = entries function
  entries.name = "entries", entries.length = 0
  keys.name = "keys", keys.length = 0
  values.name = "values", values.length = 0
  forEach.name = "forEach", forEach.length = 1
```

### C.9 Setlike declarations (§3.7.12)

```
size: accessor getter, .name = "get size", .length = 0
add: data property, .name = "add", .length = 1
delete: data property, .name = "delete", .length = 1
has: data property, .name = "has", .length = 1
clear: data property, .name = "clear", .length = 0
entries/keys/values/forEach: same as pair iterator
Symbol.iterator = entries function
```

### C.11 CreepJS lie 检测映射

来源：CreepJS `src/lies/index.ts` (953 行)，多源验证。

| CreepJS lie | H04 层 | 检测内容 |
|---|---|---|
| failed illegal error | L6 | obj.prototype[name] 抛 TypeError |
| failed undefined properties | L3 | instance getOwnPropertyDescriptor 返回 undefined |
| failed call/apply/new/extends/null | L6 | 各种非法调用抛 TypeError |
| failed toString | L4 | Function.toString.call(fn) 匹配 6 种 native 格式 |
| failed prototype in function | L3 | getter/setter 不应有 prototype |
| failed descriptor/own property/keys | L3 | fn own keys = ['length','name'] |
| failed object toString error | L4+L6+L14 | Object.create(fn).toString() 抛 + stack |
| failed at incompatible proxy | L6 | fn.arguments/fn.caller 抛 |
| failed at too much recursion | L7 | prototype cycle 抛 TypeError |
| Proxy detection | L7 | Proxy 创建后递归检测 |
| worker scope mismatch | L8 | Worker vs Window 值一致 |
| Stack trace validation | L14 | Error.stack 行模式匹配 |
| Resistance (Date.now) | L16 | 计时精度裁剪检测 |### C.10 Maplike declarations (§3.7.11)

```
size: accessor getter, .name = "get size", .length = 0
get: data property, .name = "get", .length = 1
set: data property, .name = "set", .length = 2
has: data property, .name = "has", .length = 1
delete: data property, .name = "delete", .length = 1
clear: data property, .name = "clear", .length = 0
entries/keys/values/forEach: same as pair iterator
Symbol.iterator = entries function
```
