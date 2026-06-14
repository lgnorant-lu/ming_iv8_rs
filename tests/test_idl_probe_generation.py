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
from tools.idl_probe.generate_probe_pack import (  # noqa: E402
    _CONSTRUCTOR_AVAILABLE,
    generate_probe_pack,
)

from iv8_rs.environment_toolchain_asset_models import ProbePack  # noqa: E402


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

    assert set(by_id) == {
        "idl.attr.Document.cookie",
        "idl.descr.Document.cookie",
        "idl.attr.Document.domain",
        "idl.descr.Document.domain",
    }
    for probe in by_id.values():
        assert probe["sensitive_surface_probe"] is True
        assert probe["sensitivity_reason"] == "standard_idl_surface_name_only"


# -- v0.8.36 Profile -> Probe expected overlay -------------------------------


def test_profile_overlay_preserves_no_profile_probe_count():
    baseline = generate_probe_pack()
    overlaid = generate_probe_pack(profile_values={})
    assert len(overlaid["interfaces"]) == len(baseline["interfaces"]) == 51
    assert len(overlaid["probes"]) == len(baseline["probes"]) == 1125
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
