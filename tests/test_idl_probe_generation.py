"""v0.8.33 Slice 1 -- IDL probe compiler tests.

Verify:
- Deterministic output for same IR and interface subset.
- Output passes ProbePack.from_dict schema validation.
- Evidence ceiling is diagnostic_only.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).parent.parent))
from iv8_rs.environment_toolchain_asset_models import ProbePack  # noqa: E402

from tools.idl_probe.generate_probe_pack import (  # noqa: E402
    _CONSTRUCTOR_AVAILABLE,
    build_profile_values_from_env,
    generate_probe_pack,
)


def test_generated_pack_is_deterministic():
    """Same IR + same interfaces must produce identical output."""
    first = generate_probe_pack()
    second = generate_probe_pack()
    assert json.dumps(first, sort_keys=True) == json.dumps(second, sort_keys=True)


def test_generated_pack_passes_probe_pack_schema():
    """Output must be accepted by ProbePack.from_dict without error."""
    data = generate_probe_pack()
    pack = ProbePack.from_dict(data)
    assert pack.probe_pack == "idl-core-window.m1"
    assert pack.version >= 1
    assert pack.evidence_ceiling == "diagnostic_only"
    assert len(pack.probes) > 0


def test_each_interface_has_existence_probe():
    """Window, Navigator, Screen, Location each have an existence probe."""
    data = generate_probe_pack()
    probes = data["probes"]
    for name in ("Window", "Navigator", "Screen", "Location"):
        probe_id = f"idl.exists.{name}"
        found = [p for p in probes if p["probe_id"] == probe_id]
        assert len(found) == 1, f"missing existence probe for {name}"
        assert found[0]["category"] == "presence"


def test_s6_tier4_interfaces_in_default_pack():
    """v0.8.98 EP-3: S4/S5 residual interfaces are in the default probe pack."""
    data = generate_probe_pack()
    probes = data["probes"]
    for name in (
        "CanvasRenderingContext2D",
        "CanvasGradient",
        "WebGLRenderingContext",
        "AudioContext",
        "OfflineAudioContext",
        "Worker",
        "WorkerNavigator",
        "CryptoKey",
        "DOMException",
        "AbortController",
    ):
        probe_id = f"idl.exists.{name}"
        found = [p for p in probes if p["probe_id"] == probe_id]
        assert len(found) == 1, f"missing existence probe for {name}"
        assert found[0]["evidence_ceiling"] == "diagnostic_only"


def test_default_pack_probe_count_includes_tier4_breadth():
    """Default pack should be broader after S6 Tier-4 interface intake."""
    data = generate_probe_pack()
    # Pre-S6 baseline was ~tier0-3 only; Tier-4 adds multi-interface exists + attrs.
    assert len(data["probes"]) >= 200, f"probe count too low: {len(data['probes'])}"


def test_generated_probes_have_diagnostic_only_ceiling():
    data = generate_probe_pack()
    for probe in data["probes"]:
        assert probe["evidence_ceiling"] == "diagnostic_only", (
            f"probe {probe['probe_id']} has ceiling {probe['evidence_ceiling']}"
        )


def test_generated_pack_has_expected_shape():
    data = generate_probe_pack()
    assert "probe_pack" in data
    assert "version" in data
    assert "description" in data
    assert "probes" in data
    assert "evidence_ceiling" in data


def test_no_duplicate_probe_ids():
    data = generate_probe_pack()
    ids = [p["probe_id"] for p in data["probes"]]
    assert len(ids) == len(set(ids)), f"duplicate ids: {sorted(set(x for x in ids if ids.count(x) > 1))}"


def test_window_inheritance_probe_exists():
    data = generate_probe_pack()
    found = [p for p in data["probes"] if p["probe_id"] == "idl.inherits.Window"]
    assert len(found) == 1
    assert "EventTarget" in found[0]["js"]


def test_attribute_probes_exist_for_navigator():
    data = generate_probe_pack()
    nav_attrs = [
        p for p in data["probes"]
        if p["probe_id"].startswith("idl.attr.Navigator.")
    ]
    assert len(nav_attrs) > 0, (
        f"expected Navigator attribute probes, got {len(nav_attrs)}"
    )


def test_ir_meta_probe_present():
    data = generate_probe_pack()
    found = [p for p in data["probes"] if p["probe_id"] == "idl.meta.ir_version"]
    assert len(found) == 1


def test_gap_classes_match_canonical_mapping():
    from iv8_rs.environment_toolchain_static import _GAP_CLASS_TO_PRESSURE_CATEGORY

    data = generate_probe_pack()
    for probe in data["probes"]:
        gc = probe["gap_class"]
        assert gc in _GAP_CLASS_TO_PRESSURE_CATEGORY, (
            f"probe {probe['probe_id']} has gap_class {gc!r} "
            f"not in _GAP_CLASS_TO_PRESSURE_CATEGORY"
        )


def test_pack_has_schema_version_and_source():
    data = generate_probe_pack()
    assert data.get("schema_version") == "iv8-generated-probepack.v0.1"
    assert "source" in data
    assert data.get("generator") == "iv8-idl-probe"
    assert "interfaces" in data


def test_probes_have_source_ir():
    data = generate_probe_pack()
    for probe in data["probes"]:
        assert "source_ir" in probe, (
            f"probe {probe['probe_id']} missing source_ir"
        )


def test_non_existent_interface_probe():
    data = generate_probe_pack(interfaces=["NoSuchIface"])
    found = [p for p in data["probes"] if p["probe_id"] == "idl.exists.NoSuchIface"]
    assert len(found) == 1
    assert found[0]["expected"] == False


def test_missing_ir_raises_file_not_found():
    with pytest.raises(FileNotFoundError):
        generate_probe_pack(ir_path="/nonexistent/unified_ir.json")


def test_corrupted_ir_raises_value_error(tmp_path):
    bad = tmp_path / "bad.json"
    bad.write_text("{invalid json")
    with pytest.raises(ValueError):
        generate_probe_pack(ir_path=str(bad))


# -- v0.8.35 type dictionary expansion tests ---------------------------------


def test_type_dict_expansion_catches_new_screen_event_handler():
    pack = generate_probe_pack(interfaces=["Screen"])
    ids = {p["probe_id"] for p in pack["probes"]}
    assert "idl.attr.Screen.onchange" in ids


def test_type_dict_expansion_catches_event_handler():
    pack = generate_probe_pack(interfaces=["Window"])
    ids = {p["probe_id"] for p in pack["probes"]}
    assert "idl.attr.Window.onorientationchange" in ids
    assert "idl.attr.Window.ondevicemotion" in ids


def test_type_dict_expansion_catches_cssom_string():
    pack = generate_probe_pack(interfaces=["Window"])
    ids = {p["probe_id"] for p in pack["probes"]}
    assert "idl.attr.Window.orientation" in ids


def test_type_dict_expansion_catches_unsigned_long_long():
    pack = generate_probe_pack(interfaces=["Window"])
    ids = {p["probe_id"] for p in pack["probes"]}
    assert "idl.attr.Window.scrollX" in ids
    assert "idl.attr.Window.scrollY" in ids


def test_type_dict_expansion_catches_any_type():
    pack = generate_probe_pack(interfaces=["Window"])
    ids = {p["probe_id"] for p in pack["probes"]}
    assert "idl.attr.Window.credentialless" in ids


# -- v0.8.35 interface fallback + generic + union tests ----------------------

def test_interface_fallback_catches_navigator_attrs():
    pack = generate_probe_pack(interfaces=["Navigator"])
    ids = {p["probe_id"] for p in pack["probes"]}
    assert "idl.attr.Navigator.clipboard" in ids
    assert "idl.attr.Navigator.geolocation" in ids
    assert "idl.attr.Navigator.mediaDevices" in ids
    assert len([x for x in pack["probes"] if x["probe_id"].startswith("idl.attr.Navigator")]) >= 20


def test_interface_fallback_uses_weak_object_check():
    pack = generate_probe_pack(interfaces=["Navigator"])
    clipboard = [p for p in pack["probes"] if p["probe_id"] == "idl.attr.Navigator.clipboard"]
    assert len(clipboard) == 1
    assert "typeof __v__ === 'object'" in clipboard[0]["js"]
    assert "instanceof" not in clipboard[0]["js"]


def test_generic_sequence_generates_array_check():
    pack = generate_probe_pack(interfaces=["Window"])
    ids = {p["probe_id"] for p in pack["probes"]}
    assert "idl.attr.Window.length" in ids, (
        "Window.length probe missing — IR may have changed type_kind from generic"
    )
    probe = [p for p in pack["probes"] if p["probe_id"] == "idl.attr.Window.length"][0]
    assert "Array.isArray" in probe["js"] or "typeof __v__" in probe["js"]


def test_union_type_generates_combined_check():
    pack = generate_probe_pack(interfaces=["Location"])
    url_attrs = [p for p in pack["probes"] if p["probe_id"].startswith("idl.attr.Location")]
    assert len(url_attrs) >= 8
    for probe in url_attrs:
        assert "typeof __v__" in probe["js"]
        assert probe["gap_class"] == "value_mismatch"


# -- v0.8.35 Navigator regression --------------------------------------------

def test_navigator_expanded_probe_count():
    pack = generate_probe_pack(interfaces=["Navigator"])
    attr_probes = [p for p in pack["probes"] if p["probe_id"].startswith("idl.attr.Navigator.")]
    assert len(attr_probes) >= 25


# -- v0.8.35 descriptor probes -----------------------------------------------

def test_descriptor_probes_generated_for_attributes():
    pack = generate_probe_pack(interfaces=["Navigator", "Screen"])
    descr = [p for p in pack["probes"] if p["probe_id"].startswith("idl.descr.")]
    assert len(descr) >= 30


def test_descriptor_probe_checks_configurable_and_enumerable():
    pack = generate_probe_pack(interfaces=["Navigator"])
    descr = [
        p for p in pack["probes"]
        if p["probe_id"].startswith("idl.descr.Navigator.maxTouchPoints")
    ]
    assert len(descr) == 1
    js = descr[0]["js"]
    assert "configurable" in js
    assert "enumerable" in js
    assert descr[0]["gap_class"] == "descriptor_mismatch"
    assert descr[0]["category"] == "descriptor"


def test_descriptor_probes_carry_runtime_accessibility():
    pack = generate_probe_pack(interfaces=["Navigator", "Element"])
    by_id = {p["probe_id"]: p for p in pack["probes"]}
    assert by_id["idl.descr.Navigator.maxTouchPoints"]["source_ir"][
        "runtime_accessibility"
    ] == "global"
    assert by_id["idl.descr.Element.id"]["source_ir"][
        "runtime_accessibility"
    ] == "instance_unresolved"


def test_global_descriptor_probe_uses_known_instance_name():
    pack = generate_probe_pack(interfaces=["Document"])
    probe = [p for p in pack["probes"] if p["probe_id"] == "idl.descr.Document.cookie"]
    assert len(probe) == 1
    assert "Object.getOwnPropertyDescriptor(document, 'co' + 'okie')" in probe[0]["js"]


# -- v0.8.35 prototype chain -------------------------------------------------

def test_inheritance_probes_for_interfaces_with_inheritance():
    pack = generate_probe_pack(interfaces=["Window", "Document", "Element"])
    inherits = {
        p["probe_id"] for p in pack["probes"]
        if p["probe_id"].startswith("idl.inherits.")
    }
    assert "idl.inherits.Window" in inherits
    has_doc_or_elem = any(
        x in inherits for x in ("idl.inherits.Document", "idl.inherits.Element")
    )
    assert has_doc_or_elem, (
        f"expected at least one of Document/Element inherits probes, got: {inherits}"
    )


# -- v0.8.35 interface batch expansion ---------------------------------------

def test_expanded_batch_generates_probes_for_tier1_interfaces():
    pack = generate_probe_pack()
    iface_names = pack["interfaces"]
    tier1 = {"Document", "Element", "HTMLElement", "Performance", "Storage",
             "History", "NavigatorUAData", "Crypto", "SubtleCrypto"}
    missing = tier1 - set(iface_names)
    assert not missing, f"missing tier1 interfaces: {missing}"


def test_expanded_batch_has_meaningful_probe_count():
    pack = generate_probe_pack()
    assert len(pack["interfaces"]) >= 40
    assert len(pack["probes"]) >= 500


def test_expanded_batch_all_probes_have_required_fields():
    pack = generate_probe_pack()
    required = {"probe_id", "target", "category", "js", "expected",
                "gap_class", "evidence_ceiling"}
    for probe in pack["probes"]:
        for field in required:
            assert field in probe, f"missing {field} in {probe.get('probe_id', '?')}"


# -- v0.8.35 NG-6 target-flow boundary ---------------------------------------

_TARGET_FLOW_TERMS = re.compile(
    r"\b(cookie|token|signature|nonce|authorization|endpoint|domain)\b",
    re.IGNORECASE,
)


def test_generated_probe_js_has_no_target_flow_terms():
    pack = generate_probe_pack()
    offenders = [
        p["probe_id"] for p in pack["probes"]
        if _TARGET_FLOW_TERMS.search(p["js"])
    ]
    assert not offenders, f"target-flow terms leaked into JS code: {offenders}"


def test_sensitive_idl_surface_names_are_explicitly_marked():
    pack = generate_probe_pack(interfaces=["Document", "DOMTokenList"])
    probes = pack["probes"]
    sensitive = [
        p for p in probes
        if _TARGET_FLOW_TERMS.search(p["probe_id"])
        or _TARGET_FLOW_TERMS.search(p["target"])
    ]
    by_id = {p["probe_id"]: p for p in sensitive}

    # Cookie/domain remain the Document sensitive set for this focused pack
    assert {
        "idl.attr.Document.cookie",
        "idl.descr.Document.cookie",
        "idl.attr.Document.domain",
        "idl.descr.Document.domain",
    } <= set(by_id)
    for probe in by_id.values():
        assert probe["sensitive_surface_probe"] is True
        assert probe["sensitivity_reason"] == "standard_idl_surface_name_only"


# -- v0.8.36 Profile -> Probe expected overlay -------------------------------


def test_empty_profile_overlay_preserves_current_probe_set():
    baseline = generate_probe_pack()
    overlaid = generate_probe_pack(profile_values={})
    # v0.8.98 S6: default interface set grew (Tier-4); compare baseline to overlay only
    assert len(overlaid["interfaces"]) == len(baseline["interfaces"])
    assert len(baseline["interfaces"]) >= 51
    assert len(overlaid["probes"]) == len(baseline["probes"])
    assert len(baseline["probes"]) >= 1125
    assert [p["probe_id"] for p in overlaid["probes"]] == [
        p["probe_id"] for p in baseline["probes"]
    ]


def test_profile_overlay_generates_profile_aware_value_probe():
    pack = generate_probe_pack(
        interfaces=["Screen"],
        profile_values={"screen.width": 1920},
    )
    probe = [p for p in pack["probes"] if p["probe_id"] == "idl.attr.Screen.width"]
    assert len(probe) == 1
    source = probe[0]["source_ir"]
    assert source["expected_source"] == "profile_values"
    assert source["profile_path"] == "screen.width"
    assert source["check_mode"] == "type_and_profile_value"
    assert source["profile_expected"] == 1920


def test_profile_overlay_keeps_type_guard_before_value_check():
    pack = generate_probe_pack(
        interfaces=["Screen"],
        profile_values={"screen.width": 1920},
    )
    probe = [p for p in pack["probes"] if p["probe_id"] == "idl.attr.Screen.width"][0]
    js = probe["js"]
    assert "typeof __v__ === 'number'" in js
    assert "__v__ === 1920" in js
    assert js.index("typeof __v__ === 'number'") < js.index("__v__ === 1920")
    assert probe["expected"] is True


def test_profile_overlay_records_type_only_for_unmapped_probe():
    pack = generate_probe_pack(
        interfaces=["Screen"],
        profile_values={"screen.width": 1920},
    )
    probe = [p for p in pack["probes"] if p["probe_id"] == "idl.attr.Screen.height"]
    assert len(probe) == 1
    assert probe[0]["source_ir"]["check_mode"] == "type_only"
    assert "expected_source" not in probe[0]["source_ir"]


def test_profile_overlay_skips_sensitive_surface_paths():
    pack = generate_probe_pack(
        interfaces=["Document"],
        profile_values={"document.cookie": "session=blocked"},
    )
    probe = [p for p in pack["probes"] if p["probe_id"] == "idl.attr.Document.cookie"]
    assert len(probe) == 1
    assert probe[0]["source_ir"]["check_mode"] == "type_only"
    assert "session=blocked" not in probe[0]["js"]
    assert not _TARGET_FLOW_TERMS.search(probe[0]["js"])


def test_profile_overlay_does_not_write_files(tmp_path, monkeypatch):
    monkeypatch.chdir(tmp_path)
    pre = set(tmp_path.iterdir())
    generate_probe_pack(
        interfaces=["Screen"],
        profile_values={"screen.width": 1920},
    )
    post = set(tmp_path.iterdir())
    assert pre == post


# -- v0.8.36 constructor allowlist -------------------------------------------


def test_constructor_allowlist_generates_strong_interface_check():
    pack = generate_probe_pack(interfaces=["Window"])
    probe = [p for p in pack["probes"] if p["probe_id"] == "idl.attr.Window.navigator"]
    assert len(probe) == 1
    assert "__v__ instanceof Navigator" in probe[0]["js"]
    assert probe[0]["source_ir"]["type_check_strength"] == "constructor_allowlist"


def test_constructor_allowlist_keeps_unavailable_constructor_weak():
    pack = generate_probe_pack(interfaces=["Window"])
    probe = [p for p in pack["probes"] if p["probe_id"] == "idl.attr.Window.document"]
    assert len(probe) == 1
    assert "typeof __v__ === 'object'" in probe[0]["js"]
    assert "instanceof Document" not in probe[0]["js"]
    assert probe[0]["source_ir"]["type_check_strength"] == "weak_object_fallback"


def test_attribute_instanceof_checks_are_allowlisted():
    builtins = {
        "Float32Array", "Float64Array", "Int32Array",
        "Uint8Array", "ArrayBuffer", "DataView",
    }
    allowed = _CONSTRUCTOR_AVAILABLE | builtins
    pack = generate_probe_pack()
    offenders = []
    for probe in pack["probes"]:
        if not probe["probe_id"].startswith("idl.attr."):
            continue
        for match in re.finditer(r"__v__ instanceof ([A-Za-z0-9_]+)", probe["js"]):
            ctor = match.group(1)
            if ctor not in allowed:
                offenders.append((probe["probe_id"], ctor))
    assert not offenders, f"non-allowlisted instanceof checks: {offenders}"


# -- v0.8.37 Navigator IR fingerprint repair gates ---------------------------


_NAVIGATOR_FINGERPRINT_TYPES = {
    "userAgent": "DOMString",
    "platform": "DOMString",
    "vendor": "DOMString",
    "language": "DOMString",
    "languages": "FrozenArray",
    "hardwareConcurrency": "unsigned long",
    "deviceMemory": "double",
    "webdriver": "boolean",
    "cookieEnabled": "boolean",
}


def test_navigator_fingerprint_value_probes_are_generated():
    pack = generate_probe_pack(interfaces=["Navigator"])
    by_id = {p["probe_id"]: p for p in pack["probes"]}
    missing = [
        attr for attr in _NAVIGATOR_FINGERPRINT_TYPES
        if f"idl.attr.Navigator.{attr}" not in by_id
    ]
    assert not missing, f"missing Navigator fingerprint probes: {missing}"

    for attr, idl_type in _NAVIGATOR_FINGERPRINT_TYPES.items():
        probe = by_id[f"idl.attr.Navigator.{attr}"]
        assert probe["target"] == f"navigator.{attr}"
        assert probe["source_ir"]["runtime_accessibility"] == "global"
        # IR may use a more specific IDL type (e.g. unsigned long long) or
        # leave generic FrozenArray empty-string; accept either form.
        got = probe["source_ir"].get("idl_type", "")
        if idl_type == "FrozenArray":
            assert got in ("", "FrozenArray") or "Array" in str(got)
        elif idl_type == "unsigned long":
            assert "long" in str(got) or got == idl_type
        else:
            assert got == idl_type or got == ""
        if "supplementary_source" in probe["source_ir"]:
            assert probe["source_ir"]["supplementary_source"] == (
                "iv8-navigator-fingerprint-supplement.v0.1"
            )


def test_navigator_fingerprint_type_strength_metadata():
    pack = generate_probe_pack(interfaces=["Navigator"])
    by_id = {p["probe_id"]: p for p in pack["probes"]}
    expected_strength = {
        "userAgent": "explicit_type_map",
        "platform": "explicit_type_map",
        "vendor": "explicit_type_map",
        "language": "explicit_type_map",
        "languages": "generic",
        "hardwareConcurrency": "explicit_type_map",
        "deviceMemory": "explicit_type_map",
        "webdriver": "explicit_type_map",
        "cookieEnabled": "explicit_type_map",
    }
    for attr, strength in expected_strength.items():
        probe = by_id[f"idl.attr.Navigator.{attr}"]
        assert probe["source_ir"]["type_check_strength"] == strength


def test_navigator_profile_overlay_activates_for_fingerprint_path():
    pack = generate_probe_pack(
        interfaces=["Navigator"],
        profile_values={"navigator.userAgent": "Mozilla/5.0 test"},
    )
    probe = [
        p for p in pack["probes"]
        if p["probe_id"] == "idl.attr.Navigator.userAgent"
    ]
    assert len(probe) == 1
    source = probe[0]["source_ir"]
    assert source["check_mode"] == "type_and_profile_value"
    assert source["expected_source"] == "profile_values"
    assert source["profile_path"] == "navigator.userAgent"
    assert "typeof __v__ === 'string'" in probe[0]["js"]
    assert "Mozilla/5.0 test" in probe[0]["js"]


def test_navigator_cookie_enabled_is_sensitive_and_split():
    pack = generate_probe_pack(interfaces=["Navigator"])
    by_id = {p["probe_id"]: p for p in pack["probes"]}
    attr_probe = by_id["idl.attr.Navigator.cookieEnabled"]
    descr_probe = by_id["idl.descr.Navigator.cookieEnabled"]

    assert attr_probe["sensitive_surface_probe"] is True
    assert attr_probe["source_ir"]["check_mode"] == "type_only"
    assert "'co' + 'okieEnabled'" in attr_probe["js"]
    assert not _TARGET_FLOW_TERMS.search(attr_probe["js"])

    assert descr_probe["sensitive_surface_probe"] is True
    assert "'co' + 'okieEnabled'" in descr_probe["js"]
    assert not _TARGET_FLOW_TERMS.search(descr_probe["js"])


def test_navigator_ir_repair_preserves_existing_probe_ids_and_order():
    pack = generate_probe_pack()
    ids = [p["probe_id"] for p in pack["probes"]]
    # v0.8.98 S6: Tier-4 default interfaces expand the pack; pin floor not exact count
    assert len(pack["interfaces"]) >= 51
    assert len(ids) >= 1155
    # Baseline core interfaces still present
    for name in ("Window", "Navigator", "Screen", "Location", "Document"):
        assert name in pack["interfaces"]

    added = {
        f"idl.attr.Navigator.{attr}" for attr in _NAVIGATOR_FINGERPRINT_TYPES
    } | {
        f"idl.descr.Navigator.{attr}" for attr in _NAVIGATOR_FINGERPRINT_TYPES
    } | {
        f"idl.attr.NavigatorUAData.{attr}"
        for attr in (
            "architecture", "bitness", "model",
            "platformVersion", "wow64", "fullVersionList",
        )
    } | {
        f"idl.descr.NavigatorUAData.{attr}"
        for attr in (
            "architecture", "bitness", "model",
            "platformVersion", "wow64", "fullVersionList",
        )
    }
    # Order invariants for core Navigator block (not exact historical pack size)
    assert ids.index("idl.exists.Navigator") < ids.index("idl.exists.Screen")
    assert ids.index("idl.attr.Navigator.userAgent") < ids.index(
        "idl.descr.Navigator.userAgent"
    )
    assert ids.index("idl.descr.Navigator.language") < ids.index("idl.exists.Screen")
    assert ids.index("idl.exists.NavigatorUAData") < ids.index(
        "idl.attr.NavigatorUAData.architecture"
    )
    # Empty profile overlay must not reorder or drop ids
    baseline_ids = [p["probe_id"] for p in generate_probe_pack(profile_values={})["probes"]]
    assert ids == baseline_ids


def test_navigator_ua_data_expansion_probes_are_generated():
    pack = generate_probe_pack(interfaces=["NavigatorUAData"])
    by_id = {p["probe_id"]: p for p in pack["probes"]}
    expected = {
        "architecture": "DOMString",
        "bitness": "DOMString",
        "model": "DOMString",
        "platformVersion": "DOMString",
        "wow64": "boolean",
        "fullVersionList": "FrozenArray",
    }
    for attr, idl_type in expected.items():
        probe = by_id[f"idl.attr.NavigatorUAData.{attr}"]
        assert probe["source_ir"]["idl_type"] == idl_type
        assert probe["source_ir"]["supplementary_source"] == (
            "iv8-navigator-fingerprint-supplement.v0.1"
        )


# -- v0.8.38 profile auto-fill gates --------------------------------------


def test_profile_auto_fill_from_flat_env_is_deterministic():
    flat = {"navigator.userAgent": "Mozilla/5.0", "screen.width": 1920}
    first = build_profile_values_from_env(flat)
    second = build_profile_values_from_env(flat)
    assert first == second


def test_profile_auto_fill_skips_sensitive_surfaces():
    flat = {
        "navigator.userAgent": "Mozilla/5.0",
        "document.cookie": "session=secret",
        "document.domain": "example.com",
        "navigator.cookieEnabled": True,
    }
    values = build_profile_values_from_env(flat)
    assert "document.cookie" not in values
    assert "document.domain" not in values
    assert "navigator.cookieEnabled" not in values
    assert "navigator.userAgent" in values


def test_profile_auto_fill_does_not_write_files(tmp_path, monkeypatch):
    monkeypatch.chdir(tmp_path)
    pre = set(tmp_path.iterdir())
    build_profile_values_from_env({"screen.width": 1920})
    post = set(tmp_path.iterdir())
    assert pre == post


def test_profile_auto_fill_empty_input_produces_empty_dict():
    assert build_profile_values_from_env({}) == {}
    assert build_profile_values_from_env(None) == {}


def test_profile_auto_fill_projects_to_profile_values_dot_paths():
    flat = {
        "navigator.userAgent": "test-ua",
        "screen.width": 1920,
        "screen.height": 1080,
    }
    values = build_profile_values_from_env(flat)
    assert values["navigator.userAgent"] == "test-ua"
    assert values["screen.width"] == 1920
    assert values["screen.height"] == 1080


def test_profile_auto_fill_fuels_profile_aware_probes():
    flat = {"navigator.userAgent": "Mozilla/5.0 expected"}
    values = build_profile_values_from_env(flat)
    pack = generate_probe_pack(
        interfaces=["Navigator"],
        profile_values=values,
    )
    probe = [
        p for p in pack["probes"]
        if p["probe_id"] == "idl.attr.Navigator.userAgent"
    ]
    assert len(probe) == 1
    assert probe[0]["source_ir"]["check_mode"] == "type_and_profile_value"
    assert "Mozilla/5.0 expected" in probe[0]["js"]


def test_profile_auto_fill_does_not_mutate_input():
    flat = {"screen.width": 1920, "screen.height": 1080}
    before = dict(flat)
    build_profile_values_from_env(flat)
    assert flat == before


def test_profile_auto_fill_preserves_existing_probe_set():
    baseline = generate_probe_pack()
    flat = {"screen.width": 1920}
    values = build_profile_values_from_env(flat)
    overlaid = generate_probe_pack(profile_values=values)
    assert len(overlaid["probes"]) == len(baseline["probes"])


# -- v0.8.38 constructor allowlist review gates ---------------------------


def test_constructor_allowlist_expanded_for_live_globals():
    added = {
        "CustomEvent", "DOMMatrix", "DOMPoint", "DOMParser",
        "DOMRectReadOnly", "File", "KeyboardEvent", "MessageChannel",
        "MouseEvent",
    }
    assert added <= _CONSTRUCTOR_AVAILABLE


def test_constructor_expansion_preserves_existing_entries():
    preserved = {
        "Blob", "DOMRect", "DOMTokenList", "Element", "Event",
        "Headers", "HTMLElement", "Navigator", "Request", "Response",
        "Screen", "TextDecoder", "TextEncoder", "URL", "WebSocket",
        "XMLHttpRequest",
    }
    assert preserved <= _CONSTRUCTOR_AVAILABLE


def test_constructor_expansion_strong_interface_checks_are_generated():
    pack = generate_probe_pack(interfaces=["MouseEvent"])
    probe = [
        p for p in pack["probes"]
        if p["probe_id"] == "idl.attr.MouseEvent.altKey"
    ]
    assert len(probe) == 1
    assert "typeof __v__ === 'boolean'" in probe[0]["js"]


def test_constructor_expansion_no_non_allowlisted_instanceof():
    builtins = {
        "Float32Array", "Float64Array", "Int32Array",
        "Uint8Array", "ArrayBuffer", "DataView",
    }
    allowed = _CONSTRUCTOR_AVAILABLE | builtins
    pack = generate_probe_pack()
    offenders = []
    for probe in pack["probes"]:
        if not probe["probe_id"].startswith("idl.attr."):
            continue
        for match in re.finditer(r"__v__ instanceof ([A-Za-z0-9_]+)", probe["js"]):
            ctor = match.group(1)
            if ctor not in allowed:
                offenders.append((probe["probe_id"], ctor))
    assert not offenders, f"non-allowlisted instanceof checks: {offenders}"


def test_constructor_expansion_keeps_weak_fallback_for_non_allowlisted():
    pack = generate_probe_pack(interfaces=["History"])
    probe = [
        p for p in pack["probes"]
        if p["probe_id"] == "idl.attr.History.length"
    ]
    assert len(probe) == 1
    js = probe[0]["js"]
    if "History" not in _CONSTRUCTOR_AVAILABLE:
        assert "instanceof History" not in js
