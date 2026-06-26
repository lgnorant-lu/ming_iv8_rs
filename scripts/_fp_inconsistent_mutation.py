"""FP-Inconsistent D-check mutation testing.

For each of the 25 D-check groups, inject a contradiction matching at least
one block rule in that group. The D-check should detect the contradiction
(FAIL / return False = KILLED). If it does not (SURVIVED), the D-check or
rule evaluation has a bug.

Usage: .venv\\Scripts\\python.exe scripts\\_fp_inconsistent_mutation.py
"""

import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO_ROOT / "scripts"))

from fp_inconsistent_rules import (
    parse_filterlist,
    categorize_rules,
    get_fp_field,
    match_value,
)
from evaluate_env_consistency import build_d_checks, _hardcoded_fallback_env


def load_rules():
    rules = parse_filterlist()
    applicable, _ = categorize_rules(rules)
    groups = {}
    for r in applicable:
        groups.setdefault(r["pair_key"], []).append(r)
    return applicable, groups


UA_DERIVED_FIELDS = {"ua_device", "ua_os", "ua_browser", "ua_vendor", "vendor_flavors"}


def _strip_pipe(v):
    if v is None:
        return None
    s = str(v)
    if s.endswith("|"):
        return s[:-1]
    return s


def build_combined_ua(targets):
    """Build a UA string that satisfies multiple ua_* field targets.

    targets: dict of field_name -> desired value
    Returns UA string, or None if impossible.
    """
    device = _strip_pipe(targets.get("ua_device"))
    os_name = _strip_pipe(targets.get("ua_os"))
    browser = _strip_pipe(targets.get("ua_browser"))
    vendor = _strip_pipe(targets.get("ua_vendor"))
    vf = _strip_pipe(targets.get("vendor_flavors"))

    if vf == "gCrWeb" and not os_name:
        os_name = "Android"
    if vf == "gCrWeb" and not device:
        device = "Generic Smartphone"

    parts = ["Mozilla/5.0 "]
    paren = []

    if device == "Mac" or os_name == "Mac OS X":
        paren.append("Macintosh; Intel Mac OS X 10_15_7")
    elif device == "iPhone" or os_name == "iOS":
        paren.append("iPhone; CPU iPhone OS 17_0 like Mac OS X")
    elif device == "iPad":
        paren.append("iPad; CPU OS 17_0 like Mac OS X")
    elif os_name == "Windows" or device == "desktop":
        paren.append("Windows NT 10.0; Win64; x64")
    elif os_name == "Linux":
        paren.append("X11; Linux x86_64")
    elif os_name == "Android" or (device and device not in ("Mac", "iPhone", "iPad", "desktop")):
        if device and device not in ("Mac", "iPhone", "iPad", "desktop", "Generic Smartphone"):
            paren.append(f"Linux; Android 13; {device}")
        else:
            paren.append("Linux; Android 13")
    else:
        paren.append("Windows NT 10.0; Win64; x64")

    parts.append(f"({''.join(paren)})")

    engine_parts = ["AppleWebKit/537.36", "(KHTML, like Gecko)"]

    if browser == "HeadlessChrome":
        engine_parts.append("HeadlessChrome/147.0.0.0")
    elif browser == "Edge":
        engine_parts.append("Chrome/147.0.0.0")
        engine_parts.append("Safari/537.36")
        engine_parts.append("Edg/147.0.0.0")
    elif browser == "Opera":
        engine_parts.append("Chrome/147.0.0.0")
        engine_parts.append("Safari/537.36")
        engine_parts.append("OPR/132.0.0.0")
    elif browser == "Samsung Internet":
        engine_parts.append("SamsungBrowser/22.0")
        engine_parts.append("Chrome/147.0.0.0")
        engine_parts.append("Mobile Safari/537.36")
    elif browser == "MiuiBrowser":
        engine_parts.append("MiuiBrowser/18.0")
        engine_parts.append("Chrome/147.0.0.0")
        engine_parts.append("Mobile Safari/537.36")
    elif browser == "Firefox":
        parts = ["Mozilla/5.0 "]
        if os_name == "Windows" or (not os_name and not device):
            parts.append("(Windows NT 10.0; Win64; x64)")
        elif os_name == "Linux":
            parts.append("(X11; Linux x86_64)")
        elif os_name == "Mac OS X" or device == "Mac":
            parts.append("(Macintosh; Intel Mac OS X 10.15)")
        else:
            parts.append(f"({''.join(paren)})")
        parts.append("Gecko/20100101")
        parts.append("Firefox/120.0")
        return " ".join(parts)
    elif browser == "Firefox iOS":
        parts = ["Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)"]
        parts.append("AppleWebKit/605.1.15 (KHTML, like Gecko)")
        parts.append("FxiOS/120.0 Mobile/15E148 Safari/605.1.15")
        return " ".join(parts)
    elif browser == "Chrome Mobile iOS":
        parts = ["Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)"]
        parts.append("AppleWebKit/605.1.15 (KHTML, like Gecko)")
        parts.append("CriOS/147.0.0.0 Mobile/15E148 Safari/604.1")
        return " ".join(parts)
    elif browser == "Chrome Mobile":
        engine_parts.append("Chrome/147.0.0.0")
        engine_parts.append("Mobile Safari/537.36")
    elif browser == "Mobile Safari":
        parts = ["Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)"]
        parts.append("AppleWebKit/605.1.15 (KHTML, like Gecko)")
        parts.append("Version/17.0 Mobile/15E148 Safari/604.1")
        return " ".join(parts)
    elif browser == "Safari":
        parts = ["Mozilla/5.0 "]
        if os_name == "Linux":
            parts.append("(X11; Linux x86_64)")
        elif os_name == "Windows":
            parts.append("(Windows NT 10.0; Win64; x64)")
        elif os_name == "Mac OS X" or device == "Mac":
            parts.append("(Macintosh; Intel Mac OS X 10_15_7)")
        else:
            parts.append(f"({''.join(paren)})")
        parts.append("AppleWebKit/605.1.15 (KHTML, like Gecko)")
        parts.append("Version/17.0 Safari/605.1.15")
        return " ".join(parts)
    elif browser == "Chrome" or vendor == "Google":
        engine_parts.append("Chrome/147.0.0.0")
        engine_parts.append("Safari/537.36")
    elif vendor == "Apple":
        parts = ["Mozilla/5.0 "]
        parts.append(f"({''.join(paren)})")
        parts.append("AppleWebKit/605.1.15 (KHTML, like Gecko)")
        parts.append("Version/17.0 Safari/605.1.15")
        return " ".join(parts)
    elif vendor == "Mozilla":
        parts = ["Mozilla/5.0 "]
        if os_name == "Windows" or (not os_name and not device):
            parts.append("(Windows NT 10.0; Win64; x64)")
        else:
            parts.append(f"({''.join(paren)})")
        parts.append("Gecko/20100101 Firefox/120.0")
        return " ".join(parts)
    elif vendor == "Generic_Android":
        parts = ["Mozilla/5.0 (Linux; Android 13)"]
        parts.append("AppleWebKit/537.36 (KHTML, like Gecko)")
        parts.append("Version/4.0 Mobile Safari/537.36")
        return " ".join(parts)
    elif vendor == "Spider":
        return "Mozilla/5.0 (compatible; spider/1.0)"
    else:
        engine_parts.append("Chrome/147.0.0.0")
        engine_parts.append("Safari/537.36")

    ua = " ".join(parts[:2]) + " " + " ".join(engine_parts)

    if vf in ("gCrWeb", "gCrWeb|"):
        if "wv" not in ua:
            ua += " wv"

    return ua


def set_env_field(field_name, value, env):
    """Set a non-UA env field to a target value."""
    if field_name == "hw_concurrency":
        env["navigator.hardwareConcurrency"] = int(float(value))
    elif field_name == "device_memory":
        env["navigator.deviceMemory"] = float(value)
    elif field_name == "maxTouchPoints":
        env["navigator.maxTouchPoints"] = int(float(value))
    elif field_name == "touch_support":
        rv = str(value).strip().strip('"')
        if rv == "":
            env["navigator.maxTouchPoints"] = 0
        else:
            env["navigator.maxTouchPoints"] = 5
    elif field_name == "screen_resolution":
        parts = value.split("x")
        if len(parts) == 2:
            env["screen.width"] = int(parts[0])
            env["screen.height"] = int(parts[1])
    elif field_name == "platform":
        env["navigator.platform"] = value
    elif field_name == "vendor":
        env["navigator.vendor"] = value
    elif field_name == "color_gamut":
        env["display.color-gamut"] = value
    elif field_name == "os_cpu":
        env["navigator.osCPU"] = value
    elif field_name == "plugins":
        env["navigator.plugins"] = [value]


def try_rule_mutation(rule, base_env):
    """Try to create a mutation from a single block rule.

    Returns (success, mutated_env, mutated_ua, reason).
    """
    env = dict(base_env)

    ua_targets = {}
    fa, va = rule["field_a"], rule["value_a"]
    fb, vb = rule["field_b"], rule["value_b"]

    if fa in UA_DERIVED_FIELDS:
        ua_targets[fa] = va
    else:
        set_env_field(fa, va, env)

    if fb in UA_DERIVED_FIELDS:
        ua_targets[fb] = vb
    else:
        set_env_field(fb, vb, env)

    ua = base_env.get("navigator.userAgent", "")
    if ua_targets:
        built = build_combined_ua(ua_targets)
        if built is None:
            return False, env, ua, f"cannot build UA for {ua_targets}"
        ua = built

    env["navigator.userAgent"] = ua

    actual_a = get_fp_field(fa, env, ua)
    actual_b = get_fp_field(fb, env, ua)
    a_match = match_value(fa, va, actual_a)
    b_match = match_value(fb, vb, actual_b)

    if a_match and b_match:
        return True, env, ua, None
    else:
        reasons = []
        if not a_match:
            reasons.append(f"field_a {fa}={va!r} got {actual_a!r}")
        if not b_match:
            reasons.append(f"field_b {fb}={vb!r} got {actual_b!r}")
        return False, env, ua, "; ".join(reasons)


def run_mutation_tests():
    _, groups = load_rules()
    d_checks = build_d_checks()
    base_env = _hardcoded_fallback_env()

    d_check_map = {}
    for cid, desc, fn in d_checks:
        d_check_map[cid] = (desc, fn)

    results = []

    print("=== FP-Inconsistent D-Check Mutation Testing ===")
    print()

    idx = 1
    for pk in sorted(groups.keys()):
        cid = f"D{idx:02d}"
        rules_in_group = groups[pk]
        block_rules = [r for r in rules_in_group if r["action"] == "block"]

        desc = d_check_map.get(cid, ("?", None))[0]

        if not block_rules:
            print(f"  [{cid}] {desc}")
            print(f"         NO_BLOCK_RULE - group has only allow rules, no mutation possible")
            print(f"         Result: SKIP (no block rules to test)")
            results.append({
                "cid": cid,
                "pair_key": pk,
                "status": "SKIP",
                "reason": "no block rules in group",
                "rule_count": len(rules_in_group),
            })
            idx += 1
            continue

        mutation_found = False
        tried_count = 0
        last_reason = ""
        for rule in block_rules:
            tried_count += 1
            success, env, ua, reason = try_rule_mutation(rule, base_env)
            if success:
                passes = d_check_map[cid][1](env)
                if not passes:
                    status = "KILLED"
                else:
                    status = "SURVIVED"

                print(f"  [{cid}] {desc}")
                print(f"         Mutation: {rule['field_a']}={rule['value_a']!r} + {rule['field_b']}={rule['value_b']!r}")
                print(f"         Result: {status}")
                results.append({
                    "cid": cid,
                    "pair_key": pk,
                    "status": status,
                    "rule": f"{rule['field_a']}={rule['value_a']}, {rule['field_b']}={rule['value_b']}",
                    "rule_count": len(rules_in_group),
                    "block_count": len(block_rules),
                    "tried": tried_count,
                })
                mutation_found = True
                break
            else:
                last_reason = reason

        if not mutation_found:
            print(f"  [{cid}] {desc}")
            print(f"         Tried {tried_count}/{len(block_rules)} block rules, none triggerable")
            print(f"         Last: {last_reason}")
            print(f"         Result: SURVIVED (no triggerable block rule)")
            results.append({
                "cid": cid,
                "pair_key": pk,
                "status": "SURVIVED",
                "reason": f"no triggerable block rule (tried {tried_count}/{len(block_rules)}): {last_reason}",
                "rule_count": len(rules_in_group),
                "block_count": len(block_rules),
                "tried": tried_count,
            })

        idx += 1

    print()
    killed = sum(1 for r in results if r["status"] == "KILLED")
    survived = sum(1 for r in results if r["status"] == "SURVIVED")
    skipped = sum(1 for r in results if r["status"] == "SKIP")
    total = len(results)

    print(f"--- Summary ---")
    print(f"  Total D-check mutations: {total}")
    print(f"  KILLED:   {killed}")
    print(f"  SURVIVED: {survived}")
    print(f"  SKIP:     {skipped}")

    if survived > 0:
        print()
        print(f"--- SURVIVED Details ---")
        for r in results:
            if r["status"] == "SURVIVED":
                print(f"  {r['cid']} ({r['pair_key']}): {r.get('reason', 'D-check did not detect contradiction')}")

    _, groups = load_rules()
    total_rules = sum(len(g) for g in groups.values())
    groups_with_kill = set()
    for r in results:
        if r["status"] == "KILLED":
            groups_with_kill.add(r["pair_key"])

    validated_rules = 0
    for pk, grp in groups.items():
        if pk in groups_with_kill:
            validated_rules += len(grp)

    print()
    print(f"--- Rule Coverage ---")
    print(f"  Total applicable rules: {total_rules}")
    print(f"  Rules in KILLED groups: {validated_rules}")
    print(f"  Rules in non-KILLED groups: {total_rules - validated_rules}")

    return 0 if survived == 0 else 1


if __name__ == "__main__":
    sys.exit(run_mutation_tests())
