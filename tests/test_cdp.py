"""Tests for CDP-related API (M30).

NOTE: Starting the V8 inspector server (with_devtools) or calling CDP
methods that access the inspector session causes an AV on the current
platform (V8 inspector not initialized). These tests cover safe API
calls that do not trigger a V8 inspector session access.

Full end-to-end CDP tests require the inspector platform crash to be
resolved first (tracked as a known limitation)."""

from __future__ import annotations

import socket


def test_get_devtools_url_before_start():
    """Verify get_devtools_url returns None before with_devtools."""
    from iv8_rs import JSContext

    ctx = JSContext()
    try:
        url = ctx.get_devtools_url()
        assert url is None
    finally:
        ctx.close()


def test_process_inspector_messages_not_started():
    """Verify process_inspector_messages is a no-op when devtools not started."""
    from iv8_rs import JSContext

    ctx = JSContext()
    try:
        ctx.process_inspector_messages()
    finally:
        ctx.close()


def test_cdp_process_events_not_started():
    """Verify cdp_process_events returns False when devtools not started."""
    from iv8_rs import JSContext

    ctx = JSContext()
    try:
        result = ctx.cdp_process_events()
        assert result is False
    finally:
        ctx.close()


def test_cdp_get_call_frames_not_started():
    """Verify cdp_get_call_frames returns None when devtools not started."""
    from iv8_rs import JSContext

    ctx = JSContext()
    try:
        frames = ctx.cdp_get_call_frames()
        assert frames is None
    finally:
        ctx.close()


def test_with_devtools_reports_bind_failure():
    """with_devtools should fail synchronously if the port is occupied."""
    from iv8_rs import JSContext

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.bind(("127.0.0.1", 0))
    sock.listen(1)
    port = sock.getsockname()[1]

    ctx = JSContext()
    try:
        try:
            ctx.with_devtools(port=port, wait=False)
            raise AssertionError("expected bind failure")
        except RuntimeError as e:
            assert "failed to start DevTools server" in str(e)
    finally:
        ctx.close()
        sock.close()
