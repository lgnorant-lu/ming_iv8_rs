"""Hypothesis profile strategies for H02 metamorphic testing.

Generates env dicts compatible with evaluate_env_consistency.py's flat-key format:
  "navigator.platform", "navigator.userAgent", "webgl.UNMASKED_VENDOR_WEBGL", etc.

The profile_strategy() generates internally-consistent profiles (all A01-A10 pass).
The mutated_profile_strategy() injects exactly ONE contradiction per profile.
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

# Mutation identifiers — one per A01-A10 check
MUTATION_IDS = (
    "flip_platform",        # A01
    "flip_vendor",          # A02
    "contradict_gpu",       # A03
    "shrink_screen",        # A04
    "break_pointer",        # A05
    "break_hover",          # A06
    "invalid_permission",   # A07
    "zero_audio_latency",   # A08
    "empty_fonts",          # A09
    "invalid_color_gamut",  # A10
)

# Mutation -> the single A-check it is expected to break
MUTATION_TO_CHECK = {
    "flip_platform": "A01",
    "flip_vendor": "A02",
    "contradict_gpu": "A03",
    "shrink_screen": "A04",
    "break_pointer": "A05",
    "break_hover": "A06",
    "invalid_permission": "A07",
    "zero_audio_latency": "A08",
    "empty_fonts": "A09",
    "invalid_color_gamut": "A10",
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
    """Generate a flat-key env dict where all A01-A10 checks PASS.

    The returned dict uses the same key format as
    evaluate_env_consistency.py (e.g. "navigator.platform").
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

    # Screen dimensions — ensure screen >= window
    screen_w = draw(st.sampled_from([1280, 1366, 1440, 1600, 1920, 2560]))
    screen_h = draw(st.sampled_from([720, 768, 900, 1024, 1080, 1200, 1440]))
    inner_w = draw(st.integers(min_value=400, max_value=screen_w))
    inner_h = draw(st.integers(min_value=300, max_value=screen_h))

    color_gamut = draw(st.sampled_from(_COLOR_GAMUTS))

    # Permission values
    perm_vals = {pk: draw(st.sampled_from(_PERMISSION_STATES))
                 for pk in _PERMISSION_KEYS}

    audio_bl = draw(st.floats(min_value=0.001, max_value=0.05,
                              allow_nan=False, allow_infinity=False))

    fonts = draw(st.sampled_from(_FONT_SETS))

    env = {
        "navigator.platform": platform,
        "navigator.vendor": vendor,
        "navigator.userAgent": ua,
        "webgl.UNMASKED_VENDOR_WEBGL": gpu_vendor,
        "webgl.UNMASKED_RENDERER_WEBGL": gpu_renderer,
        "screen.width": screen_w,
        "screen.height": screen_h,
        "window.innerWidth": inner_w,
        "window.innerHeight": inner_h,
        "media.pointer": "fine",
        "media.any-pointer": "coarse",    # touchscreen+mouse is valid
        "media.hover": "hover",
        "media.any-hover": "none",         # some devices lack hover
        "audio.baseLatency": audio_bl,
        "audio.outputLatency": audio_bl * 2,
        "display.color-gamut": color_gamut,
        "fonts.families": list(fonts),
    }
    env.update(perm_vals)
    return env


# ---------------------------------------------------------------------------
# Negative strategy: inject exactly ONE contradiction
# ---------------------------------------------------------------------------

@st.composite
def mutated_profile_strategy(draw):
    """Generate an env dict with exactly ONE injected contradiction.

    Returns (env, mutation_id, expected_fail_check).
    The caller can use mutation_id / expected_fail_check to verify
    that the intended A-check fails while others still pass.
    """
    env = draw(profile_strategy())
    mutation = draw(st.sampled_from(MUTATION_IDS))

    if mutation == "flip_platform":
        # A01: set platform to an OS that contradicts the UA
        ua_lower = env["navigator.userAgent"].lower()
        if "windows" in ua_lower or "win64" in ua_lower:
            env["navigator.platform"] = "MacIntel"
        elif "macintosh" in ua_lower or "mac os" in ua_lower:
            env["navigator.platform"] = "Win32"
        else:  # linux
            env["navigator.platform"] = "Win32"

    elif mutation == "flip_vendor":
        # A02: vendor doesn't match any browser in UA
        env["navigator.vendor"] = "Microsoft"

    elif mutation == "contradict_gpu":
        # A03: vendor says NVIDIA but renderer says Intel
        env["webgl.UNMASKED_VENDOR_WEBGL"] = "Google Inc. (NVIDIA)"
        env["webgl.UNMASKED_RENDERER_WEBGL"] = (
            "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)"
        )

    elif mutation == "shrink_screen":
        # A04: screen smaller than window
        env["screen.width"] = 100
        env["screen.height"] = 100

    elif mutation == "break_pointer":
        # A05: pointer=coarse but any-pointer=fine (must be coarse)
        env["media.pointer"] = "coarse"
        env["media.any-pointer"] = "fine"

    elif mutation == "break_hover":
        # A06: hover=none but any-hover=hover (must be none)
        env["media.hover"] = "none"
        env["media.any-hover"] = "hover"

    elif mutation == "invalid_permission":
        # A07: permission value not in {granted,denied,prompt}
        env["permissions.geolocation"] = "always"

    elif mutation == "zero_audio_latency":
        # A08: baseLatency == 0 fails 0 < bl < 1.0
        env["audio.baseLatency"] = 0.0

    elif mutation == "empty_fonts":
        # A09: fonts.families is empty list
        env["fonts.families"] = []

    elif mutation == "invalid_color_gamut":
        # A10: color-gamut not in valid set
        env["display.color-gamut"] = "hdr1000"

    expected_fail = MUTATION_TO_CHECK[mutation]
    return env, mutation, expected_fail
