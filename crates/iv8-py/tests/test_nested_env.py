"""Task 75: Nested environment format support tests.

Acceptance criteria:
- {"navigator": {"userAgent": "X"}} → navigator.userAgent === "X"
- {"location": {"href": "Y", "hostname": "Z"}} → location.href === "Y"
- Mixed flat + nested works
- Deep nesting works (3+ levels)
- Flat format still works (backward compatible)
"""
import iv8_rs


def test_nested_navigator():
    ctx = iv8_rs.JSContext(environment={
        "navigator": {"userAgent": "Nested/1.0", "platform": "TestOS"}
    })
    assert ctx.eval("navigator.userAgent") == "Nested/1.0"
    assert ctx.eval("navigator.platform") == "TestOS"
    ctx.close()


def test_nested_location():
    ctx = iv8_rs.JSContext(environment={
        "location": {
            "href": "https://example.com/page",
            "hostname": "example.com",
            "protocol": "https:",
            "origin": "https://example.com",
        }
    })
    assert ctx.eval("location.href") == "https://example.com/page"
    assert ctx.eval("location.hostname") == "example.com"
    ctx.close()


def test_nested_screen():
    ctx = iv8_rs.JSContext(environment={
        "screen": {"width": 2560, "height": 1440, "colorDepth": 30}
    })
    assert ctx.eval("screen.width") == 2560
    assert ctx.eval("screen.height") == 1440
    assert ctx.eval("screen.colorDepth") == 30
    ctx.close()


def test_flat_format_still_works():
    """Backward compatibility: flat format unchanged."""
    ctx = iv8_rs.JSContext(environment={
        "navigator.userAgent": "Flat/2.0",
        "screen.width": 1920,
    })
    assert ctx.eval("navigator.userAgent") == "Flat/2.0"
    assert ctx.eval("screen.width") == 1920
    ctx.close()


def test_mixed_flat_and_nested():
    """Both formats can coexist."""
    ctx = iv8_rs.JSContext(environment={
        "navigator": {"userAgent": "Mixed/1.0"},
        "screen.width": 1080,
    })
    assert ctx.eval("navigator.userAgent") == "Mixed/1.0"
    assert ctx.eval("screen.width") == 1080
    ctx.close()


def test_deep_nesting():
    """3+ level nesting flattens correctly."""
    ctx = iv8_rs.JSContext(environment={
        "navigator": {
            "userAgentData": {
                "brands": [{"brand": "Chromium", "version": "120"}]
            }
        }
    })
    # This should create navigator.userAgentData.brands as a value
    result = ctx.eval("navigator.userAgentData.brands")
    assert isinstance(result, list)
    assert result[0]["brand"] == "Chromium"
    ctx.close()


def test_abogus_style_environment():
    """Exact format used in iv8's abogus example."""
    ctx = iv8_rs.JSContext(environment={
        "location": {
            "href": "https://www.douyin.com/video/123",
            "origin": "https://www.douyin.com",
            "protocol": "https:",
            "host": "www.douyin.com",
            "hostname": "www.douyin.com",
            "port": "",
            "pathname": "/video/123",
            "search": "",
            "hash": ""
        },
        "navigator": {
            "userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/124.0.0.0",
            "language": "en-US",
            "languages": ["en-US", "en"],
        },
    })
    assert ctx.eval("location.hostname") == "www.douyin.com"
    assert ctx.eval("navigator.userAgent").startswith("Mozilla/5.0")
    assert ctx.eval("navigator.language") == "en-US"
    ctx.close()
