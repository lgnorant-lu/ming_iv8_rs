# iv8-rs User Guide (public cut)

> **Public-oriented** tutorial extract from the full private GUIDE.
> Stable contracts: [docs/api/](api/). Version deltas: [CHANGELOG.md](../CHANGELOG.md).
> Full internal GUIDE remains private-oriented (version waterfall chapters omitted).

---

## Contents

1. Install and build
2. Basic usage
3. Environment configuration
4. Network
5. DOM and page load
6. Event loop
7. Anti-detection
8. Debugger
9. CDP API
10. Trace mode
11. VM instrumentation
12. Deterministic mode
13. Global recording
14. Profiler and coverage
15. instrument_source
16. trace_diff

Further topics (profiles, environment toolchain, milestone closeouts) live in the full GUIDE (private) or [docs/api/](api/).

---
## 1. 安装与构建

```bash
cd /path/to/IV8

# 本地开发（推荐，单文件变更 ~10s）
uv run maturin develop --target-dir target-maturin --strip --profile dev

# 分发构建（LTO 优化，5-10 分钟）
uv run maturin develop --release
```

> **构建速度要点**:
> - 必须使用 `--target-dir target-maturin` 隔离 maturin 与 IDE 的 `cargo check`，
>   否则共享 `target/` 会导致每次全量重编译（308s → 2.4s）。
> - `pyproject.toml` 默认 `profile = "release"`（含 thin LTO），本地开发用
>   `--profile dev` 覆盖（319s → 9.7s）。
> - 不要启用 sccache（实测减慢 23-153%）。
> - Rust 逻辑迭代用 `cargo test`（<1s warm），仅 Python API 验证时才跑 maturin。

> **Windows / 卡巴斯基 (v0.8.93)**: 若 `maturin develop` 在新 `target-dir` 下
> 报 `failed to run custom build command` / `拒绝访问 (os error 5)`，多为安全
> 软件拦截 **新建** 的 `build-script-build.exe`（行为分析/主机入侵防护，
> 不是普通“查毒扫文件”）。
>
> ### 卡巴斯基排除项怎么加（常见踩坑）
>
> 产品名可能是 Premium / Internet Security / Standard，菜单文案略有差异。
>
> **关键点 1：Trusted applications（受信任应用程序）只能挂 EXE，不能挂文件夹。**
> 只加了“受信任应用”却把 `<local-path>` 当应用加进去，往往无效。
>
> **关键点 2：文件夹要用“排除项 / Exclusions”，不是 Trusted apps。**
>
> 推荐操作（Premium / IS 一类 UI）：
>
> 1. 打开卡巴斯基 → 右下角 **齿轮 Settings**（或主界面 **设置**）
> 2. 进入 **Threats and Exclusions** / **威胁与排除**
> 3. 点 **Manage exclusions** / **管理排除项** → **Add**
> 4. **Object**：选 **Folder**，加入（建议全部勾选“包含子文件夹”）：
>    - `<local-path>`
>    - `<repo>/target-maturin`（若使用）
>    - `<repo>/target`（若 IDE/cargo 仍用此目录）
>    - 可选：`%USERPROFILE%\.cargo`、`%USERPROFILE%\.rustup`
> 5. **Protection components**：尽量全选（至少 File Anti-Virus +
>    **System Watcher / 系统监视** + **Application Control** +
>    **Behavior Detection / 行为检测**）。只排除 File AV 不够——
>    build-script 是 **运行时拦截**。
> 6. 再回到 **Trusted applications** / **受信任应用程序** → **Add**，
>    添加 **可执行文件**（不是目录）：
>    - `cargo.exe`（rustup 工具链路径下，如
>      `%USERPROFILE%\.cargo\bin\cargo.exe`）
>    - `rustc.exe`（`\.rustup\toolchains\<toolchain>\bin\rustc.exe`）
>    - `python.exe`（本仓库 `.venv\Scripts\python.exe`）
>    - 可选：`maturin.exe`（若在 PATH/venv）
>    对每个程序勾选：**Do not scan** / **Do not monitor application activity**
>    / **Do not inherit restrictions**（文案因版本而异，核心是 **不监控活动**）
> 7. 若仍拦：打开 **Reports** / **报告** → 找被拦截的
>    `build-script-build.exe` → **Add to exclusions** / **允许**（一次授权
>    往往比手填路径准）
> 8. 改完后 **完全退出卡巴再打开**，或重启后再试 `maturin develop`
>
> 仍失败时用已验证绕过（复用缓存 target，避免执行全新 build-script）：
>
> ```powershell
> cargo build -p iv8-py --features pyo3/extension-module --target-dir "<local-path>"
> Copy-Item "<local-path>" `
>   "python\iv8_rs\_iv8.cp313-win_amd64.pyd" -Force
> ```
>
> 换全新空 target 目录会再次触发 build-script 执行拦截，尽量复用
> `<local-path>`（与日常 `cargo test` 同一缓存）。

验证：
```python
import iv8_rs
print(iv8_rs.__version__)
```

---

## 2. 基础用法

```python
import iv8_rs

# 创建 context
ctx = iv8_rs.JSContext()

# 执行 JS
result = ctx.eval("1 + 1")  # → 2
result = ctx.eval("navigator.userAgent")  # → "Mozilla/5.0 ..."

# Context manager
with iv8_rs.JSContext() as ctx:
    print(ctx.eval("'hello'"))

# 关闭
ctx.close()
```

### 其他实用方法

```python
# 暴露 Python 模块的所有 callable 到 JS 全局
import my_module
ctx.expose_module(my_module)

# 获取 console 日志
ctx.eval("console.log('hello'); console.warn('warning')")
msgs = ctx.get_console_messages()  # [{"level": "log", "text": "hello"}, ...]
ctx.clear_console_messages()

# 检查 context 状态
ctx.is_disposed()  # False (未关闭)

# Inspector 辅助
url = ctx.get_devtools_url()  # DevTools 连接 URL (需先 with_devtools)
ctx.process_inspector_messages()  # 手动处理 CDP 消息
```

### 参数

```python
ctx = iv8_rs.JSContext(
    environment=dict,       # 环境配置（见 §3）
    strict_compat=True,     # True=复刻 iv8 0.1.2 行为，False=增强模式
    time_mode="logical",    # "logical"（逻辑时间）或 "system"（系统时间）
    js_api="__iv8__",       # 工具对象名称
    random_seed=None,       # Math.random 种子（见 §12）
    crypto_seed=None,       # crypto.getRandomValues 种子
    time_freeze=None,       # 冻结时间戳（ms）
    config=dict,            # 额外配置（timezone/locale）
)
```

### strict_compat 模式

| 类型 | strict_compat=True | strict_compat=False |
|---|---|---|
| BigInt | None + 错误日志 | Python int |
| Date | "[object Date]" | datetime.datetime |
| Map | "[object Map]" | dict |
| Set | "[object Set]" | set |
| TypedArray | bytes | list[int/float] |

---

## 3. 环境配置

```python
ctx = iv8_rs.JSContext(environment={
    # 扁平 dot-path 格式
    "navigator.userAgent": "Mozilla/5.0 ...",
    "screen.width": 1920,
    "screen.height": 1080,
    "location.href": "https://example.com/",

    # 或嵌套 dict 格式（自动展平）
    "navigator": {
        "userAgent": "Mozilla/5.0 ...",
        "platform": "Win32",
        "language": "zh-CN",
    },

    # DOM 布局
    "document.body.clientWidth": 996,
    "document.body.clientHeight": 1111,
    "window.innerWidth": 1920,
    "window.innerHeight": 969,

    # WebGL
    "webgl.UNMASKED_VENDOR_WEBGL": "Google Inc. (NVIDIA)",
    "webgl.UNMASKED_RENDERER_WEBGL": "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 ...)",

    # Canvas 指纹（见 §17）
    "canvas.fingerprint.toDataURL.png": "data:image/png;base64,...",
})
```

查看所有可配置项：
```python
defaults = iv8_rs.JSContext.get_defaults()
for path, value in sorted(defaults.items()):
    print(f"{path} = {value!r}")
# 396 条
```

---

## 4. 网络

### 离线资源

```python
ctx.add_resource("https://cdn.example.com/lib.js", b"var x = 1;", 200)
ctx.add_resource("https://api.example.com/data", b'{"ok":true}', 200)
```

### Python 网络回调

```python
def handler(url, method):
    if "api.example.com" in url:
        return (200, '{"result": "ok"}')
    return None  # → NetworkError

ctx.set_network_handler(handler)
ctx.clear_network_handler()  # 清除
```

三层 fallback：ResourceBundle → Python handler → NetworkError

### netLog

```python
ctx.eval('fetch("https://api.com/x")')
entries = ctx.eval("__iv8__.netLog.entries")
# [{"method": "GET", "url": "...", "headers": [...], "body": ""}]
```

---

## 5. DOM 与页面加载

### page_load

```python
ctx.page_load(
    html='<html><body><div id="app">Hello</div></body></html>',
    base_url="https://example.com/",
)
print(ctx.eval('document.getElementById("app").textContent'))  # "Hello"
```

### 带外联脚本

```python
ctx.add_resource("https://cdn.com/lib.js", open("lib.js", "rb").read(), 200)
ctx.page_load(
    html='<html><body><script src="https://cdn.com/lib.js"></script></body></html>',
    base_url="https://example.com/",
)
```

### DOM 操作

```python
ctx.eval("""
    var div = document.createElement('div');
    div.id = 'test';
    document.body.appendChild(div);
    document.getElementById('test').textContent = 'works';
""")
```

---

## 6. 事件循环

```python
ctx.eval("setTimeout(function() { window.fired = true; }, 100)")
ctx.eval("__iv8__.eventLoop.advance(200)")  # 推进 200ms
print(ctx.eval("window.fired"))  # True

# 其他方法
ctx.eval("__iv8__.eventLoop.sleep()")       # 推进到下一个定时器
ctx.eval("__iv8__.eventLoop.drain()")       # 执行所有待处理任务
ctx.eval("__iv8__.eventLoop.getTime()")     # 当前逻辑时间(ms)
ctx.eval("__iv8__.eventLoop.reset()")       # 重置
```

---

## 7. 反检测

### MarkAsUndetectable

```python
ctx.eval("typeof __iv8__")           # "undefined"（真实 [[IsHTMLDDA]]）
ctx.eval("__iv8__ == null")          # True
ctx.eval("Boolean(__iv8__)")         # False
ctx.eval("__iv8__.page.load")        # 仍可访问属性
```

### wrapNative

```python
ctx.eval("""
    function myFn() { return 42; }
    var wrapped = __iv8__.wrapNative(myFn, 'myFn');
    wrapped.toString()  // "function myFn() { [native code] }"
""")
```

### hookNative

```python
ctx.eval("""
    __iv8__.hookNative('Navigator.prototype.userAgent', function() {
        return 'spoofed';
    })
""")
```

### window.chrome

自动安装（app/csi/loadTimes/runtime），含 connect/sendMessage 错误格式。

---

## 8. Debugger 类

JS 层 trace（monkey-patch 方式，不需要 Inspector）。

```python
dbg = iv8_rs.Debugger(ctx)

# 追踪 API 调用
dbg.trace_api("Math.random")
dbg.trace_api("btoa")
dbg.trace_apis(["Date.now", "crypto.getRandomValues"])

# 监视属性读写
dbg.watch_property("navigator", "userAgent", "read")
dbg.watch_property("document", "cookie", "both")

# 执行并获取 trace
ctx.eval("Math.random(); Math.random()")
log = dbg.get_call_log()
# [{"api": "Math.random", "result": "0.4714...", "timestamp": 0.1}, ...]

summary = dbg.get_call_summary()
# {"Math.random": 2, "btoa": 0}

# eval_traced：执行 + 返回结果 + trace
result, log = dbg.eval_traced("Math.random() + 1")
```

---

## 9. CDP API

Python 端直接驱动 V8 Inspector（不需要 Chrome DevTools）。

```python
ctx.with_devtools(port=9229, wait=False)  # wait=False 不等外部连接

# 设断点
bp_id = ctx.cdp_set_breakpoint("script.js", 10, None, "x > 5")

# 移除断点
ctx.cdp_remove_breakpoint(bp_id)

# 暂停时取值（需要先命中断点）
frames = ctx.cdp_get_call_frames()
value = ctx.cdp_evaluate_on_frame(frames[0]["callFrameId"], "x + y")

# 单步
ctx.cdp_step_over()
ctx.cdp_step_into()
ctx.cdp_resume()

# 检查是否暂停
paused = ctx.cdp_process_events()
```

---

## 10. Trace Mode

不暂停的执行追踪（CDP 条件断点 + 副作用记录）。

```python
ctx.with_devtools(port=9230, wait=False)

# 设 trace point
tp_id = ctx.set_trace_point("tdc.js", 1234, 0,
    "JSON.stringify({pc: pc, op: H[pc]})")

# 执行
ctx.eval("TDC.getData(true)")

# 取 trace
trace = ctx.get_trace_log()  # list of recorded values

# 管理
ctx.set_trace_limit(50000)   # 防止内存爆炸
ctx.clear_trace_log()
ctx.remove_trace_point(tp_id)
```

---

## 11. VM Instrumentation

高性能 JSVMP 字节码级 trace（Proxy wrap，~0.5s/50000 条）。

```python
# 自动检测 VM 变量名
vm_info = ctx.detect_chaosvm_vars(tdc_source)
# {"handler_array": "A", "index_array": "Q", "pc": "U", "stack": "S"}

# 插桩 handler 数组
ctx.instrument_chaosvm("A", pc_var="U", stack_var="S",
    capture_stack_depth=3, limit=100000)

# 执行
ctx.eval(tdc_source)
ctx.eval("TDC.getData(true)")

# 取 trace
vm_trace = ctx.get_vm_trace()
# ["0,66,2", "2,11,3", "1518,49,3", ...]  (pc,opcode,stack_depth)

# 清空 / 还原
ctx.clear_vm_trace()
ctx.uninstrument_chaosvm("A")
```

---

## 12. 决定性模式

```python
# Math.random 确定性（同 seed = 同序列）
ctx = iv8_rs.JSContext(random_seed=42)

# crypto.getRandomValues 确定性
ctx = iv8_rs.JSContext(crypto_seed=123)

# 时间冻结
ctx = iv8_rs.JSContext(time_freeze=1700000000000)
# Date.now() = 1700000000000, performance.now() = 0

# 全部组合
ctx = iv8_rs.JSContext(
    random_seed=42,
    crypto_seed=123,
    time_freeze=1700000000000,
)
```

---

## 13. 全局录制

记录所有环境属性读写 + 函数调用。

```python
ctx.start_recording(
    targets=["navigator", "screen", "document", "Math", "crypto"],
    record_reads=True,
    record_writes=True,
    record_calls=True,
    limit=50000,
)

ctx.eval("navigator.userAgent; Math.random()")

recording = ctx.stop_recording()
# ["R,navigator.userAgent,Mozilla/5.0...", "C,Math.random,0.4714"]
```

---

## 14. Profiler 与 Coverage

```python
ctx.with_devtools(port=9231, wait=False)

# CPU Profile
ctx.start_profiler()
ctx.eval("heavyComputation()")
profile = ctx.stop_profiler()  # V8 CPU Profile format

# Code Coverage
ctx.start_coverage()
ctx.eval("someCode()")
coverage = ctx.stop_coverage()  # 函数级覆盖率
```

---

## 15. instrument_source

统一注入：dispatch 表达式替换 + 源码头部 Proxy。

```python
# 自动检测 + 注入
patched_source, vm_info = iv8_rs.instrument_source(tdc_js)

# 手动参数（自动检测失败时）
patched_source, vm_info = iv8_rs.instrument_source(
    tdc_js,
    handler_array="A",
    pc_var="U",
    stack_var="g",
    index_array="Q",
)

# 执行 patched 源码
ctx.eval(patched_source)
ctx.eval("TDC.getData(true)")

# 取统一 trace（每条带 PC + 类型）
trace = ctx.get_unified_trace()
# ["D,0,66,2", "R,11537,screen.width,1280", "C,26588,Math.random,0.7255"]
# D=dispatch, R=read, C=call, W=write

ctx.clear_unified_trace()
```

---

## 16. trace_diff

```python
diff = iv8_rs.trace_diff(trace_a, trace_b)
# {
#   "index": 1234,        # 第一个分歧位置
#   "a": "D,100,5,3",     # trace_a 在该位置的值
#   "b": "D,100,7,3",     # trace_b 在该位置的值
#   "match_count": 1234,  # 分歧前匹配的条数
#   "total_a": 50000,
#   "total_b": 50000,
# }
```

---

