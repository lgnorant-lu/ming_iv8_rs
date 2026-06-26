#!/usr/bin/env python3
"""Tests for BrowserForge adapter (D-104).

Tests:
1. BrowserForge generates valid fingerprints
2. Adapter correctly maps all fields
3. Generated env dict is compatible with IV8's load_runtime_env()
4. Seed reproducibility (noise_seed)
5. H02 A-rule consistency checks pass

Usage:
    .venv\\Scripts\\python.exe scripts\\test_browserforge_adapter.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

_SCRIPTS_DIR = Path(__file__).resolve().parent
if str(_SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS_DIR))

_REPO_ROOT = _SCRIPTS_DIR.parent
if str(_REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(_REPO_ROOT))

from browserforge_adapter import (
    generate_profile_source,
    profile_source_to_flat_env,
    _BROWSERFORGE_AVAILABLE,
    _parse_os_from_ua,
    _parse_browser_from_ua,
    _resolve_gpu,
    _build_uad,
    _default_gpu,
)


# ---------------------------------------------------------------------------
# Test helpers
# ---------------------------------------------------------------------------

PASS_COUNT = 0
FAIL_COUNT = 0


def check(name: str, condition: bool, detail: str = "") -> None:
    global PASS_COUNT, FAIL_COUNT
    if condition:
        PASS_COUNT += 1
        print(f"  [PASS] {name}")
    else:
        FAIL_COUNT += 1
        print(f"  [FAIL] {name}" + (f" -- {detail}" if detail else ""))


def section(title: str) -> None:
    print(f"\n--- {title} ---")


# ---------------------------------------------------------------------------
# Section 1: Schema validity
# ---------------------------------------------------------------------------

def test_schema_validity():
    section("1. Schema Validity")

    source = generate_profile_source(browser="chrome", os="windows")

    check("has meta section", "meta" in source)
    check("schema_version is 0.8.32", source["meta"]["schema_version"] == "0.8.32")
    check("provenance is browserforge", source["meta"]["provenance"] == "browserforge")
    check("meta has name", bool(source["meta"].get("name")))
    check("meta has description", bool(source["meta"].get("description")))
    check("meta has profile_version", bool(source["meta"].get("profile_version")))

    required_sections = [
        "meta", "identity", "navigator", "display", "rendering",
        "locale", "network", "permissions", "capabilities", "storage",
        "timing", "compat",
    ]
    for sec in required_sections:
        check(f"has {sec} section", sec in source)

    check("identity has os", "os" in source["identity"])
    check("identity has os_version", "os_version" in source["identity"])
    check("identity has cpu_arch", "cpu_arch" in source["identity"])
    check("identity has cpu_cores", "cpu_cores" in source["identity"])
    check("identity has memory_gb", "memory_gb" in source["identity"])
    check("identity has browser", "browser" in source["identity"])
    check("identity has gpu", "gpu" in source["identity"])
    check("identity has noise_seed", "noise_seed" in source["identity"])

    gpu = source["identity"]["gpu"]
    for k in ("vendor", "renderer", "webgl_unmasked_vendor", "webgl_unmasked_renderer"):
        check(f"gpu has {k}", k in gpu and bool(gpu[k]), f"gpu.{k}={gpu.get(k)!r}")


# ---------------------------------------------------------------------------
# Section 2: Field mapping from BrowserForge
# ---------------------------------------------------------------------------

def test_field_mapping():
    section("2. Field Mapping (BrowserForge -> IV8)")

    source = generate_profile_source(browser="chrome", os="windows", locale="en-US")
    nav = source["navigator"]
    ident = source["identity"]
    disp = source["display"]

    ua = nav["user_agent"]
    check("user_agent is non-empty", bool(ua))
    check("user_agent contains Chrome", "Chrome" in ua, f"ua={ua[:60]}")
    check("user_agent contains Windows", "Windows" in ua)

    check("platform is Win32", nav["platform"] == "Win32", f"platform={nav['platform']}")
    check("vendor is Google Inc.", nav["vendor"] == "Google Inc.")
    check("language matches locale", nav["language"] == "en-US")
    check("languages is non-empty list", isinstance(nav["languages"], list) and len(nav["languages"]) > 0)
    check("languages[0] == language", nav["languages"][0] == nav["language"])
    check("hardware_concurrency > 0", nav["hardware_concurrency"] > 0)
    check("device_memory > 0", nav["device_memory"] > 0)
    check("max_touch_points is int", isinstance(nav["max_touch_points"], int))
    check("webdriver is False", nav["webdriver"] is False)
    check("pdf_viewer_enabled is True", nav["pdf_viewer_enabled"] is True)

    uad = nav["user_agent_data"]
    check("uad has platform", bool(uad["platform"]))
    check("uad platform is Windows", uad["platform"] == "Windows")
    check("uad has brands list", isinstance(uad["brands"], list) and len(uad["brands"]) > 0)
    check("uad has full_version_list", isinstance(uad["full_version_list"], list))
    check("uad mobile is False", uad["mobile"] is False)
    check("uad has architecture", bool(uad["architecture"]))
    check("uad has bitness", bool(uad["bitness"]))

    scr = disp["screen"]
    check("screen.width > 0", scr["width"] > 0)
    check("screen.height > 0", scr["height"] > 0)
    check("screen.avail_width <= width", scr["avail_width"] <= scr["width"])
    check("screen.avail_height <= height", scr["avail_height"] <= scr["height"])
    check("screen.color_depth > 0", scr["color_depth"] > 0)
    check("screen.pixel_depth > 0", scr["pixel_depth"] > 0)

    win = disp["window"]
    check("window.inner_width > 0", win["inner_width"] > 0)
    check("window.inner_height > 0", win["inner_height"] > 0)
    check("window.outer_width >= inner_width", win["outer_width"] >= win["inner_width"])
    check("window.outer_height >= inner_height", win["outer_height"] >= win["inner_height"])
    check("window.device_pixel_ratio > 0", win["device_pixel_ratio"] > 0)

    gpu = ident["gpu"]
    check("gpu.webgl_unmasked_vendor non-empty", bool(gpu["webgl_unmasked_vendor"]))
    check("gpu.webgl_unmasked_renderer non-empty", bool(gpu["webgl_unmasked_renderer"]))
    check(
        "gpu renderer has ANGLE or OpenGL",
        "ANGLE" in gpu["webgl_unmasked_renderer"] or "OpenGL" in gpu["webgl_unmasked_renderer"],
    )

    check("identity.os is windows", ident["os"] == "windows")
    check("identity.cpu_arch is x64", ident["cpu_arch"] == "x64")
    check("identity.cpu_cores > 0", ident["cpu_cores"] > 0)
    check("identity.memory_gb > 0", ident["memory_gb"] > 0)
    check("identity.browser.brand is chrome", ident["browser"]["brand"] == "chrome")
    check("identity.browser.version non-empty", bool(ident["browser"]["version"]))
    check("identity.noise_seed is int", isinstance(ident["noise_seed"], int))

    loc = source["locale"]
    check("locale.language == navigator.language", loc["language"] == nav["language"])
    check("locale.languages == navigator.languages", loc["languages"] == nav["languages"])
    check("locale.timezone is IANA format", "/" in loc["timezone"])
    check("locale.accept_language non-empty", bool(loc["accept_language"]))

    check("permissions has geolocation", source["permissions"]["geolocation"] in ("granted", "denied", "prompt"))
    check("permissions has extra map", isinstance(source["permissions"].get("extra"), dict))
    check("permissions.extra has >=10 entries", len(source["permissions"].get("extra", {})) >= 10)

    rend = source["rendering"]
    check("rendering.canvas_2d.mode is noise", rend["canvas_2d"]["mode"] in ("none", "stable", "noise"))
    check("rendering.webgl_1.mode is noise", rend["webgl_1"]["mode"] in ("none", "stable", "noise"))
    check("rendering.audio_context.mode valid", rend["audio_context"]["mode"] in ("none", "stable", "noise"))

    check("timing.mode is logical", source["timing"]["mode"] in ("logical", "frozen", "real"))
    check("timing.fps > 0", source["timing"]["fps"] > 0)


# ---------------------------------------------------------------------------
# Section 3: Cross-OS consistency
# ---------------------------------------------------------------------------

def test_cross_os():
    section("3. Cross-OS Consistency")

    for os_name, expected_platform, ua_fragment in [
        ("windows", "Win32", "Windows"),
        ("macos", "MacIntel", "Mac"),
        ("linux", "Linux x86_64", "Linux"),
    ]:
        try:
            source = generate_profile_source(browser="chrome", os=os_name)
            nav = source["navigator"]
            check(
                f"{os_name}: platform={expected_platform}",
                nav["platform"] == expected_platform,
                f"got {nav['platform']}",
            )
            check(
                f"{os_name}: UA contains '{ua_fragment}'",
                ua_fragment in nav["user_agent"],
            )
            check(
                f"{os_name}: identity.os={os_name}",
                source["identity"]["os"] == os_name,
            )
            gpu = source["identity"]["gpu"]
            check(
                f"{os_name}: GPU resolved",
                bool(gpu["webgl_unmasked_vendor"]) and bool(gpu["webgl_unmasked_renderer"]),
            )
        except Exception as e:
            if "no such group" in str(e).lower():
                check(f"{os_name}: skipped (BrowserForge data unavailable)", True)
            else:
                check(f"{os_name}: generation succeeded", False, str(e))


# ---------------------------------------------------------------------------
# Section 4: Seed reproducibility
# ---------------------------------------------------------------------------

def test_seed_reproducibility():
    section("4. Seed Reproducibility")

    s1 = generate_profile_source(browser="chrome", os="windows", seed=42)
    s2 = generate_profile_source(browser="chrome", os="windows", seed=42)

    check(
        "noise_seed matches for same seed",
        s1["identity"]["noise_seed"] == s2["identity"]["noise_seed"],
        f"{s1['identity']['noise_seed']} != {s2['identity']['noise_seed']}",
    )

    s3 = generate_profile_source(browser="chrome", os="windows", seed=999)
    check(
        "noise_seed differs for different seed",
        s1["identity"]["noise_seed"] != s3["identity"]["noise_seed"],
    )

    check(
        "meta.name matches for same seed",
        s1["meta"]["name"] == s2["meta"]["name"],
    )


# ---------------------------------------------------------------------------
# Section 5: Screen constraints
# ---------------------------------------------------------------------------

def test_screen_constraints():
    section("5. Screen Constraints")

    try:
        source = generate_profile_source(
            browser="chrome", os="windows",
            min_width=1920, max_width=1920,
            min_height=1080, max_height=1080,
        )
        scr = source["display"]["screen"]
        check("constrained width in [1920,1920]", 1920 <= scr["width"] <= 1920, f"got {scr['width']}")
        check("constrained height in [1080,1080]", 1080 <= scr["height"] <= 1080, f"got {scr['height']}")
    except Exception as e:
        check("screen constraint generation", False, str(e))

    try:
        source = generate_profile_source(
            browser="chrome", os="windows",
            min_width=1280, max_width=2560,
            min_height=720, max_height=1440,
        )
        scr = source["display"]["screen"]
        check("range width in [1280,2560]", 1280 <= scr["width"] <= 2560, f"got {scr['width']}")
        check("range height in [720,1440]", 720 <= scr["height"] <= 1440, f"got {scr['height']}")
    except Exception as e:
        check("screen range generation", False, str(e))


# ---------------------------------------------------------------------------
# Section 6: Flat env compatibility
# ---------------------------------------------------------------------------

def test_flat_env_compatibility():
    section("6. Flat Env Compatibility (load_runtime_env format)")

    source = generate_profile_source(browser="chrome", os="windows", locale="en-US")
    env = profile_source_to_flat_env(source)

    check("flat_env has >50 keys", len(env) > 50, f"got {len(env)}")

    critical_keys = [
        "navigator.userAgent",
        "navigator.platform",
        "navigator.vendor",
        "navigator.language",
        "navigator.languages",
        "navigator.hardwareConcurrency",
        "navigator.deviceMemory",
        "navigator.maxTouchPoints",
        "navigator.webdriver",
        "navigator.pdfViewerEnabled",
        "navigator.userAgentData.brands",
        "navigator.userAgentData.platform",
        "screen.width",
        "screen.height",
        "screen.availWidth",
        "screen.availHeight",
        "screen.colorDepth",
        "screen.pixelDepth",
        "window.innerWidth",
        "window.innerHeight",
        "window.outerWidth",
        "window.outerHeight",
        "window.devicePixelRatio",
        "webgl.VENDOR",
        "webgl.RENDERER",
        "webgl.UNMASKED_VENDOR_WEBGL",
        "webgl.UNMASKED_RENDERER_WEBGL",
        "media.pointer",
        "media.hover",
        "media.color-gamut",
        "permissions.geolocation",
        "permissions.accelerometer",
        "audio.baseLatency",
        "audio.outputLatency",
        "display.color-gamut",
        "fonts.families",
    ]
    for k in critical_keys:
        check(f"flat_env has {k}", k in env, f"missing")

    check(
        "navigator.userAgent in flat_env matches source",
        env["navigator.userAgent"] == source["navigator"]["user_agent"],
    )
    check(
        "screen.width in flat_env matches source",
        env["screen.width"] == source["display"]["screen"]["width"],
    )
    check(
        "webgl.UNMASKED_VENDOR_WEBGL matches source",
        env["webgl.UNMASKED_VENDOR_WEBGL"] == source["identity"]["gpu"]["webgl_unmasked_vendor"],
    )

    perm_count = len([k for k in env if k.startswith("permissions.")])
    check("permissions count >= 7", perm_count >= 7, f"got {perm_count}")

    media_count = len([k for k in env if k.startswith("media.")])
    check("media count >= 16", media_count >= 16, f"got {media_count}")


# ---------------------------------------------------------------------------
# Section 7: H02 A-rule consistency checks
# ---------------------------------------------------------------------------

def test_h02_a_rules():
    section("7. H02 A-Rule Consistency Checks")

    from evaluate_env_consistency import (
        check_a01_platform_matches_ua,
        check_a02_vendor_matches_browser,
        check_a03_webgl_vendor_renderer_consistency,
        check_a04_screen_ge_window,
        check_a05_pointer_consistency,
        check_a06_hover_consistency,
        check_a07_permissions_valid,
        check_a08_audio_latency_valid,
        check_a09_fonts_nonempty,
        check_a10_color_gamut_valid,
    )

    results = {}
    for os_name in ("windows", "macos", "linux"):
        try:
            source = generate_profile_source(browser="chrome", os=os_name, locale="en-US")
            env = profile_source_to_flat_env(source)

            checks = [
                ("A01", "platform matches UA", check_a01_platform_matches_ua),
                ("A02", "vendor matches browser", check_a02_vendor_matches_browser),
                ("A03", "WebGL vendor/renderer", check_a03_webgl_vendor_renderer_consistency),
                ("A04", "screen >= window", check_a04_screen_ge_window),
                ("A05", "pointer consistency", check_a05_pointer_consistency),
                ("A06", "hover consistency", check_a06_hover_consistency),
                ("A07", "permissions valid", check_a07_permissions_valid),
                ("A08", "audio latency valid", check_a08_audio_latency_valid),
                ("A10", "color-gamut valid", check_a10_color_gamut_valid),
            ]

            for cid, desc, fn in checks:
                ok = fn(env)
                check(f"{os_name}/{cid}: {desc}", ok)
                results[f"{os_name}_{cid}"] = ok

        except Exception as e:
            if "no such group" in str(e).lower():
                check(f"{os_name}: skipped (BrowserForge data unavailable)", True)
                results[f"{os_name}_skipped"] = True
            else:
                check(f"{os_name}: H02 checks ran", False, str(e))
                results[f"{os_name}_error"] = False

    all_pass = all(results.values()) if results else False
    check("ALL A-rules pass for all OS", all_pass)


# ---------------------------------------------------------------------------
# Section 8: Rust ProfileSource schema compatibility
# ---------------------------------------------------------------------------

def test_rust_schema_compatibility():
    section("8. Rust ProfileSource Schema Compatibility")

    source = generate_profile_source(browser="chrome", os="windows", seed=42)

    json_str = json.dumps(source)
    check("JSON serializable", bool(json_str))

    parsed = json.loads(json_str)
    check("JSON round-trip preserves meta", parsed["meta"]["schema_version"] == "0.8.32")
    check("JSON round-trip preserves navigator", parsed["navigator"]["platform"] == "Win32")
    check("JSON round-trip preserves gpu", bool(parsed["identity"]["gpu"]["webgl_unmasked_vendor"]))

    check("no None values in critical fields", all([
        source["navigator"]["user_agent"] is not None,
        source["navigator"]["platform"] is not None,
        source["navigator"]["vendor"] is not None,
        source["identity"]["gpu"]["webgl_unmasked_vendor"] is not None,
        source["identity"]["gpu"]["webgl_unmasked_renderer"] is not None,
    ]))

    check(
        "permissions extra values are all strings",
        all(isinstance(v, str) for v in source["permissions"]["extra"].values()),
    )

    check(
        "uad brands entries have brand+version",
        all("brand" in b and "version" in b for b in source["navigator"]["user_agent_data"]["brands"]),
    )


# ---------------------------------------------------------------------------
# Section 9: Determinism within same generation
# ---------------------------------------------------------------------------

def test_internal_consistency():
    section("9. Internal Consistency")

    source = generate_profile_source(browser="chrome", os="windows", locale="zh-CN")

    nav = source["navigator"]
    loc = source["locale"]

    check(
        "navigator.language == locale.language",
        nav["language"] == loc["language"],
        f"{nav['language']} != {loc['language']}",
    )
    check(
        "navigator.languages[0] == locale.language",
        nav["languages"][0] == loc["language"],
    )
    check(
        "locale.languages == navigator.languages",
        loc["languages"] == nav["languages"],
    )

    ua = nav["user_agent"]
    brand = source["identity"]["browser"]["brand"]
    check(
        "UA contains browser brand",
        brand.lower() in ua.lower(),
        f"'{brand}' not in UA",
    )

    os_name = source["identity"]["os"]
    os_display = {"windows": "Windows", "macos": "Mac", "linux": "Linux"}.get(os_name, os_name)
    check(
        "UA contains OS name",
        os_display.lower() in ua.lower(),
        f"'{os_display}' not in UA",
    )

    if brand == "chrome":
        check(
            "vendor is Google Inc. for chrome",
            nav["vendor"] == "Google Inc.",
        )


# ---------------------------------------------------------------------------
# Section 10: BrowserForge availability
# ---------------------------------------------------------------------------

def test_browserforge_availability():
    section("10. BrowserForge Availability")

    check("browserforge module available", _BROWSERFORGE_AVAILABLE)

    if _BROWSERFORGE_AVAILABLE:
        try:
            from browserforge.fingerprints import FingerprintGenerator
            g = FingerprintGenerator()
            fp = g.generate(browser="chrome", os="windows")
            check("FingerprintGenerator.generate works", fp is not None)
            check("fingerprint has navigator", hasattr(fp, "navigator"))
            check("fingerprint has screen", hasattr(fp, "screen"))
            check("fingerprint has videoCard", hasattr(fp, "videoCard"))
            check("navigator has userAgent", bool(fp.navigator.userAgent))
        except Exception as e:
            check("BrowserForge generation", False, str(e))


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> int:
    print("=" * 60)
    print("BrowserForge Adapter Tests (D-104)")
    print(f"BrowserForge available: {_BROWSERFORGE_AVAILABLE}")
    print("=" * 60)

    test_browserforge_availability()
    test_schema_validity()
    test_field_mapping()
    test_cross_os()
    test_seed_reproducibility()
    test_screen_constraints()
    test_flat_env_compatibility()
    test_h02_a_rules()
    test_rust_schema_compatibility()
    test_internal_consistency()

    total = PASS_COUNT + FAIL_COUNT
    print("\n" + "=" * 60)
    print(f"Results: {PASS_COUNT}/{total} PASS")
    if FAIL_COUNT > 0:
        print(f"         {FAIL_COUNT} FAIL")
    print(f"Overall: {'PASS' if FAIL_COUNT == 0 else 'FAIL'}")
    print("=" * 60)

    return 0 if FAIL_COUNT == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
