"""
Task 62: 内存泄漏验证（100 轮长跑）

验证 iv8-rs 在 100 轮 JSContext 创建/使用/销毁循环中内存稳定。
目标：≤ +5MB 漂移（iv8 基准 +2MB）。
"""
import pytest
import gc
import iv8_rs


def get_memory_mb():
    """获取当前进程内存使用（MB）。"""
    try:
        import psutil
        import os
        process = psutil.Process(os.getpid())
        return process.memory_info().rss / 1024 / 1024
    except ImportError:
        return None


class TestMemoryStability:
    """内存稳定性测试。"""

    def test_100_context_cycles_no_leak(self):
        """100 轮 JSContext 创建/eval/销毁，内存漂移 ≤ +5MB。"""
        gc.collect()
        mem_before = get_memory_mb()

        for i in range(100):
            ctx = iv8_rs.JSContext()
            # 执行一些典型操作
            ctx.eval("navigator.userAgent + screen.width")
            ctx.eval("Date.now()")
            ctx.eval("Math.random()")
            ctx.close()

        gc.collect()
        mem_after = get_memory_mb()

        if mem_before is not None and mem_after is not None:
            drift_mb = mem_after - mem_before
            print(f"\nMemory drift after 100 cycles: {drift_mb:.1f} MB")
            assert drift_mb <= 5.0, f"Memory drift too large: {drift_mb:.1f} MB > 5 MB"
        else:
            # psutil not available, just verify no crash
            pass

    def test_100_page_load_cycles(self):
        """100 轮 page_load 循环，内存稳定。"""
        gc.collect()
        mem_before = get_memory_mb()

        html = "<html><body><div id='t'></div><p class='item'>text</p></body></html>"
        for i in range(100):
            ctx = iv8_rs.JSContext()
            ctx.page_load(html)
            ctx.eval("document.getElementById('t') !== null")
            ctx.eval("document.querySelector('.item').textContent")
            ctx.close()

        gc.collect()
        mem_after = get_memory_mb()

        if mem_before is not None and mem_after is not None:
            drift_mb = mem_after - mem_before
            print(f"\nMemory drift after 100 page_load cycles: {drift_mb:.1f} MB")
            assert drift_mb <= 10.0, f"Memory drift too large: {drift_mb:.1f} MB > 10 MB"

    def test_50_crypto_cycles(self):
        """50 轮 crypto 操作循环，内存稳定。"""
        gc.collect()
        mem_before = get_memory_mb()

        for i in range(50):
            ctx = iv8_rs.JSContext()
            ctx.eval("crypto.getRandomValues(new Uint8Array(32))")
            ctx.eval("crypto.randomUUID()")
            ctx.eval_promise("""
                crypto.subtle.digest('SHA-256', new Uint8Array([1,2,3]))
                    .then(function(buf) { return buf.byteLength; })
            """)
            ctx.close()

        gc.collect()
        mem_after = get_memory_mb()

        if mem_before is not None and mem_after is not None:
            drift_mb = mem_after - mem_before
            print(f"\nMemory drift after 50 crypto cycles: {drift_mb:.1f} MB")
            assert drift_mb <= 5.0, f"Memory drift too large: {drift_mb:.1f} MB > 5 MB"

    def test_single_context_long_run(self):
        """单个 context 执行 1000 次 eval，内存稳定。"""
        ctx = iv8_rs.JSContext()
        gc.collect()
        mem_before = get_memory_mb()

        for i in range(1000):
            ctx.eval(f"var x{i % 10} = {i}; x{i % 10} + 1")

        gc.collect()
        mem_after = get_memory_mb()

        if mem_before is not None and mem_after is not None:
            drift_mb = mem_after - mem_before
            print(f"\nMemory drift after 1000 evals: {drift_mb:.1f} MB")
            assert drift_mb <= 5.0, f"Memory drift too large: {drift_mb:.1f} MB > 5 MB"

        ctx.close()

    def test_context_manager_no_leak(self):
        """with 语句 context manager 不泄漏。"""
        gc.collect()
        mem_before = get_memory_mb()

        for i in range(50):
            with iv8_rs.JSContext() as ctx:
                ctx.eval("1+1")
                ctx.eval("navigator.userAgent")

        gc.collect()
        mem_after = get_memory_mb()

        if mem_before is not None and mem_after is not None:
            drift_mb = mem_after - mem_before
            print(f"\nMemory drift after 50 context manager cycles: {drift_mb:.1f} MB")
            assert drift_mb <= 5.0, f"Memory drift too large: {drift_mb:.1f} MB > 5 MB"

    def test_resource_bundle_no_leak(self):
        """add_resource 不泄漏。"""
        gc.collect()
        mem_before = get_memory_mb()

        for i in range(50):
            ctx = iv8_rs.JSContext()
            for j in range(10):
                ctx.add_resource(f'https://example.com/resource_{j}', f'data_{j}' * 100, 200)
            ctx.eval("1+1")
            ctx.close()

        gc.collect()
        mem_after = get_memory_mb()

        if mem_before is not None and mem_after is not None:
            drift_mb = mem_after - mem_before
            print(f"\nMemory drift after 50 resource bundle cycles: {drift_mb:.1f} MB")
            assert drift_mb <= 10.0, f"Memory drift too large: {drift_mb:.1f} MB > 10 MB"
