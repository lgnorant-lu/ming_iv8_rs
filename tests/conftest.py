"""Shared pytest fixtures for iv8-rs test suite."""
from __future__ import annotations

from collections.abc import Iterator

import pytest

iv8_rs = pytest.importorskip("iv8_rs")


@pytest.fixture
def ctx() -> Iterator[iv8_rs.JSContext]:
    """Basic JSContext with default environment."""
    c = iv8_rs.JSContext()
    yield c
    c.close()


@pytest.fixture
def ctx_custom() -> Iterator[iv8_rs.JSContext]:
    """JSContext with custom Chrome 124 fingerprint."""
    c = iv8_rs.JSContext(environment={
        "navigator.userAgent": (
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
            "AppleWebKit/537.36 (KHTML, like Gecko) "
            "Chrome/124.0.0.0 Safari/537.36"
        ),
        "navigator.platform": "Win32",
        "navigator.language": "zh-CN",
        "navigator.languages": '["zh-CN", "en"]',
        "navigator.hardwareConcurrency": 16,
        "navigator.deviceMemory": 8,
        "navigator.maxTouchPoints": 0,
        "screen.width": 1920,
        "screen.height": 1080,
        "screen.availWidth": 1920,
        "screen.availHeight": 1040,
        "screen.colorDepth": 24,
        "screen.pixelDepth": 24,
        "webgl.UNMASKED_VENDOR_WEBGL": "Google Inc. (NVIDIA)",
        "webgl.UNMASKED_RENDERER_WEBGL": (
            "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 Direct3D11)"
        ),
    })
    yield c
    c.close()


@pytest.fixture
def ctx_seeded() -> Iterator[iv8_rs.JSContext]:
    """JSContext with fixed random seed and frozen time for deterministic tests."""
    c = iv8_rs.JSContext(random_seed=42, time_freeze=1700000000000)
    yield c
    c.close()
