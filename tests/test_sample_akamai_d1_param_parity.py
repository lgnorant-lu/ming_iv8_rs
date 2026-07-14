"""Sample-track: Akamai d1 parameter parity (VM-013 algorithm_ref).

Placement: test_sample_* = instance/sample track, not design-layer product API.
Algorithm golden lives in docs/samples/adapters/akamai_dvc.py (no _ref required).
"""

from __future__ import annotations

import sys
import threading
from pathlib import Path

import pytest

_ADAPTERS = Path(__file__).resolve().parents[1] / "docs" / "samples" / "adapters"
if str(_ADAPTERS) not in sys.path:
    sys.path.insert(0, str(_ADAPTERS))

threading.stack_size(128 * 1024 * 1024)
iv8_rs = pytest.importorskip("iv8_rs")


def test_d1_iv8_matches_python_oracle_all_golden():
    import akamai_dvc

    result_box: dict = {}

    def work():
        ctx = iv8_rs.JSContext()
        akamai_dvc.install_d1(ctx)
        mismatches = []
        for delta, payload, count, total, expected in akamai_dvc.GOLDEN_CASES:
            iv8_val = akamai_dvc.d1_iv8(ctx, delta, payload, count, total)
            py_val = akamai_dvc.d1_oracle(delta, payload, count, total)
            if not (iv8_val == py_val == expected):
                mismatches.append((delta, payload, iv8_val, py_val, expected))
        result_box["mismatches"] = mismatches

    t = threading.Thread(target=work)
    t.start()
    t.join()
    assert result_box.get("mismatches") == [], result_box.get("mismatches")


def test_d1_parity_report_helper():
    import akamai_dvc

    result_box: dict = {}

    def work():
        result_box["rep"] = akamai_dvc.parity_report()

    t = threading.Thread(target=work)
    t.start()
    t.join()
    rep = result_box["rep"]
    assert rep["all_match"] is True
    assert len(rep["cases"]) >= 4
