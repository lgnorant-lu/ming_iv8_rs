r"""
iv8 0.1.2 vs iv8-rs 对照探针。
在两个引擎中执行相同的 JS，对比输出差异。
用 .venv-probe 的 iv8 和 .venv 的 iv8_rs。

运行方式：
  .venv\Scripts\python.exe tests\probe_compare.py
"""
import json
import os
import subprocess
import sys

# 探针 JS 代码列表：每个是 (名称, JS表达式)
PROBES = [
    # ─── 基础类型 ───
    ("typeof_undefined", "typeof undefined"),
    ("typeof_null", "typeof null"),
    ("typeof_number", "typeof 42"),
    ("typeof_string", "typeof 'hello'"),
    ("typeof_boolean", "typeof true"),
    ("typeof_function", "typeof function(){}"),
    ("typeof_object", "typeof {}"),
    ("typeof_symbol", "typeof Symbol()"),

    # ─── navigator ───
    ("navigator.userAgent_type", "typeof navigator.userAgent"),
    ("navigator.platform_type", "typeof navigator.platform"),
    ("navigator.language_type", "typeof navigator.language"),
    ("navigator.webdriver", "navigator.webdriver"),
    ("navigator.webdriver_type", "typeof navigator.webdriver"),
    ("navigator.maxTouchPoints_type", "typeof navigator.maxTouchPoints"),
    ("navigator.hardwareConcurrency_type", "typeof navigator.hardwareConcurrency"),

    # ─── screen ───
    ("screen.width_type", "typeof screen.width"),
    ("screen.height_type", "typeof screen.height"),
    ("screen.colorDepth_type", "typeof screen.colorDepth"),

    # ─── window ───
    ("window===globalThis", "window === globalThis"),
    ("window===self", "window === self"),
    ("typeof_window", "typeof window"),

    # ─── chrome ───
    ("typeof_chrome", "typeof chrome"),
    ("typeof_chrome.runtime", "typeof chrome.runtime"),
    ("typeof_chrome.app", "typeof chrome.app"),
    ("chrome.runtime.connect_type", "typeof chrome.runtime.connect"),

    # ─── __iv8__ ───
    ("typeof___iv8__", "typeof __iv8__"),
    ("iv8_in_window", "'__iv8__' in window"),
    ("iv8_keys_visible", "Object.keys(globalThis).includes('__iv8__')"),
    ("typeof_wrapNative", "typeof __iv8__.wrapNative"),
    ("typeof_hookNative", "typeof __iv8__.hookNative"),
    ("typeof_eventLoop", "typeof __iv8__.eventLoop"),

    # ─── wrapNative ───
    ("wrapNative_toString", "__iv8__.wrapNative.toString()"),
    ("wrapNative_fn_toString", "__iv8__.wrapNative(function test(){}, 'test').toString()"),

    # ─── Date/Time ───
    ("typeof_Date.now", "typeof Date.now"),
    ("Date.now_deterministic", "Date.now() === Date.now()"),
    ("typeof_performance.now", "typeof performance.now"),

    # ─── crypto ───
    ("typeof_crypto", "typeof crypto"),
    ("typeof_crypto.subtle", "typeof crypto.subtle"),
    ("typeof_crypto.getRandomValues", "typeof crypto.getRandomValues"),
    ("typeof_crypto.randomUUID", "typeof crypto.randomUUID"),

    # ─── DOM ───
    ("typeof_document", "typeof document"),
    ("typeof_document.getElementById", "typeof document.getElementById"),
    ("typeof_document.querySelector", "typeof document.querySelector"),
    ("typeof_document.createElement", "typeof document.createElement"),

    # ─── 全局函数 ───
    ("typeof_setTimeout", "typeof setTimeout"),
    ("typeof_setInterval", "typeof setInterval"),
    ("typeof_clearTimeout", "typeof clearTimeout"),
    ("typeof_fetch", "typeof fetch"),
    ("typeof_XMLHttpRequest", "typeof XMLHttpRequest"),
    ("typeof_atob", "typeof atob"),
    ("typeof_btoa", "typeof btoa"),
    ("typeof_URL", "typeof URL"),
    ("typeof_URLSearchParams", "typeof URLSearchParams"),
    ("typeof_TextEncoder", "typeof TextEncoder"),
    ("typeof_Event", "typeof Event"),
    ("typeof_location", "typeof location"),
    ("typeof_getComputedStyle", "typeof getComputedStyle"),

    # ─── btoa/atob ───
    ("btoa_hello", "btoa('hello')"),
    ("atob_aGVsbG8=", "atob('aGVsbG8=')"),

    # ─── eventLoop ───
    ("eventLoop.getTime_type", "typeof __iv8__.eventLoop.getTime"),
    ("eventLoop.advance_type", "typeof __iv8__.eventLoop.advance"),
]


def run_iv8_probe(probe_js):
    """Run a JS expression in iv8 0.1.2 and return the result."""
    script = f'''
import iv8
ctx = iv8.JSContext()
try:
    result = ctx.eval({repr(probe_js)})
    print(repr(result))
except Exception as ex:
    print("ERROR:" + type(ex).__name__ + ":" + str(ex)[:100])
finally:
    ctx.close()
'''
    try:
        result = subprocess.run(
            [r".venv-probe\Scripts\python.exe", "-c", script],
            capture_output=True, text=True, timeout=5,
            cwd=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        )
        # iv8 prints a banner on import — strip it
        lines = result.stdout.strip().split('\n')
        # Find the last non-empty line (the actual result)
        for line in reversed(lines):
            if line.strip() and not line.startswith('=') and 'iv8 Version' not in line and 'GitHub' not in line:
                return line.strip()
        return result.stdout.strip()
    except Exception as e:
        return f"EXEC_ERROR:{e}"


def run_iv8rs_probe(probe_js):
    """Run a JS expression in iv8-rs and return the result."""
    script = f'''
import iv8_rs
ctx = iv8_rs.JSContext()
try:
    result = ctx.eval({repr(probe_js)})
    print(repr(result))
except Exception as ex:
    print("ERROR:" + type(ex).__name__ + ":" + str(ex)[:100])
finally:
    ctx.close()
'''
    try:
        result = subprocess.run(
            [r".venv\Scripts\python.exe", "-c", script],
            capture_output=True, text=True, timeout=5,
            cwd=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        )
        return result.stdout.strip()
    except Exception as e:
        return f"EXEC_ERROR:{e}"


def main():
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')

    print("=" * 80)
    print("iv8 0.1.2 vs iv8-rs comparison probe")
    print("=" * 80)
    print()

    results = []
    matches = 0
    mismatches = 0
    errors = 0

    for name, js in PROBES:
        iv8_result = run_iv8_probe(js)
        iv8rs_result = run_iv8rs_probe(js)

        is_match = iv8_result == iv8rs_result
        status = "MATCH" if is_match else "DIFF"

        if is_match:
            matches += 1
        elif "ERROR" in iv8_result or "ERROR" in iv8rs_result:
            errors += 1
            status = "ERR"
        else:
            mismatches += 1

        results.append({
            "name": name,
            "js": js,
            "iv8": iv8_result,
            "iv8rs": iv8rs_result,
            "match": is_match,
        })

        if not is_match:
            print(f"[{status}] {name}")
            print(f"   iv8:    {iv8_result}")
            print(f"   iv8-rs: {iv8rs_result}")
            print()

    print()
    print("=" * 80)
    print(f"总计: {len(PROBES)} 项")
    print(f"  匹配: {matches} [OK]")
    print(f"  不匹配: {mismatches} [FAIL]")
    print(f"  错误: {errors} [WARN]")
    print(f"  匹配率: {matches/len(PROBES)*100:.1f}%")
    print("=" * 80)

    # Save results
    with open("tests/probe_results.json", "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2, ensure_ascii=False)
    print("\n详细结果已保存到 tests/probe_results.json")


if __name__ == "__main__":
    main()
