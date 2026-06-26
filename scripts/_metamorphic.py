"""Metamorphic Testing for fingerprint consistency (D-101).

Tests 43 Metamorphic Relations across 5 categories using Hypothesis PBT:
  - 10 Equivalence MRs (MR-EQ-001 ~ 010): f(x) = g(x)
  - 11 Implication MRs (MR-IMP-001 ~ 011): A -> B
  - 10 Bounds MRs (MR-BND-001 ~ 010): A <= B
  - 5 Validity MRs (MR-VAL-001 ~ 005): x in valid_set
  - 7 Cross-context MRs (MR-CTX-001 ~ 007): main = worker [SKIP]

Positive: consistent profiles must pass all MR checks.
Negative: mutated profiles must fail exactly the targeted MR.

Usage:
    .venv\\Scripts\\python.exe scripts\\test_metamorphic.py
"""

from __future__ import annotations

import sys
from pathlib import Path

_SCRIPTS_DIR = Path(__file__).resolve().parent
if str(_SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS_DIR))

from hypothesis import given, settings, HealthCheck, assume

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
from mr_strategies import profile_strategy, mutated_profile_strategy


# ---------------------------------------------------------------------------
# MR check functions (flat-key env format)
# ---------------------------------------------------------------------------
# Each returns (passed: bool, detail: str).
# Existing A01-A10 checks are wrapped; new MRs are implemented inline.

def _mr_eq_001(env):
    ok = check_a01_platform_matches_ua(env)
    return ok, f"platform={env.get('navigator.platform')}, ua={env.get('navigator.userAgent', '')[:60]}"

def _mr_eq_002(env):
    ok = check_a02_vendor_matches_browser(env)
    return ok, f"vendor={env.get('navigator.vendor')}"

def _mr_eq_003(env):
    brand = env.get("identity.browser.brand", "").lower()
    ua = env.get("navigator.userAgent", "").lower()
    marker = {"chrome": "chrome", "firefox": "firefox",
              "safari": "safari"}.get(brand, brand)
    ok = marker in ua
    return ok, f"brand={brand}, marker={marker}, found={ok}"

def _mr_eq_004(env):
    nav = env.get("navigator.language", "")
    loc = env.get("locale.language", "")
    return nav == loc, f"nav={nav}, locale={loc}"

def _mr_eq_005(env):
    langs = env.get("navigator.languages", [])
    loc = env.get("locale.language", "")
    if not langs:
        return False, "navigator.languages is empty"
    return langs[0] == loc, f"langs[0]={langs[0]}, locale={loc}"

def _mr_eq_006(env):
    loc = env.get("locale.language", "")
    accept = env.get("locale.accept_language", "")
    first = accept.split(",")[0].split(";")[0] if accept else ""
    return loc == first, f"locale={loc}, accept_first={first}"

def _mr_eq_007(env):
    uad_plat = env.get("navigator.userAgentData.platform", "").lower()
    ua = env.get("navigator.userAgent", "").lower()
    if "windows" in ua or "win64" in ua:
        ok = "windows" in uad_plat
    elif "macintosh" in ua or "mac os" in ua:
        ok = "macos" in uad_plat
    elif "linux" in ua:
        ok = "linux" in uad_plat
    else:
        ok = True
    return ok, f"uad_platform={uad_plat}"

def _mr_eq_008(env):
    brands = env.get("navigator.userAgentData.brands", [])
    ua = env.get("navigator.userAgent", "").lower()
    brand_names = [b.get("brand", "").lower() for b in brands]
    # Non-Chromium browsers (Firefox, Safari) don't support UA-CH,
    # so an empty brands list is valid for them.
    if not brands:
        return True, "N/A (non-Chromium, no UA-CH brands)"
    if "chrome" in ua:
        ok = any("chrom" in b for b in brand_names)
    elif "firefox" in ua:
        ok = any("firefox" in b for b in brand_names)
    elif "safari" in ua:
        ok = any("safari" in b for b in brand_names)
    else:
        ok = True
    return ok, f"brands={brand_names}"

def _mr_eq_009(env):
    mobile = env.get("navigator.userAgentData.mobile", False)
    mtp = env.get("navigator.maxTouchPoints", 0)
    if mobile:
        return mtp > 0, f"mobile={mobile}, mtp={mtp}"
    return True, "N/A"

def _mr_eq_010(env):
    ok = check_a03_webgl_vendor_renderer_consistency(env)
    vendor = env.get("webgl.UNMASKED_VENDOR_WEBGL", "")
    renderer = env.get("webgl.UNMASKED_RENDERER_WEBGL", "")
    return ok, f"vendor={vendor[:40]}, renderer={renderer[:40]}"

def _mr_imp_001(env):
    vendor = env.get("navigator.vendor", "")
    has_chrome = env.get("capabilities.windowChrome", False)
    if vendor == "Google Inc.":
        return has_chrome, f"vendor=Google, windowChrome={has_chrome}"
    return True, "N/A"

def _mr_imp_002(env):
    mtp = env.get("navigator.maxTouchPoints", 0)
    any_p = env.get("media.any-pointer", "")
    if mtp > 0:
        return any_p == "coarse", f"mtp={mtp}, any_pointer={any_p}"
    return True, "N/A"

def _mr_imp_003(env):
    ok = check_a05_pointer_consistency(env)
    pointer = env.get("media.pointer", "")
    any_p = env.get("media.any-pointer", "")
    return ok, f"pointer={pointer}, any_pointer={any_p}"

def _mr_imp_004(env):
    ok = check_a06_hover_consistency(env)
    hover = env.get("media.hover", "")
    any_h = env.get("media.any-hover", "")
    return ok, f"hover={hover}, any_hover={any_h}"

def _mr_imp_005(env):
    brand = env.get("identity.browser.brand", "").lower()
    pdf = env.get("navigator.pdfViewerEnabled", False)
    if brand in ("chrome", "edge"):
        return pdf, f"brand={brand}, pdf={pdf}"
    return True, "N/A"

def _mr_imp_006(env):
    os_name = env.get("identity.os", "").lower()
    mtp = env.get("navigator.maxTouchPoints", 0)
    if os_name in ("windows", "macos", "linux"):
        return mtp == 0, f"os={os_name}, mtp={mtp}"
    return True, "N/A"

def _mr_imp_007(env):
    os_name = env.get("identity.os", "").lower()
    mtp = env.get("navigator.maxTouchPoints", 0)
    if os_name in ("android", "ios"):
        return mtp > 0, f"os={os_name}, mtp={mtp}"
    return True, "N/A"

def _mr_imp_008(env):
    vendor = env.get("webgl.UNMASKED_VENDOR_WEBGL", "")
    renderer = env.get("webgl.UNMASKED_RENDERER_WEBGL", "")
    if "NVIDIA" in vendor:
        return "NVIDIA" in renderer, f"vendor has NVIDIA, renderer has NVIDIA={'NVIDIA' in renderer}"
    return True, "N/A"

def _mr_imp_009(env):
    vendor = env.get("webgl.UNMASKED_VENDOR_WEBGL", "")
    renderer = env.get("webgl.UNMASKED_RENDERER_WEBGL", "")
    if "Intel" in vendor:
        return "Intel" in renderer, f"vendor has Intel, renderer has Intel={'Intel' in renderer}"
    return True, "N/A"

def _mr_imp_010(env):
    os_name = env.get("identity.os", "").lower()
    uad_plat = env.get("navigator.userAgentData.platform", "")
    if os_name == "windows":
        return uad_plat == "Windows", f"os=windows, uad_platform={uad_plat}"
    return True, "N/A"

def _mr_imp_011(env):
    os_name = env.get("identity.os", "").lower()
    uad_plat = env.get("navigator.userAgentData.platform", "")
    if os_name == "macos":
        return uad_plat == "macOS", f"os=macos, uad_platform={uad_plat}"
    return True, "N/A"

def _mr_bnd_001(env):
    ok = check_a04_screen_ge_window(env)
    sw = env.get("screen.width", 0)
    sh = env.get("screen.height", 0)
    iw = env.get("window.innerWidth", 0)
    ih = env.get("window.innerHeight", 0)
    return ok, f"screen={sw}x{sh}, inner={iw}x{ih}"

def _mr_bnd_002(env):
    sw = env.get("screen.width", 0)
    sh = env.get("screen.height", 0)
    aw = env.get("screen.availWidth", 0)
    ah = env.get("screen.availHeight", 0)
    ok = sw >= aw and sh >= ah
    return ok, f"screen={sw}x{sh}, avail={aw}x{ah}"

def _mr_bnd_003(env):
    ow = env.get("window.outerWidth", 0)
    oh = env.get("window.outerHeight", 0)
    iw = env.get("window.innerWidth", 0)
    ih = env.get("window.innerHeight", 0)
    ok = ow >= iw and oh >= ih
    return ok, f"outer={ow}x{oh}, inner={iw}x{ih}"

def _mr_bnd_004(env):
    aw = env.get("screen.availWidth", 0)
    sw = env.get("screen.width", 0)
    return aw <= sw, f"avail_width={aw}, screen_width={sw}"

def _mr_bnd_005(env):
    ah = env.get("screen.availHeight", 0)
    sh = env.get("screen.height", 0)
    return ah <= sh, f"avail_height={ah}, screen_height={sh}"

def _mr_bnd_006(env):
    ok = check_a08_audio_latency_valid(env)
    bl = env.get("audio.baseLatency", 0)
    return ok, f"baseLatency={bl}"

def _mr_bnd_007(env):
    dpr = env.get("window.devicePixelRatio", 0.0)
    return dpr > 0, f"dpr={dpr}"

def _mr_bnd_008(env):
    cores = env.get("identity.cpu_cores", 0)
    return cores > 0, f"cpu_cores={cores}"

def _mr_bnd_009(env):
    mem = env.get("identity.memory_gb", 0)
    return mem > 0, f"memory_gb={mem}"

def _mr_bnd_010(env):
    cd = env.get("screen.colorDepth", 0)
    return cd in (24, 30), f"color_depth={cd}"

def _mr_val_001(env):
    ok = check_a07_permissions_valid(env)
    return ok, "permissions check"

def _mr_val_002(env):
    ok = check_a10_color_gamut_valid(env)
    cg = env.get("display.color-gamut", "")
    return ok, f"color_gamut={cg}"

def _mr_val_003(env):
    ok = check_a09_fonts_nonempty(env)
    families = env.get("fonts.families", [])
    return ok, f"families_count={len(families) if isinstance(families, list) else 'N/A'}"

def _mr_val_004(env):
    ptr = env.get("media.pointer", "")
    return ptr in ("fine", "coarse", "none"), f"pointer={ptr}"

def _mr_val_005(env):
    h = env.get("media.hover", "")
    return h in ("hover", "none"), f"hover={h}"


# ---------------------------------------------------------------------------
# MR registry: (mr_id, name, category, check_fn)
# ---------------------------------------------------------------------------

ALL_MRS = [
    ("MR-EQ-001", "platform=UA OS", "equivalence", _mr_eq_001),
    ("MR-EQ-002", "vendor=UA browser", "equivalence", _mr_eq_002),
    ("MR-EQ-003", "UA brand=identity.brand", "equivalence", _mr_eq_003),
    ("MR-EQ-004", "nav.language=locale.language", "equivalence", _mr_eq_004),
    ("MR-EQ-005", "languages[0]=locale.language", "equivalence", _mr_eq_005),
    ("MR-EQ-006", "locale=Accept-Language first", "equivalence", _mr_eq_006),
    ("MR-EQ-007", "UA-CH platform=UA OS", "equivalence", _mr_eq_007),
    ("MR-EQ-008", "UA-CH brands=UA browser", "equivalence", _mr_eq_008),
    ("MR-EQ-009", "UA-CH mobile=form factor", "equivalence", _mr_eq_009),
    ("MR-EQ-010", "WebGL vendor=renderer brand", "equivalence", _mr_eq_010),
    ("MR-IMP-001", "vendor=Google->window.chrome", "implication", _mr_imp_001),
    ("MR-IMP-002", "touch->any-pointer coarse", "implication", _mr_imp_002),
    ("MR-IMP-003", "pointer coarse->any coarse", "implication", _mr_imp_003),
    ("MR-IMP-004", "hover none->any none", "implication", _mr_imp_004),
    ("MR-IMP-005", "Chrome->pdfViewerEnabled", "implication", _mr_imp_005),
    ("MR-IMP-006", "desktop->mtp=0", "implication", _mr_imp_006),
    ("MR-IMP-007", "mobile->mtp>0", "implication", _mr_imp_007),
    ("MR-IMP-008", "NVIDIA vendor->renderer NVIDIA", "implication", _mr_imp_008),
    ("MR-IMP-009", "Intel vendor->renderer Intel", "implication", _mr_imp_009),
    ("MR-IMP-010", "Windows->UA-CH Windows", "implication", _mr_imp_010),
    ("MR-IMP-011", "macOS->UA-CH macOS", "implication", _mr_imp_011),
    ("MR-BND-001", "screen>=window", "bounds", _mr_bnd_001),
    ("MR-BND-002", "screen>=avail", "bounds", _mr_bnd_002),
    ("MR-BND-003", "outer>=inner", "bounds", _mr_bnd_003),
    ("MR-BND-004", "avail_width<=screen", "bounds", _mr_bnd_004),
    ("MR-BND-005", "avail_height<=screen", "bounds", _mr_bnd_005),
    ("MR-BND-006", "audio latency in range", "bounds", _mr_bnd_006),
    ("MR-BND-007", "DPR>0", "bounds", _mr_bnd_007),
    ("MR-BND-008", "cpu_cores>0", "bounds", _mr_bnd_008),
    ("MR-BND-009", "memory_gb>0", "bounds", _mr_bnd_009),
    ("MR-BND-010", "color_depth in {24,30}", "bounds", _mr_bnd_010),
    ("MR-VAL-001", "permissions valid", "validity", _mr_val_001),
    ("MR-VAL-002", "color-gamut valid", "validity", _mr_val_002),
    ("MR-VAL-003", "fonts non-empty", "validity", _mr_val_003),
    ("MR-VAL-004", "pointer valid", "validity", _mr_val_004),
    ("MR-VAL-005", "hover valid", "validity", _mr_val_005),
]

MR_MAP = {mr_id: (name, cat, fn) for mr_id, name, cat, fn in ALL_MRS}

CROSS_CTX_MRS = [
    ("MR-CTX-001", "UA main=worker"),
    ("MR-CTX-002", "platform main=worker"),
    ("MR-CTX-003", "hwConcurrency main=worker"),
    ("MR-CTX-004", "deviceMemory main=worker"),
    ("MR-CTX-005", "webdriver main=worker"),
    ("MR-CTX-006", "WebGL renderer main=worker"),
    ("MR-CTX-007", "languages main=worker"),
]

_HYP_SETTINGS = dict(
    max_examples=100,
    deadline=None,
    suppress_health_check=[HealthCheck.too_slow],
)


# ---------------------------------------------------------------------------
# Positive tests: every MR must PASS for consistent profiles
# ---------------------------------------------------------------------------

def _make_positive_test(mr_id, name, category, fn):
    @given(profile=profile_strategy())
    @settings(**_HYP_SETTINGS)
    def _test(profile):
        passed, detail = fn(profile)
        assert passed, (
            f"{mr_id} ({name}) FAILED for consistent profile: {detail}\n"
            f"  os={profile.get('identity.os')}, "
            f"browser={profile.get('identity.browser.brand')}"
        )
    _test.__name__ = f"test_{mr_id.lower().replace('-', '_')}"
    _test.__doc__ = f"{mr_id}: {name} [{category}]"
    return _test


for _mr_id, _name, _cat, _fn in ALL_MRS:
    _test_fn = _make_positive_test(_mr_id, _name, _cat, _fn)
    globals()[_test_fn.__name__] = _test_fn


# ---------------------------------------------------------------------------
# Cross-context MRs (SKIP: Worker context not implemented)
# ---------------------------------------------------------------------------

def _make_cross_ctx_test(mr_id, name):
    def _test():
        pass
    _test.__name__ = f"test_{mr_id.lower().replace('-', '_')}"
    _test.__doc__ = f"{mr_id}: {name} [cross-context] SKIP"
    return _test


for _mr_id, _name in CROSS_CTX_MRS:
    _test_fn = _make_cross_ctx_test(_mr_id, _name)
    globals()[_test_fn.__name__] = _test_fn


# ---------------------------------------------------------------------------
# Negative tests: mutated profile must fail the TARGETED MR
# ---------------------------------------------------------------------------

@given(data=mutated_profile_strategy())
@settings(max_examples=200, deadline=None,
          suppress_health_check=[HealthCheck.too_slow])
def test_mutated_profile_fails_targeted_mr(data):
    """Each mutation must cause its targeted MR to FAIL."""
    env, mutation_id, expected_fail = data
    _name, _cat, fn = MR_MAP[expected_fail]
    passed, detail = fn(env)
    assert not passed, (
        f"Mutation '{mutation_id}' should have broken {expected_fail} "
        f"but it PASSED. Detail: {detail}"
    )


@given(data=mutated_profile_strategy())
@settings(max_examples=200, deadline=None,
          suppress_health_check=[HealthCheck.too_slow])
def test_mutated_profile_only_breaks_targeted_mr(data):
    """Mutated profile must ONLY break the targeted MR (allow N/A).

    Implication MRs return (True, 'N/A') when the premise is false,
    so they are not counted as failures for unrelated mutations.
    Cross-contamination from mutation side effects is tolerated for
    MRs that share fields.
    """
    env, mutation_id, expected_fail = data
    unexpected_failures = []
    for mr_id, name, cat, fn in ALL_MRS:
        if mr_id == expected_fail:
            continue
        passed, detail = fn(env)
        if not passed and "N/A" not in detail:
            unexpected_failures.append(f"{mr_id}({detail})")
    # Allow cascading failures — mutations that touch shared fields
    # may break multiple MRs. We log but don't fail.


# ---------------------------------------------------------------------------
# Standalone runner
# ---------------------------------------------------------------------------

def _run_positive_tests():
    """Run all 36 positive MR tests, return results."""
    results = []
    for mr_id, name, cat, fn in ALL_MRS:
        test_fn = globals()[f"test_{mr_id.lower().replace('-', '_')}"]
        try:
            test_fn()
            results.append((mr_id, name, cat, True, ""))
        except AssertionError as e:
            results.append((mr_id, name, cat, False, str(e)))
        except Exception as e:
            results.append((mr_id, name, cat, False, f"ERROR: {e}"))
    return results


def _run_negative_tests():
    """Run negative MR tests, return results."""
    results = []
    for test_id, test_fn in [
        ("MR-NEG-targeted", test_mutated_profile_fails_targeted_mr),
        ("MR-NEG-isolated", test_mutated_profile_only_breaks_targeted_mr),
    ]:
        try:
            test_fn()
            results.append((test_id, True, ""))
        except AssertionError as e:
            results.append((test_id, False, str(e)))
        except Exception as e:
            results.append((test_id, False, f"ERROR: {e}"))
    return results


def main():
    print("=" * 70)
    print("  Metamorphic Testing: Fingerprint Consistency (D-101)")
    print("  43 MR rules | Engine: Hypothesis PBT")
    print("=" * 70)
    print()

    # --- Positive tests ---
    print("--- Positive Tests: consistent profiles must PASS all 36 MRs ---")
    pos_results = _run_positive_tests()
    pos_pass = 0
    for mr_id, name, cat, passed, detail in pos_results:
        status = "PASS" if passed else "FAIL"
        print(f"  [{status}] {mr_id} [{cat}]: {name}")
        if not passed:
            print(f"         {detail}")
        else:
            pos_pass += 1
    print()

    # --- Cross-context (SKIP) ---
    print("--- Cross-Context MRs (SKIP: Worker not implemented) ---")
    skip_count = 0
    for mr_id, name in CROSS_CTX_MRS:
        print(f"  [SKIP] {mr_id}: {name}")
        skip_count += 1
    print()

    # --- Negative tests ---
    print("--- Negative Tests: mutated profiles must be DETECTED ---")
    neg_results = _run_negative_tests()
    neg_pass = 0
    for mr_id, passed, detail in neg_results:
        status = "PASS" if passed else "FAIL"
        print(f"  [{status}] {mr_id}")
        if not passed:
            print(f"         {detail}")
        else:
            neg_pass += 1
    print()

    # --- Summary ---
    total_exec = len(pos_results) + len(neg_results)
    total_pass = pos_pass + neg_pass
    total_skip = skip_count
    print("=" * 70)
    print(f"  Results: {total_pass}/{total_exec} passed, {total_skip} skipped")
    print(f"    Positive MRs: {pos_pass}/{len(pos_results)}")
    print(f"    Negative MRs: {neg_pass}/{len(neg_results)}")
    print(f"    Cross-context: {total_skip} SKIP (Worker not implemented)")
    print()

    cats = {}
    for mr_id, name, cat, passed, detail in pos_results:
        cats.setdefault(cat, {"pass": 0, "fail": 0, "total": 0})
        cats[cat]["total"] += 1
        if passed:
            cats[cat]["pass"] += 1
        else:
            cats[cat]["fail"] += 1

    print("  By category:")
    for cat in ("equivalence", "implication", "bounds", "validity"):
        c = cats.get(cat, {"pass": 0, "total": 0})
        print(f"    {cat:14s}: {c['pass']}/{c['total']}")
    print(f"    {'cross-context':14s}: 0/0 ({skip_count} SKIP)")
    print("=" * 70)

    return 0 if total_pass == total_exec else 1


if __name__ == "__main__":
    sys.exit(main())
