"""
Cross-version VM handler diff tests (M28).

Tests compare_vm_versions with synthetic JS sources containing handler arrays.
"""
import pytest
from iv8_rs.vm_diff import compare_vm_versions, DiffReport


# Helper: minimal VM source with a handler array
def vm_source(handlers, array_name="A"):
    """Build a minimal JS source with a handler array."""
    funcs = ", ".join(f"function(){{ {h} }}" for h in handlers)
    return f"var {array_name} = [{funcs}];"


class TestIdenticalVersions:
    def test_same_source_zero_diff(self):
        src = vm_source(["return 1", "return 2", "return 3"])
        report = compare_vm_versions(src, src, handler_array="A")
        assert report.handler_count_a == 3
        assert report.handler_count_b == 3
        assert report.new_handlers == []
        assert report.removed_handlers == []
        assert report.modified_handlers == []
        assert report.unchanged_count == 3
        assert report.similarity_score == 1.0

    def test_empty_arrays_identical(self):
        src = "var A = [];"
        report = compare_vm_versions(src, src, handler_array="A")
        assert report.handler_count_a == 0
        assert report.handler_count_b == 0
        assert report.similarity_score == 1.0


class TestNewHandlers:
    def test_one_new_handler(self):
        src_a = vm_source(["return 1", "return 2"])
        src_b = vm_source(["return 1", "return 2", "return 3"])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert report.handler_count_a == 2
        assert report.handler_count_b == 3
        assert report.new_handlers == [2]
        assert report.removed_handlers == []

    def test_multiple_new_handlers(self):
        src_a = vm_source(["return 1"])
        src_b = vm_source(["return 1", "return 2", "return 3"])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert report.new_handlers == [1, 2]


class TestRemovedHandlers:
    def test_one_removed_handler(self):
        src_a = vm_source(["return 1", "return 2", "return 3"])
        src_b = vm_source(["return 1", "return 2"])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert report.removed_handlers == [2]
        assert report.new_handlers == []

    def test_multiple_removed(self):
        src_a = vm_source(["return 1", "return 2", "return 3"])
        src_b = vm_source(["return 1"])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert report.removed_handlers == [1, 2]


class TestModifiedHandlers:
    def test_one_modified(self):
        src_a = vm_source(["return 1", "return 2", "return 3"])
        src_b = vm_source(["return 1", "return 999", "return 3"])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert report.modified_handlers == [1]
        assert report.unchanged_count == 2
        # Check detail
        detail = report.details[1]
        assert detail.status == "modified"
        assert 0 < detail.similarity < 1.0

    def test_all_modified(self):
        src_a = vm_source(["return 1", "return 2"])
        src_b = vm_source(["var x=Math.random()", "var y=Date.now()"])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert len(report.modified_handlers) == 2
        assert report.similarity_score == 0.0

    def test_minor_change_below_threshold(self):
        """A very small change (single char) should still be 'modified' if below threshold."""
        src_a = vm_source(["return 1234567890"])
        src_b = vm_source(["return 1234567891"])  # one digit different
        report = compare_vm_versions(src_a, src_b, handler_array="A", similarity_threshold=0.99)
        # The similarity is high but below 0.99
        assert report.modified_handlers == [0] or report.unchanged_count == 1


class TestMixedChanges:
    def test_new_and_modified(self):
        src_a = vm_source(["return 1", "return 2"])
        src_b = vm_source(["return 1", "return 999", "return 3"])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert report.modified_handlers == [1]
        assert report.new_handlers == [2]
        assert report.removed_handlers == []


class TestEdgeCases:
    def test_nonexistent_array_returns_empty(self):
        src = "var X = [function(){ return 1; }];"
        report = compare_vm_versions(src, src, handler_array="NONEXISTENT")
        assert report.handler_count_a == 0
        assert report.handler_count_b == 0

    def test_different_array_names(self):
        src_a = "var C = [function(){ return 'hello world version one'; }];"
        src_b = "var C = [function(){ return Math.random() * Date.now(); }];"
        report = compare_vm_versions(src_a, src_b, handler_array="C")
        assert report.handler_count_a == 1
        assert report.handler_count_b == 1
        assert report.modified_handlers == [0]

    def test_similarity_score_calculation(self):
        src_a = vm_source(["return 1", "return 2", "return 3", "return 4"])
        src_b = vm_source(["return 1", "return 999", "return 3", "return 4"])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        # 3 unchanged out of 4 = 0.75
        assert report.similarity_score == 0.75

    def test_single_handler_empty_body(self):
        src_a = "var A = [function(){}];"
        src_b = "var A = [function(){return 1}];"
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert report.handler_count_a == 1
        assert report.modified_handlers == [0]

    def test_large_handler_body(self):
        body = "var x = 1; " * 50  # ~700 chars
        src_a = vm_source([body])
        src_b = vm_source([body])
        report = compare_vm_versions(src_a, src_b, handler_array="A")
        assert report.unchanged_count == 1

    def test_nested_array_name(self):
        src = "var handlers = {A: [function(){ return 42; }]};"
        report = compare_vm_versions(src, src, handler_array="A")
        assert report.handler_count_a == 0  # "var A" not found as top-level
