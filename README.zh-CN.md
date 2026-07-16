# iv8-rs

高保真 **类浏览器 JS 运行时** Python 扩展（V8 + Rust + PyO3）。  
面向：Web JS 逆向、可控复现执行、反爬 / 指纹 **宿主** 模拟，以及 ChaosVM 类脚本的插桩与分析。

**当前**：里程碑 continuum 至 **v0.8.102** · 包版本 **0.8.12**（D-151 双轨）— [CHANGELOG](CHANGELOG.md)  
**PyPI 包名（规划）：** `ming_iv8_rs` · **import：** `import iv8_rs`（模块名暂不改）  
English：[README.md](README.md) · API 契约：[docs/api/](docs/api/) · 覆盖审计：[docs/api/COVERAGE.md](docs/api/COVERAGE.md)

## 缘起：为什么做 iv8-rs

初见，看到了 **[iv8](https://github.com/jofpin/iv8)**（PyPI 上的 `iv8` 0.1.2 一脉）：把 V8 嵌进 Python、用环境字典驱动浏览器面——这条路对 Web 逆向很对味。我们想要的是**同类能力**，但再往下挖一层：

- **更稳的宿主面**：不只是“能 eval”，还要 brand、getter、Worker、Intl、DOM 集合经得起高信号检测；
- **更可控的复现**：离线 ResourceBundle、确定性种子、逻辑时钟，方便 CI 与双引擎对照；
- **插桩 + 分析二位一体**：一边在宿主里跑脚本，一边用 `instrument_source` / CDP / 统一 trace 把 ChaosVM、TDC 一类路径拆开看——左脚踩右脚有点夸张，但方向就是**运行时执行 + 观测分析**绑在一起。

设计上参考了 iv8 的产品直觉（Python 友好、环境可注入、离线优先），内核则选了 **Rust + PyO3 + 大面积 codegen/native 浏览器面**，把性能、类型边界和长期维护压在同一条链路上。  
**同类项目没有必要踩。** 本仓库与 PyPI 包 `iv8` 0.1.x 是**相关谱系 / 双引擎 oracle**，产品名是 **iv8-rs**——不是“取代谁”，是“从同一灵感走得更深”。

## 设计哲学：双分支、同一闭环

iv8-rs 不是“只会补环境”，也不是“只会反编译”。产品赌注是 **双分支一体**：

| 分支 | 名称 | 回答的问题 |
|---|---|---|
| **A** | **运行时环境（宿主）** | 脚本能否在**可控的、类浏览器宿主**里 **跑起来**？ |
| **B** | **运行时分析** | 这次运行 **做了什么**，能否被观测、结构化、再推理？ |

两者应 **互相喂养**：宿主越真，trace 越干净、Illegal invocation 越少；分析越深，越能反指缺面、错 brand、残缺网络。口语里说的 **左脚踩右脚上天**，指的就是：保真与可观测 **一起爬坡**，而不是两套互不相关的工具。

当前交付是 **Branch A 权重高** + **Branch B 可用脊梁**（插桩、统一 trace、入口平面、诊断报告）。更深的 IR / SSA / 完整反编译管线，是 Branch B 的 **明确期望态（north star）**——不是空喊“全都没做”，也不是假装“已经做完”，而是写清 **往哪爬**，同时 Branch A 继续扛生产。

### Branch A — 运行时环境（宿主）

脚本必须相信自己活在“页面里”：同线程 V8 isolate、大面积浏览器面、离线优先 I/O、确定性旋钮服务 CI。

| 层 | 内容（示例，非穷尽） |
|---|---|
| **内核** | V8 isolate、栈/线程亲和、ICU/Intl + 时区 Redetect、逻辑/系统时钟、种子 |
| **浏览器面** | Window / Navigator / Screen / Location / DOM 解析与查询、事件、集合、Worker |
| **媒体与密码学** | Canvas、WebGL 参数、Audio、SubtleCrypto |
| **网络** | ResourceBundle → 可选 Python handler → 错误（默认不静默公网抓取） |
| **身份** | Profile、点路径 environment、storage、cookie/headers |
| **反检测积木** | wrap/hook native、chrome 对象、toString / brand 卫生（保真，不是“过所有检测器”） |
| **产品诚实** | 非目标：非完整 Chromium、非布局奢侈项、非一键过站 |

Branch A 支撑 **可控复现**：同输入、同种子、离线 chunk、双引擎对照。

### Branch B — 运行时分析（观测与结构化）

宿主能跑载荷之后，Branch B 把运行变成 **证据**：trace、diff、入口计划、诊断报告——期望态再往下是更丰富的中间表示。

| 层 | 今天（已交付脊梁） | 期望态（north star，不声称已全部完成） |
|---|---|---|
| **插桩** | `instrument_source` / ChaosVM Path A、全局 `instrument_chaosvm`、env Proxy | 更广 VM 族、更少误 Illegal invocation、expose_handlers 策略 |
| **Trace 平面** | 统一 D/R/C/W、VM trace、`trace_diff`、recording / profiler / coverage | 稳定 schema、流式、多 run 语料一等公民 |
| **调试 / CDP** | DevTools 挂载、断点、单步、`Debugger` | 更深 scope/对象 UX、CI 友好程序化会话 |
| **入口 / bundler** | `prepare_entry` / `run_with_entry` / multi-entry；chunk 调用方自备 | 更完整 bundler 图，仍无静默拉网 |
| **结构与推理** | CFG / taint / pattern / crypto 检测、report 模型、环境平面（诊断） | **IR / SSA 风格**视图、opcode/handler IR、字符串数组与 deobf 报告成管线、跨版本 VM diff 为默认工作流 |
| **格式与适配** | 版本化 schema、corpus manifest | 更多中间格式、dual-oracle 包、对接外部 RE 工具 |

Branch B 必须 **诚实**：环境工具链默认 **仅报告**；分析不是“过站包”。表中 IR/SSA 等是 **设计意图**——部分已有或脚手架在，**完整管线**仍是爬坡目标，不是“尚未启动”的借口，也不是“已经做完”的话术。

### A 与 B 如何互相加强

```text
        +------------------+
        |  待测 JS 载荷    |
        +--------+---------+
                 |
        +--------v---------+
        |  Branch A: 宿主  |  跑 / 冻结 / 离线 / brand
        +--------+---------+
                 |  traces、事件、缺口
        +--------v---------+
        |  Branch B: 分析  |  插桩、diff、plan、report
        +--------+---------+
                 |  缺面、错值、入口修复
                 +-----> 回到 Branch A（profile、shim、网络）
```

1. **A → B：** 宿主不真，trace 是噪声，Illegal invocation 盖住真逻辑。  
2. **B → A：** diff 与 probe 指出哪些 getter、plugin、网络边仍在说谎。  
3. **闭环：** 每个里程碑应要么加固宿主，要么加深观测，要么收紧两者契约。

产品命题：**一个 Python 进程、两条互补分支**，而不是一堆互不相关的脚本。

## 为什么是这条技术路线

| 路径 | 常见缺口 |
|---|---|
| 纯 Node / 纯 Python | 浏览器面薄；`instanceof`、getter、Worker、Intl 常不对 |
| 仅完整 CDP 浏览器 | 重、难做离线 VM 插桩、CI 难确定性 |
| 薄 stub | 过不了 brand check、canvas/WebGL/crypto、DOM 集合 |

**iv8-rs** 嵌入 V8，提供大面积原生浏览器面、离线 ResourceBundle 网络、确定性种子、ChaosVM / `instrument_source` Path A、多 bundler 入口平面，以及 **诊断向** 环境工具链——单进程、同线程 isolate，边界诚实（**不是**完整 Chrome）。

## 能力（按能力域，不按版本流水）

版本细节见 [CHANGELOG](CHANGELOG.md)。稳定调用契约见 [docs/api/](docs/api/)。

### 运行时宿主

- `JSContext`：`environment` / `profile` / 默认合并；上下文管理器；`close`
- 同线程 isolate；导入时配置 128MB Python 线程栈
- `time_mode`：`logical` | `system`；`random_seed` / `crypto_seed` / `time_freeze`
- `config`：`timezone` / `locale` / `storage_path`；时区 → 进程 `TZ` + V8 Redetect（ICU 77）
- 双轨版本：里程碑 tag vs 包版本（[docs/api/versioning.md](docs/api/versioning.md)）

### 浏览器面与 DOM

- Window / Navigator / Screen / Location / History / Performance / document（codegen + native）
- html5ever 解析、CSS Level 4 选择器、EventTarget 三阶段派发
- `page_load` / `page_load_with_headers`；NodeList 可迭代；`getElementsByTagName('*')`
- 集合 / plugins / HTMLAll 等结构；Worker + WorkerNavigator
- Profile 驱动身份（Chrome 系默认 profile；点路径 environment）

### Crypto / Canvas / WebGL / Audio

- SubtleCrypto：AES / RSA / ECDSA-ECDH / HMAC / HKDF / PBKDF2 等
- Canvas 2D（tiny-skia + 确定性噪声 / 固定指纹模式）
- WebGL 参数面 + 环境 UNMASKED_* / call log
- AudioContext / OfflineAudio；字体 metrics（profile 接入处）

### 网络与事件循环

- 三层网络：ResourceBundle → Python `set_network_handler` → 错误（默认非静默公网抓取）
- `add_resource`；XHR / fetch / WebSocket 面
- 逻辑时钟 / 系统时钟；advance / sleep / tick / drain（见 GUIDE）
- Cookie / Headers / storage 持久化

### 反检测原语

- wrapNative / hookNative / `window.chrome` / MarkAsUndetectable
- toString / toStringTag / prototype brand 卫生（持续演进）
- 高信号 Illegal invocation 修复
- **不**承诺“过所有检测器”——只承诺宿主保真 + 边界写清楚

### 插桩与可观测

- 模块级 `instrument_source`（ChaosVM Path A，闭包 handler，如 TDC）
- 实例 `instrument_chaosvm`（仅 **全局** handler 表）
- 统一 / VM trace；`trace_diff`；trace point；recording / profiler / coverage
- CDP：`with_devtools`、断点、单步、scope、程序化 API
- `Debugger`：`trace_api`、`watch_property`、`eval_traced`、snapshot

### 入口 / 多 bundler

- `prepare_entry` / `run_with_entry` / `plan_multi_entry`
- Webpack / Parcel / Browserify / Vite 邻接桥；**chunk 文本由调用方提供**
- Corpus runner 离线多用例

### 环境工具链（诊断平面）

- Probe / gap / candidate / pressure / MAPE-K 风格报告
- 默认 **仅报告 / 不自动 apply / 不静默写 profile**
- **不是**一键过站包；样本 adapters 与产品 API 分离

### Workers

- 独立 isolate + OS 线程 + structured clone（方案 A）
- WorkerNavigator / import 路径；残留见 residual ledgers

## 非目标 / 诚实边界

| 不声称 | 现实 |
|---|---|
| 完整 Chromium / Blink | 大面积 IDL + 有意 stub；parity 持续工作 |
| 自动拉取全部 bundler chunk | 离线优先；调用方供码 |
| 静默公网网络产品 | ResourceBundle 优先 |
| 环境工具链自动修宿主 | 默认诊断报告 |
| 与 PyPI 包 `iv8` 0.1.x 同一产品 | 相关谱系 / 双引擎 oracle；**本产品为 iv8-rs** |
| 完整布局引擎 / 深度指纹奢侈项 | 延后（v0.9+ / residual） |

全局边界：[docs/api/overview.md](docs/api/overview.md)。

## 安装

需要 **Rust 工具链**、**Python 3.13+**、ICU **77** 数据（包内 `icudtl.dat`；可用 `IV8_ICUDTL_PATH` 覆盖）。

```bash
git clone <repo>
cd iv8-rs

# 本地开发（快迭代）
uv run maturin develop --target-dir target-maturin --strip --profile dev

# 发行构建（LTO，慢）
uv run maturin develop --release
```

**栈：** `import iv8_rs` 后创建 `JSContext`（模块已设 `threading.stack_size(128MB)`）。完整内核 Rust 测试：`RUST_MIN_STACK=134217728`。

可选：cargo / maturin 使用独立 `--target-dir`，避免与 IDE 争用 `target/`。

## 快速开始

```python
import iv8_rs

with iv8_rs.JSContext() as ctx:
    print(ctx.eval("navigator.userAgent"))

ctx = iv8_rs.JSContext(
    profile="default",
    environment={
        "timezone": "Asia/Shanghai",
        "navigator.userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ...",
        "screen.width": 1920,
        "screen.height": 1080,
    },
    time_mode="system",
    config={"timezone": "Asia/Shanghai"},
)
print(ctx.eval("Intl.DateTimeFormat().resolvedOptions().timeZone"))
ctx.close()
```

完整方法面：[docs/api/runtime/jscontext.md](docs/api/runtime/jscontext.md)。  
插桩：[docs/api/instrumentation/](docs/api/instrumentation/)。  
覆盖审计：[docs/api/COVERAGE.md](docs/api/COVERAGE.md)。

## 文档地图

| 文档 | 用途 |
|---|---|
| **[docs/api/](docs/api/)** | 稳定 API 契约（分层；公仓向） |
| **[docs/GUIDE.public.md](docs/GUIDE.public.md)** | 公仓教程裁剪（§1–16）；完整 GUIDE 偏私仓演进日记 |
| **[CHANGELOG.md](CHANGELOG.md)** | 版本增量 |
| **[docs/quality-harness/](docs/quality-harness/)** | 质量门禁定义 |
| **[docs/conventions/](docs/conventions/)** | 命名 / 测试 / 文档 / docstring 规范 |
| **[README.md](README.md)** | English README |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | 提交约定 |

验收 / roadmap 树默认 **私仓** 材料，不视为公仓产品 API。

## 架构（鸟瞰）

```text
Python (iv8_rs)
    │  PyO3
    ▼
iv8-py  ──►  iv8-core (V8 isolate, DOM, crypto, canvas, network, inspector)
                ├── iv8-undetect   (wrap / hook / chrome 原语)
                ├── iv8-surface    (IDL 模板 / codegen)
                └── iv8-profile    (profile 矩阵)
```

```text
iv8-rs/
├── crates/          # Rust workspace
├── python/iv8_rs/   # 包表面、profiles、analysis、toolchain
├── tests/           # Python 集成
└── docs/            # GUIDE、api/、quality-harness、roadmap（公/私混合）
```

## 开发

```bash
cargo test --workspace
cargo test -p iv8-core --lib
uv run python -m pytest tests -q
```

提交格式、栈大小、非授权项见 [CONTRIBUTING.md](CONTRIBUTING.md) 与仓库内 agent 说明。  
公有远程 / 包版本 bump / Release **需显式授权**。

## 致谢

- **[iv8](https://github.com/jofpin/iv8)** — 同类 Python↔V8 宿主思路的重要灵感与对照谱系  
- V8 / PyO3 / maturin / html5ever 等上游生态

## 免责声明与使用边界（请仔细阅读）

本软件仅供 **研究、教育、互操作测试、调试与正当软件工程** 用途。

**无担保。** 软件按 **现状（AS IS）** 提供，不作任何明示或默示担保，包括但不限于
适销性、特定用途适用性、不侵权、浏览器仿真准确度或“不可检测”。详见
[Apache License 2.0](LICENSE)。

**使用责任自负。** 你须自行确保用法符合所在地法律、法规、目标网站/服务条款及
第三方权利。作者与贡献者不对你的用法负责。

**禁止 / 超出范围（不完全列举）：**

- 未授权访问、欺诈、撞库、账号接管
- **未经许可** 绕过安全、反爬、验证码或访问控制
- 以违法或违约方式针对生产系统或用户
- 基于本引擎分发恶意软件、钓鱼工具或“一键过站”攻击包
- 将本项目宣传为完整 Chromium，或承诺通过所有检测器

**不是过站产品。** 环境工具链默认为 **诊断 / 报告向**。站点专用逆向笔记与
overlay **不是** 产品 API，也不提供攻击配方。

**指纹 / 反爬。** 反检测原语是 **宿主保真积木**，不承诺过任何具体厂商检测。

**谱系说明。** 提及相关包（如 PyPI \iv8\ 0.1.x）仅为技术对照。本产品为
**iv8-rs** / **ming_iv8_rs**。

**双仓说明。** 公有树为私有开发历史的路径过滤视图；提交说明可能仍含开发过程
叙述，勿将公仓 \git log\ 当作完整私有过程记录。

**责任限制。** 在法律允许的最大范围内，作者与贡献者不对因使用或无法使用本软件
产生的任何索赔、损害或其他责任负责，包括因你的滥用产生的费用。

克隆、安装或使用本软件即表示你知悉并接受本声明。

## License

[Apache License 2.0](LICENSE)
