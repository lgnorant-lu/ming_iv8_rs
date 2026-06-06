"""M0 smoke test — verifies all core features work end-to-end."""
import threading

import iv8_rs


def test_basic_eval():
    ctx = iv8_rs.JSContext()
    assert ctx.eval("1 + 1") == 2
    assert ctx.eval("'hello'") == "hello"
    assert ctx.eval("true") is True
    assert ctx.eval("false") is False
    assert ctx.eval("null") is None
    assert ctx.eval("undefined") is None
    assert ctx.eval("[1, 2, 3]") == [1, 2, 3]
    assert ctx.eval("({a: 1, b: 'two'})") == {"a": 1, "b": "two"}
    ctx.close()


def test_state_persistence():
    ctx = iv8_rs.JSContext()
    ctx.eval("var x = 42")
    assert ctx.eval("x + 1") == 43
    ctx.close()


def test_js_error():
    ctx = iv8_rs.JSContext()
    try:
        ctx.eval("throw new TypeError('boom')")
        assert False, "should have raised"
    except iv8_rs.JSError as e:
        assert "boom" in str(e)
    ctx.close()


def test_compile_error():
    ctx = iv8_rs.JSContext()
    try:
        ctx.eval("function(")
        assert False, "should have raised"
    except iv8_rs.JSCompileError as e:
        assert "at" in str(e) or ":" in str(e)
    ctx.close()


def test_get_defaults():
    defaults = iv8_rs.JSContext.get_defaults()
    assert len(defaults) >= 390
    assert "Chrome" in defaults["navigator.userAgent"]


def test_close_and_disposed():
    ctx = iv8_rs.JSContext()
    assert not ctx.is_disposed()
    ctx.close()
    assert ctx.is_disposed()
    ctx.close()

    try:
        ctx.eval("1 + 1")
        assert False, "closed context should reject eval"
    except RuntimeError as e:
        assert "closed" in str(e)


def test_cross_thread_drop_does_not_crash():
    holder = [iv8_rs.JSContext()]
    assert holder[0].eval("1 + 1") == 2

    def worker():
        holder.pop()

    t = threading.Thread(target=worker)
    t.start()
    t.join()
    assert holder == []


def test_typed_array_to_bytes():
    ctx = iv8_rs.JSContext()
    result = ctx.eval("new Uint8Array([1, 2, 3])")
    assert result == b"\x01\x02\x03"
    ctx.close()


def test_float_special_values():
    import math
    ctx = iv8_rs.JSContext()
    assert math.isnan(ctx.eval("NaN"))
    assert ctx.eval("Infinity") == float("inf")
    assert ctx.eval("-Infinity") == float("-inf")
    ctx.close()


def test_nested_objects():
    ctx = iv8_rs.JSContext()
    result = ctx.eval("({x: {y: {z: 42}}})")
    assert result["x"]["y"]["z"] == 42
    ctx.close()


def test_empty_eval():
    ctx = iv8_rs.JSContext()
    assert ctx.eval("") is None
    ctx.close()


if __name__ == "__main__":
    test_basic_eval()
    test_state_persistence()
    test_js_error()
    test_compile_error()
    test_get_defaults()
    test_close_and_disposed()
    test_typed_array_to_bytes()
    test_float_special_values()
    test_nested_objects()
    test_empty_eval()
    print("ALL 10 SMOKE TESTS PASSED [OK]")
