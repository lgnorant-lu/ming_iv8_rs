"""v0.8.33 Slice 1 -- IDL probe compiler tests.

Verify:
- Deterministic output for same IR and interface subset.
- Output passes ProbePack.from_dict schema validation.
- Evidence ceiling is diagnostic_only.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).parent.parent))
from tools.idl_probe.generate_probe_pack import generate_probe_pack  # noqa: E402

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

_NEW_TYPE_ATTRS = (
    "onorientationchange",    # EventHandler
    "orientation",            # CSSOMString
    "scrollX",                # unsigned long long
    "credentialless",         # any
)


def test_type_dict_expansion_catches_new_navigator_types():
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
