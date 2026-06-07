from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "tests" / "fixtures" / "v0.8"


def load_fixture(family: str, name: str = "schema.json") -> dict:
    with (FIXTURES / family / name).open("r", encoding="utf-8") as fh:
        return json.load(fh)


def assert_fields(data: dict, fields: list[str]) -> None:
    missing = [field for field in fields if field not in data]
    assert not missing, f"missing fields: {missing}"


def assert_no_strong_evidence(records: list[dict]) -> None:
    overclaims = [record for record in records if record.get("strength") == "strong"]
    assert not overclaims, f"unexpected strong evidence: {overclaims}"


def assert_diagnostic(data: dict, code: str) -> None:
    codes = {item.get("code") for item in data.get("diagnostics", [])}
    assert code in codes
