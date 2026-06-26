"""Hypothesis profile strategies for H02/H03 metamorphic testing.

Generates env dicts compatible with evaluate_env_consistency.py's flat-key format:
  "navigator.platform", "navigator.userAgent", "webgl.UNMASKED_VENDOR_WEBGL", etc.

The profile_strategy() generates internally-consistent profiles (all MR checks pass).
The mutated_profile_strategy() injects exactly ONE contradiction per profile.

D-101 (v0.8.83): Extended from 10 to 43 MR rules across 5 categories.
"""

from __future__ import annotations

from hypothesis import strategies as st, assume


# ---------------------------------------------------------------------------
# Static lookup tables
# ---------------------------------------------------------------------------

OS_NAMES = ("windows", "macos", "linux")

# (os_name -> platform string)
_PLATFORM_MAP = {
    "windows": "Win32",
    "macos": "MacIntel",
    "linux": "Linux x86_64",
}

# (os_name -> UA-CH platform string)
_UACH_PLATFORM_MAP = {
    "windows": "Windows",
    "macos": "macOS",
    "linux": "Linux",
}

_BROWSER_BRANDS = ("chrome", "firefox", "safari")

# (browser_brand -> navigator.vendor)
_VENDOR_MAP = {
    "chrome": "Google Inc.",
    "firefox": "Mozilla",          # evaluate uses "Mozilla" check
    "safari": "Apple Computer",
}

# (browser_brand -> UA marker the check looks for)
_BROWSER_UA_MARKER = {
    "chrome": "chrome",
    "firefox": "firefox",
    "safari": "safari",
}

_CHROME_VERSIONS = ("120", "125", "130", "135", "140", "145", "147")

# GPU presets per OS: (vendor, renderer) pairs that are self-consistent
_GPU_PRESETS = {
    "windows": [
        ("Google Inc. (NVIDIA)",
         "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 (0x00002882) Direct3D11 vs_5_0 ps_5_0, D3D11)"),
        ("Google Inc. (Intel)",
         "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0, D3D11)"),
        ("Google Inc. (AMD)",
         "ANGLE (AMD, AMD Radeon RX 6700 XT Direct3D11 vs_5_0 ps_5_0, D3D11)"),
    ],
    "macos": [
        ("Google Inc. (Apple)",
         "ANGLE (Apple, ANGLE Metal Renderer: Apple M1, Unspecified Version)"),
    ],
    "linux": [
        ("Google Inc. (NVIDIA)",
         "ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 OpenGL)"),
        ("Google Inc. (Intel)",
         "ANGLE (Intel, Intel(R) Iris Xe Graphics OpenGL)"),
    ],
}

_COLOR_GAMUTS = ("srgb", "p3", "rec2020")
_PERMISSION_STATES = ("granted", "denied", "prompt")
_PERMISSION_KEYS = (
    "permissions.geolocation",
    "permissions.notifications",
    "permissions.camera",
    "permissions.microphone",
    "permissions.accelerometer",
)
_FONT_SETS = (
    ["Arial", "Calibri", "Consolas", "Segoe UI", "Times New Roman"],
    ["Arial", "Helvetica", "Times New Roman", "Courier New"],
    ["Roboto", "Noto Sans", "Ubuntu", "DejaVu Sans"],
)

_LOCALES = ("zh-CN", "en-US", "en-GB", "ja-JP", "ko-KR",
            "de-DE", "fr-FR", "es-ES", "pt-BR", "ru-RU")

_COLOR_DEPTHS = (24, 30)
_DPR_VALUES = (1.0, 1.25, 1.5, 2.0, 3.0)
_CPU_CORES = (4, 6, 8, 12, 16)
_MEMORY_GB = (4, 8, 16, 32)

# Mutation identifiers — one per MR rule
# Original 10 (A01-A10)
MUTATION_IDS_A = (
    "flip_platform",        # A01 / MR-EQ-001
    "flip_vendor",          # A02 / MR-EQ-002
    "contradict_gpu",       # A03 / MR-EQ-010
    "shrink_screen",        # A04 / MR-BND-001
    "break_pointer",        # A05 / MR-IMP-003
    "break_hover",          # A06 / MR-IMP-004
    "invalid_permission",   # A07 / MR-VAL-001
    "zero_audio_latency",   # A08 / MR-BND-006
    "empty_fonts",          # A09 / MR-VAL-003
    "invalid_color_gamut",  # A10 / MR-VAL-002
)

# Extended mutations for D-101 (target new MRs)
MUTATION_IDS_EXT = (
    "mismatch_language",         # MR-EQ-004
    "mismatch_uach_platform",    # MR-IMP-010
    "negative_dpr",              # MR-BND-007
    "invalid_pointer_val",       # MR-VAL-004
    "invalid_hover_val",         # MR-VAL-005
    "zero_cpu_cores",            # MR-BND-008
    "zero_memory",               # MR-BND-009
    "bad_color_depth",           # MR-BND-010
    "outer_lt_inner",            # MR-BND-003
    "avail_gt_screen",           # MR-BND-002
    "uach_mobile_no_touch",      # MR-EQ-009
    "no_window_chrome",          # MR-IMP-001
    "chrome_no_pdf",             # MR-IMP-005
    "desktop_has_touch",         # MR-IMP-006
    "nvidia_wrong_renderer",     # MR-IMP-008
    "mismatch_lang_accept",      # MR-EQ-006
    "mismatch_uach_brands",      # MR-EQ-008
)

MUTATION_IDS = MUTATION_IDS_A + MUTATION_IDS_EXT

# Mutation -> the single MR it is expected to break
MUTATION_TO_CHECK = {
    "flip_platform": "MR-EQ-001",
    "flip_vendor": "MR-EQ-002",
    "contradict_gpu": "MR-EQ-010",
    "shrink_screen": "MR-BND-001",
    "break_pointer": "MR-IMP-003",
    "break_hover": "MR-IMP-004",
    "invalid_permission": "MR-VAL-001",
    "zero_audio_latency": "MR-BND-006",
    "empty_fonts": "MR-VAL-003",
    "invalid_color_gamut": "MR-VAL-002",
    "mismatch_language": "MR-EQ-004",
    "mismatch_uach_platform": "MR-IMP-010",
    "negative_dpr": "MR-BND-007",
    "invalid_pointer_val": "MR-VAL-004",
    "invalid_hover_val": "MR-VAL-005",
    "zero_cpu_cores": "MR-BND-008",
    "zero_memory": "MR-BND-009",
    "bad_color_depth": "MR-BND-010",
    "outer_lt_inner": "MR-BND-003",
    "avail_gt_screen": "MR-BND-002",
    "uach_mobile_no_touch": "MR-EQ-009",
    "no_window_chrome": "MR-IMP-001",
    "chrome_no_pdf": "MR-IMP-005",
    "desktop_has_touch": "MR-IMP-006",
    "nvidia_wrong_renderer": "MR-IMP-008",
    "mismatch_lang_accept": "MR-EQ-006",
    "mismatch_uach_brands": "MR-EQ-008",
}


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _ua_for(os_name: str, browser_brand: str, version: str) -> str:
    """Build a UA string consistent with os + browser."""
    if browser_brand == "chrome":
        if os_name == "windows":
            return (f"Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
                    f"AppleWebKit/537.36 (KHTML, like Gecko) "
                    f"Chrome/{version}.0.0.0 Safari/537.36")
        elif os_name == "macos":
            return (f"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
                    f"AppleWebKit/537.36 (KHTML, like Gecko) "
                    f"Chrome/{version}.0.0.0 Safari/537.36")
        else:  # linux
            return (f"Mozilla/5.0 (X11; Linux x86_64) "
                    f"AppleWebKit/537.36 (KHTML, like Gecko) "
                    f"Chrome/{version}.0.0.0 Safari/537.36")
    elif browser_brand == "firefox":
        if os_name == "windows":
            return "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0"
        elif os_name == "macos":
            return "Mozilla/5.0 (Macintosh; Intel Mac OS X 14.0; rv:121.0) Gecko/20100101 Firefox/121.0"
        else:
            return "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0"
    else:  # safari (macos only)
        return ("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
                "AppleWebKit/605.1.15 (KHTML, like Gecko) "
                "Version/17.0 Safari/605.1.15")


# ---------------------------------------------------------------------------
# Positive strategy: internally-consistent env dict
# ---------------------------------------------------------------------------

@st.composite
def profile_strategy(draw):
    """Generate a flat-key env dict where all MR checks PASS.

    The returned dict uses the same key format as
    evaluate_env_consistency.py (e.g. "navigator.platform").

    D-101: Extended to include identity, locale, UA-CH, screen detail,
    and capability fields for the full 43-MR rule set.
    """
    os_name = draw(st.sampled_from(OS_NAMES))
    # Safari only makes sense on macOS; Firefox has limited UA variants
    browser_brand = draw(st.sampled_from(_BROWSER_BRANDS).filter(
        lambda b: b != "safari" or os_name == "macos"
    ))
    version = draw(st.sampled_from(_CHROME_VERSIONS))

    platform = _PLATFORM_MAP[os_name]
    vendor = _VENDOR_MAP[browser_brand]
    ua = _ua_for(os_name, browser_brand, version)

    gpu_vendor, gpu_renderer = draw(st.sampled_from(_GPU_PRESETS[os_name]))

    # Screen dimensions — ensure screen >= avail >= inner, outer >= inner
    screen_w = draw(st.sampled_from([1280, 1366, 1440, 1600, 1920, 2560]))
    screen_h = draw(st.sampled_from([720, 768, 900, 1024, 1080, 1200, 1440]))
    avail_w = screen_w
    avail_h = draw(st.integers(min_value=screen_h - 80, max_value=screen_h))
    inner_w = draw(st.integers(min_value=400, max_value=avail_w))
    inner_h = draw(st.integers(min_value=300, max_value=avail_h))
    outer_w = draw(st.integers(min_value=inner_w, max_value=screen_w))
    outer_h = draw(st.integers(min_value=inner_h, max_value=screen_h))

    color_gamut = draw(st.sampled_from(_COLOR_GAMUTS))
    color_depth = draw(st.sampled_from(_COLOR_DEPTHS))
    dpr = draw(st.sampled_from(_DPR_VALUES))

    # Permission values
    perm_vals = {pk: draw(st.sampled_from(_PERMISSION_STATES))
                 for pk in _PERMISSION_KEYS}

    audio_bl = draw(st.floats(min_value=0.001, max_value=0.05,
                              allow_nan=False, allow_infinity=False))

    fonts = draw(st.sampled_from(_FONT_SETS))

    # Locale and language
    locale = draw(st.sampled_from(_LOCALES))
    languages = [locale, "en"] if locale != "en" else ["en"]
    accept_lang = f"{locale},{locale.split('-')[0]};q=0.9,en;q=0.8"

    # Identity fields
    cpu_cores = draw(st.sampled_from(_CPU_CORES))
    memory_gb = draw(st.sampled_from(_MEMORY_GB))

    # UA-CH (userAgentData) — only Chrome/Edge have brands
    uach_platform = _UACH_PLATFORM_MAP[os_name]
    is_chrome = browser_brand in ("chrome",)
    uach_brands = (
        [{"brand": "Chromium", "version": version},
         {"brand": "Google Chrome", "version": version}]
        if is_chrome else []
    )

    env = {
        "navigator.platform": platform,
        "navigator.vendor": vendor,
        "navigator.userAgent": ua,
        "navigator.language": locale,
        "navigator.languages": list(languages),
        "navigator.maxTouchPoints": 0,
        "navigator.pdfViewerEnabled": True if is_chrome else False,
        "navigator.userAgentData.platform": uach_platform,
        "navigator.userAgentData.mobile": False,
        "navigator.userAgentData.brands": uach_brands,
        "identity.os": os_name,
        "identity.browser.brand": browser_brand,
        "identity.cpu_cores": cpu_cores,
        "identity.memory_gb": memory_gb,
        "webgl.UNMASKED_VENDOR_WEBGL": gpu_vendor,
        "webgl.UNMASKED_RENDERER_WEBGL": gpu_renderer,
        "screen.width": screen_w,
        "screen.height": screen_h,
        "screen.availWidth": avail_w,
        "screen.availHeight": avail_h,
        "screen.colorDepth": color_depth,
        "window.innerWidth": inner_w,
        "window.innerHeight": inner_h,
        "window.outerWidth": outer_w,
        "window.outerHeight": outer_h,
        "window.devicePixelRatio": dpr,
        "media.pointer": "fine",
        "media.any-pointer": "coarse",    # touchscreen+mouse is valid
        "media.hover": "hover",
        "media.any-hover": "none",         # some devices lack hover
        "audio.baseLatency": audio_bl,
        "audio.outputLatency": audio_bl * 2,
        "display.color-gamut": color_gamut,
        "fonts.families": list(fonts),
        "locale.language": locale,
        "locale.accept_language": accept_lang,
        "capabilities.windowChrome": True if is_chrome else False,
    }
    env.update(perm_vals)
    return env


# ---------------------------------------------------------------------------
# Negative strategy: inject exactly ONE contradiction
# ---------------------------------------------------------------------------

@st.composite
def mutated_profile_strategy(draw):
    """Generate an env dict with exactly ONE injected contradiction.

    Returns (env, mutation_id, expected_fail_mr).
    The caller can use mutation_id / expected_fail_mr to verify
    that the intended MR fails while others still pass.
    """
    env = draw(profile_strategy())
    mutation = draw(st.sampled_from(MUTATION_IDS))

    if mutation == "flip_platform":
        ua_lower = env["navigator.userAgent"].lower()
        if "windows" in ua_lower or "win64" in ua_lower:
            env["navigator.platform"] = "MacIntel"
        elif "macintosh" in ua_lower or "mac os" in ua_lower:
            env["navigator.platform"] = "Win32"
        else:  # linux
            env["navigator.platform"] = "Win32"

    elif mutation == "flip_vendor":
        env["navigator.vendor"] = "Microsoft"

    elif mutation == "contradict_gpu":
        env["webgl.UNMASKED_VENDOR_WEBGL"] = "Google Inc. (NVIDIA)"
        env["webgl.UNMASKED_RENDERER_WEBGL"] = (
            "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)"
        )

    elif mutation == "shrink_screen":
        env["screen.width"] = 100
        env["screen.height"] = 100

    elif mutation == "break_pointer":
        env["media.pointer"] = "coarse"
        env["media.any-pointer"] = "fine"

    elif mutation == "break_hover":
        env["media.hover"] = "none"
        env["media.any-hover"] = "hover"

    elif mutation == "invalid_permission":
        env["permissions.geolocation"] = "always"

    elif mutation == "zero_audio_latency":
        env["audio.baseLatency"] = 0.0

    elif mutation == "empty_fonts":
        env["fonts.families"] = []

    elif mutation == "invalid_color_gamut":
        env["display.color-gamut"] = "hdr1000"

    elif mutation == "mismatch_language":
        env["navigator.language"] = "ja-JP"

    elif mutation == "mismatch_uach_platform":
        # MR-IMP-010 (Windows) or MR-IMP-011 (macOS): swap UA-CH platform
        os_name = env["identity.os"]
        if os_name == "windows":
            env["navigator.userAgentData.platform"] = "macOS"
        elif os_name == "macos":
            env["navigator.userAgentData.platform"] = "Windows"
        else:
            # Linux: set to Windows to break MR-EQ-007 instead
            env["navigator.userAgentData.platform"] = "Windows"

    elif mutation == "negative_dpr":
        env["window.devicePixelRatio"] = -1.0

    elif mutation == "invalid_pointer_val":
        env["media.pointer"] = "stylus"

    elif mutation == "invalid_hover_val":
        env["media.hover"] = "always"

    elif mutation == "zero_cpu_cores":
        env["identity.cpu_cores"] = 0

    elif mutation == "zero_memory":
        env["identity.memory_gb"] = 0

    elif mutation == "bad_color_depth":
        env["screen.colorDepth"] = 16

    elif mutation == "outer_lt_inner":
        env["window.outerWidth"] = env["window.innerWidth"] - 1

    elif mutation == "avail_gt_screen":
        env["screen.availHeight"] = env["screen.height"] + 100

    elif mutation == "uach_mobile_no_touch":
        env["navigator.userAgentData.mobile"] = True
        env["navigator.maxTouchPoints"] = 0

    elif mutation == "no_window_chrome":
        if env["navigator.vendor"] == "Google Inc.":
            env["capabilities.windowChrome"] = False
        else:
            env["navigator.vendor"] = "Google Inc."
            env["capabilities.windowChrome"] = False

    elif mutation == "chrome_no_pdf":
        if env["identity.browser.brand"] == "chrome":
            env["navigator.pdfViewerEnabled"] = False
        else:
            env["identity.browser.brand"] = "chrome"
            env["navigator.pdfViewerEnabled"] = False

    elif mutation == "desktop_has_touch":
        env["navigator.maxTouchPoints"] = 5

    elif mutation == "nvidia_wrong_renderer":
        env["webgl.UNMASKED_VENDOR_WEBGL"] = "Google Inc. (NVIDIA)"
        env["webgl.UNMASKED_RENDERER_WEBGL"] = (
            "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)"
        )

    elif mutation == "mismatch_lang_accept":
        env["locale.accept_language"] = "ja-JP,ja;q=0.9,en;q=0.8"

    elif mutation == "mismatch_uach_brands":
        # MR-EQ-008 requires Chrome (non-empty UA-CH brands).
        # Convert to a consistent Chrome profile, then break brands.
        os_name = env["identity.os"]
        env["identity.browser.brand"] = "chrome"
        env["navigator.vendor"] = "Google Inc."
        env["navigator.userAgent"] = _ua_for(os_name, "chrome", "120")
        env["navigator.userAgentData.brands"] = [
            {"brand": "Chromium", "version": "120"},
            {"brand": "Google Chrome", "version": "120"},
        ]
        env["capabilities.windowChrome"] = True
        env["navigator.pdfViewerEnabled"] = True
        # Now break MR-EQ-008: brands don't contain "chrom"
        env["navigator.userAgentData.brands"] = [
            {"brand": "Firefox", "version": "121"}
        ]

    # Determine expected_fail MR — some mutations have OS-dependent targets
    if mutation == "mismatch_uach_platform":
        os_name = env["identity.os"]
        if os_name == "windows":
            expected_fail = "MR-IMP-010"
        elif os_name == "macos":
            expected_fail = "MR-IMP-011"
        else:
            expected_fail = "MR-EQ-007"
    else:
        expected_fail = MUTATION_TO_CHECK[mutation]
    return env, mutation, expected_fail
