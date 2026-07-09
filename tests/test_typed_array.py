"""REQ-V2-CONV-005: TypedArray preserves element types when strict_compat=False.

In strict_compat=True (default, v0.1) TypedArrays convert to bytes (raw
memcpy). In strict_compat=False they convert to Python list of typed
scalars (int / float). Round-trip via expose() also preserves the kind.
"""

import iv8_rs

# ---------- strict_compat=True: bytes (back-compat) ----------

def test_uint8_array_strict_compat_returns_bytes():
    ctx = iv8_rs.JSContext(strict_compat=True)
    val = ctx.eval("new Uint8Array([1, 2, 3])")
    assert isinstance(val, bytes)
    assert val == b"\x01\x02\x03"


def test_int32_array_strict_compat_returns_bytes():
    ctx = iv8_rs.JSContext(strict_compat=True)
    val = ctx.eval("new Int32Array([0x01020304])")
    assert isinstance(val, bytes)
    # Little-endian
    assert val == b"\x04\x03\x02\x01"


# ---------- strict_compat=False: typed Python list ----------

def test_uint8_array_returns_int_list():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Uint8Array([1, 255, 0])")
    assert val == [1, 255, 0]
    assert all(isinstance(x, int) for x in val)


def test_int8_array_signed_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Int8Array([-128, 0, 127])")
    assert val == [-128, 0, 127]


def test_uint16_array_max_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Uint16Array([0, 65535])")
    assert val == [0, 65535]


def test_int16_array_signed():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Int16Array([-32768, 32767])")
    assert val == [-32768, 32767]


def test_uint32_array_large_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Uint32Array([0, 4294967295])")
    assert val == [0, 4294967295]


def test_int32_array_signed():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Int32Array([-2147483648, 2147483647])")
    assert val == [-2147483648, 2147483647]


def test_float32_array_returns_floats():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Float32Array([1.5, -2.5, 0.0])")
    assert val[0] == 1.5
    assert val[1] == -2.5
    assert val[2] == 0.0
    assert all(isinstance(x, float) for x in val)


def test_float64_array_full_precision():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Float64Array([3.141592653589793, -1e-300])")
    assert val[0] == 3.141592653589793
    assert val[1] == -1e-300


def test_uint8_clamped_array_small_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Uint8ClampedArray([0, 128, 255])")
    assert val == [0, 128, 255]


def test_bigint64_array_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new BigInt64Array([0n, 9223372036854775807n, -9223372036854775808n])")
    assert val[0] == 0
    assert val[1] == 9223372036854775807
    assert val[2] == -9223372036854775808


def test_biguint64_array_values():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new BigUint64Array([0n, 18446744073709551615n])")
    assert val == [0, 18446744073709551615]


# ---------- empty / edge cases ----------

def test_empty_typed_array():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("new Uint8Array(0)")
    assert val == []


def test_typed_array_in_object():
    ctx = iv8_rs.JSContext(strict_compat=False)
    val = ctx.eval("({data: new Uint16Array([1, 2, 3])})")
    assert val == {"data": [1, 2, 3]}
