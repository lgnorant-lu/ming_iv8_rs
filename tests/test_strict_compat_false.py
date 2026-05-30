"""M7: type conversion enhancements when strict_compat=False.

Covers the strict_compat=False enhancements that resolve L-09/L-10 and add
new BigInt/Date/Map/Set support. strict_compat=True (default) still produces
v0.1 behavior — verified for regression elsewhere.
"""

import datetime

import pytest

import iv8_rs


# ---------- BigInt -> Python int ----------

def test_bigint_small_positive():
    ctx = iv8_rs.JSContext(strict_compat=False)
    assert ctx.eval("123n") == 123

def test_bigint_small_negative():
    ctx = iv8_rs.JSContext(strict_compat=False)
    assert ctx.eval("-456n") == -456

def test_bigint_zero():
    ctx = iv8_rs.JSContext(strict_compat=False)
    assert ctx.eval("0n") == 0

def test_bigint_large_positive():
    ctx = iv8_rs.JSContext(strict_compat=False)
    # 2^100 = 1267650600228229401496703205376 (well beyond i64)
    big = 2 ** 100
    assert ctx.eval(f"{big}n") == big

def test_bigint_large_negative():
    ctx = iv8_rs.JSContext(strict_compat=False)
    big = -(2 ** 200)
    assert ctx.eval(f"({big}n)") == big

def test_bigint_arithmetic_result():
    ctx = iv8_rs.JSContext(strict_compat=False)
    assert ctx.eval("(2n ** 64n) + 1n") == 2 ** 64 + 1

def test_bigint_returns_python_int_type():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("999999999999999999999n")
    assert isinstance(val, int)
    assert not isinstance(val, bool)

def test_bigint_strict_compat_true_returns_none():
    # Default mode preserves v0.1 behavior (BigInt -> None + error log).
    ctx = iv8_rs.JSContext()
    assert ctx.eval("1n") is None


# ---------- Date -> Python datetime ----------

def test_date_epoch():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Date(0)")
    assert isinstance(val, datetime.datetime)
    assert val == datetime.datetime(1970, 1, 1, tzinfo=datetime.timezone.utc)

def test_date_specific_timestamp():
    ctx = iv8_rs.JSContext(strict_compat=False)
    # 2026-01-01 00:00:00 UTC = 1767225600000 ms
    val = ctx.eval("new Date(1767225600000)")
    assert val == datetime.datetime(2026, 1, 1, tzinfo=datetime.timezone.utc)

def test_date_milliseconds_precision():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Date(1234567890123)")
    expected = datetime.datetime.fromtimestamp(
        1234567890.123, tz=datetime.timezone.utc
    )
    assert val == expected

def test_date_strict_compat_true_returns_string():
    ctx = iv8_rs.JSContext()
    val = ctx.eval("new Date(0)")
    assert isinstance(val, str)
    assert "[object Date]" in val or "1970" in val


# ---------- Map -> Python dict ----------

def test_map_string_keys():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Map([['a', 1], ['b', 2]])")
    assert val == {"a": 1, "b": 2}

def test_map_preserves_insertion_order():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Map([['z', 1], ['a', 2], ['m', 3]])")
    assert list(val.keys()) == ["z", "a", "m"]

def test_map_numeric_keys_preserved():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Map([[1, 'one'], [2, 'two']])")
    assert val == {1: "one", 2: "two"}

def test_map_empty():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Map()")
    assert val == {}

def test_map_strict_compat_true_returns_string():
    ctx = iv8_rs.JSContext()
    val = ctx.eval("new Map([['a', 1]])")
    assert isinstance(val, str)
    assert "[object Map]" == val


# ---------- Set -> Python set ----------

def test_set_string_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Set(['a', 'b', 'c'])")
    assert val == {"a", "b", "c"}

def test_set_numeric_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Set([1, 2, 3, 1, 2])")
    assert val == {1, 2, 3}

def test_set_empty():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Set()")
    assert val == set()

def test_set_strict_compat_true_returns_string():
    ctx = iv8_rs.JSContext()
    val = ctx.eval("new Set([1, 2])")
    assert isinstance(val, str)
    assert "[object Set]" == val


# ---------- Composition ----------

def test_object_with_bigint_value():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("({timestamp: 9999999999999n})")
    assert val["timestamp"] == 9999999999999

def test_array_of_dates():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("[new Date(0), new Date(1000)]")
    assert len(val) == 2
    assert val[0] == datetime.datetime(1970, 1, 1, tzinfo=datetime.timezone.utc)
    assert val[1] == datetime.datetime(
        1970, 1, 1, 0, 0, 1, tzinfo=datetime.timezone.utc
    )

def test_map_with_object_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Map([['outer', {inner: 42}]])")
    assert val == {"outer": {"inner": 42}}
