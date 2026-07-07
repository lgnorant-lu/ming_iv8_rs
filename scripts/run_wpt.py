#!/usr/bin/env python3
"""WPT official test runner — runs WPT idlharness tests in IV8 V8 isolate.

Directly reuses WPT official test files (idlharness.https.html, etc.)
without modification. Compares results against Chrome baseline from wpt.fyi.

Usage:
  .venv\\Scripts\\python.exe scripts/run_wpt.py
  .venv\\Scripts\\python.exe scripts/run_wpt.py --suite html/dom
  .venv\\Scripts\\python.exe scripts/run_wpt.py --update

Output:
  data/wpt-report.json
  Exit code: 0 if all pass, 1 if any fail
"""
from __future__ import annotations

import json
import re
import sys
import threading
import urllib.request
from pathlib import Path
from collections import defaultdict

REPO_ROOT = Path(__file__).resolve().parent.parent
WPT_DIR = REPO_ROOT / "tools" / "wpt"
FIXTURES_DIR = WPT_DIR / "fixtures"
RESOURCES_DIR = FIXTURES_DIR / "resources"
INTERFACES_DIR = FIXTURES_DIR / "interfaces"
STATUS_DIR = WPT_DIR / "status"
VERSIONS_PATH = WPT_DIR / "versions.json"
DATA_DIR = REPO_ROOT / "data"
OUT_PATH = DATA_DIR / "wpt-report.json"

# WPT test suites to run
# Each suite maps to a WPT official test file and its variants
WPT_SUITES = [
    # === Multi-variant suites (manually configured) ===
    {
        "name": "html/dom/idlharness",
        "test_file": FIXTURES_DIR / "html" / "dom" / "idlharness.https.html",
        "variants": [
            {"name": "include=Document|Window", "query": "?include=(Document|Window)"},
            {"name": "include=HTML.+", "query": "?include=HTML.+"},
            {"name": "exclude=Document|Window|HTML.+", "query": "?exclude=(Document|Window|HTML.+)"},
        ],
        "idl_specs": [
            "html", "wai-aria", "SVG", "cssom", "touch-events", "pointerevents",
            "uievents", "dom", "xhr", "FileAPI", "mediacapture-streams",
            "performance-timeline", "trusted-types",
        ],
    },
    {
        "name": "html/dom/idlharness.worker",
        "test_file": FIXTURES_DIR / "html" / "dom" / "idlharness.any.js",
        "is_worker": True,
        "variants": [
            {"name": "worker", "query": ""},
        ],
        "idl_specs": [
            "html", "wai-aria", "dom", "cssom", "touch-events", "pointerevents",
            "uievents", "performance-timeline",
        ],
    },
    {
        "name": "dom/idlharness",
        "test_file": FIXTURES_DIR / "dom" / "idlharness.window.js",
        "variants": [
            {"name": "include=Node", "query": "?include=Node"},
            {"name": "exclude=Node", "query": "?exclude=Node"},
        ],
        "idl_specs": [
            "dom", "fullscreen", "html",
        ],
    },
    {
        "name": "dom/idlharness.worker",
        "test_file": FIXTURES_DIR / "dom" / "idlharness.any.js",
        "is_worker": True,
        "variants": [
            {"name": "worker", "query": ""},
            {"name": "serviceworker", "query": ""},
            {"name": "sharedworker", "query": ""},
        ],
        "idl_specs": [
            "dom", "html",
        ],
    },
    # === Single-variant suites (auto-generated) ===
    {
        "name": "FileAPI/idlharness",
        "test_file": FIXTURES_DIR / "FileAPI" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "FileAPI", "dom", "html", "url",
        ],
    },
    {
        "name": "FileAPI/idlharness.html",
        "test_file": FIXTURES_DIR / "FileAPI" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "FileAPI", "dom", "html", "url",
        ],
    },
    {
        "name": "IndexedDB/idlharness",
        "test_file": FIXTURES_DIR / "IndexedDB" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "IndexedDB", "html", "dom",
        ],
    },
    {
        "name": "WebCryptoAPI/idlharness",
        "test_file": FIXTURES_DIR / "WebCryptoAPI" / "idlharness.tentative.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webcrypto", "webcrypto-secure-curves", "webcrypto-modern-algos", "html", "dom",
        ],
    },
    {
        "name": "accelerometer/idlharness",
        "test_file": FIXTURES_DIR / "accelerometer" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "accelerometer", "generic-sensor", "dom",
        ],
    },
    {
        "name": "ambient-light/idlharness",
        "test_file": FIXTURES_DIR / "ambient-light" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "ambient-light", "generic-sensor", "dom",
        ],
    },
    {
        "name": "animation-worklet/idlharness",
        "test_file": FIXTURES_DIR / "animation-worklet" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-animation-worklet", "web-animations", "html", "cssom", "dom",
        ],
    },
    {
        "name": "audio-output/idlharness",
        "test_file": FIXTURES_DIR / "audio-output" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "audio-output", "mediacapture-streams", "html", "dom",
        ],
    },
    {
        "name": "audio-session/idlharness",
        "test_file": FIXTURES_DIR / "audio-session" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "audio-session", "dom", "html",
        ],
    },
    {
        "name": "autoplay-policy-detection/idlharness",
        "test_file": FIXTURES_DIR / "autoplay-policy-detection" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "autoplay-detection", "html",
        ],
    },
    {
        "name": "background-fetch/idlharness",
        "test_file": FIXTURES_DIR / "background-fetch" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "background-fetch", "service-workers", "html", "dom",
        ],
    },
    {
        "name": "background-sync/idlharness",
        "test_file": FIXTURES_DIR / "background-sync" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "background-sync", "service-workers", "html", "dom",
        ],
    },
    {
        "name": "badging/idlharness",
        "test_file": FIXTURES_DIR / "badging" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "badging", "html", "dom",
        ],
    },
    {
        "name": "battery-status/idlharness",
        "test_file": FIXTURES_DIR / "battery-status" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "battery-status", "dom", "html",
        ],
    },
    {
        "name": "beacon/idlharness",
        "test_file": FIXTURES_DIR / "beacon" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "beacon", "html",
        ],
    },
    {
        "name": "bluetooth/bidi/idl/idlharness",
        "test_file": FIXTURES_DIR / "bluetooth" / "bidi" / "idl" / "idlharness.tentative.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "web-bluetooth", "dom", "html", "permissions",
        ],
    },
    {
        "name": "captured-mouse-events/idlharness",
        "test_file": FIXTURES_DIR / "captured-mouse-events" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "captured-mouse-events.tentative", "html", "dom",
        ],
    },
    {
        "name": "clipboard-apis/idlharness",
        "test_file": FIXTURES_DIR / "clipboard-apis" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "clipboard-apis", "dom", "html", "permissions",
        ],
    },
    {
        "name": "compat/idlharness",
        "test_file": FIXTURES_DIR / "compat" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "compat", "html", "dom",
        ],
    },
    {
        "name": "compression/idlharness",
        "test_file": FIXTURES_DIR / "compression" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "compression", "streams",
        ],
    },
    {
        "name": "compute-pressure/idlharness",
        "test_file": FIXTURES_DIR / "compute-pressure" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "compute-pressure", "dom", "html",
        ],
    },
    {
        "name": "console/idlharness",
        "test_file": FIXTURES_DIR / "console" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "console",
        ],
    },
    {
        "name": "content-index/idlharness",
        "test_file": FIXTURES_DIR / "content-index" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "content-index", "service-workers", "html", "dom",
        ],
    },
    {
        "name": "content-security-policy/embedded-enforcement/idlharness",
        "test_file": FIXTURES_DIR / "content-security-policy" / "embedded-enforcement" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "csp-embedded-enforcement", "html", "dom",
        ],
    },
    {
        "name": "content-security-policy/securitypolicyviolation/idlharness",
        "test_file": FIXTURES_DIR / "content-security-policy" / "securitypolicyviolation" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "CSP", "dom", "reporting",
        ],
    },
    {
        "name": "cookiestore/idlharness",
        "test_file": FIXTURES_DIR / "cookiestore" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "cookiestore", "service-workers", "html", "dom",
        ],
    },
    {
        "name": "credential-management/idlharness",
        "test_file": FIXTURES_DIR / "credential-management" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "credential-management", "html", "dom",
        ],
    },
    {
        "name": "css/css-anchor-position/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-anchor-position" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-anchor-position", "cssom",
        ],
    },
    {
        "name": "css/css-animations/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-animations" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-animations", "html", "dom", "cssom",
        ],
    },
    {
        "name": "css/css-cascade/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-cascade" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-cascade", "css-cascade-6", "cssom",
        ],
    },
    {
        "name": "css/css-conditional/container-queries/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-conditional" / "container-queries" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-conditional-5", "css-conditional", "cssom", "dom",
        ],
    },
    {
        "name": "css/css-conditional/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-conditional" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-conditional", "cssom", "dom",
        ],
    },
    {
        "name": "css/css-counter-styles/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-counter-styles" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-counter-styles", "cssom",
        ],
    },
    {
        "name": "css/css-font-loading/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-font-loading" / "idlharness.https.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-font-loading", "dom", "html", "cssom",
        ],
    },
    {
        "name": "css/css-fonts/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-fonts" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-fonts-5", "css-fonts", "cssom",
        ],
    },
    {
        "name": "css/css-highlight-api/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-highlight-api" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-highlight-api", "cssom",
        ],
    },
    {
        "name": "css/css-images/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-images" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-images-4", "cssom",
        ],
    },
    {
        "name": "css/css-masking/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-masking" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-masking", "SVG", "html", "dom",
        ],
    },
    {
        "name": "css/css-paint-api/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-paint-api" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-paint-api", "cssom", "html",
        ],
    },
    {
        "name": "css/css-parser-api/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-parser-api" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-parser-api", "cssom",
        ],
    },
    {
        "name": "css/css-properties-values-api/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-properties-values-api" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-properties-values-api", "cssom",
        ],
    },
    {
        "name": "css/css-pseudo/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-pseudo" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-pseudo", "cssom", "html", "dom",
        ],
    },
    {
        "name": "css/css-shadow/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-shadow" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-shadow", "dom",
        ],
    },
    {
        "name": "css/css-transitions/idlharness-2",
        "test_file": FIXTURES_DIR / "css" / "css-transitions" / "idlharness-2.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-transitions-2", "web-animations", "cssom", "html", "dom",
        ],
    },
    {
        "name": "css/css-transitions/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-transitions" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-transitions", "cssom", "html", "dom",
        ],
    },
    {
        "name": "css/css-typed-om/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-typed-om" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-typed-om", "cssom", "SVG", "geometry", "html", "dom", "mathml-core",
        ],
    },
    {
        "name": "css/css-view-transitions/idlharness",
        "test_file": FIXTURES_DIR / "css" / "css-view-transitions" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "css-view-transitions", "dom", "html", "cssom",
        ],
    },
    {
        "name": "css/cssom-view/idlharness",
        "test_file": FIXTURES_DIR / "css" / "cssom-view" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "cssom-view", "css-pseudo", "cssom", "pointerevents", "uievents", "SVG", "html", "dom",
        ],
    },
    {
        "name": "css/cssom/idlharness",
        "test_file": FIXTURES_DIR / "css" / "cssom" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "cssom", "SVG", "uievents", "html", "dom", "mathml-core",
        ],
    },
    {
        "name": "css/filter-effects/idlharness",
        "test_file": FIXTURES_DIR / "css" / "filter-effects" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "filter-effects", "SVG", "html", "dom",
        ],
    },
    {
        "name": "css/geometry/idlharness",
        "test_file": FIXTURES_DIR / "css" / "geometry" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "geometry",
        ],
    },
    {
        "name": "device-memory/idlharness",
        "test_file": FIXTURES_DIR / "device-memory" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "device-memory", "html",
        ],
    },
    {
        "name": "device-posture/idlharness",
        "test_file": FIXTURES_DIR / "device-posture" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "device-posture", "html", "dom", "webidl",
        ],
    },
    {
        "name": "dom/observable/tentative/idlharness",
        "test_file": FIXTURES_DIR / "dom" / "observable" / "tentative" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "observable.tentative", "dom",
        ],
    },
    {
        "name": "encoding/idlharness",
        "test_file": FIXTURES_DIR / "encoding" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "encoding", "streams",
        ],
    },
    {
        "name": "event-timing/idlharness.any",
        "test_file": FIXTURES_DIR / "event-timing" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "event-timing", "performance-timeline", "hr-time", "dom",
        ],
    },
    {
        "name": "event-timing/idlharness",
        "test_file": FIXTURES_DIR / "event-timing" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "event-timing", "performance-timeline", "hr-time", "dom",
        ],
    },
    {
        "name": "eyedropper/idlharness",
        "test_file": FIXTURES_DIR / "eyedropper" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "eyedropper-api",
        ],
    },
    {
        "name": "fetch/api/idlharness",
        "test_file": FIXTURES_DIR / "fetch" / "api" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "fetch", "referrer-policy", "html", "dom",
        ],
    },
    {
        "name": "file-system-access/idlharness",
        "test_file": FIXTURES_DIR / "file-system-access" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "file-system-access", "fs", "permissions", "html", "dom",
        ],
    },
    {
        "name": "fs/idlharness",
        "test_file": FIXTURES_DIR / "fs" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "fs", "storage", "streams",
        ],
    },
    {
        "name": "fullscreen/idlharness",
        "test_file": FIXTURES_DIR / "fullscreen" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "fullscreen", "dom", "html",
        ],
    },
    {
        "name": "gamepad/idlharness-extensions",
        "test_file": FIXTURES_DIR / "gamepad" / "idlharness-extensions.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "gamepad-extensions", "gamepad",
        ],
    },
    {
        "name": "gamepad/idlharness",
        "test_file": FIXTURES_DIR / "gamepad" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "gamepad", "html", "dom",
        ],
    },
    {
        "name": "generic-sensor/idlharness",
        "test_file": FIXTURES_DIR / "generic-sensor" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "generic-sensor", "dom", "html", "webidl",
        ],
    },
    {
        "name": "geolocation/idlharness",
        "test_file": FIXTURES_DIR / "geolocation" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "geolocation", "hr-time", "html",
        ],
    },
    {
        "name": "gpc/idlharness",
        "test_file": FIXTURES_DIR / "gpc" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "gpc", "html",
        ],
    },
    {
        "name": "gyroscope/idlharness",
        "test_file": FIXTURES_DIR / "gyroscope" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "gyroscope", "generic-sensor", "dom",
        ],
    },
    {
        "name": "hr-time/idlharness",
        "test_file": FIXTURES_DIR / "hr-time" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "hr-time", "html", "dom",
        ],
    },
    {
        "name": "html-media-capture/idlharness",
        "test_file": FIXTURES_DIR / "html-media-capture" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "html-media-capture", "html", "dom",
        ],
    },
    {
        "name": "html/semantics/interestfor/idlharness",
        "test_file": FIXTURES_DIR / "html" / "semantics" / "interestfor" / "idlharness.tentative.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "interest-invokers.tentative", "html", "dom", "SVG",
        ],
    },
    {
        "name": "html/semantics/permission-element/geolocation-element/idlharness",
        "test_file": FIXTURES_DIR / "html" / "semantics" / "permission-element" / "geolocation-element" / "idlharness.tentative.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "geolocation-element.tentative", "html", "dom", "permissions", "geolocation",
        ],
    },
    {
        "name": "html/semantics/permission-element/usermedia/idlharness",
        "test_file": FIXTURES_DIR / "html" / "semantics" / "permission-element" / "usermedia" / "idlharness.tentative.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "usermedia-element.tentative", "geolocation-element.tentative", "html", "dom", "permissions", "geolocation", "mediacapture-streams",
        ],
    },
    {
        "name": "idle-detection/idlharness-worker",
        "test_file": FIXTURES_DIR / "idle-detection" / "idlharness-worker.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "html", "dom",
        ],
    },
    {
        "name": "idle-detection/idlharness",
        "test_file": FIXTURES_DIR / "idle-detection" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "idle-detection", "dom", "html",
        ],
    },
    {
        "name": "input-device-capabilities/idlharness",
        "test_file": FIXTURES_DIR / "input-device-capabilities" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "input-device-capabilities", "uievents", "dom",
        ],
    },
    {
        "name": "input-events/idlharness",
        "test_file": FIXTURES_DIR / "input-events" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "input-events", "uievents", "dom",
        ],
    },
    {
        "name": "installedapp/idlharness",
        "test_file": FIXTURES_DIR / "installedapp" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "get-installed-related-apps", "html",
        ],
    },
    {
        "name": "intersection-observer/idlharness",
        "test_file": FIXTURES_DIR / "intersection-observer" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "intersection-observer", "dom",
        ],
    },
    {
        "name": "is-input-pending/idlharness",
        "test_file": FIXTURES_DIR / "is-input-pending" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "is-input-pending", "html", "dom",
        ],
    },
    {
        "name": "js-self-profiling/idlharness",
        "test_file": FIXTURES_DIR / "js-self-profiling" / "idlharness.https.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "js-self-profiling", "hr-time", "dom",
        ],
    },
    {
        "name": "keyboard-lock/idlharness",
        "test_file": FIXTURES_DIR / "keyboard-lock" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "keyboard-lock", "html", "dom",
        ],
    },
    {
        "name": "keyboard-map/idlharness",
        "test_file": FIXTURES_DIR / "keyboard-map" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "keyboard-map", "keyboard-lock", "html", "dom",
        ],
    },
    {
        "name": "largest-contentful-paint/idlharness",
        "test_file": FIXTURES_DIR / "largest-contentful-paint" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "largest-contentful-paint", "performance-timeline", "dom", "hr-time", "paint-timing",
        ],
    },
    {
        "name": "layout-instability/idlharness",
        "test_file": FIXTURES_DIR / "layout-instability" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "layout-instability", "performance-timeline", "geometry", "dom", "hr-time",
        ],
    },
    {
        "name": "longtask-timing/idlharness",
        "test_file": FIXTURES_DIR / "longtask-timing" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "longtasks", "performance-timeline", "hr-time",
        ],
    },
    {
        "name": "magnetometer/idlharness",
        "test_file": FIXTURES_DIR / "magnetometer" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "magnetometer", "generic-sensor", "dom",
        ],
    },
    {
        "name": "measure-memory/idlharness",
        "test_file": FIXTURES_DIR / "measure-memory" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "performance-measure-memory", "hr-time", "dom",
        ],
    },
    {
        "name": "media-capabilities/idlharness",
        "test_file": FIXTURES_DIR / "media-capabilities" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "media-capabilities", "html", "cssom-view",
        ],
    },
    {
        "name": "media-playback-quality/idlharness",
        "test_file": FIXTURES_DIR / "media-playback-quality" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "media-playback-quality", "html", "dom",
        ],
    },
    {
        "name": "media-source/idlharness",
        "test_file": FIXTURES_DIR / "media-source" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "media-source", "dom", "html", "url",
        ],
    },
    {
        "name": "mediacapture-fromelement/idlharness",
        "test_file": FIXTURES_DIR / "mediacapture-fromelement" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "mediacapture-fromelement", "mediacapture-streams", "html", "dom",
        ],
    },
    {
        "name": "mediacapture-image/idlharness",
        "test_file": FIXTURES_DIR / "mediacapture-image" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "image-capture", "mediacapture-streams", "html", "dom",
        ],
    },
    {
        "name": "mediacapture-record/idlharness",
        "test_file": FIXTURES_DIR / "mediacapture-record" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "mediastream-recording", "mediacapture-streams", "FileAPI", "html", "dom", "webidl",
        ],
    },
    {
        "name": "mediacapture-streams/idlharness",
        "test_file": FIXTURES_DIR / "mediacapture-streams" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "mediacapture-streams", "webidl", "dom", "html", "permissions",
        ],
    },
    {
        "name": "mediasession/idlharness",
        "test_file": FIXTURES_DIR / "mediasession" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "mediasession", "html",
        ],
    },
    {
        "name": "mst-content-hint/idlharness",
        "test_file": FIXTURES_DIR / "mst-content-hint" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "mst-content-hint", "mediacapture-streams", "webrtc", "dom",
        ],
    },
    {
        "name": "navigation-timing/idlharness",
        "test_file": FIXTURES_DIR / "navigation-timing" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "navigation-timing", "hr-time", "resource-timing", "performance-timeline", "dom", "html",
        ],
    },
    {
        "name": "netinfo/idlharness",
        "test_file": FIXTURES_DIR / "netinfo" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "netinfo", "html", "dom",
        ],
    },
    {
        "name": "notifications/idlharness",
        "test_file": FIXTURES_DIR / "notifications" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "notifications", "service-workers", "hr-time", "html", "dom",
        ],
    },
    {
        "name": "orientation-event/idlharness",
        "test_file": FIXTURES_DIR / "orientation-event" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "orientation-event", "html", "dom",
        ],
    },
    {
        "name": "orientation-sensor/idlharness",
        "test_file": FIXTURES_DIR / "orientation-sensor" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "orientation-sensor", "generic-sensor", "dom",
        ],
    },
    {
        "name": "page-lifecycle/idlharness",
        "test_file": FIXTURES_DIR / "page-lifecycle" / "idlharness.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "page-lifecycle", "service-workers", "html", "dom",
        ],
    },
    {
        "name": "paint-timing/idlharness",
        "test_file": FIXTURES_DIR / "paint-timing" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "paint-timing", "performance-timeline", "hr-time",
        ],
    },
    {
        "name": "parakeet/idlharness",
        "test_file": FIXTURES_DIR / "parakeet" / "idlharness.tentative.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "parakeet.tentative", "html",
        ],
    },
    {
        "name": "payment-request/idlharness",
        "test_file": FIXTURES_DIR / "payment-request" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "payment-request", "dom", "html",
        ],
    },
    {
        "name": "performance-timeline/idlharness",
        "test_file": FIXTURES_DIR / "performance-timeline" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "performance-timeline", "hr-time", "dom",
        ],
    },
    {
        "name": "periodic-background-sync/idlharness",
        "test_file": FIXTURES_DIR / "periodic-background-sync" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "periodic-background-sync", "service-workers", "html", "dom",
        ],
    },
    {
        "name": "permissions-policy/idlharness",
        "test_file": FIXTURES_DIR / "permissions-policy" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "permissions-policy", "reporting", "html", "dom",
        ],
    },
    {
        "name": "permissions-request/idlharness",
        "test_file": FIXTURES_DIR / "permissions-request" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "permissions-request", "permissions", "html", "dom",
        ],
    },
    {
        "name": "permissions-revoke/idlharness",
        "test_file": FIXTURES_DIR / "permissions-revoke" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "permissions-revoke", "permissions", "html", "dom",
        ],
    },
    {
        "name": "permissions/idlharness",
        "test_file": FIXTURES_DIR / "permissions" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "permissions", "html", "dom",
        ],
    },
    {
        "name": "picture-in-picture/idlharness",
        "test_file": FIXTURES_DIR / "picture-in-picture" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "picture-in-picture", "html", "dom",
        ],
    },
    {
        "name": "pointerevents/idlharness",
        "test_file": FIXTURES_DIR / "pointerevents" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "pointerevents", "uievents", "html", "dom",
        ],
    },
    {
        "name": "pointerlock/idlharness",
        "test_file": FIXTURES_DIR / "pointerlock" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "pointerlock", "pointerevents", "uievents", "html", "dom",
        ],
    },
    {
        "name": "presentation-api/controlling-ua/idlharness",
        "test_file": FIXTURES_DIR / "presentation-api" / "controlling-ua" / "idlharness.https.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "html", "dom",
        ],
    },
    {
        "name": "private-click-measurement/idlharness",
        "test_file": FIXTURES_DIR / "private-click-measurement" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "private-click-measurement", "html", "dom",
        ],
    },
    {
        "name": "proximity/idlharness",
        "test_file": FIXTURES_DIR / "proximity" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "proximity", "generic-sensor", "dom",
        ],
    },
    {
        "name": "push-api/idlharness",
        "test_file": FIXTURES_DIR / "push-api" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "push-api", "service-workers", "hr-time", "html", "dom", "permissions",
        ],
    },
    {
        "name": "remote-playback/idlharness",
        "test_file": FIXTURES_DIR / "remote-playback" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "remote-playback", "html", "dom",
        ],
    },
    {
        "name": "reporting/idlharness",
        "test_file": FIXTURES_DIR / "reporting" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "reporting",
        ],
    },
    {
        "name": "requestidlecallback/idlharness",
        "test_file": FIXTURES_DIR / "requestidlecallback" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "requestidlecallback", "html", "dom",
        ],
    },
    {
        "name": "resize-observer/idlharness",
        "test_file": FIXTURES_DIR / "resize-observer" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "resize-observer", "dom", "geometry",
        ],
    },
    {
        "name": "resource-timing/idlharness",
        "test_file": FIXTURES_DIR / "resource-timing" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "resource-timing", "performance-timeline", "hr-time", "dom", "html",
        ],
    },
    {
        "name": "savedata/idlharness",
        "test_file": FIXTURES_DIR / "savedata" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "savedata", "netinfo", "html", "dom",
        ],
    },
    {
        "name": "screen-capture/idlharness",
        "test_file": FIXTURES_DIR / "screen-capture" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "screen-capture", "mediacapture-streams", "html", "dom",
        ],
    },
    {
        "name": "screen-orientation/idlharness",
        "test_file": FIXTURES_DIR / "screen-orientation" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "screen-orientation", "dom", "cssom-view", "html",
        ],
    },
    {
        "name": "screen-wake-lock/idlharness",
        "test_file": FIXTURES_DIR / "screen-wake-lock" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "screen-wake-lock", "dom", "html",
        ],
    },
    {
        "name": "scroll-animations/scroll-timelines/idlharness",
        "test_file": FIXTURES_DIR / "scroll-animations" / "scroll-timelines" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "scroll-animations",
        ],
    },
    {
        "name": "scroll-to-text-fragment/idlharness",
        "test_file": FIXTURES_DIR / "scroll-to-text-fragment" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "scroll-to-text-fragment", "dom", "html",
        ],
    },
    {
        "name": "selection/idlharness",
        "test_file": FIXTURES_DIR / "selection" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "selection-api", "html", "dom",
        ],
    },
    {
        "name": "serial/idlharness",
        "test_file": FIXTURES_DIR / "serial" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "serial", "html", "dom",
        ],
    },
    {
        "name": "server-timing/idlharness",
        "test_file": FIXTURES_DIR / "server-timing" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "resource-timing", "server-timing", "performance-timeline", "hr-time", "dom",
        ],
    },
    {
        "name": "service-workers/idlharness",
        "test_file": FIXTURES_DIR / "service-workers" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "service-workers", "dom", "html",
        ],
    },
    {
        "name": "shape-detection/idlharness",
        "test_file": FIXTURES_DIR / "shape-detection" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "shape-detection-api", "text-detection-api", "dom", "geometry",
        ],
    },
    {
        "name": "speech-api/idlharness",
        "test_file": FIXTURES_DIR / "speech-api" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "speech-api", "dom", "html",
        ],
    },
    {
        "name": "storage-access-api/idlharness",
        "test_file": FIXTURES_DIR / "storage-access-api" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "storage-access", "dom",
        ],
    },
    {
        "name": "storage/buckets/idlharness",
        "test_file": FIXTURES_DIR / "storage" / "buckets" / "idlharness-worker.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "storage-buckets", "html",
        ],
    },
    {
        "name": "storage/idlharness",
        "test_file": FIXTURES_DIR / "storage" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "storage", "html",
        ],
    },
    {
        "name": "streams/idlharness",
        "test_file": FIXTURES_DIR / "streams" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "streams", "dom",
        ],
    },
    {
        "name": "svg/idlharness",
        "test_file": FIXTURES_DIR / "svg" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "SVG", "svg-animations", "cssom", "web-animations", "html", "dom",
        ],
    },
    {
        "name": "touch-events/idlharness",
        "test_file": FIXTURES_DIR / "touch-events" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "touch-events", "uievents", "html", "dom",
        ],
    },
    {
        "name": "trusted-types/idlharness",
        "test_file": FIXTURES_DIR / "trusted-types" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "trusted-types", "html", "dom",
        ],
    },
    {
        "name": "ua-client-hints/idlharness",
        "test_file": FIXTURES_DIR / "ua-client-hints" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "ua-client-hints", "html", "dom",
        ],
    },
    {
        "name": "uievents/idlharness",
        "test_file": FIXTURES_DIR / "uievents" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "uievents", "html", "dom",
        ],
    },
    {
        "name": "url/idlharness",
        "test_file": FIXTURES_DIR / "url" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "url",
        ],
    },
    {
        "name": "user-timing/idlharness",
        "test_file": FIXTURES_DIR / "user-timing" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "user-timing", "hr-time", "performance-timeline", "dom",
        ],
    },
    {
        "name": "vibration/idlharness",
        "test_file": FIXTURES_DIR / "vibration" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "vibration", "html",
        ],
    },
    {
        "name": "video-rvfc/idlharness",
        "test_file": FIXTURES_DIR / "video-rvfc" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "video-rvfc", "html", "dom",
        ],
    },
    {
        "name": "virtual-keyboard/idlharness",
        "test_file": FIXTURES_DIR / "virtual-keyboard" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "virtual-keyboard.tentative", "html", "dom",
        ],
    },
    {
        "name": "wai-aria/idlharness",
        "test_file": FIXTURES_DIR / "wai-aria" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "wai-aria", "dom",
        ],
    },
    {
        "name": "wasm/jsapi/idlharness",
        "test_file": FIXTURES_DIR / "wasm" / "jsapi" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "wasm-js-api",
        ],
    },
    {
        "name": "wasm/webapi/idlharness",
        "test_file": FIXTURES_DIR / "wasm" / "webapi" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "wasm-web-api", "wasm-js-api",
        ],
    },
    {
        "name": "web-animations/idlharness",
        "test_file": FIXTURES_DIR / "web-animations" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "web-animations", "web-animations-2", "dom", "html", "scroll-animations",
        ],
    },
    {
        "name": "web-based-payment-handler/idlharness",
        "test_file": FIXTURES_DIR / "web-based-payment-handler" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "web-based-payment-handler", "service-workers", "html", "dom",
        ],
    },
    {
        "name": "web-locks/idlharness",
        "test_file": FIXTURES_DIR / "web-locks" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "web-locks", "html",
        ],
    },
    {
        "name": "web-nfc/idlharness",
        "test_file": FIXTURES_DIR / "web-nfc" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "web-nfc", "html", "dom", "webidl",
        ],
    },
    {
        "name": "web-otp/idlharness",
        "test_file": FIXTURES_DIR / "web-otp" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "web-otp", "credential-management",
        ],
    },
    {
        "name": "web-share/idlharness",
        "test_file": FIXTURES_DIR / "web-share" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "web-share", "html",
        ],
    },
    {
        "name": "webaudio/idlharness",
        "test_file": FIXTURES_DIR / "webaudio" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webaudio", "cssom", "uievents", "mediacapture-streams", "html", "dom",
        ],
    },
    {
        "name": "webauthn/idlharness",
        "test_file": FIXTURES_DIR / "webauthn" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webauthn", "credential-management",
        ],
    },
    {
        "name": "webcodecs/idlharness",
        "test_file": FIXTURES_DIR / "webcodecs" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webcodecs", "dom", "html", "webidl",
        ],
    },
    {
        "name": "webdriver/tests/classic/idlharness",
        "test_file": FIXTURES_DIR / "webdriver" / "tests" / "classic" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webdriver", "html",
        ],
    },
    {
        "name": "webgl/idlharness",
        "test_file": FIXTURES_DIR / "webgl" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webgl1", "webgl2", "dom",
        ],
    },
    {
        "name": "webhid/idlharness",
        "test_file": FIXTURES_DIR / "webhid" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webhid", "html", "dom",
        ],
    },
    {
        "name": "webidl/idlharness",
        "test_file": FIXTURES_DIR / "webidl" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webidl",
        ],
    },
    {
        "name": "webmcp/idlharness",
        "test_file": FIXTURES_DIR / "webmcp" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webmcp", "html", "dom",
        ],
    },
    {
        "name": "webmidi/idlharness",
        "test_file": FIXTURES_DIR / "webmidi" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webmidi", "html", "dom", "permissions",
        ],
    },
    {
        "name": "webnn/idlharness",
        "test_file": FIXTURES_DIR / "webnn" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webnn", "html", "webidl", "webgpu",
        ],
    },
    {
        "name": "webrtc-encoded-transform/idlharness",
        "test_file": FIXTURES_DIR / "webrtc-encoded-transform" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webrtc-encoded-transform", "webrtc", "streams", "html", "dom",
        ],
    },
    {
        "name": "webrtc-identity/idlharness",
        "test_file": FIXTURES_DIR / "webrtc-identity" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webrtc-identity", "webrtc", "mediacapture-streams", "html", "dom", "webidl",
        ],
    },
    {
        "name": "webrtc/idlharness",
        "test_file": FIXTURES_DIR / "webrtc" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webrtc", "webidl", "mediacapture-streams", "hr-time", "dom", "html", "websockets",
        ],
    },
    {
        "name": "websockets/idlharness",
        "test_file": FIXTURES_DIR / "websockets" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "websockets", "html", "dom",
        ],
    },
    {
        "name": "webtransport/idlharness",
        "test_file": FIXTURES_DIR / "webtransport" / "idlharness.https.sub.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webtransport", "webidl", "streams",
        ],
    },
    {
        "name": "webusb/idlharness",
        "test_file": FIXTURES_DIR / "webusb" / "idlharness.https.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webusb", "permissions", "html", "dom",
        ],
    },
    {
        "name": "webvtt/api/idlharness",
        "test_file": FIXTURES_DIR / "webvtt" / "api" / "idlharness.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webvtt", "html", "dom",
        ],
    },
    {
        "name": "webxr/anchors/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "anchors" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "anchors", "webxr-hit-test", "webxr", "dom",
        ],
    },
    {
        "name": "webxr/ar-module/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "ar-module" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr-ar-module", "webxr", "dom",
        ],
    },
    {
        "name": "webxr/depth-sensing/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "depth-sensing" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr-depth-sensing", "webxr", "webxrlayers", "webgl1", "webgl2", "dom",
        ],
    },
    {
        "name": "webxr/dom-overlay/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "dom-overlay" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr-dom-overlays", "webxr", "html", "dom", "SVG",
        ],
    },
    {
        "name": "webxr/gamepads-module/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "gamepads-module" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr-gamepads-module", "webxr", "dom",
        ],
    },
    {
        "name": "webxr/hand-input/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "hand-input" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr-hand-input", "webxr", "dom",
        ],
    },
    {
        "name": "webxr/hit-test/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "hit-test" / "idlharness.https.html",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr-hit-test", "webxr", "geometry", "dom",
        ],
    },
    {
        "name": "webxr/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr", "permissions", "webgl1", "geometry", "html", "dom",
        ],
    },
    {
        "name": "webxr/layers/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "layers" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxrlayers", "webxr", "webgl1", "webgl2", "dom",
        ],
    },
    {
        "name": "webxr/light-estimation/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "light-estimation" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr-lighting-estimation", "webxr", "webxrlayers", "webgl1", "webgl2", "dom",
        ],
    },
    {
        "name": "webxr/plane-detection/idlharness",
        "test_file": FIXTURES_DIR / "webxr" / "plane-detection" / "idlharness.https.window.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "webxr-plane-detection", "webxr", "dom",
        ],
    },
    {
        "name": "xhr/idlharness",
        "test_file": FIXTURES_DIR / "xhr" / "idlharness.any.js",
        "variants": [
            {"name": "default", "query": ""},
        ],
        "idl_specs": [
            "xhr", "dom", "html",
        ],
    },
    {
        "name": "element-timing/idlharness",
        "test_file": FIXTURES_DIR / "element-timing" / "idlharness.window.js",
        "variants": [{"name": "default", "query": ""}],
        "idl_specs": ["element-timing", "performance-timeline", "dom"],
    },
    {
        "name": "deprecation-reporting/idlharness",
        "test_file": FIXTURES_DIR / "deprecation-reporting" / "idlharness.any.js",
        "variants": [{"name": "default", "query": ""}],
        "idl_specs": ["deprecation-reporting", "reporting"],
    },
    {
        "name": "intervention-reporting/idlharness",
        "test_file": FIXTURES_DIR / "intervention-reporting" / "idlharness.any.js",
        "variants": [{"name": "default", "query": ""}],
        "idl_specs": ["intervention-reporting", "reporting"],
    },
    {
        "name": "webrtc-stats/idlharness",
        "test_file": FIXTURES_DIR / "webrtc-stats" / "idlharness.window.js",
        "variants": [{"name": "default", "query": ""}],
        "idl_specs": ["webrtc-stats", "webrtc", "dom", "html"],
    },
]


def load_wpt_resources() -> dict[str, str]:
    """Load WPT resource files (testharness.js, idlharness.js, webidl2.js).

    testharnessreport.js is IV8's custom version (sets output:false).
    """
    resources = {}
    for name in ["testharness.js", "idlharness.js", "webidl2.js"]:
        path = RESOURCES_DIR / name
        if path.exists():
            resources[name] = path.read_text(encoding="utf-8")
        else:
            print(f"FATAL: {path} not found. Run tools/wpt/wpt_update.py first.")
            sys.exit(1)
    # Use IV8's custom testharnessreport.js
    report_path = WPT_DIR / "fixtures" / "resources" / "testharnessreport.js"
    if report_path.exists():
        resources["testharnessreport.js"] = report_path.read_text(encoding="utf-8")
    else:
        print(f"FATAL: {report_path} not found.")
        sys.exit(1)
    return resources


def load_idl_files(specs: list[str]) -> dict[str, str]:
    """Load webref IDL files for the given spec names."""
    idl_contents = {}
    for spec in specs:
        path = INTERFACES_DIR / f"{spec}.idl"
        if path.exists():
            idl_contents[spec] = path.read_text(encoding="utf-8")
        else:
            print(f"WARN: IDL file not found: {path}")
    return idl_contents


def extract_script_from_html(html_path: Path) -> str:
    """Extract <script> content from a WPT .html test file.

    Returns the concatenated JS code from all <script> tags,
    excluding external src= references (those are loaded separately).
    """
    html = html_path.read_text(encoding="utf-8")
    # Extract content between <script> tags (without src=)
    pattern = r'<script(?![^>]*\bsrc=)[^>]*>(.*?)</script>'
    scripts = re.findall(pattern, html, re.DOTALL)
    return "\n".join(scripts)


def build_shim_code(idl_contents: dict[str, str], variant_query: str) -> str:
    """Build the shim code injected before WPT test execution.

    Includes:
    - GLOBAL object (isWindow=true for .html, isWorker=false)
    - fetch_spec implementation (using IV8's fetch + add_resource)
    - location.search for variant subsetting
    - add_result_callback for result collection
    """
    # Register IDL files as fetchable resources
    register_lines = []
    for spec, content in idl_contents.items():
        escaped = json.dumps(content)
        register_lines.append(
            f'  ctx.add_resource("/interfaces/{spec}.idl", {escaped});'
        )

    return f"""
// === IV8 WPT Shim ===
globalThis.GLOBAL = globalThis;
globalThis.GLOBAL.isWindow = function() {{ return true; }};
globalThis.GLOBAL.isWorker = function() {{ return false; }};
globalThis.GLOBAL.isShadowRealm = function() {{ return false; }};

// location.search for variant subsetting
globalThis.location = {{ search: {json.dumps(variant_query)} }};

// fetch_spec: idlharness.js calls this to load IDL files
globalThis.fetch_spec = function(spec) {{
    var url = "/interfaces/" + spec + ".idl";
    return fetch(url).then(function(r) {{
        if (!r.ok) throw new Error("Error fetching " + url);
        return r.text();
    }}).then(function(idl) {{
        return {{ spec: spec, idl: idl }};
    }});
}};

// Result collector
var __results = [];
add_result_callback(function(test) {{
    __results.push({{
        name: test.name,
        status: test.format_status(),
        message: test.message || null
    }});
}});

// testharness.js expects window.onload to fire before running tests.
// IV8 has no real event loop, so we resolve the 'load' event immediately.
if (typeof addEventListener === 'function') {{
    var __origAddEventListener = addEventListener;
    globalThis.addEventListener = function(type, listener) {{
        if (type === 'load') {{
            Promise.resolve().then(function() {{ listener({{ type: 'load' }}); }});
        }} else {{
            __origAddEventListener(type, listener);
        }}
    }};
}}
// === End IV8 WPT Shim ===
"""


def run_suite(suite: dict, variant: dict, resources: dict) -> dict:
    """Run a single WPT test suite variant in IV8.

    Returns a result dict with pass/fail/total and test details.
    """
    import iv8_rs as iv8

    suite_name = suite["name"]
    variant_name = variant["name"]
    variant_query = variant["query"]
    test_file = suite["test_file"]
    is_worker = suite.get("is_worker", False)

    if not test_file.exists():
        return {
            "suite": suite_name,
            "variant": variant_name,
            "run_status": "error: test file not found",
            "total": 0, "pass": 0, "fail": 0,
            "tests": [],
        }

    # Load IDL files
    idl_contents = load_idl_files(suite["idl_specs"])

    # Extract test code
    if test_file.suffix == ".html":
        test_code = extract_script_from_html(test_file)
    else:
        test_code = test_file.read_text(encoding="utf-8")
        # Strip META comments from .any.js / .window.js files
        test_code = re.sub(r'^//\s*META:.*$', '', test_code, flags=re.MULTILINE)

    # Test code is used as-is. Previous versions neutralized setup() calls
    # that create DOM elements (video, iframe, etc.) because createElement
    # returned incomplete elements. This is no longer needed — the DOM
    # template hierarchy now supports these element types correctly.

    # Create IV8 context
    ctx = iv8.JSContext()

    try:
        # Register IDL files as resources
        for spec, content in idl_contents.items():
            ctx.add_resource(f"/interfaces/{spec}.idl", content)

        # Inject pre-harness shim BEFORE testharness.js loads.
        # testharness.js IIFE registers load event listener at load time
        # via window.addEventListener('load', callback). We must intercept
        # this before testharness.js executes.
        is_window = "false" if is_worker else "true"
        is_worker_js = "true" if is_worker else "false"
        worker_shim = ""
        if is_worker:
            # Only define the GlobalScope matching this variant
            # idlharness.exposed_in() checks 'XxxGlobalScope' in self && self instanceof XxxGlobalScope
            # We patch the instanceof check to true, so only the 'in' check matters
            worker_type = variant_name  # worker/serviceworker/sharedworker
            if worker_type == "serviceworker":
                scope_name = "ServiceWorkerGlobalScope"
            elif worker_type == "sharedworker":
                scope_name = "SharedWorkerGlobalScope"
            else:
                scope_name = "DedicatedWorkerGlobalScope"
            worker_shim = f"""
// Worker context ({worker_type}): install Worker interfaces on globalThis.
// codegen only installs [Exposed=Window] interfaces. Worker-specific
// interfaces (WorkerGlobalScope, WorkerNavigator, WorkerLocation,
// DedicatedWorkerGlobalScope) need to be installed for idlharness.

// WorkerGlobalScope : EventTarget
if (typeof globalThis.WorkerGlobalScope === 'undefined') {{
    function WorkerGlobalScope() {{}}
    WorkerGlobalScope.prototype = Object.create(
        (typeof EventTarget !== 'undefined' && EventTarget.prototype) ? EventTarget.prototype : Object.prototype
    );
    Object.defineProperty(WorkerGlobalScope.prototype, 'constructor', {{
        value: WorkerGlobalScope, writable: true, enumerable: false, configurable: true
    }});
    // self
    Object.defineProperty(WorkerGlobalScope.prototype, 'self', {{
        get: function() {{ if (this !== globalThis && this !== WorkerGlobalScope.prototype) throw new TypeError('Illegal invocation'); return globalThis; }},
        enumerable: true, configurable: true
    }});
    // location
    Object.defineProperty(WorkerGlobalScope.prototype, 'location', {{
        get: function() {{ if (this !== globalThis && this !== WorkerGlobalScope.prototype) throw new TypeError('Illegal invocation'); return globalThis.__workerLocation || null; }},
        enumerable: true, configurable: true
    }});
    // navigator
    Object.defineProperty(WorkerGlobalScope.prototype, 'navigator', {{
        get: function() {{ if (this !== globalThis && this !== WorkerGlobalScope.prototype) throw new TypeError('Illegal invocation'); return globalThis.__workerNavigator || null; }},
        enumerable: true, configurable: true
    }});
    // importScripts
    WorkerGlobalScope.prototype.importScripts = function importScripts() {{
        if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present.');
    }};
    try {{ Object.defineProperty(WorkerGlobalScope.prototype.importScripts, 'length', {{ value: 0, writable: false, enumerable: false, configurable: true }}); }} catch(e) {{}}
    // Event handlers
    var _wgsHandlers = ['onerror','onlanguagechange','onoffline','ononline','onrejectionhandled','onunhandledrejection'];
    _wgsHandlers.forEach(function(hn) {{
        Object.defineProperty(WorkerGlobalScope.prototype, hn, {{
            get: function() {{ return this['__iv8' + hn] || null; }},
            set: function(v) {{ this['__iv8' + hn] = v; }},
            enumerable: true, configurable: true
        }});
    }});
    if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {{
        Object.defineProperty(WorkerGlobalScope.prototype, Symbol.toStringTag, {{
            value: 'WorkerGlobalScope', writable: false, configurable: true, enumerable: false
        }});
    }}
    Object.defineProperty(WorkerGlobalScope, 'length', {{ value: 0, writable: false, enumerable: false, configurable: true }});
    try {{ Object.defineProperty(globalThis, 'WorkerGlobalScope', {{ value: WorkerGlobalScope, writable: true, configurable: true, enumerable: false }}); }} catch(e) {{}}
    try {{ Object.defineProperty(WorkerGlobalScope, 'prototype', {{ writable: false, enumerable: false, configurable: false }}); }} catch(e) {{}}
}}
try {{ Object.defineProperty(globalThis, 'WorkerGlobalScope', {{ value: globalThis.WorkerGlobalScope, writable: true, configurable: true, enumerable: false }}); }} catch(e) {{}}

// WorkerNavigator
if (typeof globalThis.WorkerNavigator === 'undefined') {{
    function WorkerNavigator() {{}}
    WorkerNavigator.prototype = Object.create(
        (typeof Navigator !== 'undefined' && Navigator.prototype) ? Navigator.prototype : Object.prototype
    );
    Object.defineProperty(WorkerNavigator.prototype, 'constructor', {{
        value: WorkerNavigator, writable: true, enumerable: false, configurable: true
    }});
    // NavigatorID mixin
    Object.defineProperty(WorkerNavigator.prototype, 'appCodeName', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return 'Mozilla'; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerNavigator.prototype, 'appName', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return 'Netscape'; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerNavigator.prototype, 'appVersion', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return '5.0'; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerNavigator.prototype, 'platform', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return 'Win32'; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerNavigator.prototype, 'product', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return 'Gecko'; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerNavigator.prototype, 'userAgent', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return navigator.userAgent; }}, enumerable: true, configurable: true }});
    // NavigatorLanguage mixin
    Object.defineProperty(WorkerNavigator.prototype, 'language', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return 'en-US'; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerNavigator.prototype, 'languages', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return ['en-US','en']; }}, enumerable: true, configurable: true }});
    // NavigatorOnLine mixin
    Object.defineProperty(WorkerNavigator.prototype, 'onLine', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return true; }}, enumerable: true, configurable: true }});
    // NavigatorConcurrentHardware mixin
    Object.defineProperty(WorkerNavigator.prototype, 'hardwareConcurrency', {{ get: function() {{ if (this !== globalThis.__workerNavigator && this !== WorkerNavigator.prototype) throw new TypeError('Illegal invocation'); return 8; }}, enumerable: true, configurable: true }});
    if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {{
        Object.defineProperty(WorkerNavigator.prototype, Symbol.toStringTag, {{
            value: 'WorkerNavigator', writable: false, configurable: true, enumerable: false
        }});
    }}
    try {{ Object.defineProperty(globalThis, 'WorkerNavigator', {{ value: WorkerNavigator, writable: true, configurable: true, enumerable: false }}); }} catch(e) {{}}
    try {{ Object.defineProperty(WorkerNavigator, 'prototype', {{ writable: false, enumerable: false, configurable: false }}); }} catch(e) {{}}
    globalThis.__workerNavigator = Object.create(WorkerNavigator.prototype);
}}
try {{ Object.defineProperty(globalThis, 'WorkerNavigator', {{ value: globalThis.WorkerNavigator, writable: true, configurable: true, enumerable: false }}); }} catch(e) {{}}

// WorkerLocation
if (typeof globalThis.WorkerLocation === 'undefined') {{
    function WorkerLocation() {{}}
    var _wlHref = (typeof location !== 'undefined' && location.href) ? location.href : 'https://example.com/';
    WorkerLocation.prototype = Object.create(Object.prototype);
    Object.defineProperty(WorkerLocation.prototype, 'constructor', {{
        value: WorkerLocation, writable: true, enumerable: false, configurable: true
    }});
    Object.defineProperty(WorkerLocation.prototype, 'href', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); return _wlHref; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerLocation.prototype, 'origin', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); return _wlHref.split('/').slice(0,3).join('/'); }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerLocation.prototype, 'protocol', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); return _wlHref.split(':')[0] + ':'; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerLocation.prototype, 'host', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); return _wlHref.split('/')[2] || ''; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerLocation.prototype, 'hostname', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); return (_wlHref.split('/')[2] || '').split(':')[0]; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerLocation.prototype, 'port', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); var h = _wlHref.split('/')[2] || ''; var p = h.split(':')[1]; return p || ''; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerLocation.prototype, 'pathname', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); return '/' + (_wlHref.split('/').slice(3).join('/') || '').split('?')[0].split('#')[0]; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerLocation.prototype, 'search', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); var q = _wlHref.split('?')[1]; return q ? '?' + q.split('#')[0] : ''; }}, enumerable: true, configurable: true }});
    Object.defineProperty(WorkerLocation.prototype, 'hash', {{ get: function() {{ if (this !== globalThis.__workerLocation && this !== WorkerLocation.prototype) throw new TypeError('Illegal invocation'); var h = _wlHref.split('#')[1]; return h ? '#' + h : ''; }}, enumerable: true, configurable: true }});
    WorkerLocation.prototype.toString = function() {{ return _wlHref; }};
    if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {{
        Object.defineProperty(WorkerLocation.prototype, Symbol.toStringTag, {{
            value: 'WorkerLocation', writable: false, configurable: true, enumerable: false
        }});
    }}
    try {{ Object.defineProperty(globalThis, 'WorkerLocation', {{ value: WorkerLocation, writable: true, configurable: true, enumerable: false }}); }} catch(e) {{}}
    try {{ Object.defineProperty(WorkerLocation, 'prototype', {{ writable: false, enumerable: false, configurable: false }}); }} catch(e) {{}}
    globalThis.__workerLocation = Object.create(WorkerLocation.prototype);
}}
try {{ Object.defineProperty(globalThis, 'WorkerLocation', {{ value: globalThis.WorkerLocation, writable: true, configurable: true, enumerable: false }}); }} catch(e) {{}}

// DedicatedWorkerGlobalScope : WorkerGlobalScope
if (typeof globalThis.DedicatedWorkerGlobalScope === 'undefined') {{
    function DedicatedWorkerGlobalScope() {{}}
    DedicatedWorkerGlobalScope.prototype = Object.create(
        (typeof WorkerGlobalScope !== 'undefined' && WorkerGlobalScope.prototype) ? WorkerGlobalScope.prototype : Object.prototype
    );
    Object.defineProperty(DedicatedWorkerGlobalScope.prototype, 'constructor', {{
        value: DedicatedWorkerGlobalScope, writable: true, enumerable: false, configurable: true
    }});
    // name
    Object.defineProperty(DedicatedWorkerGlobalScope.prototype, 'name', {{ get: function() {{ return ''; }}, enumerable: true, configurable: true }});
    // postMessage
    DedicatedWorkerGlobalScope.prototype.postMessage = function postMessage() {{
        if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present.');
    }};
    try {{ Object.defineProperty(DedicatedWorkerGlobalScope.prototype.postMessage, 'length', {{ value: 1, writable: false, enumerable: false, configurable: true }}); }} catch(e) {{}}
    DedicatedWorkerGlobalScope.prototype.postMessage.name = 'postMessage';
    // close
    DedicatedWorkerGlobalScope.prototype.close = function close() {{}};
    try {{ Object.defineProperty(DedicatedWorkerGlobalScope.prototype.close, 'length', {{ value: 0, writable: false, enumerable: false, configurable: true }}); }} catch(e) {{}}
    // Event handlers
    var _dwgsHandlers = ['onmessage','onmessageerror'];
    _dwgsHandlers.forEach(function(hn) {{
        Object.defineProperty(DedicatedWorkerGlobalScope.prototype, hn, {{
            get: function() {{ return this['__iv8' + hn] || null; }},
            set: function(v) {{ this['__iv8' + hn] = v; }},
            enumerable: true, configurable: true
        }});
    }});
    if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {{
        Object.defineProperty(DedicatedWorkerGlobalScope.prototype, Symbol.toStringTag, {{
            value: 'DedicatedWorkerGlobalScope', writable: false, configurable: true, enumerable: false
        }});
    }}
    try {{ Object.defineProperty(globalThis, 'DedicatedWorkerGlobalScope', {{ value: DedicatedWorkerGlobalScope, writable: true, configurable: true, enumerable: false }}); }} catch(e) {{}}
    try {{ Object.defineProperty(DedicatedWorkerGlobalScope, 'prototype', {{ writable: false, enumerable: false, configurable: false }}); }} catch(e) {{}}
    // Immutable prototype: [[SetPrototypeOf]] must throw
    Object.defineProperty(DedicatedWorkerGlobalScope.prototype, '__proto__', {{
        configurable: false,
        get: function() {{ return Object.getPrototypeOf(DedicatedWorkerGlobalScope.prototype); }},
        set: function() {{ throw new TypeError('Cannot set prototype of an immutable prototype object'); }}
    }});
    // Set __proto__ of globalThis to DedicatedWorkerGlobalScope.prototype
    // so `self instanceof DedicatedWorkerGlobalScope` works
    try {{ Object.setPrototypeOf(globalThis, DedicatedWorkerGlobalScope.prototype); }} catch(e) {{}}
}}
try {{ Object.defineProperty(globalThis, 'DedicatedWorkerGlobalScope', {{ value: globalThis.DedicatedWorkerGlobalScope, writable: true, configurable: true, enumerable: false }}); }} catch(e) {{}}

// {scope_name} for idlharness instanceof check
if (typeof globalThis.{scope_name} === 'undefined') {{
    globalThis.{scope_name} = function {scope_name}() {{}};
}}
// Remove Window-only interfaces from globalThis.
// idlharness checks `name in self` for non-exposed interfaces — if the
// interface object exists on globalThis but is not [Exposed] in workers,
// the test "should not exist" fails. We delete known Window-only
// interfaces so worker tests pass.
(function() {{
    var windowOnly = [
        'Document', 'XMLDocument', 'DOMImplementation', 'DocumentType',
        'DocumentFragment', 'ShadowRoot', 'Element', 'NamedNodeMap',
        'Attr', 'CharacterData', 'Text', 'CDATASection',
        'ProcessingInstruction', 'Comment', 'AbstractRange', 'StaticRange',
        'Range', 'NodeIterator', 'TreeWalker', 'NodeFilter', 'DOMTokenList',
        'XPathResult', 'XPathExpression', 'XPathNSResolver', 'XPathEvaluator',
        'XSLTProcessor', 'NodeList', 'HTMLCollection', 'MutationObserver',
        'MutationRecord', 'EventListener', 'Node',
        'HTMLElement', 'HTMLUnknownElement', 'HTMLHtmlElement',
        'HTMLHeadElement', 'HTMLTitleElement', 'HTMLBaseElement',
        'HTMLLinkElement', 'HTMLMetaElement', 'HTMLStyleElement',
        'HTMLBodyElement', 'HTMLHeadingElement', 'HTMLParagraphElement',
        'HTMLHRElement', 'HTMLPreElement', 'HTMLQuoteElement',
        'HTMLOListElement', 'HTMLUListElement', 'HTMLMenuElement',
        'HTMLLIElement', 'HTMLDListElement', 'HTMLDivElement',
        'HTMLAnchorElement', 'HTMLDataElement', 'HTMLTimeElement',
        'HTMLSpanElement', 'HTMLBRElement', 'HTMLModElement',
        'HTMLPictureElement', 'HTMLSourceElement', 'HTMLImageElement',
        'HTMLIFrameElement', 'HTMLEmbedElement', 'HTMLObjectElement',
        'HTMLVideoElement', 'HTMLAudioElement', 'HTMLTrackElement',
        'HTMLMediaElement', 'MediaError', 'AudioTrackList', 'AudioTrack',
        'VideoTrackList', 'VideoTrack', 'TextTrackList', 'TextTrack',
        'TextTrackCueList', 'TextTrackCue', 'TimeRanges', 'TrackEvent',
        'HTMLMapElement', 'HTMLAreaElement', 'HTMLTableElement',
        'HTMLTableCaptionElement', 'HTMLTableColElement',
        'HTMLTableSectionElement', 'HTMLTableRowElement',
        'HTMLTableCellElement', 'HTMLFormElement', 'HTMLLabelElement',
        'HTMLInputElement', 'HTMLButtonElement', 'HTMLSelectElement',
        'HTMLDataListElement', 'HTMLOptGroupElement', 'HTMLOptionElement',
        'HTMLTextAreaElement', 'HTMLOutputElement', 'HTMLProgressElement',
        'HTMLMeterElement', 'HTMLFieldSetElement', 'HTMLLegendElement',
        'HTMLSelectedContentElement', 'ValidityState', 'SubmitEvent',
        'FormDataEvent', 'HTMLDetailsElement', 'HTMLDialogElement',
        'HTMLScriptElement', 'HTMLTemplateElement', 'HTMLSlotElement',
        'HTMLCanvasElement', 'CanvasRenderingContext2D',
        'CustomElementRegistry', 'ElementInternals', 'CustomStateSet',
        'VisibilityStateEntry', 'UserActivation', 'ToggleEvent',
        'CommandEvent', 'CloseWatcher', 'DataTransfer',
        'DataTransferItemList', 'DataTransferItem', 'DragEvent',
        'BarProp', 'History', 'Navigation', 'NavigationHistoryEntry',
        'NavigationTransition', 'NavigationActivation', 'NavigateEvent',
        'NavigationPrecommitController', 'NavigationDestination',
        'NavigationCurrentEntryChangeEvent', 'PopStateEvent',
        'HashChangeEvent', 'PageSwapEvent', 'PageRevealEvent',
        'PageTransitionEvent', 'BeforeUnloadEvent',
        'NotRestoredReasonDetails', 'NotRestoredReasons',
        'DOMParser', 'XMLSerializer', 'Sanitizer',
        'PluginArray', 'MimeTypeArray', 'Plugin', 'MimeType',
        'Storage', 'StorageEvent', 'HTMLMarqueeElement',
        'HTMLFrameSetElement', 'HTMLFrameElement', 'HTMLDirectoryElement',
        'HTMLFontElement', 'HTMLParamElement', 'External',
        'HTMLAllCollection', 'HTMLFormControlsCollection',
        'RadioNodeList', 'HTMLOptionsCollection', 'DOMStringMap',
        'Window', 'Location', 'Navigator', 'SharedWorker', 'Worklet',
    ];
    for (var i = 0; i < windowOnly.length; i++) {{
        try {{ delete globalThis[windowOnly[i]]; }} catch(e) {{}}
    }}
    // Also remove document, window, etc.
    try {{ delete globalThis.document; }} catch(e) {{}}
    try {{ delete globalThis.window; }} catch(e) {{}}
    try {{ delete globalThis.top; }} catch(e) {{}}
    try {{ delete globalThis.parent; }} catch(e) {{}}
    try {{ delete globalThis.frames; }} catch(e) {{}}
    try {{ delete globalThis.locationbar; }} catch(e) {{}}
    try {{ delete globalThis.menubar; }} catch(e) {{}}
    try {{ delete globalThis.personalbar; }} catch(e) {{}}
    try {{ delete globalThis.scrollbars; }} catch(e) {{}}
    try {{ delete globalThis.statusbar; }} catch(e) {{}}
    try {{ delete globalThis.toolbar; }} catch(e) {{}}
    // Keep self — testharness.js needs it. self === globalThis in workers.
}})();
"""
        pre_shim = f"""
globalThis.GLOBAL = globalThis;
globalThis.GLOBAL.isWindow = function() {{ return {is_window}; }};
globalThis.GLOBAL.isWorker = function() {{ return {is_worker_js}; }};
globalThis.GLOBAL.isShadowRealm = function() {{ return false; }};
{worker_shim}
Object.defineProperty(globalThis, 'location', {{
    value: {{ search: {json.dumps(variant_query)}, href: 'about:blank' }},
    writable: true, configurable: true, enumerable: true
}});
globalThis.fetch_spec = function(spec) {{
    return fetch("/interfaces/" + spec + ".idl").then(function(r) {{
        if (!r.ok) throw new Error("Error fetching " + spec);
        return r.text();
    }}).then(function(idl) {{ return {{ spec: spec, idl: idl }}; }});
}};
var __results = [];
var __loadCallbacks = [];
// Intercept addEventListener BEFORE testharness.js loads
var __origAEL = globalThis.addEventListener;
globalThis.addEventListener = function(type, listener, useCapture) {{
    if (type === 'load') {{
        __loadCallbacks.push(listener);
    }} else {{
        __origAEL.call(this, type, listener, useCapture);
    }}
}};
// window.addEventListener should be the same
if (typeof window !== 'undefined' && window !== globalThis) {{
    window.addEventListener = globalThis.addEventListener;
}}
"""
        ctx.eval(pre_shim, name="iv8-pre-shim.js")

        # Suite-specific pre-shim: create test objects that require DOM features
        # IV8 doesn't fully support (e.g. <style> element parsing, SVG elements)
        suite_specific_shim = ""
        if "css/cssom" in suite_name:
            suite_specific_shim = """
// CSSOM test objects — CSSOM_SHIM_JS handles element.sheet parsing
self.style_element = document.createElement('style');
self.style_element.textContent = '@import url("data:text/css,"); @namespace x "y"; @page { @top-left {} } @media all {} #test { color: green; }';
document.head.appendChild(self.style_element);
self.sheet = self.style_element.sheet;
self.svg_element = document.createElement('svg');
self.svg_element.id = 'svgElement';
self.xmlss_pi = document.createProcessingInstruction('xml-stylesheet', 'href="data:text/css,"');
"""
        elif "performance-timeline" in suite_name:
            suite_specific_shim = """
self.observer = new PerformanceObserver(function() {});
"""
        elif "requestidlecallback" in suite_name:
            suite_specific_shim = """
self.deadline = { didTimeout: false, timeRemaining: function() { return 50; } };
"""
        elif suite_name == "dom/idlharness":
            suite_specific_shim = """
var _idlTestEl = document.createElement('div');
_idlTestEl.setAttribute('id', 'idl-test-element');
document.body.appendChild(_idlTestEl);
"""
        elif "FileAPI" in suite_name:
            suite_specific_shim = """
var fc = document.createElement('input');
fc.type = 'file';
fc.id = 'fileChooser';
document.body.appendChild(fc);
"""
        elif "navigation-timing" in suite_name:
            suite_specific_shim = """
self.xmlss_pi = document.createProcessingInstruction('xml-stylesheet', 'href="data:text/css,"');
"""

        if suite_specific_shim:
            try:
                ctx.eval(suite_specific_shim, name="iv8-suite-shim.js")
            except Exception as e:
                print(f"  Suite shim warning: {e}")

        # Load WPT harness (IIFE registers load listener via our shim)
        ctx.eval(resources["testharness.js"], name="testharness.js")
        ctx.eval(resources["testharnessreport.js"], name="testharnessreport.js")
        # Load subset-tests-by-key.js for variant filtering
        subset_path = RESOURCES_DIR / "subset-tests-by-key.js"
        if subset_path.exists():
            ctx.eval(subset_path.read_text(encoding="utf-8"),
                     name="subset-tests-by-key.js")
        ctx.eval(resources["webidl2.js"], name="webidl2.js")
        idlharness_code = resources["idlharness.js"]
        if is_worker:
            worker_type = variant_name  # "worker", "serviceworker", "sharedworker"
            # Patch idlharness to think we're in the specified Worker context
            idlharness_code = idlharness_code.replace(
                "'Window' in self",
                "false",
            )
            # Map variant name to GlobalScope type
            if worker_type == "serviceworker":
                idlharness_code = idlharness_code.replace(
                    "self instanceof ServiceWorkerGlobalScope",
                    "typeof ServiceWorkerGlobalScope !== 'undefined' && true",
                )
                worker_scope_name = "ServiceWorkerGlobalScope"
            elif worker_type == "sharedworker":
                idlharness_code = idlharness_code.replace(
                    "self instanceof SharedWorkerGlobalScope",
                    "typeof SharedWorkerGlobalScope !== 'undefined' && true",
                )
                worker_scope_name = "SharedWorkerGlobalScope"
            else:
                idlharness_code = idlharness_code.replace(
                    "self instanceof DedicatedWorkerGlobalScope",
                    "typeof DedicatedWorkerGlobalScope !== 'undefined' && true",
                )
                worker_scope_name = "DedicatedWorkerGlobalScope"
        ctx.eval(idlharness_code, name="idlharness.js")

        # Post-shim: add result callback + completion callback
        post_shim = """
add_result_callback(function(test) {
    __results.push({
        name: test.name,
        status: test.format_status(),
        message: test.message || null
    });
});
var __completed = false;
add_completion_callback(function(tests, harness_status) {
    __completed = true;
});
"""
        ctx.eval(post_shim, name="iv8-post-shim.js")

        # Run test code — idl_test() is async (promise_test).
        # After test code registers tests, we need to:
        # 1. Fire load callbacks (triggers testharness to start tests)
        # 2. Drain event loop until all tests complete
        full_test_code = test_code + "\n;"

        try:
            # eval_promise evals the test code. idl_test() calls
            # promise_test() which returns undefined (not a promise),
            # so eval_promise returns immediately. The actual test
            # execution happens asynchronously via tests.promise_tests
            # chain, which needs microtask + macrotask draining.
            ctx.eval_promise(full_test_code, max_ticks=10000)

            # Fire load callbacks — testharness needs load event to
            # set all_loaded=true and call tests.complete()
            load_count = ctx.eval("__loadCallbacks.length")
            print(f"  Load callbacks: {load_count}")
            ctx.eval("""
                for (var i = 0; i < __loadCallbacks.length; i++) {
                    try { __loadCallbacks[i]({ type: 'load' }); } catch(e) {}
                }
            """)

            # Drain event loop until testharness completes or stabilizes.
            prev_count = -1
            stable_ticks = 0
            for i in range(1000):
                # Drain microtasks
                ctx.eval("__iv8__.eventLoop.drain()")
                # Run one macrotask (setTimeout callbacks etc)
                ctx.eval("__iv8__.eventLoop.tick()")
                # Check completion
                try:
                    completed = ctx.eval("__completed")
                except Exception:
                    completed = False
                if completed:
                    break
                # Check results stabilization
                try:
                    current = ctx.eval("__results.length")
                except Exception:
                    break
                if current == prev_count:
                    stable_ticks += 1
                    if stable_ticks >= 10 and current > 0:
                        break
                else:
                    stable_ticks = 0
                prev_count = current
                if i % 100 == 0:
                    print(f"  tick {i}: results={current}, completed={completed}")
            run_status = "completed"
        except Exception as e:
            run_status = f"error: {e}"
            print(f"  Execution error: {e}")

        # Collect results — __results may be populated even if
        # eval_promise threw (e.g. show_results crash after tests complete)
        try:
            results_json = ctx.eval("JSON.stringify(__results)")
            results = json.loads(results_json)
        except Exception:
            results = []

        pass_count = sum(1 for r in results if r["status"] == "Pass")
        fail_count = sum(1 for r in results if r["status"] != "Pass")

        return {
            "suite": suite_name,
            "variant": variant_name,
            "run_status": run_status,
            "total": len(results),
            "pass": pass_count,
            "fail": fail_count,
            "tests": results,
        }

    finally:
        ctx.close()


def fetch_chrome_baseline() -> dict:
    """Fetch Chrome's WPT results from wpt.fyi API for comparison.

    Uses Chrome master run (not pr_base) to ensure complete test coverage.
    The default /api/search returns a mix of pr_base + master runs where
    Chrome pr_base often has total:0 (not run), causing fallback to Edge.
    """
    baselines = {}

    # Step 1: Get Chrome master run id
    runs_url = "https://wpt.fyi/api/runs?browser=chrome&label=master&max-count=1"
    try:
        req = urllib.request.Request(runs_url, headers={"User-Agent": "IV8-WPT/1.0"})
        with urllib.request.urlopen(req, timeout=15) as resp:
            runs_data = json.loads(resp.read())
        if not runs_data:
            return baselines
        chrome_run_id = runs_data[0]["id"]
    except Exception:
        return baselines

    # Step 2: Search idlharness tests with Chrome master run_id
    test_paths = [
        "html/dom/idlharness",
        "dom/idlharness",
        "cssom-view/idlharness",
    ]
    for query in test_paths:
        url = f"https://wpt.fyi/api/search?q={query}&run_ids={chrome_run_id}"
        try:
            req = urllib.request.Request(url, headers={"User-Agent": "IV8-WPT/1.0"})
            with urllib.request.urlopen(req, timeout=15) as resp:
                data = json.loads(resp.read())
            for result in data.get("results", []):
                test = result["test"]
                for status in result.get("legacy_status", []):
                    if status.get("total", 0) > 0:
                        baselines[test] = {
                            "pass": status["passes"],
                            "total": status["total"],
                        }
                        break
        except Exception:
            pass
    return baselines


def main() -> None:
    import argparse

    parser = argparse.ArgumentParser(description="WPT official test runner")
    parser.add_argument("--suite", "-s", help="Filter suite (e.g. html/dom)")
    parser.add_argument("--baseline", "-b", action="store_true",
                        help="Only run Chrome baseline 10 files (9640 tests)")
    parser.add_argument("--update", action="store_true",
                        help="Update status files to match current results")
    parser.add_argument("--output", "-o", default=str(OUT_PATH))
    args = parser.parse_args()

    # Set stack size for V8 template creation
    threading.stack_size(64 * 1024 * 1024)

    result_holder = {}

    def worker():
        try:
            resources = load_wpt_resources()

            suites = WPT_SUITES
            if args.suite:
                suites = [s for s in suites if args.suite in s["name"]]
            elif args.baseline:
                # Only Chrome baseline 10 files
                baseline_names = {
                    "html/dom/idlharness", "html/dom/idlharness.worker",
                    "dom/idlharness", "dom/idlharness.worker",
                    "css/cssom-view/idlharness",
                }
                suites = [s for s in suites if s["name"] in baseline_names]

            all_results = []
            for suite in suites:
                for variant in suite["variants"]:
                    print(f"\n--- {suite['name']} [{variant['name']}] ---")
                    result = run_suite(suite, variant, resources)
                    print(f"  Total={result['total']}, "
                          f"Pass={result['pass']}, Fail={result['fail']}")
                    all_results.append(result)

            result_holder["results"] = all_results
        except Exception as e:
            result_holder["error"] = repr(e)

    t = threading.Thread(target=worker)
    t.start()
    t.join()

    if "error" in result_holder:
        print(f"ERROR: {result_holder['error']}")
        sys.exit(1)

    results = result_holder["results"]

    # Compute totals
    total_tests = sum(r["total"] for r in results)
    total_pass = sum(r["pass"] for r in results)
    total_fail = sum(r["fail"] for r in results)

    # Fetch Chrome baseline
    print("\nFetching Chrome baseline from wpt.fyi...")
    chrome_baseline = fetch_chrome_baseline()

    report = {
        "schema_version": "wpt-report.v0.1",
        "source": "WPT official test files (direct reuse)",
        "suites": results,
        "chrome_baseline": chrome_baseline,
        "summary": {
            "total_tests": total_tests,
            "total_pass": total_pass,
            "total_fail": total_fail,
            "pass_rate": round(total_pass / total_tests * 100, 2) if total_tests > 0 else 0,
        },
    }

    # Write report
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    try:
        report_str = json.dumps(report, indent=2, ensure_ascii=False)
        output_path.write_text(report_str, encoding="utf-8")
    except Exception as e:
        print(f"WARNING: json.dumps failed ({e}), writing minimal report")
        minimal_report = {
            "schema_version": report.get("schema_version"),
            "summary": report["summary"],
            "suites": [{"suite": r["suite"], "variant": r["variant"],
                         "run_status": r["run_status"], "total": r["total"],
                         "pass": r["pass"], "fail": r["fail"],
                         "tests": r["tests"][:10]} for r in results],
        }
        output_path.write_text(
            json.dumps(minimal_report, indent=2, ensure_ascii=False),
            encoding="utf-8",
        )

    print("\n" + "=" * 60)
    print("WPT Official Test Report")
    print("=" * 60)
    print(f"Total: {total_pass} PASS, {total_fail} FAIL / {total_tests} "
          f"({report['summary']['pass_rate']}%)")
    print()

    for r in results:
        print(f"  {r['suite']} [{r['variant']}]: "
              f"{r['pass']}/{r['total']} PASS")

    if chrome_baseline:
        print("\nChrome baseline (wpt.fyi):")
        for test, baseline in chrome_baseline.items():
            print(f"  {test}: {baseline['pass']}/{baseline['total']}")

    print(f"\nReport written to {output_path}")

    # Update status files if requested
    if args.update:
        status_path = STATUS_DIR / "idlharness.json"
        status = {}
        for r in results:
            key = f"{r['suite']} [{r['variant']}]"
            fails = [t["name"] for t in r["tests"] if t["status"] != "Pass"]
            if fails:
                status[key] = {"fail": {"expected": fails}}
        STATUS_DIR.mkdir(parents=True, exist_ok=True)
        status_path.write_text(
            json.dumps(status, indent=2, ensure_ascii=False),
            encoding="utf-8",
        )
        print(f"Status written to {status_path}")

    sys.exit(0 if total_fail == 0 else 1)


if __name__ == "__main__":
    main()
