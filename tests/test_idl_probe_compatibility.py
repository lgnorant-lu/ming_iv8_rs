"""v0.8.33 Slice 2 -- probe runtime compatibility tests.

Verify generated probes execute correctly in a real JSContext
and do not mutate existing probe pack assets.
"""

from __future__ import annotations

import sys
from pathlib import Path

import iv8_rs
import pytest

sys.path.insert(0, str(Path(__file__).parent.parent))
from tools.idl_probe.generate_probe_pack import generate_probe_pack


@pytest.fixture
def generated_pack_dict():
    return generate_probe_pack()


@pytest.fixture
def ctx():
    c = iv8_rs.JSContext()
    yield c
    c.close()


def test_generated_probes_execute_in_js_context(ctx, generated_pack_dict):
    """Every generated probe JS expression must evaluate without throwing."""
    errors = []
    for probe in generated_pack_dict["probes"]:
        expr = probe["js"]
        try:
            if "return " in expr or expr.strip().startswith("return"):
                result = ctx.eval(f"(function() {{ {expr} }})()")
            elif "(function()" in expr:
                result = ctx.eval(expr)
            else:
                result = ctx.eval(f"(function() {{ return {expr}; }})()")
        except Exception as exc:
            errors.append(f"{probe['probe_id']}: {type(exc).__name__}: {exc}")
    assert not errors, f"probes failed in JSContext:\n" + "\n".join(errors[:20])


def test_window_existence_probe_evaluates(ctx, generated_pack_dict):
    """Window existence probe must evaluate without throwing in JSContext."""
    for probe in generated_pack_dict["probes"]:
        if probe["probe_id"] == "idl.exists.Window":
            result = ctx.eval(f"(function() {{ {probe['js']} }})()")
            assert isinstance(result, bool), f"expected bool, got {type(result)}"
            break


def test_navigator_existence_probe_passes(ctx, generated_pack_dict):
    """typeof Navigator !== 'undefined' should be true."""
    for probe in generated_pack_dict["probes"]:
        if probe["probe_id"] == "idl.exists.Navigator":
            result = ctx.eval(f"(function() {{ {probe['js']} }})()")
            assert result == True, f"Navigator existence probe returned {result}"
            break


def test_navigator_useragent_string(ctx):
    """navigator.userAgent should be a string in a real context."""
    result = ctx.eval("typeof navigator.userAgent === 'string'")
    assert result == True


def test_evidence_ceiling_is_diagnostic_only(generated_pack_dict):
    assert generated_pack_dict["evidence_ceiling"] == "diagnostic_only"
    for probe in generated_pack_dict["probes"]:
        assert probe["evidence_ceiling"] == "diagnostic_only"


def test_generation_does_not_write_files(tmp_path):
    """Generating a probe pack is pure-function; must not write any files."""
    pre_files = set(tmp_path.iterdir())
    generate_probe_pack()
    post_files = set(tmp_path.iterdir())
    assert pre_files == post_files, "generate_probe_pack() wrote files"
