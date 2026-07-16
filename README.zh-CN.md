# iv8-rs

高保真 **类浏览器 JS 运行时** Python 扩展（V8 + Rust + PyO3）。  
面向：Web JS 逆向、可控复现执行、反爬 / 指纹 **宿主** 模拟。

**当前**：里程碑 continuum 至 **v0.8.102** · 包版本 **0.8.12**（D-151 双轨）— [CHANGELOG](CHANGELOG.md)  
英文主文档：[README.md](README.md) · API 契约：[docs/api/](docs/api/)

## 为什么是 iv8-rs

| 路径 | 缺口 |
|---|---|
| 纯 Node / 纯 Python | 浏览器面薄弱；`instanceof`、getter、Worker、Intl 常不对 |
| 仅完整 CDP 浏览器 | 重、难做离线 VM 插桩、CI 难确定性 |
| 薄 stub | 过不了 brand check、canvas/WebGL/crypto、DOM 集合 |

**iv8-rs** 嵌入 V8，提供大面积原生浏览器面、离线 ResourceBundle 网络、确定性种子、ChaosVM/`instrument_source` Path A、多 bundler 入口平面，以及 **诊断向** 环境工具链——单进程、同线程 isolate，边界诚实（**不是**完整 Chrome）。

## 能力（按能力域，不按版本流水）

版本细节见 [CHANGELOG](CHANGELOG.md)。稳定契约见 [docs/api/](docs/api/)。

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
- **不**承诺“过所有检测器”

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
| **[docs/api/](docs/api/)** | 稳定 API 契约（分层） |
| **[docs/GUIDE.md](docs/GUIDE.md)** | 长教程、演进说明 |
| **[CHANGELOG.md](CHANGELOG.md)** | 版本增量 |
| **[docs/quality-harness/](docs/quality-harness/)** | 质量门禁定义 |
| **[README.md](README.md)** | English README |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | 提交约定 |

## 架构

```text
Python (iv8_rs)
    │  PyO3
    ▼
iv8-py  ──►  iv8-core (V8 isolate, DOM, crypto, canvas, network, inspector)
                ├── iv8-undetect
                ├── iv8-surface / codegen
                └── iv8-profile
```

## 开发

```bash
cargo test --workspace
cargo test -p iv8-core --lib
uv run python -m pytest tests -q
```

公有远程 / 包版本 bump / Release 需显式授权。

## License

MIT
