"""H02 Harness Mutation Testing (D-105).

Injects deliberate defects into a valid runtime env, one per A-class rule,
and verifies each defect is caught by its corresponding check.

A "killed" mutant means the harness detected the injected defect (check
returned False). A "survived" mutant means the harness missed it — a
false-negative in the harness itself.

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


# Each mutation: (id, description, rule, mutator fn, checker fn)
# The mutator receives a fresh deep copy of the baseline env and injects
# exactly one defect. The checker is the A-class rule that should catch it.

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


MUTATIONS = [
    ("M01", "platform -> MacIntel (contradicts Windows UA)", "A01",
     _m01_inject, check_a01_platform_matches_ua),
    ("M02", "vendor -> Microsoft (matches no browser)", "A02",
     _m02_inject, check_a02_vendor_matches_browser),
    ("M03", "webgl renderer -> Intel (contradicts NVIDIA vendor)", "A03",
     _m03_inject, check_a03_webgl_vendor_renderer_consistency),
    ("M04", "screen.width -> 800 (< window.innerWidth)", "A04",
     _m04_inject, check_a04_screen_ge_window),
    ("M05", "pointer=coarse, any-pointer=fine", "A05",
     _m05_inject, check_a05_pointer_consistency),
    ("M06", "hover=none, any-hover=hover", "A06",
     _m06_inject, check_a06_hover_consistency),
    ("M07", "permissions.geolocation -> unknown", "A07",
     _m07_inject, check_a07_permissions_valid),
    ("M08", "audio.baseLatency -> 0", "A08",
     _m08_inject, check_a08_audio_latency_valid),
    ("M09", "fonts.families -> []", "A09",
     _m09_inject, check_a09_fonts_nonempty),
    ("M10", "display.color-gamut -> invalid", "A10",
     _m10_inject, check_a10_color_gamut_valid),
]


def run():
    print("=== H02 Harness Mutation Testing ===")
    print()

    baseline = load_runtime_env()

    # Sanity: baseline must pass every A-class check, otherwise a "kill"
    # might be caused by the baseline itself rather than the mutation.
    print("--- Baseline sanity (all A-class checks must pass) ---")
    all_checks = [
        ("A01", check_a01_platform_matches_ua),
        ("A02", check_a02_vendor_matches_browser),
        ("A03", check_a03_webgl_vendor_renderer_consistency),
        ("A04", check_a04_screen_ge_window),
        ("A05", check_a05_pointer_consistency),
        ("A06", check_a06_hover_consistency),
        ("A07", check_a07_permissions_valid),
        ("A08", check_a08_audio_latency_valid),
        ("A09", check_a09_fonts_nonempty),
        ("A10", check_a10_color_gamut_valid),
    ]
    baseline_ok = True
    for cid, fn in all_checks:
        ok = fn(baseline)
        print(f"  [{'PASS' if ok else 'FAIL'}] {cid}")
        if not ok:
            baseline_ok = False
    if not baseline_ok:
        print()
        print("[ERROR] Baseline env fails A-class checks; aborting mutation test.")
        return 1
    print()

    print("--- Mutation Results ---")
    killed = 0
    survived = 0
    survivors = []

    for mid, desc, rule, mutator, checker in MUTATIONS:
        env_mut = copy.deepcopy(baseline)
        mutator(env_mut)
        detected = checker(env_mut)
        if not detected:
            status = "KILLED"
            killed += 1
        else:
            status = "SURVIVED"
            survived += 1
            survivors.append((mid, desc, rule))
        print(f"  [{status}] {mid}: {desc} (detected by {rule})")

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
        for mid, desc, rule in survivors:
            print(f"  {mid} ({rule}): {desc}")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(run())
