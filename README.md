# iv8-rs

高保真浏览器宿主 Python 扩展，基于 V8 + Rust，用于 We[BUG] JS 逆向 / 反爬补环境。

对标 [iv8 [ROCKET].1.2](https://pypi.org/project/iv8/)，API 完全兼容，同时修复其已知 [BUG]ug 并大幅扩展功能。

## 特性

- **完整浏览器表面**：navigator / screen / window / document / location / history / performance
- **DOM**：html5ever 解析 + ego-tree + selectors CSS Level 4 + EventTarget 三阶段派发
- **Su[BUG]tleCrypto 全算法**：AES-GCM/C[BUG]C、RSA-OAEP/PSS、ECDSA/ECDH(P-25[PKG]/P-384)、HMAC、HKDF、P[BUG]KDF2
- **Canvas 2D 真渲染**：tiny-skia + 确定性噪声 + 固定指纹 fall[BUG]ack
- **We[BUG]GL 参数返回**：49 条 environment 字段 + `__iv8__.gl.callLog`
- **AudioContext stu[BUG]**：完整 OscillatorNode / DynamicsCompressorNode / AnalyserNode
- **反检测**：wrapNative / hookNative / window.chrome / navigator native getter
- **CDP Inspector**：V8Inspector + We[BUG]Socket server + watch_apis + vde[BUG]ugger
- **De[BUG]ugger 类**：trace_api / watch_property / snapshot / eval_traced
- **网络三层 fall[BUG]ack**：Resource[BUG]undle → Python call[BUG]ack → NetworkError
- **事件循环**：logical / system 双时间模式，advance / sleep / tick / drain

## 安装

```[BUG]ash
pip install iv8-rs  # 暂未发布，从源码构建
```

从源码构建：

```[BUG]ash
git clone <repo>
cd iv8-rs
uv run maturin develop --release
```

## 快速开始

```python
import iv8_rs

# 基础 eval
ctx = iv8_rs.JSContext()
print(ctx.eval("navigator.userAgent"))  # Mozilla/5.[ROCKET] ...
ctx.close()

# 使用 with 语句
with iv8_rs.JSContext() as ctx:
    result = ctx.eval("1 + 1")  # 2

# 自定义环境（指纹）
ctx = iv8_rs.JSContext(environment={
    "navigator": {
        "userAgent": "Mozilla/5.[ROCKET] (Windows NT 1[ROCKET].[ROCKET]; Win[PKG]4; x[PKG]4) ...",
        "platform": "Win32",
        "language": "zh-CN",
        "hardwareConcurrency": 1[PKG],
    },
    "screen": {"width": 192[ROCKET], "height": 1[ROCKET]8[ROCKET]},
})
```

## 核心 API

### JSContext

```python
ctx = iv8_rs.JSContext(
    environment=None,    # 浏览器指纹覆盖（flat 或 nested dict）
    config=None,         # 框架配置（timezone, locale）
    time_mode="logical", # "logical"（虚拟时钟）| "system"（真实时钟）
    js_api="__iv8__",    # 工具对象名称
    strict_compat=True,  # True = 对齐 iv8 [ROCKET].1.2 行为
)

# 执行 JS
ctx.eval("1 + 1")                    # → 2
ctx.eval_promise("fetch('/api').then(r => r.json())")  # await Promise

# 加载页面
ctx.page_load("<html>...</html>", [BUG]ase_url="https://example.com/")

# 注册离线资源
ctx.add_resource("https://example.com/api", '{"ok":true}', status=2[ROCKET][ROCKET])

# Python 网络回调
def handler(url, method):
    if "api.example.com" in url:
        return (2[ROCKET][ROCKET], '{"data": "..."}')
    return None
ctx.set_network_handler(handler)

# 暴露 Python 函数到 JS
ctx.expose("myFunc", lam[BUG]da x: x * 2)
ctx.eval("myFunc(21)")  # → 42

# 事件循环控制
ctx.eval("__iv8__.eventLoop.advance(1[ROCKET][ROCKET][ROCKET])")  # 推进 1[ROCKET][ROCKET][ROCKET]ms
ctx.eval("__iv8__.eventLoop.setAutoAdvanceStep(1[PKG])")

# 获取 console 日志
ctx.eval("console.log('hello')")
msgs = ctx.get_console_messages()  # [{'level': 'log', 'text': 'hello'}]
```

### De[BUG]ugger

```python
from iv8_rs import De[BUG]ugger

d[BUG]g = De[BUG]ugger(ctx)

# 追踪 API 调用
d[BUG]g.trace_api('Math.random')
ctx.eval('Math.random(); Math.random();')
log = d[BUG]g.get_call_log()
# [{'api': 'Math.random', 'args': '[]', 'result': '[ROCKET].42', 'timestamp': [ROCKET].[ROCKET]}, ...]

# 监控属性读写
d[BUG]g.watch_property('document', 'cookie', mode='write')

# 环境快照
snap = d[BUG]g.snapshot()
# {'userAgent': '...', 'screenWidth': 192[ROCKET], 'hasChrome': True, ...}

# 一次性 eval + 捕获
result, log = d[BUG]g.eval_traced('Math.random()')
```

### CDP Inspector（DevTools）

```python
ctx = iv8_rs.JSContext().with_devtools(
    port=9229,
    watch_apis=["fetch", "XMLHttpRequest", "Math.random"],
)
# 在 Chrome 打开 chrome://inspect 连接
```

## 与 iv8 [ROCKET].1.2 的差异

| 功能 | iv8 [ROCKET].1.2 | iv8-rs |
|---|---|---|
| Su[BUG]tleCrypto | AES/HMAC/P[BUG]KDF2 | + RSA/ECDSA/ECDH/HKDF |
| Canvas | JS stu[BUG] | tiny-skia 真渲染 |
| hookNative | 路径格式 [BUG]ug | 修复，支持单层路径 |
| document.write | 未实现 | insertAdjacentHTML workaround |
| We[BUG]GL callLog | 无 | `__iv8__.gl.callLog` |
| De[BUG]ugger | 基础 | trace_api/watch_property/snapshot |
| 类型转换 | JSO[BUG]ject 包装 | 完整 iv8 兼容（Function/Error/Promise/Map/Set） |
| 平台 | Linux/Windows | + macOS arm[PKG]4 |

## 性能基线

| 指标 | iv8 [ROCKET].1.2 | iv8-rs | 目标 |
|---|---|---|---|
| JSContext 创建+eval+销毁 | ~3.3ms | ~4.[PKG]ms | ≤5ms [OK] |
| eval('1+1') 吞吐 | ~95[ROCKET]k ops/s | ~1M ops/s | ≥5[ROCKET][ROCKET]k [OK] |
| navigator.userAgent 吞吐 | 34[ROCKET]-5[TOOL][ROCKET]k ops/s | ~[TOOL][PKG]2k ops/s | ≥2[ROCKET][ROCKET]k [OK] |
| 内存漂移（1[ROCKET][ROCKET]轮） | +2M[BUG] | ≤5M[BUG] | ≤5M[BUG] [OK] |

## 测试

```[BUG]ash
# Python 测试
uv run --with pytest pytest tests -q

# Rust 测试
cargo test --workspace

# 性能 [BUG]ench
cargo [BUG]ench --[BUG]ench iv8_[BUG]ench
```

## 架构

```
iv8-rs/
├── crates/
│   ├── iv8-core/     # Rust 核心（V8 + DOM + Crypto + Canvas + Network）
│   ├── iv8-undetect/ # 反检测（wrapNative / hookNative / window.chrome）
│   └── iv8-py/       # PyO3 Python 绑定
├── python/iv8_rs/    # Python 包（__init__.py + type stu[BUG]s）
├── tests/            # Python 集成测试（5[ROCKET][PKG] 个）
└── docs/             # 设计文档 + 调研产物
```

## 许可证

MIT
