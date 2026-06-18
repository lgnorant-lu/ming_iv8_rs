from __future__ import annotations

import json
import os
import tempfile
from typing import List
from unittest.mock import patch

import pytest

from iv8_rs.trace import (
    CompressedEntry,
    CompressedTrace,
    StructuredTrace,
    TraceEntry,
    _parse_entry,
    compress_trace,
    parse_trace,
    parse_trace_stream,
)


# ─── TraceEntry ─────────────────────────────────────────────────────────────────


class TestTraceEntry:
    def test_construct(self) -> None:
        e = TraceEntry(type="D", pc=42, target="5", value="3", raw="D,42,5,3")
        assert e.type == "D"
        assert e.pc == 42
        assert e.target == "5"
        assert e.value == "3"
        assert e.raw == "D,42,5,3"

    def test_is_dispatch(self) -> None:
        e = TraceEntry(type="D", pc=42, target="5", value="3", raw="D,42,5,3")
        assert e.is_dispatch is True
        assert e.is_read is False
        assert e.is_call is False
        assert e.is_write is False

    def test_is_read(self) -> None:
        e = TraceEntry(type="R", pc=100, target="x", value="1", raw="R,100,x,1")
        assert e.is_dispatch is False
        assert e.is_read is True
        assert e.is_call is False
        assert e.is_write is False

    def test_is_call(self) -> None:
        e = TraceEntry(type="C", pc=200, target="fn", value="ok", raw="C,200,fn,ok")
        assert e.is_dispatch is False
        assert e.is_read is False
        assert e.is_call is True
        assert e.is_write is False

    def test_is_write(self) -> None:
        e = TraceEntry(type="W", pc=300, target="y", value="42", raw="W,300,y,42")
        assert e.is_dispatch is False
        assert e.is_read is False
        assert e.is_call is False
        assert e.is_write is True


# ─── _parse_entry ───────────────────────────────────────────────────────────────


class TestParseEntry:
    def test_empty_string(self) -> None:
        assert _parse_entry("") is None

    def test_too_short(self) -> None:
        assert _parse_entry("ab") is None

    def test_less_than_3_parts(self) -> None:
        assert _parse_entry("D,1") is None

    def test_invalid_type(self) -> None:
        assert _parse_entry("X,1,2") is None

    def test_three_field_dispatch(self) -> None:
        e = _parse_entry("D,5,3")
        assert e is not None
        assert e.type == "D"
        assert e.pc == -1
        assert e.target == "5"
        assert e.value == "3"
        assert e.raw == "D,5,3"

    def test_three_field_read(self) -> None:
        e = _parse_entry("R,screen.width,1920")
        assert e is not None
        assert e.type == "R"
        assert e.pc == -1
        assert e.target == "screen.width"
        assert e.value == "1920"

    def test_four_field(self) -> None:
        e = _parse_entry("D,42,15,3")
        assert e is not None
        assert e.type == "D"
        assert e.pc == 42
        assert e.target == "15"
        assert e.value == "3"

    def test_non_integer_pc_falls_back_to_neg1(self) -> None:
        e = _parse_entry("D,abc,15,3")
        assert e is not None
        assert e.pc == -1
        assert e.target == "15"
        assert e.value == "3"

    def test_read_with_pc(self) -> None:
        e = _parse_entry("R,100,navigator.userAgent,Mozilla/5.0")
        assert e is not None
        assert e.type == "R"
        assert e.pc == 100
        assert e.target == "navigator.userAgent"
        assert e.value == "Mozilla/5.0"

    def test_write_with_pc(self) -> None:
        e = _parse_entry("W,50,x,hello")
        assert e is not None
        assert e.type == "W"
        assert e.pc == 50
        assert e.target == "x"
        assert e.value == "hello"

    def test_call_with_pc(self) -> None:
        e = _parse_entry("C,10,alert,undefined")
        assert e is not None
        assert e.type == "C"
        assert e.pc == 10
        assert e.target == "alert"
        assert e.value == "undefined"


# ─── StructuredTrace ────────────────────────────────────────────────────────────


class TestStructuredTrace:
    def test_empty(self) -> None:
        t = StructuredTrace([])
        assert len(t) == 0

    def test_len(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("R", 1, "x", "1", "R,1,x,1"),
        ])
        assert len(t) == 2

    def test_iter(self) -> None:
        entries = [
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("R", 1, "x", "1", "R,1,x,1"),
        ]
        t = StructuredTrace(entries)
        assert list(t) == entries

    def test_getitem_index(self) -> None:
        entries = [
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("R", 1, "x", "1", "R,1,x,1"),
        ]
        t = StructuredTrace(entries)
        assert t[0] == entries[0]
        assert t[1] == entries[1]

    def test_getitem_slice(self) -> None:
        entries = [
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("R", 1, "x", "1", "R,1,x,1"),
            TraceEntry("C", 2, "f", "v", "C,2,f,v"),
        ]
        t = StructuredTrace(entries)
        sliced = t[1:3]
        assert isinstance(sliced, StructuredTrace)
        assert len(sliced) == 2
        assert sliced[0] == entries[1]
        assert sliced[1] == entries[2]

    def test_dispatches(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("R", 1, "x", "1", "R,1,x,1"),
            TraceEntry("D", 2, "3", "1", "D,2,3,1"),
        ])
        assert len(t.dispatches) == 2
        assert all(e.type == "D" for e in t.dispatches)

    def test_reads(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("R", 1, "x", "1", "R,1,x,1"),
            TraceEntry("R", 2, "y", "2", "R,2,y,2"),
        ])
        assert len(t.reads) == 2
        assert all(e.type == "R" for e in t.reads)

    def test_calls(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("C", 1, "fn", "v", "C,1,fn,v"),
        ])
        assert len(t.calls) == 1
        assert t.calls[0].target == "fn"

    def test_writes(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("W", 1, "x", "42", "W,1,x,42"),
            TraceEntry("W", 2, "y", "43", "W,2,y,43"),
        ])
        assert len(t.writes) == 2
        assert all(e.type == "W" for e in t.writes)

    # --- filter ---

    def test_filter_type(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("R", 1, "x", "1", "R,1,x,1"),
            TraceEntry("C", 2, "f", "v", "C,2,f,v"),
            TraceEntry("W", 3, "y", "2", "W,3,y,2"),
        ])
        filtered = t.filter(type="D")
        assert len(filtered) == 1
        assert filtered[0].type == "D"

    def test_filter_target(self) -> None:
        t = StructuredTrace([
            TraceEntry("R", 1, "screen.width", "1920", ""),
            TraceEntry("R", 2, "screen.height", "1080", ""),
            TraceEntry("W", 3, "x", "1", ""),
        ])
        filtered = t.filter(target="screen")
        assert len(filtered) == 2
        assert all("screen" in e.target for e in filtered)

    def test_filter_pc_range(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "1", "0", ""),
            TraceEntry("D", 20, "2", "1", ""),
            TraceEntry("D", 30, "3", "2", ""),
            TraceEntry("R", 40, "x", "1", ""),
        ])
        filtered = t.filter(pc_range=(15, 35))
        assert len(filtered) == 2
        assert filtered[0].pc == 20
        assert filtered[1].pc == 30

    def test_filter_combined(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "1", "0", ""),
            TraceEntry("D", 20, "2", "1", ""),
            TraceEntry("R", 20, "x", "1", ""),
            TraceEntry("D", 30, "3", "2", ""),
        ])
        filtered = t.filter(type="D", pc_range=(15, 35))
        assert len(filtered) == 2
        assert filtered[0].pc == 20
        assert filtered[1].pc == 30

    def test_filter_no_args_returns_copy(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", ""),
        ])
        filtered = t.filter()
        assert len(filtered) == 1

    def test_between(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "1", "0", ""),
            TraceEntry("D", 20, "2", "1", ""),
            TraceEntry("D", 30, "3", "2", ""),
            TraceEntry("R", 40, "x", "1", ""),
        ])
        sliced = t.between(20, 40)
        assert len(sliced) == 3
        assert sliced[0].pc == 20
        assert sliced[1].pc == 30
        assert sliced[2].pc == 40

    # --- summary ---

    def test_summary_empty(self) -> None:
        t = StructuredTrace([])
        s = t.summary()
        assert s == {"total": 0, "counts_by_type": {}, "pc_range": None, "unique_targets": 0, "unique_opcodes": 0}

    def test_summary_with_entries(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 20, "3", "1", ""),
            TraceEntry("R", 10, "x", "value", ""),
            TraceEntry("W", 30, "y", "42", ""),
            TraceEntry("C", -1, "fn", "ok", ""),
        ])
        s = t.summary()
        assert s["total"] == 5
        assert s["counts_by_type"]["D"] == 2
        assert s["counts_by_type"]["R"] == 1
        assert s["counts_by_type"]["W"] == 1
        assert s["counts_by_type"]["C"] == 1
        assert s["pc_range"] == (10, 30)
        assert s["unique_targets"] == 3  # x, y, fn (D entries excluded)
        assert s["unique_opcodes"] == 2  # 5, 3

    def test_summary_negative_pc_ignored_in_range(self) -> None:
        t = StructuredTrace([
            TraceEntry("R", -1, "x", "1", ""),
            TraceEntry("R", 50, "y", "2", ""),
        ])
        s = t.summary()
        assert s["pc_range"] == (50, 50)

    def test_summary_no_d_entries(self) -> None:
        t = StructuredTrace([
            TraceEntry("R", 10, "x", "1", ""),
        ])
        s = t.summary()
        assert s["unique_opcodes"] == 0

    # --- to_jsonl ---

    def test_to_jsonl(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", "D,1,2,0"),
            TraceEntry("R", 2, "x", "42", "R,2,x,42"),
        ])
        with tempfile.NamedTemporaryFile(mode="r+", encoding="utf-8", delete=False) as f:
            path = f.name
        try:
            t.to_jsonl(path)
            with open(path, encoding="utf-8") as f:
                lines = f.readlines()
            assert len(lines) == 2
            obj0 = json.loads(lines[0])
            assert obj0 == {"type": "D", "pc": 1, "target": "2", "value": "0"}
            obj1 = json.loads(lines[1])
            assert obj1 == {"type": "R", "pc": 2, "target": "x", "value": "42"}
        finally:
            os.unlink(path)

    # --- to_dataframe ---

    def test_to_dataframe(self) -> None:
        pd = pytest.importorskip("pandas")
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", ""),
            TraceEntry("R", 2, "x", "42", ""),
        ])
        df = t.to_dataframe()
        assert len(df) == 2
        assert list(df.columns) == ["type", "pc", "target", "value"]
        assert df.iloc[0]["type"] == "D"
        assert df.iloc[0]["pc"] == 1
        assert df.iloc[1]["value"] == "42"

    def test_to_dataframe_empty(self) -> None:
        pd = pytest.importorskip("pandas")
        t = StructuredTrace([])
        df = t.to_dataframe()
        assert len(df) == 0

    def test_to_dataframe_import_error(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", ""),
        ])
        orig_import = __import__

        def mock_import(name, *args, **kwargs):
            if name == "pandas":
                raise ImportError("No module named pandas")
            return orig_import(name, *args, **kwargs)

        with patch("builtins.__import__", side_effect=mock_import):
            with pytest.raises(ImportError, match="pandas is required"):
                t.to_dataframe()

    # --- sequence extraction ---

    def test_pc_sequence(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 20, "3", "1", ""),
            TraceEntry("R", 30, "x", "1", ""),
        ])
        assert t.pc_sequence() == [10, 20]

    def test_value_sequence(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 20, "3", "1", ""),
            TraceEntry("R", 30, "x", "", ""),
        ])
        assert t.value_sequence() == ["0", "1"]

    def test_unique_pcs(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 20, "3", "1", ""),
            TraceEntry("D", 10, "5", "2", ""),
            TraceEntry("R", 30, "x", "1", ""),
        ])
        assert t.unique_pcs() == {10, 20}

    def test_unique_pcs_filters_negative(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", -1, "5", "0", ""),
        ])
        assert t.unique_pcs() == set()

    def test_index_by_pc(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 20, "3", "1", ""),
            TraceEntry("R", 10, "x", "1", ""),
            TraceEntry("W", -1, "g", "0", ""),
        ])
        idx = t.index_by_pc()
        assert 10 in idx
        assert 20 in idx
        assert -1 not in idx
        assert len(idx[10]) == 2
        assert len(idx[20]) == 1

    def test_index_by_target(self) -> None:
        t = StructuredTrace([
            TraceEntry("R", 10, "x", "1", ""),
            TraceEntry("W", 20, "x", "2", ""),
            TraceEntry("R", 30, "y", "3", ""),
            TraceEntry("D", 40, "", "0", ""),
        ])
        idx = t.index_by_target()
        assert "x" in idx
        assert "y" in idx
        assert "" not in idx
        assert len(idx["x"]) == 2
        assert len(idx["y"]) == 1

    # --- repr ---

    def test_repr(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 1, "2", "0", ""),
            TraceEntry("R", 1, "x", "1", ""),
        ])
        r = repr(t)
        assert "StructuredTrace" in r
        assert "2 entries" in r
        assert "D=1" in r
        assert "R=1" in r

    def test_repr_empty(self) -> None:
        t = StructuredTrace([])
        r = repr(t)
        assert "0 entries" in r


# ─── parse_trace ────────────────────────────────────────────────────────────────


class TestParseTrace:
    def test_empty_list(self) -> None:
        t = parse_trace([])
        assert isinstance(t, StructuredTrace)
        assert len(t) == 0

    def test_mixed_valid_and_invalid(self) -> None:
        raw = [
            "D,1,2,0",
            "invalid",
            "R,100,screen.width,1920",
            "",
            "W,50,x,hello",
            "ab",
        ]
        t = parse_trace(raw)
        assert len(t) == 3
        assert t[0].type == "D"
        assert t[1].type == "R"
        assert t[2].type == "W"

    def test_all_valid_four_field(self) -> None:
        raw = [
            "D,10,5,0",
            "C,20,toString,function",
            "R,30,location.href,http://x",
        ]
        t = parse_trace(raw)
        assert len(t) == 3
        assert t[0].pc == 10
        assert t[1].target == "toString"
        assert t[2].value == "http://x"

    def test_three_field_format(self) -> None:
        raw = [
            "D,5,3",
            "R,navigator.userAgent,Mozilla",
            "C,fn,undefined",
            "W,x,42",
        ]
        t = parse_trace(raw)
        assert len(t) == 4
        assert all(e.pc == -1 for e in t)
        assert t[0].target == "5"
        assert t[1].target == "navigator.userAgent"

    def test_unicode_in_values(self) -> None:
        raw = [
            "R,100,unicode,\u4e2d\u6587",
            "D,1,2,\u03b1",
        ]
        t = parse_trace(raw)
        assert len(t) == 2
        assert t[0].value == "\u4e2d\u6587"
        assert t[1].value == "\u03b1"

    def test_hex_values(self) -> None:
        raw = [
            "D,1,2,0xff",
            "R,10,prop,0xABCD",
        ]
        t = parse_trace(raw)
        assert len(t) == 2
        assert t[0].value == "0xff"
        assert t[1].value == "0xABCD"


# ─── parse_trace_stream ─────────────────────────────────────────────────────────


class TestParseTraceStream:
    def test_from_list(self) -> None:
        t = parse_trace_stream(["D,1,2,0", "R,10,x,1"])
        assert len(t) == 2

    def test_from_generator(self) -> None:
        def gen():
            yield "D,1,2,0"
            yield "R,10,x,1"

        t = parse_trace_stream(gen())
        assert len(t) == 2

    def test_empty(self) -> None:
        t = parse_trace_stream([])
        assert len(t) == 0

    def test_bytes_lines(self) -> None:
        data = [b"D,1,2,0", b"R,10,x,42"]
        t = parse_trace_stream(data)
        assert len(t) == 2
        assert t[0].type == "D"
        assert t[1].value == "42"

    def test_mixed_bytes_and_str(self) -> None:
        data = [b"D,1,2,0", "R,10,x,1"]
        t = parse_trace_stream(data)
        assert len(t) == 2

    def test_with_newlines(self) -> None:
        data = ["D,1,2,0\n", "R,10,x,1\r\n"]
        t = parse_trace_stream(data)
        assert len(t) == 2
        assert t[0].value == "0"

    def test_invalid_lines_skipped(self) -> None:
        data = ["D,1,2,0", "", "bad", "X,1,2"]
        t = parse_trace_stream(data)
        assert len(t) == 1


# ─── CompressedTrace ────────────────────────────────────────────────────────────


class TestCompressedTrace:
    def test_empty(self) -> None:
        ct = CompressedTrace([])
        assert len(ct) == 0

    def test_len(self) -> None:
        ct = CompressedTrace([
            CompressedEntry("D", 10, "5", "0", 3),
            CompressedEntry("D", 20, "3", "1", 2),
        ])
        assert len(ct) == 2

    def test_total_dispatches(self) -> None:
        ct = CompressedTrace([
            CompressedEntry("D", 10, "5", "0", 3),
            CompressedEntry("R", 10, "x", "1", 1),
        ])
        assert ct.total_dispatches == 3

    def test_total_dispatches_no_d(self) -> None:
        ct = CompressedTrace([
            CompressedEntry("R", 10, "x", "1", 1),
        ])
        assert ct.total_dispatches == 0

    def test_compression_ratio(self) -> None:
        ct = CompressedTrace([
            CompressedEntry("D", 10, "5", "0", 3),
            CompressedEntry("D", 20, "3", "1", 2),
        ])
        ratio = ct.compression_ratio
        assert ratio == pytest.approx(2 / 5)

    def test_compression_ratio_empty(self) -> None:
        ct = CompressedTrace([])
        assert ct.compression_ratio == 1.0

    def test_expand(self) -> None:
        ct = CompressedTrace([
            CompressedEntry("D", 10, "5", "0", 2),
            CompressedEntry("R", 20, "x", "1", 1),
        ])
        st = ct.expand()
        assert isinstance(st, StructuredTrace)
        assert len(st) == 3
        assert st[0].type == "D"
        assert st[0].pc == 10
        assert st[1].type == "D"
        assert st[1].pc == 10
        assert st[2].type == "R"
        assert st[2].pc == 20
        assert st[2].value == "1"

    def test_repr(self) -> None:
        ct = CompressedTrace([
            CompressedEntry("D", 10, "5", "0", 3),
        ])
        r = repr(ct)
        assert "CompressedTrace" in r
        assert "1 entries" in r
        assert "ratio=" in r


# ─── compress_trace ─────────────────────────────────────────────────────────────


class TestCompressTrace:
    def test_empty(self) -> None:
        t = StructuredTrace([])
        ct = compress_trace(t)
        assert isinstance(ct, CompressedTrace)
        assert len(ct) == 0

    def test_single_entry(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
        ])
        ct = compress_trace(t)
        assert len(ct) == 1
        assert ct.entries[0].count == 1

    def test_merge_consecutive_same_dispatch(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 10, "5", "0", ""),
        ])
        ct = compress_trace(t)
        assert len(ct) == 1
        assert ct.entries[0].count == 3
        assert ct.entries[0].pc == 10

    def test_no_merge_different_pc(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 20, "5", "0", ""),
        ])
        ct = compress_trace(t)
        assert len(ct) == 2
        assert ct.entries[0].count == 1
        assert ct.entries[1].count == 1

    def test_no_merge_different_target(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 10, "6", "0", ""),
        ])
        ct = compress_trace(t)
        assert len(ct) == 2

    def test_no_merge_different_value_bug18(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 10, "5", "1", ""),
        ])
        ct = compress_trace(t)
        assert len(ct) == 2

    def test_non_dispatch_entries_kept_with_count1(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("R", 10, "x", "1", ""),
            TraceEntry("R", 10, "x", "1", ""),
            TraceEntry("W", 20, "y", "2", ""),
        ])
        ct = compress_trace(t)
        # R entries are adjacent with same values but are not D so not merged
        assert len(ct) == 4
        assert all(e.count == 1 for e in ct.entries)

    def test_mixed_merge_and_preserve(self) -> None:
        t = StructuredTrace([
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("R", 20, "x", "1", ""),
            TraceEntry("D", 10, "5", "0", ""),
            TraceEntry("D", 10, "5", "0", ""),
        ])
        ct = compress_trace(t)
        assert len(ct) == 3
        assert ct.entries[0].count == 2  # first two D merged
        assert ct.entries[1].count == 1  # R
        assert ct.entries[2].count == 2  # last two D merged


# ─── has_instrumentation ────────────────────────────────────────────────────────



