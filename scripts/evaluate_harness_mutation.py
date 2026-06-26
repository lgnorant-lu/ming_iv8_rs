"""H02/H03 Harness Mutation Testing (D-105/D-101).

Injects deliberate defects into a valid runtime env, one per MR rule,
and verifies each defect is caught by its corresponding check.

A "killed" mutant means the harness detected the injected defect (check
returned False). A "survived" mutant means the harness missed it — a
false-negative in the harness itself.

D-101 (v0.8.83): Extended from 10 to 27 mutations covering all 36
executable MRs (7 cross-context MRs are SKIP, not mutation-tested).

Usage: python scripts/evaluate_harness_mutation.py
Exit code: 0 = all mutants killed, 1 = at least one mutant survived
"""

import copy
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO_ROOT))

from scripts.evaluate_env_consistency import (
    load_runtime_env,
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

_SCRIPTS_DIR = Path(__file__).resolve().parent
if str(_SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS_DIR))

from _metamorphic import MR_MAP


def _enrich_baseline(env: dict) -> dict:
    """Add extended fields to runtime env for new MR mutations.

    The runtime env only has A01-A10 fields. This adds the extra fields
    needed by the D-101 MR rules, consistent with the runtime values.
    """
    ua = env.get("navigator.userAgent", "").lower()
    if "windows" in ua or "win64" in ua:
        os_name = "windows"
        platform_uach = "Windows"
    elif "macintosh" in ua or "mac os" in ua:
        os_name = "macos"
        platform_uach = "macOS"
    else:
        os_name = "linux"
        platform_uach = "Linux"

    vendor = env.get("navigator.vendor", "")
    is_chrome = "Google Inc." in vendor

    env.setdefault("identity.os", os_name)
    env.setdefault("identity.browser.brand", "chrome" if is_chrome else "firefox")
    env.setdefault("identity.cpu_cores", env.get("navigator.hardwareConcurrency", 8))
    env.setdefault("identity.memory_gb", int(env.get("navigator.deviceMemory", 8)))

    env.setdefault("navigator.language", "en-US")
    env.setdefault("navigator.languages", ["en-US", "en"])
    env.setdefault("navigator.maxTouchPoints", env.get("navigator.maxTouchPoints", 0))
    env.setdefault("navigator.pdfViewerEnabled", True if is_chrome else False)

    env.setdefault("navigator.userAgentData.platform", platform_uach)
    env.setdefault("navigator.userAgentData.mobile", False)
    env.setdefault("navigator.userAgentData.brands",
                  [{"brand": "Chromium", "version": "147"},
                   {"brand": "Google Chrome", "version": "147"}] if is_chrome else [])

    sw = env.get("screen.width", 1920)
    sh = env.get("screen.height", 1080)
    iw = env.get("window.innerWidth", sw)
    ih = env.get("window.innerHeight", sh)
    env.setdefault("screen.availWidth", sw)
    env.setdefault("screen.availHeight", sh - 40)
    env.setdefault("screen.colorDepth", 24)
    env.setdefault("window.outerWidth", sw)
    env.setdefault("window.outerHeight", sh)
    env.setdefault("window.devicePixelRatio", 1.0)

    env.setdefault("locale.language", "en-US")
    env.setdefault("locale.accept_language", "en-US,en;q=0.9")
    env.setdefault("capabilities.windowChrome", True if is_chrome else False)

    return env


# ---------------------------------------------------------------------------
# Original mutations (A01-A10)
# ---------------------------------------------------------------------------

def _m01_inject(env):
    env["navigator.platform"] = "MacIntel"

def _m02_inject(env):
    env["navigator.vendor"] = "Microsoft"

def _m03_inject(env):
    env["webgl.UNMASKED_RENDERER_WEBGL"] = (
        "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)"
    )

def _m04_inject(env):
    env["screen.width"] = 800

def _m05_inject(env):
    env["media.pointer"] = "coarse"
    env["media.any-pointer"] = "fine"

def _m06_inject(env):
    env["media.hover"] = "none"
    env["media.any-hover"] = "hover"

def _m07_inject(env):
    env["permissions.geolocation"] = "unknown"

def _m08_inject(env):
    env["audio.baseLatency"] = 0

def _m09_inject(env):
    env["fonts.families"] = []

def _m10_inject(env):
    env["display.color-gamut"] = "invalid"


# ---------------------------------------------------------------------------
# Extended mutations (D-101: target new MRs)
# ---------------------------------------------------------------------------

def _m11_inject(env):
    env["navigator.language"] = "ja-JP"

def _m12_inject(env):
    env["navigator.userAgentData.platform"] = "macOS"

def _m13_inject(env):
    env["window.devicePixelRatio"] = -1.0

def _m14_inject(env):
    env["media.pointer"] = "stylus"

def _m15_inject(env):
    env["media.hover"] = "always"

def _m16_inject(env):
    env["identity.cpu_cores"] = 0

def _m17_inject(env):
    env["identity.memory_gb"] = 0

def _m18_inject(env):
    env["screen.colorDepth"] = 16

def _m19_inject(env):
    env["window.outerWidth"] = env.get("window.innerWidth", 1920) - 1

def _m20_inject(env):
    env["screen.availHeight"] = env.get("screen.height", 1080) + 100

def _m21_inject(env):
    env["navigator.userAgentData.mobile"] = True

def _m22_inject(env):
    env["navigator.vendor"] = "Google Inc."
    env["capabilities.windowChrome"] = False

def _m23_inject(env):
    env["identity.browser.brand"] = "chrome"
    env["navigator.pdfViewerEnabled"] = False

def _m24_inject(env):
    env["navigator.maxTouchPoints"] = 5

def _m25_inject(env):
    env["webgl.UNMASKED_VENDOR_WEBGL"] = "Google Inc. (NVIDIA)"
    env["webgl.UNMASKED_RENDERER_WEBGL"] = (
        "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)"
    )

def _m26_inject(env):
    env["locale.accept_language"] = "ja-JP,ja;q=0.9,en;q=0.8"

def _m27_inject(env):
    env["identity.browser.brand"] = "chrome"
    env["navigator.vendor"] = "Google Inc."
    env["navigator.userAgentData.brands"] = [
        {"brand": "Firefox", "version": "121"}
    ]


MUTATIONS = [
    ("M01", "platform -> MacIntel (contradicts Windows UA)", "MR-EQ-001",
     _m01_inject, "MR-EQ-001"),
    ("M02", "vendor -> Microsoft (matches no browser)", "MR-EQ-002",
     _m02_inject, "MR-EQ-002"),
    ("M03", "webgl renderer -> Intel (contradicts NVIDIA vendor)", "MR-EQ-010",
     _m03_inject, "MR-EQ-010"),
    ("M04", "screen.width -> 800 (< window.innerWidth)", "MR-BND-001",
     _m04_inject, "MR-BND-001"),
    ("M05", "pointer=coarse, any-pointer=fine", "MR-IMP-003",
     _m05_inject, "MR-IMP-003"),
    ("M06", "hover=none, any-hover=hover", "MR-IMP-004",
     _m06_inject, "MR-IMP-004"),
    ("M07", "permissions.geolocation -> unknown", "MR-VAL-001",
     _m07_inject, "MR-VAL-001"),
    ("M08", "audio.baseLatency -> 0", "MR-BND-006",
     _m08_inject, "MR-BND-006"),
    ("M09", "fonts.families -> []", "MR-VAL-003",
     _m09_inject, "MR-VAL-003"),
    ("M10", "display.color-gamut -> invalid", "MR-VAL-002",
     _m10_inject, "MR-VAL-002"),
    ("M11", "navigator.language -> ja-JP (mismatches locale)", "MR-EQ-004",
     _m11_inject, "MR-EQ-004"),
    ("M12", "UA-CH platform -> macOS (contradicts Windows)", "MR-IMP-010",
     _m12_inject, "MR-IMP-010"),
    ("M13", "devicePixelRatio -> -1.0", "MR-BND-007",
     _m13_inject, "MR-BND-007"),
    ("M14", "media.pointer -> stylus (invalid value)", "MR-VAL-004",
     _m14_inject, "MR-VAL-004"),
    ("M15", "media.hover -> always (invalid value)", "MR-VAL-005",
     _m15_inject, "MR-VAL-005"),
    ("M16", "identity.cpu_cores -> 0", "MR-BND-008",
     _m16_inject, "MR-BND-008"),
    ("M17", "identity.memory_gb -> 0", "MR-BND-009",
     _m17_inject, "MR-BND-009"),
    ("M18", "screen.colorDepth -> 16 (invalid)", "MR-BND-010",
     _m18_inject, "MR-BND-010"),
    ("M19", "outerWidth < innerWidth", "MR-BND-003",
     _m19_inject, "MR-BND-003"),
    ("M20", "availHeight > screen.height", "MR-BND-002",
     _m20_inject, "MR-BND-002"),
    ("M21", "UA-CH mobile=true, maxTouchPoints=0", "MR-EQ-009",
     _m21_inject, "MR-EQ-009"),
    ("M22", "vendor=Google, windowChrome=false", "MR-IMP-001",
     _m22_inject, "MR-IMP-001"),
    ("M23", "brand=chrome, pdfViewerEnabled=false", "MR-IMP-005",
     _m23_inject, "MR-IMP-005"),
    ("M24", "desktop maxTouchPoints=5", "MR-IMP-006",
     _m24_inject, "MR-IMP-006"),
    ("M25", "NVIDIA vendor, Intel renderer", "MR-IMP-008",
     _m25_inject, "MR-IMP-008"),
    ("M26", "Accept-Language mismatches locale", "MR-EQ-006",
     _m26_inject, "MR-EQ-006"),
    ("M27", "UA-CH brands don't match UA browser", "MR-EQ-008",
     _m27_inject, "MR-EQ-008"),
]


def run():
    print("=== Harness Mutation Testing (D-101: 27 mutations) ===")
    print()

    baseline = load_runtime_env()
    baseline = _enrich_baseline(baseline)

    print("--- Baseline sanity (all 36 MR checks must pass) ---")
    baseline_ok = True
    for mr_id in sorted(MR_MAP.keys()):
        _name, _cat, fn = MR_MAP[mr_id]
        passed, detail = fn(baseline)
        status = "PASS" if passed else "FAIL"
        print(f"  [{status}] {mr_id}: {detail}")
        if not passed:
            baseline_ok = False
    if not baseline_ok:
        print()
        print("[ERROR] Baseline env fails MR checks; aborting mutation test.")
        return 1
    print()

    print("--- Mutation Results ---")
    killed = 0
    survived = 0
    survivors = []

    for mid, desc, _rule, mutator, mr_id in MUTATIONS:
        env_mut = copy.deepcopy(baseline)
        mutator(env_mut)
        _name, _cat, fn = MR_MAP[mr_id]
        passed, detail = fn(env_mut)
        if not passed:
            status = "KILLED"
            killed += 1
        else:
            status = "SURVIVED"
            survived += 1
            survivors.append((mid, desc, mr_id))
        print(f"  [{status}] {mid}: {desc} (detected by {mr_id})")

    print()
    print(f"Total: {killed} killed, {survived} survived")
    if killed + survived > 0:
        score = killed / (killed + survived) * 100
    else:
        score = 0.0
    print(f"Mutation Score: {score:.0f}%")

    if survivors:
        print()
        print("--- Survivors (harness false-negatives) ---")
        for mid, desc, mr_id in survivors:
            print(f"  {mid} ({mr_id}): {desc}")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(run())
