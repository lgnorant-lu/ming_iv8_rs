#!/usr/bin/env python3
"""BrowserForge -> IV8 ProfileSource adapter (D-104).

Generates a statistically consistent browser fingerprint using BrowserForge's
Bayesian network, then converts it to IV8's ProfileSource JSON schema.

Usage:
    .venv\\Scripts\\python.exe scripts\\browserforge_adapter.py
    .venv\\Scripts\\python.exe scripts\\browserforge_adapter.py --output data/browserforge_profile.json
    .venv\\Scripts\\python.exe scripts\\browserforge_adapter.py --profile desktop/windows/chrome --seed 42

See: docs/roadmap/v0.8/analysis/browserforge-integration-design.md
"""

from __future__ import annotations

import argparse
import json
import random
import re
import sys
from pathlib import Path
from typing import Any

try:
    from browserforge.fingerprints import Fingerprint, FingerprintGenerator, Screen
    _BROWSERFORGE_AVAILABLE = True
except ImportError:
    _BROWSERFORGE_AVAILABLE = False


SCHEMA_VERSION = "0.8.32"


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

def generate_profile_source(
    *,
    browser: str | None = None,
    os: str | None = None,
    min_width: int | None = None,
    max_width: int | None = None,
    min_height: int | None = None,
    max_height: int | None = None,
    locale: str | None = None,
    seed: int | None = None,
) -> dict[str, Any]:
    """Generate an IV8 ProfileSource dict from BrowserForge.

    Parameters:
        browser: Target browser ('chrome', 'firefox', 'safari', 'edge').
        os: Target OS ('windows', 'macos', 'linux', 'android', 'ios').
        min_width/max_width/min_height/max_height: Screen constraints.
        locale: Locale string (e.g. 'zh-CN').
        seed: Random seed for reproducibility (affects noise_seed and meta name only;
              BrowserForge's own generation is not seed-controlled).

    Returns:
        IV8 ProfileSource dict matching source.rs schema.
    """
    rng = random.Random(seed)

    if _BROWSERFORGE_AVAILABLE:
        screen = None
        if any(v is not None for v in (min_width, max_width, min_height, max_height)):
            screen = Screen(
                min_width=min_width,
                max_width=max_width,
                min_height=min_height,
                max_height=max_height,
            )
        generator = FingerprintGenerator(screen=screen, strict=False)
        last_err: Exception | None = None
        for _attempt in range(3):
            try:
                fp = generator.generate(browser=browser, os=os, locale=locale)
                break
            except Exception as e:
                last_err = e
                generator = FingerprintGenerator(strict=False)
        else:
            raise last_err  # type: ignore[misc]
    else:
        fp = _stub_fingerprint(browser, os, locale, rng)

    return _fingerprint_to_profile_source(fp, rng)


def profile_source_to_flat_env(source: dict[str, Any]) -> dict[str, Any]:
    """Expand a ProfileSource dict into a flat dot-path env dict.

    Mirrors Rust build_flat_env in matrix.rs. Used for H02 consistency testing
    without requiring the Rust runtime.
    """
    env: dict[str, Any] = {}

    nav = source["navigator"]
    ident = source["identity"]
    disp = source["display"]
    loc = source["locale"]
    perms = source["permissions"]
    media = disp["media"]
    rendering = source["rendering"]
    timing = source["timing"]

    env["config.features.profile"] = source["meta"]["name"]
    env["config.features.browserVersion"] = ident["browser"]["version"]

    env["navigator.userAgent"] = nav["user_agent"]
    env["navigator.appVersion"] = "5.0"
    env["navigator.platform"] = nav["platform"]
    env["navigator.vendor"] = nav["vendor"]
    env["navigator.vendorSub"] = ""
    env["navigator.product"] = "Gecko"
    env["navigator.productSub"] = "20030107"
    env["navigator.language"] = nav["language"]
    env["navigator.hardwareConcurrency"] = nav["hardware_concurrency"]
    env["navigator.deviceMemory"] = nav["device_memory"]
    env["navigator.maxTouchPoints"] = nav["max_touch_points"]
    env["navigator.cookieEnabled"] = True
    env["navigator.onLine"] = True
    env["navigator.doNotTrack"] = None
    env["navigator.webdriver"] = nav["webdriver"]
    env["navigator.appName"] = "Netscape"
    env["navigator.appCodeName"] = "Mozilla"
    env["navigator.pdfViewerEnabled"] = nav["pdf_viewer_enabled"]
    env["navigator.languages"] = nav["languages"]

    uad = nav["user_agent_data"]
    env["navigator.userAgentData.brands"] = uad["brands"]
    env["navigator.userAgentData.mobile"] = uad["mobile"]
    env["navigator.userAgentData.platform"] = uad["platform"]
    env["navigator.userAgentData.architecture"] = uad["architecture"]
    env["navigator.userAgentData.bitness"] = uad["bitness"]
    env["navigator.userAgentData.model"] = ""
    env["navigator.userAgentData.platformVersion"] = uad["platform_version"]
    env["navigator.userAgentData.wow64"] = False
    env["navigator.userAgentData.fullVersionList"] = uad["full_version_list"]

    scr = disp["screen"]
    win = disp["window"]
    env["screen.width"] = scr["width"]
    env["screen.height"] = scr["height"]
    env["screen.availWidth"] = scr["avail_width"]
    env["screen.availHeight"] = scr["avail_height"]
    env["screen.colorDepth"] = scr["color_depth"]
    env["screen.pixelDepth"] = scr["pixel_depth"]
    env["screen.availLeft"] = 0
    env["screen.availTop"] = 0

    env["window.innerWidth"] = win["inner_width"]
    env["window.innerHeight"] = win["inner_height"]
    env["window.devicePixelRatio"] = win["device_pixel_ratio"]
    env["window.outerWidth"] = win["outer_width"]
    env["window.outerHeight"] = win["outer_height"]

    env["webgl.VENDOR"] = ident["gpu"]["vendor"]
    env["webgl.RENDERER"] = ident["gpu"]["renderer"]
    env["webgl.UNMASKED_VENDOR_WEBGL"] = ident["gpu"]["webgl_unmasked_vendor"]
    env["webgl.UNMASKED_RENDERER_WEBGL"] = ident["gpu"]["webgl_unmasked_renderer"]

    env["canvas.mode"] = rendering["canvas_2d"]["mode"]
    env["webgl.mode"] = rendering["webgl_1"]["mode"]

    for name, state in [
        ("geolocation", perms["geolocation"]),
        ("notifications", perms["notifications"]),
        ("camera", perms["camera"]),
        ("microphone", perms["microphone"]),
        ("clipboard-read", perms["clipboard-read"]),
        ("clipboard-write", perms["clipboard-write"]),
        ("local-fonts", perms["local-fonts"]),
    ]:
        env[f"permissions.{name}"] = state
    for name, state in perms.get("extra", {}).items():
        env[f"permissions.{name}"] = state

    for name in (
        "pointer", "hover", "color-gamut", "prefers-color-scheme",
        "prefers-contrast", "prefers-reduced-motion", "prefers-reduced-data",
        "forced-colors", "dynamic-range", "scripting", "update",
        "any-pointer", "any-hover", "display-mode", "inverted-colors",
        "prefers-reduced-transparency",
    ):
        snake = name.replace("-", "_")
        env[f"media.{name}"] = media.get(snake, media.get(name, ""))

    env["fonts.mode"] = rendering["fonts"]["mode"]
    env["fonts.families"] = rendering["fonts"].get("families", [])
    env["audio.mode"] = rendering["audio_context"]["mode"]
    env["audio.baseLatency"] = 0.005
    env["audio.outputLatency"] = 0.01
    env["display.color-gamut"] = media.get("color_gamut", "srgb")

    env["network.effectiveType"] = "4g"
    env["network.downlink"] = 10.0
    env["network.rtt"] = 50
    env["network.saveData"] = False

    env["identity.browser.brand"] = ident["browser"]["brand"]
    env["identity.os"] = ident["os"]
    env["locale.timezone"] = loc["timezone"]

    fps = timing["fps"]
    env["timers.raf_interval_ms"] = 1000.0 / fps if fps > 0 else 16.67

    for k, v in source.get("compat", {}).get("flat_env_overrides", {}).items():
        env[k] = v

    return env


# ---------------------------------------------------------------------------
# Core conversion
# ---------------------------------------------------------------------------

def _fingerprint_to_profile_source(
    fp: Any, rng: random.Random
) -> dict[str, Any]:
    """Convert BrowserForge Fingerprint to IV8 ProfileSource JSON."""

    ua = fp.navigator.userAgent
    os_name, os_version = _parse_os_from_ua(ua, fp.navigator.platform)
    browser_brand, browser_version = _parse_browser_from_ua(ua)

    gpu_config = _resolve_gpu(fp.videoCard, os_name)

    languages = list(fp.navigator.languages) if fp.navigator.languages else ["en"]
    primary_lang = fp.navigator.language or languages[0]
    accept_lang = _build_accept_language(languages)

    noise_seed = rng.getrandbits(64)

    scr = fp.screen
    color_depth = scr.colorDepth or 24
    pixel_depth = scr.pixelDepth or color_depth

    avail_width = scr.availWidth or scr.width
    avail_height = scr.availHeight or scr.height

    outer_width = scr.outerWidth or scr.width
    outer_height = scr.outerHeight or scr.height

    inner_width = scr.innerWidth if scr.innerWidth and scr.innerWidth > 0 else outer_width
    inner_height = (
        scr.innerHeight
        if scr.innerHeight and scr.innerHeight > 0
        else max(outer_height - 111, 1)
    )

    dpr = scr.devicePixelRatio or 1.0

    source: dict[str, Any] = {
        "meta": {
            "schema_version": SCHEMA_VERSION,
            "name": f"browserforge_{browser_brand}_{os_name}_{rng.randint(1000, 9999)}",
            "description": "Generated by BrowserForge Bayesian network",
            "profile_version": "1",
            "provenance": "browserforge",
        },
        "identity": {
            "os": os_name,
            "os_version": os_version,
            "cpu_arch": _arch_from_platform(fp.navigator.platform),
            "cpu_cores": fp.navigator.hardwareConcurrency or 8,
            "memory_gb": fp.navigator.deviceMemory or 8,
            "browser": {"brand": browser_brand, "version": browser_version},
            "gpu": gpu_config,
            "noise_seed": noise_seed,
        },
        "navigator": {
            "user_agent": ua,
            "platform": fp.navigator.platform,
            "vendor": fp.navigator.vendor,
            "language": primary_lang,
            "languages": languages,
            "hardware_concurrency": fp.navigator.hardwareConcurrency or 8,
            "device_memory": fp.navigator.deviceMemory or 8,
            "max_touch_points": fp.navigator.maxTouchPoints or 0,
            "webdriver": False,
            "pdf_viewer_enabled": True,
            "user_agent_data": _build_uad(ua, browser_brand, browser_version),
            "connection": {
                "effective_type": "4g",
                "rtt": 50,
                "downlink": 10.0,
                "save_data": False,
            },
        },
        "display": {
            "screen": {
                "width": scr.width,
                "height": scr.height,
                "avail_width": avail_width,
                "avail_height": avail_height,
                "color_depth": color_depth,
                "pixel_depth": pixel_depth,
            },
            "window": {
                "inner_width": inner_width,
                "inner_height": inner_height,
                "outer_width": outer_width,
                "outer_height": outer_height,
                "device_pixel_ratio": dpr,
            },
            "media": _default_media_prefs(),
        },
        "rendering": _default_rendering(),
        "locale": {
            "timezone": _tz_from_locale(primary_lang),
            "language": primary_lang,
            "languages": languages,
            "accept_language": accept_lang,
            "geolocation": {"mode": "prompt", "based_on_ip": True},
        },
        "network": _default_network(),
        "permissions": _default_permissions(),
        "capabilities": _default_capabilities(),
        "storage": _default_storage(),
        "timing": {"mode": "logical", "fps": 60, "performance_timing": "generated"},
        "compat": {},
    }
    return source


# ---------------------------------------------------------------------------
# OS / browser parsing
# ---------------------------------------------------------------------------

_UA_OS_PATTERNS = [
    (r"Windows NT ([\d.]+)", "windows"),
    (r"Mac OS X ([\d_]+)", "macos"),
    (r"Linux", "linux"),
    (r"Android ([\d.]+)", "android"),
    (r"iPhone OS ([\d_]+)", "ios"),
]

_UA_BROWSER_PATTERNS = [
    (r"Edg/([\d.]+)", "edge"),
    (r"OPR/([\d.]+)", "opera"),
    (r"Chrome/([\d.]+)", "chrome"),
    (r"Firefox/([\d.]+)", "firefox"),
    (r"Version/([\d.]+).*Safari", "safari"),
]


def _parse_os_from_ua(ua: str, platform: str) -> tuple[str, str]:
    for pattern, os_name in _UA_OS_PATTERNS:
        m = re.search(pattern, ua)
        if m:
            version = m.group(1).replace("_", ".")
            if os_name == "windows":
                version = f"{version}.0"
            return os_name, version
    p = platform.lower()
    if "win" in p:
        return "windows", "10.0.0"
    if "mac" in p:
        return "macos", "14.0.0"
    if "linux" in p:
        return "linux", "6.0"
    return "unknown", "0"


def _parse_browser_from_ua(ua: str) -> tuple[str, str]:
    for pattern, brand in _UA_BROWSER_PATTERNS:
        m = re.search(pattern, ua)
        if m:
            version = m.group(1)
            major = version.split(".")[0]
            return brand, major
    return "chrome", "147"


def _arch_from_platform(platform: str) -> str:
    p = platform.lower()
    if "win" in p:
        return "x64"
    if "mac" in p:
        return "arm64"
    if "linux" in p:
        return "x64"
    if "arm" in p:
        return "arm64"
    return "x64"


# ---------------------------------------------------------------------------
# GPU resolution
# ---------------------------------------------------------------------------

def _resolve_gpu(video_card: Any, os_name: str) -> dict[str, str]:
    """Resolve GPU fields from BrowserForge VideoCard + OS context."""
    if video_card is None:
        return _default_gpu(os_name)

    vendor = getattr(video_card, "vendor", "") or ""
    renderer = getattr(video_card, "renderer", "") or ""

    if "ANGLE" in renderer:
        gpu_name = _extract_gpu_vendor_name(renderer)
        return {
            "vendor": gpu_name,
            "renderer": renderer,
            "webgl_unmasked_vendor": f"Google Inc. ({gpu_name})",
            "webgl_unmasked_renderer": renderer,
        }

    if "Intel" in vendor or "Intel" in renderer:
        if os_name == "macos":
            return {
                "vendor": "Intel Inc.",
                "renderer": "Intel Iris OpenGL Engine",
                "webgl_unmasked_vendor": "Intel Inc.",
                "webgl_unmasked_renderer": "ANGLE (Intel, Intel(R) Iris(TM) Plus Graphics, OpenGL)",
            }
        return {
            "vendor": "Intel",
            "renderer": "Intel(R) UHD Graphics",
            "webgl_unmasked_vendor": "Google Inc. (Intel)",
            "webgl_unmasked_renderer": _angle_renderer("Intel", "Intel(R) UHD Graphics", os_name),
        }

    if "NVIDIA" in vendor or "NVIDIA" in renderer:
        return {
            "vendor": "NVIDIA",
            "renderer": renderer,
            "webgl_unmasked_vendor": "Google Inc. (NVIDIA)",
            "webgl_unmasked_renderer": _angle_renderer("NVIDIA", renderer, os_name),
        }

    if "AMD" in vendor or "ATI" in vendor:
        return {
            "vendor": "AMD",
            "renderer": renderer,
            "webgl_unmasked_vendor": "Google Inc. (AMD)",
            "webgl_unmasked_renderer": _angle_renderer("AMD", renderer, os_name),
        }

    if "Apple" in vendor:
        return {
            "vendor": "Apple",
            "renderer": renderer,
            "webgl_unmasked_vendor": "Google Inc. (Apple)",
            "webgl_unmasked_renderer": renderer,
        }

    return _default_gpu(os_name)


def _angle_renderer(vendor: str, renderer: str, os_name: str) -> str:
    backend = {
        "windows": "Direct3D11 vs_5_0 ps_5_0, D3D11",
        "macos": "Metal",
        "linux": "OpenGL",
    }.get(os_name, "OpenGL")
    return f"ANGLE ({vendor}, {renderer}, {backend})"


def _extract_gpu_vendor_name(angle_renderer: str) -> str:
    m = re.match(r"ANGLE \((\w+)", angle_renderer)
    return m.group(1) if m else "Unknown"


def _default_gpu(os_name: str) -> dict[str, str]:
    if os_name == "macos":
        return {
            "vendor": "Apple",
            "renderer": "Apple M1",
            "webgl_unmasked_vendor": "Google Inc. (Apple)",
            "webgl_unmasked_renderer": "ANGLE (Apple, ANGLE Metal Renderer: Apple M1, Unspecified Version)",
        }
    return {
        "vendor": "NVIDIA",
        "renderer": "NVIDIA GeForce RTX 4060",
        "webgl_unmasked_vendor": "Google Inc. (NVIDIA)",
        "webgl_unmasked_renderer": (
            "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 (0x00002882) "
            "Direct3D11 vs_5_0 ps_5_0, D3D11)"
        ),
    }


# ---------------------------------------------------------------------------
# userAgentData (Client Hints)
# ---------------------------------------------------------------------------

def _build_uad(ua: str, brand: str, version: str) -> dict[str, Any]:
    full_version = _extract_full_version(ua, brand)
    brand_map: dict[str, list[tuple[str, str]]] = {
        "chrome": [("Chromium", version), ("Google Chrome", version)],
        "edge": [("Chromium", version), ("Microsoft Edge", version)],
        "firefox": [],
        "safari": [],
    }
    brands = brand_map.get(brand, [])
    full_brands = [{"brand": b, "version": full_version} for b, _ in brands]

    platform_str = (
        "Windows" if "Windows" in ua
        else "macOS" if "Mac" in ua
        else "Linux" if "Linux" in ua
        else "Unknown"
    )

    return {
        "platform": platform_str,
        "platform_version": _extract_platform_version(ua),
        "architecture": "x86",
        "bitness": "64",
        "mobile": False,
        "brands": [{"brand": b, "version": v} for b, v in brands],
        "full_version_list": full_brands,
    }


def _extract_full_version(ua: str, brand: str) -> str:
    patterns = {
        "chrome": r"Chrome/([\d.]+)",
        "edge": r"Edg/([\d.]+)",
    }
    pattern = patterns.get(brand)
    if pattern:
        m = re.search(pattern, ua)
        if m:
            return m.group(1)
    return "147.0.0.0"


def _extract_platform_version(ua: str) -> str:
    m = re.search(r"Windows NT ([\d.]+)", ua)
    if m:
        return f"{m.group(1)}.0"
    m = re.search(r"Mac OS X ([\d_]+)", ua)
    if m:
        return m.group(1).replace("_", ".")
    return "10.0.0"


# ---------------------------------------------------------------------------
# Default section builders (mirror defaults.rs)
# ---------------------------------------------------------------------------

def _default_media_prefs() -> dict[str, str]:
    return {
        "pointer": "fine",
        "hover": "hover",
        "color_gamut": "srgb",
        "prefers_color_scheme": "light",
        "prefers_contrast": "no-preference",
        "prefers_reduced_motion": "no-preference",
        "prefers_reduced_data": "no-preference",
        "forced_colors": "none",
        "dynamic_range": "srgb",
        "scripting": "yes",
        "update": "fast",
        "any_pointer": "coarse",
        "any_hover": "none",
        "display_mode": "browser",
        "inverted_colors": "none",
        "prefers_reduced_transparency": "no-preference",
    }


def _default_rendering() -> dict[str, Any]:
    return {
        "canvas_2d": {"mode": "noise", "sub_seed": None},
        "webgl_1": {"mode": "noise", "sub_seed": None},
        "webgl_2": {"mode": "noise", "sub_seed": None},
        "webgpu": {"mode": "unsupported"},
        "audio_context": {"mode": "noise", "sub_seed": None},
        "client_rects": {"mode": "noise", "sub_seed": None},
        "fonts": {"mode": "common", "families": []},
    }


def _default_network() -> dict[str, Any]:
    return {
        "proxy": None,
        "webrtc": {"mode": "disabled"},
        "dns": {"mode": "system"},
        "headers": {"ua": "profile", "accept_language": "profile", "client_hints": "profile"},
        "tls": {"mode": "unsupported"},
    }


def _default_permissions() -> dict[str, Any]:
    return {
        "geolocation": "prompt",
        "notifications": "prompt",
        "camera": "prompt",
        "microphone": "prompt",
        "clipboard-read": "prompt",
        "clipboard-write": "granted",
        "local-fonts": "prompt",
        "extra": {
            "accelerometer": "granted",
            "gyroscope": "granted",
            "magnetometer": "granted",
            "ambient-light-sensor": "granted",
            "background-sync": "granted",
            "midi": "granted",
            "screen-wake-lock": "granted",
            "push": "prompt",
            "bluetooth": "prompt",
            "persistent-storage": "prompt",
            "idle-detection": "prompt",
            "nfc": "prompt",
            "storage-access": "prompt",
            "window-management": "prompt",
            "payment-handler": "prompt",
            "periodic-background-sync": "prompt",
        },
    }


def _default_capabilities() -> dict[str, bool]:
    return {
        "window_chrome": True,
        "notifications": True,
        "battery": False,
        "bluetooth": False,
        "webgpu": False,
        "media_devices": True,
        "storage": True,
    }


def _default_storage() -> dict[str, Any]:
    return {
        "local_storage": True,
        "session_storage": True,
        "indexed_db": True,
        "cookies": True,
        "history_length": 1,
    }


_LOCALE_TZ = {
    "zh-CN": "Asia/Shanghai",
    "zh-TW": "Asia/Taipei",
    "en-US": "America/New_York",
    "en-GB": "Europe/London",
    "ja": "Asia/Tokyo",
    "ko": "Asia/Seoul",
    "de": "Europe/Berlin",
    "fr": "Europe/Paris",
    "es": "Europe/Madrid",
    "ru": "Europe/Moscow",
}


def _tz_from_locale(lang: str) -> str:
    for key, tz in _LOCALE_TZ.items():
        if lang.startswith(key) or key.startswith(lang):
            return tz
    if lang.startswith("zh"):
        return "Asia/Shanghai"
    if lang.startswith("en"):
        return "America/New_York"
    return "America/New_York"


def _build_accept_language(languages: list[str]) -> str:
    if not languages:
        return "en;q=0.9"
    parts = [languages[0]]
    for i, lang in enumerate(languages[1:], 1):
        parts.append(f"{lang};q={round(1.0 - 0.1 * i, 1)}")
    parts.append("en;q=0.5")
    return ",".join(parts)


# ---------------------------------------------------------------------------
# Stub fingerprint (fallback when browserforge is not installed)
# ---------------------------------------------------------------------------

class _StubScreen:
    def __init__(self, rng: random.Random, os_name: str):
        dims = {
            "windows": (1920, 1080),
            "macos": (1440, 900),
            "linux": (1920, 1080),
        }
        self.width, self.height = dims.get(os_name, (1920, 1080))
        self.availWidth = self.width
        self.availHeight = self.height - 40
        self.colorDepth = 24
        self.pixelDepth = 24
        self.innerWidth = 0
        self.innerHeight = 0
        self.outerWidth = self.width
        self.outerHeight = self.height
        self.devicePixelRatio = 1.0


class _StubNavigator:
    def __init__(self, rng: random.Random, os_name: str, browser: str, locale: str | None):
        os_ua = {
            "windows": "Windows NT 10.0; Win64; x64",
            "macos": "Macintosh; Intel Mac OS X 10_15_7",
            "linux": "X11; Linux x86_64",
        }
        platform = {
            "windows": "Win32",
            "macos": "MacIntel",
            "linux": "Linux x86_64",
        }
        self.platform = platform.get(os_name, "Win32")
        chrome_ver = rng.choice(["131", "147", "128"])
        os_part = os_ua.get(os_name, os_ua["windows"])
        self.userAgent = (
            f"Mozilla/5.0 ({os_part}) AppleWebKit/537.36 "
            f"(KHTML, like Gecko) Chrome/{chrome_ver}.0.0.0 Safari/537.36"
        )
        self.vendor = "Google Inc."
        self.hardwareConcurrency = rng.choice([4, 8, 12, 16])
        self.deviceMemory = rng.choice([4, 8, 16])
        self.maxTouchPoints = 0
        lang = locale or "en-US"
        self.language = lang
        self.languages = [lang]


class _StubVideoCard:
    def __init__(self, os_name: str):
        self.vendor = "Google Inc. (NVIDIA)"
        self.renderer = (
            "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 (0x00002882) "
            "Direct3D11 vs_5_0 ps_5_0, D3D11)"
        )


class _StubFingerprint:
    def __init__(self, rng: random.Random, browser: str | None, os_name: str | None, locale: str | None):
        os_n = os_name or "windows"
        browser_n = browser or "chrome"
        self.navigator = _StubNavigator(rng, os_n, browser_n, locale)
        self.screen = _StubScreen(rng, os_n)
        self.videoCard = _StubVideoCard(os_n)


def _stub_fingerprint(
    browser: str | None, os_name: str | None, locale: str | None, rng: random.Random
) -> _StubFingerprint:
    return _StubFingerprint(rng, browser, os_name, locale)


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def _parse_profile_flag(profile: str) -> dict[str, str | None]:
    """Parse --profile flag like 'desktop/windows/chrome'."""
    parts = profile.split("/")
    result: dict[str, str | None] = {"device": None, "os": None, "browser": None}
    if len(parts) >= 1:
        result["device"] = parts[0]
    if len(parts) >= 2:
        result["os"] = parts[1]
    if len(parts) >= 3:
        result["browser"] = parts[2]
    return result


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Generate IV8 ProfileSource JSON from BrowserForge fingerprints."
    )
    parser.add_argument(
        "--output", "-o",
        type=Path,
        help="Output file path (default: stdout)",
    )
    parser.add_argument(
        "--profile", "-p",
        type=str,
        default="desktop/windows/chrome",
        help="Profile target as device/os/browser (e.g. desktop/windows/chrome)",
    )
    parser.add_argument(
        "--browser", "-b",
        type=str,
        default=None,
        help="Target browser (chrome, firefox, safari, edge)",
    )
    parser.add_argument(
        "--os",
        type=str,
        default=None,
        help="Target OS (windows, macos, linux, android, ios)",
    )
    parser.add_argument(
        "--locale", "-l",
        type=str,
        default=None,
        help="Locale string (e.g. zh-CN, en-US)",
    )
    parser.add_argument(
        "--seed", "-s",
        type=int,
        default=None,
        help="Random seed for reproducibility",
    )
    parser.add_argument(
        "--min-width", type=int, default=None,
    )
    parser.add_argument(
        "--max-width", type=int, default=None,
    )
    parser.add_argument(
        "--min-height", type=int, default=None,
    )
    parser.add_argument(
        "--max-height", type=int, default=None,
    )
    parser.add_argument(
        "--flat-env",
        action="store_true",
        help="Also output flat_env expansion (for H02 testing)",
    )

    args = parser.parse_args(argv)

    parsed = _parse_profile_flag(args.profile)
    browser = args.browser or parsed.get("browser")
    os_name = args.os or parsed.get("os")

    source = generate_profile_source(
        browser=browser,
        os=os_name,
        min_width=args.min_width,
        max_width=args.max_width,
        min_height=args.min_height,
        max_height=args.max_height,
        locale=args.locale,
        seed=args.seed,
    )

    output: dict[str, Any] = {"profile_source": source}
    if args.flat_env:
        output["flat_env"] = profile_source_to_flat_env(source)

    json_str = json.dumps(output, indent=2, ensure_ascii=False)

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json_str, encoding="utf-8")
        print(f"[OK] Profile written to {args.output}")
        print(f"     name: {source['meta']['name']}")
        print(f"     provenance: {source['meta']['provenance']}")
        print(f"     UA: {source['navigator']['user_agent'][:80]}")
        print(f"     platform: {source['navigator']['platform']}")
        print(f"     screen: {source['display']['screen']['width']}x{source['display']['screen']['height']}")
        print(f"     gpu: {source['identity']['gpu']['webgl_unmasked_vendor']}")
    else:
        print(json_str)

    return 0


if __name__ == "__main__":
    sys.exit(main())
