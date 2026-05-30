"""REQ-V2-NET-002: confirm Python network handler is always-on.

The handler runs as the second tier in the three-layer fallback chain
(ResourceBundle -> Python handler -> NetworkError) regardless of the
strict_compat flag. This was implicitly the case in v0.1; v0.2 makes it
explicit and adds tests for both modes plus error paths.
"""

import iv8_rs


def _make_handler(captured):
    def handler(url, method):
        captured.append((method, url))
        return (200, b'{"ok": true}')
    return handler


def test_handler_runs_in_strict_compat_true():
    captured = []
    ctx = iv8_rs.JSContext(strict_compat=True)
    ctx.set_network_handler(_make_handler(captured))

    ctx.eval(
        """
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.test/x', false);
        xhr.send();
        """
    )

    assert captured == [("GET", "https://api.test/x")]


def test_handler_runs_in_strict_compat_false():
    captured = []
    ctx = iv8_rs.JSContext(strict_compat=False)
    ctx.set_network_handler(_make_handler(captured))

    ctx.eval(
        """
        var xhr = new XMLHttpRequest();
        xhr.open('POST', 'https://api.test/post', false);
        xhr.send();
        """
    )

    assert captured == [("POST", "https://api.test/post")]


def test_resource_bundle_takes_priority_over_handler():
    captured = []
    ctx = iv8_rs.JSContext()
    ctx.add_resource("https://api.test/cached", b"from-bundle", 200)
    ctx.set_network_handler(_make_handler(captured))

    result = ctx.eval(
        """
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.test/cached', false);
        xhr.send();
        xhr.responseText
        """
    )
    assert result == "from-bundle"
    assert captured == []  # handler not invoked


def test_handler_returns_none_falls_through_to_error():
    ctx = iv8_rs.JSContext()
    ctx.set_network_handler(lambda url, method: None)

    result = ctx.eval(
        """
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://offline.test/missing', false);
        xhr.send();
        xhr.status
        """
    )
    # XHR signals network error via status=0 (browser convention)
    assert result == 0


def test_clear_handler_disables_callback():
    captured = []
    ctx = iv8_rs.JSContext()
    ctx.set_network_handler(_make_handler(captured))
    ctx.clear_network_handler()

    # No bundle, no handler -> network error
    ctx.eval(
        """
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.test/x', false);
        xhr.send();
        """
    )

    assert captured == []


def test_handler_works_with_fetch_too():
    captured = []
    ctx = iv8_rs.JSContext()
    ctx.set_network_handler(_make_handler(captured))

    ctx.eval("fetch('https://api.test/fetched').then(r => r.text())")
    assert captured == [("GET", "https://api.test/fetched")]


def test_set_handler_rejects_non_callable():
    ctx = iv8_rs.JSContext()
    try:
        ctx.set_network_handler("not callable")
        raise AssertionError("expected TypeError")
    except TypeError as e:
        assert "callable" in str(e)
