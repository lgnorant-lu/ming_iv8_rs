"""FP-Inconsistent filterlist parser and rule evaluator.

Parses rules from data/fp_inconsistent_filterlist_raw.txt into structured form,
categorizes applicable vs excluded rules, and provides field mapping helpers
for evaluating rules against an IV8 runtime env.

Source: https://github.com/hariv/fp_inconsistent (rules/filterlist.txt)
FP-Inconsistent has no license; we extract facts/methods only.
"""

import json
import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
FILTERLIST_PATH = REPO_ROOT / "data" / "fp_inconsistent_filterlist_raw.txt"
RULES_JSON_PATH = REPO_ROOT / "data" / "fp_inconsistent_rules.json"

EXCLUDED_FIELDS = {"ip_location", "timezone"}

INVALID_UA_OS_VALUES = {"Apple Mail"}

FIELD_PAIR_GROUPS = [
    "hw_concurrency<>ua_device",
    "maxTouchPoints<>ua_device",
    "screen_resolution<>ua_device",
    "platform<>vendor",
    "platform<>ua_browser",
    "platform<>ua_os",
    "device_memory<>ua_device",
    "ua_vendor<>vendor",
    "ua_browser<>ua_os",
    "platform<>vendor_flavors",
    "ua_browser<>vendor",
    "color_gamut<>ua_device",
    "os_cpu<>ua_browser",
    "device_memory<>ua_os",
    "os_cpu<>ua_os",
    "maxTouchPoints<>ua_os",
    "maxTouchPoints<>touch_support",
    "plugins<>ua_device",
    "touch_support<>ua_device",
    "plugins<>ua_os",
    "maxTouchPoints<>platform",
    "touch_support<>ua_os",
    "ua_browser<>vendor_flavors",
    "os_cpu<>platform",
    "ua_os<>vendor",
]


def parse_filterlist(path=FILTERLIST_PATH):
    """Parse filterlist.txt into list of rule dicts.

    Each rule: {field_a, value_a, field_b, value_b, action, pair_key}
    """
    text = path.read_text(encoding="utf-8")
    lines = [l.strip() for l in text.split("\n") if l.strip()]
    rules = []
    for line in lines:
        parts = line.split("|$$|")
        parts = [p.strip() for p in parts]
        if len(parts) < 5:
            continue
        field_a = parts[0]
        value_a = parts[1]
        field_b = parts[2]
        value_b = parts[3]
        action = parts[4] if len(parts) > 4 else "block"
        action = action.strip().lower()
        if action not in ("block", "allow"):
            action = "block"

        pair_key = "|".join(sorted([field_a, field_b]))
        rules.append({
            "field_a": field_a,
            "value_a": value_a,
            "field_b": field_b,
            "value_b": value_b,
            "action": action,
            "pair_key": pair_key,
        })
    return rules


def categorize_rules(rules):
    """Split rules into applicable and excluded.

    Excluded: any rule involving ip_location or timezone, or rules with
    ua_os values that cannot be produced by parse_ua_os (e.g. "Apple Mail"
    is a mail client, not an OS).
    """
    applicable = []
    excluded = []
    for r in rules:
        if r["field_a"] in EXCLUDED_FIELDS or r["field_b"] in EXCLUDED_FIELDS:
            excluded.append(r)
        elif r["field_a"] == "ua_os" and r["value_a"] in INVALID_UA_OS_VALUES:
            excluded.append(r)
        elif r["field_b"] == "ua_os" and r["value_b"] in INVALID_UA_OS_VALUES:
            excluded.append(r)
        else:
            applicable.append(r)
    return applicable, excluded


def save_rules_json(rules, path=RULES_JSON_PATH):
    applicable, excluded = categorize_rules(rules)
    data = {
        "total_rules": len(rules),
        "applicable_count": len(applicable),
        "excluded_count": len(excluded),
        "excluded_reason": "ip_location/timezone (IV8 has no network layer) or invalid ua_os values (e.g. Apple Mail is a mail client, not an OS)",
        "applicable_rules": applicable,
        "excluded_rules": excluded,
    }
    path.write_text(json.dumps(data, indent=2), encoding="utf-8")
    return data


# --- UA Parsing ---

def parse_ua_device(ua):
    """Derive ua_device from User-Agent string.

    Returns device category matching FP-Inconsistent values:
    'Mac', 'iPhone', 'iPad', or specific Android model strings,
    or 'desktop' for Windows/Linux.
    """
    ua_lower = ua.lower()
    if "iphone" in ua_lower:
        return "iPhone"
    if "ipad" in ua_lower:
        return "iPad"
    if "macintosh" in ua_lower or "mac os x" in ua_lower:
        return "Mac"
    if "android" in ua_lower:
        m = re.search(r"Android[^;]*;\s*([^)]+)\)", ua)
        if m:
            model = m.group(1).strip()
            if model:
                return model
        return "Generic Smartphone"
    if "windows nt" in ua_lower:
        return "desktop"
    if "linux" in ua_lower:
        return "desktop"
    return "desktop"


def parse_ua_os(ua):
    """Derive ua_os from User-Agent string.

    Returns: 'Windows', 'Android', 'iOS', 'Mac OS X', 'Linux', etc.
    """
    ua_lower = ua.lower()
    if "windows nt" in ua_lower:
        return "Windows"
    if "android" in ua_lower:
        return "Android"
    if "iphone" in ua_lower or "ipad" in ua_lower:
        return "iOS"
    if "mac os x" in ua_lower or "macintosh" in ua_lower:
        return "Mac OS X"
    if "linux" in ua_lower:
        return "Linux"
    return "Unknown"


def parse_ua_browser(ua):
    """Derive ua_browser from User-Agent string.

    Returns: 'Chrome', 'Firefox', 'Safari', 'Edge', 'Opera',
    'Chrome Mobile', 'Chrome Mobile iOS', 'Mobile Safari',
    'Firefox iOS', 'Samsung Internet', 'MiuiBrowser', 'HeadlessChrome', etc.
    """
    ua_lower = ua.lower()
    if "headlesschrome" in ua_lower:
        return "HeadlessChrome"
    if "edg/" in ua_lower or "edge/" in ua_lower:
        return "Edge"
    if "opera" in ua_lower or "opr/" in ua_lower:
        return "Opera"
    if "samsungbrowser" in ua_lower:
        return "Samsung Internet"
    if "miuibrowser" in ua_lower:
        return "MiuiBrowser"
    if "firefox/" in ua_lower:
        if "fxios" in ua_lower:
            return "Firefox iOS"
        return "Firefox"
    if "crios" in ua_lower:
        return "Chrome Mobile iOS"
    if "chrome/" in ua_lower:
        if "mobile" in ua_lower:
            return "Chrome Mobile"
        return "Chrome"
    if "safari/" in ua_lower:
        if "mobile" in ua_lower:
            return "Mobile Safari"
        return "Safari"
    return "Chrome"


def parse_ua_vendor(ua):
    """Derive ua_vendor from User-Agent string.

    Returns: 'Google', 'Apple', 'Generic_Android', 'Spider', etc.
    """
    ua_lower = ua.lower()
    if "chrome" in ua_lower or "chromium" in ua_lower:
        return "Google"
    if "safari" in ua_lower and "chrome" not in ua_lower:
        return "Apple"
    if "firefox" in ua_lower:
        return "Mozilla"
    if "android" in ua_lower:
        return "Generic_Android"
    return "Google"


def parse_vendor_flavors(ua, vendor):
    """Derive vendor_flavors from UA and vendor.

    FP-Inconsistent uses vendor_flavors like 'gCrWeb' (Google Chrome WebView).
    """
    ua_lower = ua.lower()
    if "android" in ua_lower and "chrome" in ua_lower:
        if "wv" in ua_lower or "webview" in ua_lower:
            return "gCrWeb"
    return ""


# --- Env Field Mappers ---

def get_fp_field(field_name, env, ua):
    """Get a FP-Inconsistent field value from the IV8 runtime env.

    Maps FP-Inconsistent field names to IV8 env keys.
    """
    if field_name == "hw_concurrency":
        v = env.get("navigator.hardwareConcurrency", 0)
        try:
            return float(v)
        except (TypeError, ValueError):
            return float(v) if v else 0.0

    if field_name == "device_memory":
        v = env.get("navigator.deviceMemory", 0)
        try:
            return float(v)
        except (TypeError, ValueError):
            return float(v) if v else 0.0

    if field_name == "maxTouchPoints":
        v = env.get("navigator.maxTouchPoints", 0)
        try:
            return int(v)
        except (TypeError, ValueError):
            return int(v) if v else 0

    if field_name == "touch_support":
        mtp = env.get("navigator.maxTouchPoints", 0)
        try:
            mtp = int(mtp)
        except (TypeError, ValueError):
            mtp = 0
        return "" if mtp == 0 else "true"

    if field_name == "screen_resolution":
        sw = env.get("screen.width", 0)
        sh = env.get("screen.height", 0)
        return f"{sw}x{sh}"

    if field_name == "platform":
        return env.get("navigator.platform", "")

    if field_name == "vendor":
        return env.get("navigator.vendor", "")

    if field_name == "color_gamut":
        return env.get("display.color-gamut", "")

    if field_name == "os_cpu":
        oscpu = env.get("navigator.osCPU", "")
        if oscpu:
            return oscpu
        platform = env.get("navigator.platform", "")
        ua_os = parse_ua_os(ua)
        if "Win" in platform:
            return "Windows NT"
        if "Mac" in platform:
            return "Intel Mac OS X"
        if "Linux" in platform:
            return "Linux x86_64"
        return ""

    if field_name == "plugins":
        plugins = env.get("navigator.plugins", [])
        if isinstance(plugins, list) and plugins:
            return plugins[0]
        return ""

    if field_name == "ua_device":
        return parse_ua_device(ua)

    if field_name == "ua_browser":
        return parse_ua_browser(ua)

    if field_name == "ua_os":
        return parse_ua_os(ua)

    if field_name == "ua_vendor":
        return parse_ua_vendor(ua)

    if field_name == "vendor_flavors":
        return parse_vendor_flavors(ua, env.get("navigator.vendor", ""))

    return None


def match_value(field_name, rule_value, actual_value):
    """Check if a rule value matches the actual runtime value.

    Different fields have different matching semantics:
    - Numeric fields: compare as floats (with tolerance for -1 sentinel)
    - String fields: substring or exact match
    """
    if actual_value is None:
        return False

    if field_name in ("hw_concurrency", "device_memory"):
        try:
            rv = float(rule_value)
            av = float(actual_value)
            return abs(rv - av) < 0.01
        except (TypeError, ValueError):
            return str(rule_value) == str(actual_value)

    if field_name == "maxTouchPoints":
        try:
            rv = int(float(rule_value))
            av = int(actual_value)
            return rv == av
        except (TypeError, ValueError):
            return str(rule_value) == str(actual_value)

    if field_name == "touch_support":
        rv = str(rule_value).strip().strip('"')
        if rv.endswith("|"):
            rv = rv[:-1].strip().strip('"')
        av = str(actual_value).strip()
        if rv == "" and av == "":
            return True
        if rv == '""' and av == "":
            return True
        return rv == av

    if field_name == "screen_resolution":
        return str(rule_value) == str(actual_value)

    if field_name == "plugins":
        rv = str(rule_value).strip()
        av = str(actual_value).strip()
        if rv in av or av in rv:
            return True
        return rv == av

    if field_name in ("platform", "os_cpu"):
        rv = str(rule_value).strip()
        av = str(actual_value).strip()
        if rv == av:
            return True
        if rv in av or av in rv:
            return True
        return False

    if field_name in ("vendor", "color_gamut", "ua_device",
                      "ua_browser", "ua_os", "ua_vendor",
                      "vendor_flavors"):
        rv = str(rule_value).strip()
        av = str(actual_value).strip()
        if rv == av:
            return True
        if rv.endswith("|"):
            candidates = [p.strip() for p in rv[:-1].split("|") if p.strip()]
            candidates.append("")
            return av in candidates
        return False

    return str(rule_value) == str(actual_value)


def evaluate_rule(rule, env, ua):
    """Evaluate a single FP-Inconsistent rule against the env.

    Returns: 'block_match' if a block rule matches (inconsistent),
             'allow_match' if an allow rule matches (consistent),
             'no_match' if neither.
    """
    actual_a = get_fp_field(rule["field_a"], env, ua)
    actual_b = get_fp_field(rule["field_b"], env, ua)

    a_match = match_value(rule["field_a"], rule["value_a"], actual_a)
    b_match = match_value(rule["field_b"], rule["value_b"], actual_b)

    if a_match and b_match:
        if rule["action"] == "block":
            return "block_match"
        else:
            return "allow_match"
    return "no_match"


def evaluate_group(pair_key, rules_in_group, env, ua):
    """Evaluate all rules in a field-pair group.

    Returns: (passes, details)
    - passes=True if no block rule matches
    - details: dict with matched block/allow rules
    """
    block_matches = []
    allow_matches = []
    for rule in rules_in_group:
        result = evaluate_rule(rule, env, ua)
        if result == "block_match":
            block_matches.append(rule)
        elif result == "allow_match":
            allow_matches.append(rule)

    passes = len(block_matches) == 0
    details = {
        "pair_key": pair_key,
        "total_rules": len(rules_in_group),
        "block_matches": block_matches,
        "allow_matches": allow_matches,
    }
    return passes, details


def generate_rules_json():
    """Parse filterlist and save structured rules JSON."""
    rules = parse_filterlist()
    data = save_rules_json(rules)
    return data


if __name__ == "__main__":
    data = generate_rules_json()
    print(f"Total rules parsed: {data['total_rules']}")
    print(f"Applicable rules: {data['applicable_count']}")
    print(f"Excluded rules: {data['excluded_count']}")
    print(f"Saved to: {RULES_JSON_PATH}")
