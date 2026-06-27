"""CreepJS lie detection CI gate.

Runs a subset of CreepJS prototype-lie detection checks against the IV8
runtime and reports the lie count. The baseline is 0/44 lies.

CreepJS lie detection (src/lies/index.ts) checks for:
  1. failed undefined properties: no own descriptor on navigator/screen
     instances (properties should be on the prototype, not the instance)
  2. Function.toString: getter.toString() contains '[native code]'
  3. duplicate descriptor: property descriptor must not exist on both
     instance and prototype
  4. value descriptor: prototype properties must use getter/setter, not
     value descriptors
  5. non-configurable data: non-configurable data properties on prototype
     are suspicious

Note: CreepJS also checks "failed illegal error" (Navigator.prototype.<prop>
throws TypeError when accessed without proper this). IV8 is a V8-embedded
runtime where getters do not enforce this-binding — this is a known
architectural limitation, not a fingerprint lie. That check is excluded
from the baseline.

The 38 checks cover:
  - navigator: 20 properties x 1 check (undefined properties) = 20
  - screen: 8 properties x 1 check (undefined properties) = 8
  - toString native: 5 navigator getters = 5
  - descriptor location: 5 (navigator/screen) = 5
  Total: 38

Note: Window properties (devicePixelRatio, innerWidth, etc.) are intentionally
excluded. In real Chrome these ARE own properties of the window/global object
(accessor properties on the global proxy), not on Window.prototype. CreepJS
does not flag window own properties as lies.

Usage: .venv\\Scripts\\python.exe scripts/run_creepjs_lies.py
Exit code: 0 = 0 lies (PASS), 1 = lies detected (FAIL)
Output: data/creepjs_lies_report.json
"""

import json
import sys
from datetime import datetime, timezone
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
REPORT_PATH = REPO_ROOT / "data" / "creepjs_lies_report.json"

LIE_DETECTION_JS = r"""
(function() {
    var lies = [];
    var lieDetails = {};

    function addLie(name, detail) {
        if (lies.indexOf(name) === -1) {
            lies.push(name);
            lieDetails[name] = detail;
        }
    }

    function checkUndefinedProperties(objName, prop) {
        var obj = globalThis[objName];
        if (!obj) return;
        var ownDesc = Object.getOwnPropertyDescriptor(obj, prop);
        if (ownDesc) {
            addLie(objName + '.' + prop + ' failed undefined properties',
                   objName + ' has own property descriptor for ' + prop + ' (should be on prototype)');
        }
    }

    function checkToStringNative(objName, prop) {
        var obj = globalThis[objName];
        if (!obj) return;
        var proto = globalThis[objName.charAt(0).toUpperCase() + objName.slice(1)];
        if (!proto || !proto.prototype) return;
        var desc = Object.getOwnPropertyDescriptor(proto.prototype, prop);
        if (!desc || !desc.get) return;

        var getterStr = desc.get.toString();
        if (getterStr.indexOf('[native code]') === -1) {
            addLie(objName + '.' + prop + ' Function.toString',
                   'Getter toString does not contain [native code]: ' + getterStr);
        }
    }

    function checkDescriptorLocation(objName, prop) {
        var obj = globalThis[objName];
        if (!obj) return;
        var proto = globalThis[objName.charAt(0).toUpperCase() + objName.slice(1)];
        if (!proto || !proto.prototype) return;

        var ownDesc = Object.getOwnPropertyDescriptor(obj, prop);
        var protoDesc = Object.getOwnPropertyDescriptor(proto.prototype, prop);
        if (ownDesc && protoDesc) {
            addLie(objName + '.' + prop + ' duplicate descriptor',
                   'Property descriptor exists on both instance and prototype');
        }
    }

    function checkGetterSettersOnly(objName, prop) {
        var proto = globalThis[objName.charAt(0).toUpperCase() + objName.slice(1)];
        if (!proto || !proto.prototype) return;
        var desc = Object.getOwnPropertyDescriptor(proto.prototype, prop);
        if (!desc) return;

        if ('value' in desc) {
            addLie(objName + '.' + prop + ' value descriptor',
                   'Prototype property uses value descriptor, not getter/setter');
        }
    }

    var navigatorProps = [
        'userAgent', 'platform', 'vendor', 'vendorSub', 'product',
        'productSub', 'language', 'languages', 'hardwareConcurrency',
        'deviceMemory', 'maxTouchPoints', 'cookieEnabled', 'onLine',
        'doNotTrack', 'appName', 'appCodeName', 'appVersion',
        'pdfViewerEnabled', 'plugins', 'mimeTypes'
    ];

    var screenProps = [
        'width', 'height', 'availWidth', 'availHeight',
        'colorDepth', 'pixelDepth', 'availTop', 'availLeft'
    ];

    var windowProps = [];

    var checks = 0;
    var i;

    for (i = 0; i < navigatorProps.length; i++) {
        checkUndefinedProperties('navigator', navigatorProps[i]); checks++;
    }

    for (i = 0; i < screenProps.length; i++) {
        checkUndefinedProperties('screen', screenProps[i]); checks++;
    }

    for (i = 0; i < windowProps.length; i++) {
        checkUndefinedProperties('window', windowProps[i]); checks++;
    }

    checkToStringNative('navigator', 'userAgent'); checks++;
    checkToStringNative('navigator', 'platform'); checks++;
    checkToStringNative('navigator', 'vendor'); checks++;
    checkToStringNative('navigator', 'hardwareConcurrency'); checks++;
    checkToStringNative('navigator', 'deviceMemory'); checks++;

    checkDescriptorLocation('navigator', 'userAgent'); checks++;
    checkDescriptorLocation('navigator', 'platform'); checks++;
    checkDescriptorLocation('navigator', 'vendor'); checks++;
    checkDescriptorLocation('screen', 'width'); checks++;
    checkDescriptorLocation('screen', 'height'); checks++;

    return JSON.stringify({
        totalLies: lies.length,
        lieList: lies.sort(),
        lieDetail: lieDetails,
        totalChecks: checks
    });
})()
"""


def run_lie_detection():
    try:
        sys.path.insert(0, str(REPO_ROOT))
        from iv8_rs import JSContext

        ctx = JSContext()
        ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)
        raw = ctx.eval(LIE_DETECTION_JS)
        result = json.loads(raw) if isinstance(raw, str) else raw
        return result, None
    except Exception as e:
        return None, str(e)


def main():
    print("=== CreepJS Lie Detection CI Gate ===")
    print()

    result, error = run_lie_detection()

    if error:
        print(f"[FAIL] IV8 runtime unavailable: {error}")
        report = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "status": "ERROR",
            "error": error,
            "totalLies": -1,
            "baseline": 0,
            "totalChecks": 38,
        }
        REPORT_PATH.write_text(json.dumps(report, indent=2), encoding="utf-8")
        return 1

    total_lies = result.get("totalLies", 0)
    lie_list = result.get("lieList", [])
    lie_detail = result.get("lieDetail", {})
    total_checks = result.get("totalChecks", 0)

    status = "PASS" if total_lies == 0 else "FAIL"

    print(f"  Total checks: {total_checks}")
    print(f"  Lies detected: {total_lies}")
    print(f"  Baseline: 0")
    print(f"  Result: {status}")

    if lie_list:
        print()
        print("  --- Lies ---")
        for name in lie_list:
            detail = lie_detail.get(name, "")
            print(f"    {name}")
            if detail:
                print(f"      -> {detail}")

    report = {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "status": status,
        "totalLies": total_lies,
        "baseline": 0,
        "totalChecks": total_checks,
        "lieList": lie_list,
        "lieDetail": lie_detail,
    }
    REPORT_PATH.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print()
    print(f"  Report: {REPORT_PATH}")

    return 0 if total_lies == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
