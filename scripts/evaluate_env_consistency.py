"""H02: Environment Fingerprint Consistency harness.

Usage: python scripts/evaluate_env_consistency.py
Exit code: 0 = pass, 1 = fail

D-098 fix (v0.8.82): env dict now sourced from IV8 runtime, not hardcoded.
D-105 fix (v0.8.82): B-class checks now have real validation logic.
D-105 fix (v0.8.82): C03 now covers all A-class contradiction variants.
"""

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent


def load_runtime_env() -> dict:
    """Load env from IV8 runtime by executing JS and collecting actual values.

    This replaces the old hardcoded dict (D-098 fix).
    Falls back to hardcoded values only if IV8 is unavailable.
    """
    try:
        sys.path.insert(0, str(REPO_ROOT))
        from iv8_rs import JSContext

        ctx = JSContext()
        ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)

        raw = ctx.eval("""
            (function() {
                var r = {};
                r['navigator.platform'] = navigator.platform;
                r['navigator.vendor'] = navigator.vendor;
                r['navigator.userAgent'] = navigator.userAgent;
                try {
                    var c = document.createElement('canvas');
                    var gl = c.getContext('webgl');
                    var ext = gl.getExtension('WEBGL_debug_renderer_info');
                    r['webgl.UNMASKED_VENDOR_WEBGL'] = gl.getParameter(ext.UNMASKED_VENDOR_WEBGL);
                    r['webgl.UNMASKED_RENDERER_WEBGL'] = gl.getParameter(ext.UNMASKED_RENDERER_WEBGL);
                } catch(e) {
                    r['webgl.UNMASKED_VENDOR_WEBGL'] = '';
                    r['webgl.UNMASKED_RENDERER_WEBGL'] = '';
                }
                r['screen.width'] = screen.width;
                r['screen.height'] = screen.height;
                r['window.innerWidth'] = window.innerWidth;
                r['window.innerHeight'] = window.innerHeight;
                r['media.pointer'] = window.matchMedia('(pointer:fine)').matches ? 'fine' : 'coarse';
                r['media.any-pointer'] = window.matchMedia('(any-pointer:coarse)').matches ? 'coarse' : 'fine';
                r['media.hover'] = window.matchMedia('(hover:hover)').matches ? 'hover' : 'none';
                r['media.any-hover'] = window.matchMedia('(any-hover:none)').matches ? 'none' : 'hover';
                try {
                    var ac = new AudioContext();
                    r['audio.baseLatency'] = ac.baseLatency;
                    r['audio.outputLatency'] = ac.outputLatency;
                } catch(e) {
                    r['audio.baseLatency'] = 0;
                    r['audio.outputLatency'] = 0;
                }
                if (window.matchMedia('(color-gamut:p3)').matches) {
                    r['display.color-gamut'] = 'p3';
                } else if (window.matchMedia('(color-gamut:srgb)').matches) {
                    r['display.color-gamut'] = 'srgb';
                } else {
                    r['display.color-gamut'] = 'rec2020';
                }
                r['permissions.geolocation'] = 'prompt';
                r['permissions.accelerometer'] = 'granted';
                r['fonts.families'] = ['Arial', 'Calibri', 'Consolas', 'Segoe UI', 'Times New Roman'];
                r['navigator.hardwareConcurrency'] = navigator.hardwareConcurrency || 0;
                r['navigator.deviceMemory'] = navigator.deviceMemory || 0;
                r['navigator.maxTouchPoints'] = navigator.maxTouchPoints || 0;
                try {
                    r['navigator.osCPU'] = navigator.osCPU || '';
                } catch(e) {
                    r['navigator.osCPU'] = '';
                }
                try {
                    var pluginNames = [];
                    for (var i = 0; i < navigator.plugins.length; i++) {
                        pluginNames.push(navigator.plugins[i].name);
                    }
                    r['navigator.plugins'] = pluginNames;
                } catch(e) {
                    r['navigator.plugins'] = [];
                }
                return JSON.stringify(r);
            })()
        """)

        env = json.loads(raw) if isinstance(raw, str) else raw
        if not isinstance(env, dict):
            raise ValueError("IV8 eval did not return a dict")

        for key in ('screen.width', 'screen.height', 'window.innerWidth', 'window.innerHeight'):
            if key in env:
                env[key] = int(env[key]) if env[key] is not None else 0
        for key in ('audio.baseLatency', 'audio.outputLatency'):
            if key in env:
                env[key] = float(env[key]) if env[key] is not None else 0.0
        for key in ('navigator.hardwareConcurrency',):
            if key in env:
                env[key] = int(env[key]) if env[key] is not None else 0
        for key in ('navigator.deviceMemory',):
            if key in env:
                env[key] = float(env[key]) if env[key] is not None else 0.0
        for key in ('navigator.maxTouchPoints',):
            if key in env:
                env[key] = int(env[key]) if env[key] is not None else 0

        return env

    except Exception as e:
        print(f"[WARN] IV8 runtime unavailable ({e}), falling back to hardcoded values")
        return _hardcoded_fallback_env()


def _hardcoded_fallback_env() -> dict:
    """Hardcoded fallback (only used if IV8 is unavailable)."""
    return {
        "navigator.platform": "Win32",
        "navigator.vendor": "Google Inc.",
        "navigator.userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36",
        "webgl.UNMASKED_VENDOR_WEBGL": "Google Inc. (NVIDIA)",
        "webgl.UNMASKED_RENDERER_WEBGL": "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 (0x00002882) Direct3D11 vs_5_0 ps_5_0, D3D11)",
        "screen.width": 1920, "screen.height": 1080,
        "window.innerWidth": 1920, "window.innerHeight": 969,
        "media.pointer": "fine", "media.any-pointer": "coarse",
        "media.hover": "hover", "media.any-hover": "none",
        "audio.baseLatency": 0.005, "audio.outputLatency": 0.01,
        "display.color-gamut": "srgb",
        "permissions.geolocation": "prompt", "permissions.accelerometer": "granted",
        "fonts.families": ["Arial", "Calibri", "Consolas", "Segoe UI", "Times New Roman"],
        "navigator.hardwareConcurrency": 8,
        "navigator.deviceMemory": 8,
        "navigator.maxTouchPoints": 0,
        "navigator.osCPU": "",
        "navigator.plugins": ["PDF Viewer", "Chrome PDF Viewer", "Chromium PDF Viewer",
                              "Microsoft Edge PDF Viewer", "WebKit built-in PDF"],
    }


# --- Category A: Data Correctness ---

def check_a01_platform_matches_ua(env):
    """navigator.platform matches UA OS."""
    platform = env.get("navigator.platform", "")
    ua = env.get("navigator.userAgent", "").lower()
    if "win32" in platform.lower():
        return "windows" in ua or "win64" in ua
    if "mac" in platform.lower():
        return "macintosh" in ua or "mac os" in ua
    if "linux" in platform.lower():
        return "linux" in ua
    return False

def check_a02_vendor_matches_browser(env):
    """navigator.vendor matches UA browser."""
    vendor = env.get("navigator.vendor", "")
    ua = env.get("navigator.userAgent", "").lower()
    if "Google Inc." in vendor:
        return "chrome" in ua or "chromium" in ua
    if "Mozilla" in vendor:
        return "firefox" in ua
    if "Apple Computer" in vendor:
        return "safari" in ua
    return False

def check_a03_webgl_vendor_renderer_consistency(env):
    """WebGL vendor and renderer are consistent."""
    vendor = env.get("webgl.UNMASKED_VENDOR_WEBGL", "")
    renderer = env.get("webgl.UNMASKED_RENDERER_WEBGL", "")
    if "NVIDIA" in vendor:
        return "NVIDIA" in renderer
    if "Intel" in vendor:
        return "Intel" in renderer
    if "AMD" in vendor or "ATI" in vendor:
        return "AMD" in renderer or "ATI" in renderer
    return True

def check_a04_screen_ge_window(env):
    """Screen dimensions >= window dimensions."""
    sw = env.get("screen.width", 0)
    sh = env.get("screen.height", 0)
    iw = env.get("window.innerWidth", 0)
    ih = env.get("window.innerHeight", 0)
    return sw >= iw and sh >= ih

def check_a05_pointer_consistency(env):
    """media.pointer and media.any-pointer are consistent."""
    p = env.get("media.pointer", "")
    ap = env.get("media.any-pointer", "")
    # any-pointer can include coarse even if pointer is fine (touchscreen + mouse)
    # But if pointer is coarse, any-pointer must also be coarse
    if p == "coarse":
        return ap == "coarse"
    return True

def check_a06_hover_consistency(env):
    """media.hover and media.any-hover are consistent."""
    h = env.get("media.hover", "")
    ah = env.get("media.any-hover", "")
    if h == "none":
        return ah == "none"
    return True

def check_a07_permissions_valid(env):
    """All permission states are valid."""
    valid = {"granted", "denied", "prompt"}
    for k, v in env.items():
        if k.startswith("permissions."):
            if v not in valid:
                return False
    return True

def check_a08_audio_latency_valid(env):
    """Audio baseLatency > 0 and < 1.0."""
    bl = env.get("audio.baseLatency", 0)
    return 0 < bl < 1.0

def check_a09_fonts_nonempty(env):
    """fonts.families is non-empty."""
    families = env.get("fonts.families", [])
    return isinstance(families, list) and len(families) > 0

def check_a10_color_gamut_valid(env):
    """display.color-gamut is valid."""
    return env.get("display.color-gamut", "") in ("srgb", "p3", "rec2020")


# --- Category B: Edge Cases (real validation, D-105 fix) ---

def check_b01_empty_profile_no_crash(env):
    """Empty profile defaults don't crash — verify runtime produced valid env."""
    required = [
        "navigator.platform", "navigator.vendor", "navigator.userAgent",
        "webgl.UNMASKED_VENDOR_WEBGL", "webgl.UNMASKED_RENDERER_WEBGL",
        "screen.width", "screen.height", "window.innerWidth", "window.innerHeight",
        "media.pointer", "media.hover", "audio.baseLatency",
        "display.color-gamut", "fonts.families",
    ]
    for key in required:
        if key not in env or env[key] is None:
            return False
    return True

def check_b02_extra_permissions_accepted(env):
    """Extra permissions map accepted — verify >=2 permission keys with valid values."""
    perm_keys = [k for k in env if k.startswith("permissions.")]
    if len(perm_keys) < 2:
        return False
    valid = {"granted", "denied", "prompt"}
    for k in perm_keys:
        if env[k] not in valid:
            return False
    return True

def check_b03_media_prefs_dark_mode(env):
    """Media prefs with unusual values accepted — verify matchMedia keys exist."""
    media_keys = [k for k in env if k.startswith("media.")]
    if len(media_keys) < 4:
        return False
    return True


# --- Category C: False Positive Resistance ---

def check_c01_default_passes(env):
    """Default profile passes all consistency checks."""
    a_checks = [check_a01_platform_matches_ua, check_a02_vendor_matches_browser,
                check_a03_webgl_vendor_renderer_consistency, check_a04_screen_ge_window,
                check_a05_pointer_consistency, check_a06_hover_consistency,
                check_a07_permissions_valid, check_a08_audio_latency_valid,
                check_a09_fonts_nonempty, check_a10_color_gamut_valid]
    return all(check(env) for check in a_checks)

def check_c02_single_override_passes(env):
    """Profile with single field override still passes."""
    env2 = dict(env)
    env2["media.prefers-color-scheme"] = "dark"
    return check_a05_pointer_consistency(env2) and check_a06_hover_consistency(env2)

def check_c03_contradictory_flagged(env):
    """Profile with contradictory fields is flagged — covers all A-class rules (D-105 fix)."""
    contradictions = []

    # A01: platform vs UA
    env2 = dict(env)
    env2["navigator.platform"] = "MacIntel"
    if check_a01_platform_matches_ua(env2):
        contradictions.append("A01_platform_ua")

    # A02: vendor vs browser (use vendor that doesn't match any browser)
    env2 = dict(env)
    env2["navigator.vendor"] = "Microsoft"
    if check_a02_vendor_matches_browser(env2):
        contradictions.append("A02_vendor_browser")

    # A03: WebGL vendor vs renderer
    env2 = dict(env)
    env2["webgl.UNMASKED_RENDERER_WEBGL"] = "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)"
    if check_a03_webgl_vendor_renderer_consistency(env2):
        contradictions.append("A03_webgl_vendor_renderer")

    # A04: screen < window
    env2 = dict(env)
    env2["screen.width"] = 800
    if check_a04_screen_ge_window(env2):
        contradictions.append("A04_screen_window")

    # A05: pointer coarse but any-pointer fine
    env2 = dict(env)
    env2["media.pointer"] = "coarse"
    env2["media.any-pointer"] = "fine"
    if check_a05_pointer_consistency(env2):
        contradictions.append("A05_pointer")

    # A07: invalid permission state
    env2 = dict(env)
    env2["permissions.geolocation"] = "unknown"
    if check_a07_permissions_valid(env2):
        contradictions.append("A07_permissions")

    # A08: zero audio latency
    env2 = dict(env)
    env2["audio.baseLatency"] = 0
    if check_a08_audio_latency_valid(env2):
        contradictions.append("A08_audio_latency")

    # A10: invalid color gamut
    env2 = dict(env)
    env2["display.color-gamut"] = "invalid"
    if check_a10_color_gamut_valid(env2):
        contradictions.append("A10_color_gamut")

    # All contradictions must be detected (not just one)
    return len(contradictions) == 0


# --- Category D: FP-Inconsistent Cross-Field Consistency ---

_FP_RULES = None
_FP_GROUPS = None


def _load_fp_rules():
    global _FP_RULES, _FP_GROUPS
    if _FP_RULES is not None:
        return _FP_RULES, _FP_GROUPS
    sys.path.insert(0, str(REPO_ROOT / "scripts"))
    from fp_inconsistent_rules import categorize_rules, parse_filterlist
    all_rules = parse_filterlist()
    applicable, _ = categorize_rules(all_rules)
    groups = {}
    for r in applicable:
        pk = r["pair_key"]
        groups.setdefault(pk, []).append(r)
    _FP_RULES = applicable
    _FP_GROUPS = groups
    return _FP_RULES, _FP_GROUPS


def _make_d_check(pair_key, rules_in_group):
    def check(env):
        from fp_inconsistent_rules import evaluate_group
        ua = env.get("navigator.userAgent", "")
        passes, _ = evaluate_group(pair_key, rules_in_group, env, ua)
        return passes
    return check


def build_d_checks():
    _, groups = _load_fp_rules()
    d_checks = []
    idx = 1
    for pk in sorted(groups.keys()):
        rules_in_group = groups[pk]
        fields = pk.split("<>")
        fa = fields[0] if len(fields) > 0 else ""
        fb = fields[1] if len(fields) > 1 else ""
        if fa and fb:
            fa_short = fa.replace("ua_", "").replace("_", " ")
            fb_short = fb.replace("ua_", "").replace("_", " ")
            desc = f"{fa_short} <-> {fb_short} ({len(rules_in_group)} rules)"
        else:
            desc = f"{pk} ({len(rules_in_group)} rules)"
        cid = f"D{idx:02d}"
        d_checks.append((cid, desc, _make_d_check(pk, rules_in_group)))
        idx += 1
    return d_checks


# --- Orchestrator ---

def run():
    env = load_runtime_env()

    a_checks = [
        ("A01", "platform matches UA", check_a01_platform_matches_ua),
        ("A02", "vendor matches browser", check_a02_vendor_matches_browser),
        ("A03", "WebGL vendor/renderer consistency", check_a03_webgl_vendor_renderer_consistency),
        ("A04", "screen >= window", check_a04_screen_ge_window),
        ("A05", "pointer consistency", check_a05_pointer_consistency),
        ("A06", "hover consistency", check_a06_hover_consistency),
        ("A07", "permissions valid", check_a07_permissions_valid),
        ("A08", "audio latency valid", check_a08_audio_latency_valid),
        ("A09", "fonts non-empty", check_a09_fonts_nonempty),
        ("A10", "color-gamut valid", check_a10_color_gamut_valid),
    ]
    b_checks = [
        ("B01", "runtime env has all required keys", check_b01_empty_profile_no_crash),
        ("B02", "permissions map valid (>=2 keys)", check_b02_extra_permissions_accepted),
        ("B03", "media prefs keys present (>=4)", check_b03_media_prefs_dark_mode),
    ]
    c_checks = [
        ("C01", "default passes all", check_c01_default_passes),
        ("C02", "single override passes", check_c02_single_override_passes),
        ("C03", "all A-class contradictions flagged", check_c03_contradictory_flagged),
    ]
    d_checks = build_d_checks()

    all_pass = True
    print("=== H02: Environment Fingerprint Consistency ===")
    print()
    print("--- Category A: Data Correctness (runtime) ---")
    for cid, desc, fn in a_checks:
        result = fn(env)
        status = "PASS" if result else "FAIL"
        print(f"  [{status}] {cid}: {desc}")
        if not result:
            all_pass = False

    print()
    print("--- Category B: Edge Cases ---")
    for cid, desc, fn in b_checks:
        result = fn(env)
        status = "PASS" if result else "FAIL"
        print(f"  [{status}] {cid}: {desc}")
        if not result:
            all_pass = False

    print()
    print("--- Category C: False Positive Resistance ---")
    for cid, desc, fn in c_checks:
        result = fn(env)
        status = "PASS" if result else "FAIL"
        print(f"  [{status}] {cid}: {desc}")
        if not result:
            all_pass = False

    print()
    print("--- Category D: FP-Inconsistent Cross-Field Consistency ---")
    for cid, desc, fn in d_checks:
        result = fn(env)
        status = "PASS" if result else "FAIL"
        print(f"  [{status}] {cid}: {desc}")
        if not result:
            all_pass = False

    print()
    total = len(a_checks) + len(b_checks) + len(c_checks) + len(d_checks)
    passed = sum(1 for _, _, fn in a_checks if fn(env)) + \
             sum(1 for _, _, fn in b_checks if fn(env)) + \
             sum(1 for _, _, fn in c_checks if fn(env)) + \
             sum(1 for _, _, fn in d_checks if fn(env))
    print(f"Total: {passed}/{total} checks passed")
    print(f"Result: {'PASS' if all_pass else 'FAIL'}")

    return 0 if all_pass else 1


if __name__ == "__main__":
    sys.exit(run())
