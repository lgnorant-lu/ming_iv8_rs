"""Task 22: expose Python-side integration tests."""
import iv8_rs
import pytest


def test_expose_basic_add():
    ctx = iv8_rs.JSContext()
    ctx.expose("add", lambda a, b: a + b)
    assert ctx.eval("add(3, 4)") == 7
    ctx.close()


def test_expose_string_return():
    ctx = iv8_rs.JSContext()
    ctx.expose("greet", lambda name: f"hello {name}")
    assert ctx.eval('greet("world")') == "hello world"
    ctx.close()


def test_expose_no_args():
    ctx = iv8_rs.JSContext()
    ctx.expose("getVersion", lambda: "0.1.0")
    assert ctx.eval("getVersion()") == "0.1.0"
    ctx.close()


def test_expose_python_exception_becomes_js_error():
    ctx = iv8_rs.JSContext()

    def fail():
        raise ValueError("boom")

    ctx.expose("fail", fail)
    with pytest.raises(iv8_rs.JSError) as exc_info:
        ctx.eval("fail()")
    assert "boom" in str(exc_info.value)
    ctx.close()


def test_expose_return_dict():
    ctx = iv8_rs.JSContext()
    ctx.expose("getObj", lambda: {"a": 1, "b": "two"})
    result = ctx.eval("JSON.stringify(getObj())")
    assert '"a":1' in result or '"a": 1' in result
    assert '"b":"two"' in result or '"b": "two"' in result
    ctx.close()


def test_expose_return_list():
    ctx = iv8_rs.JSContext()
    ctx.expose("getArr", lambda: [1, 2, 3])
    assert ctx.eval("getArr().length") == 3
    assert ctx.eval("getArr()[0]") == 1
    assert ctx.eval("getArr()[2]") == 3
    ctx.close()


def test_expose_return_none_is_undefined():
    ctx = iv8_rs.JSContext()
    ctx.expose("noop", lambda: None)
    assert ctx.eval("noop() === undefined") is True
    ctx.close()


def test_expose_typed_args():
    """JS types are correctly converted to Python types."""
    ctx = iv8_rs.JSContext()
    received = {}

    def capture(a, b, c):
        received["a"] = (type(a).__name__, a)
        received["b"] = (type(b).__name__, b)
        received["c"] = (type(c).__name__, c)
        return "ok"

    ctx.expose("capture", capture)
    ctx.eval('capture(42, "hi", true)')
    assert received["a"] == ("int", 42)
    assert received["b"] == ("str", "hi")
    assert received["c"] == ("bool", True)
    ctx.close()


def test_expose_null_undefined_args():
    """null and undefined both become None in Python."""
    ctx = iv8_rs.JSContext()
    received = []

    def capture(a, b):
        received.append(a)
        received.append(b)

    ctx.expose("capture", capture)
    ctx.eval("capture(null, undefined)")
    assert received == [None, None]
    ctx.close()


def test_expose_array_arg():
    """JS array becomes Python list."""
    ctx = iv8_rs.JSContext()
    received = []

    def capture(arr):
        received.append(arr)

    ctx.expose("capture", capture)
    ctx.eval("capture([1, 2, 3])")
    assert received == [[1, 2, 3]]
    ctx.close()


def test_expose_object_arg():
    """JS object becomes Python dict."""
    ctx = iv8_rs.JSContext()
    received = []

    def capture(obj):
        received.append(obj)

    ctx.expose("capture", capture)
    ctx.eval('capture({x: 1, y: "two"})')
    assert received[0]["x"] == 1
    assert received[0]["y"] == "two"
    ctx.close()


def test_expose_stateful():
    """Exposed function can maintain state via closure."""
    ctx = iv8_rs.JSContext()
    counter = [0]

    def increment():
        counter[0] += 1
        return counter[0]

    ctx.expose("increment", increment)
    assert ctx.eval("increment()") == 1
    assert ctx.eval("increment()") == 2
    assert ctx.eval("increment()") == 3
    ctx.close()


def test_expose_not_callable_raises_typeerror():
    ctx = iv8_rs.JSContext()
    with pytest.raises(TypeError):
        ctx.expose("bad", 42)
    ctx.close()


def test_expose_return_float():
    ctx = iv8_rs.JSContext()
    ctx.expose("pi", lambda: 3.14159)
    result = ctx.eval("pi()")
    assert abs(result - 3.14159) < 1e-10
    ctx.close()


def test_expose_return_bool():
    ctx = iv8_rs.JSContext()
    ctx.expose("isEven", lambda n: n % 2 == 0)
    assert ctx.eval("isEven(4)") is True
    assert ctx.eval("isEven(3)") is False
    ctx.close()


def test_expose_return_bytes():
    """Python bytes → JS Uint8Array → Python bytes."""
    ctx = iv8_rs.JSContext()
    ctx.expose("getData", lambda: b"\x01\x02\x03")
    result = ctx.eval("getData()")
    assert result == b"\x01\x02\x03"
    ctx.close()


def test_expose_multiple_functions():
    """Multiple expose calls coexist."""
    ctx = iv8_rs.JSContext()
    ctx.expose("double", lambda x: x * 2)
    ctx.expose("triple", lambda x: x * 3)
    assert ctx.eval("double(5) + triple(5)") == 25
    ctx.close()
