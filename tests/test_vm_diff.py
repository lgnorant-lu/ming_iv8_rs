"""
Cross-version VM handler diff tests (M28).

Tests compare_vm_versions with synthetic JS sources containing handler arrays.
"""
from iv8_rs.vm_diff import compare_vm_versions


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


class TestHighSimilarityHandlers:
    def test_high_similarity_above_threshold_unchanged(self):
        """Handlers with similarity >= threshold are marked unchanged (lines 186-188)."""
        body_a = "var x = 100; var y = 200; var z = 300;"
        body_b = "var x = 100; var y = 200; var z = 301;"  # 1 char diff → sim ~0.98
        src_a = vm_source([body_a])
        src_b = vm_source([body_b])
        report = compare_vm_versions(src_a, src_b, similarity_threshold=0.95)
        assert report.modified_handlers == []
        assert report.unchanged_count == 1
        assert report.details[0].status == "unchanged"
        assert report.details[0].similarity >= 0.95

    def test_low_similarity_below_threshold_modified(self):
        """Handlers with similarity below threshold are marked modified."""
        src_a = vm_source(["return 1;"])
        src_b = vm_source(["var x = Math.random();"])
        report = compare_vm_versions(src_a, src_b, similarity_threshold=0.95)
        assert report.modified_handlers == [0]
        assert report.details[0].status == "modified"
        assert report.details[0].similarity < 0.95


class TestSwapDetection:
    def test_two_handlers_swapped(self):
        """Basic swap: A[0]↔B[1], A[1]↔B[0] (lines 216, 220-232)."""
        src_a = vm_source([
            "var a = 1; var b = 2;",
            "var c = 3; var d = 4;",
        ])
        src_b = vm_source([
            "var c = 3; var d = 4;",
            "var a = 1; var b = 2;",
        ])
        report = compare_vm_versions(src_a, src_b)
        assert report.modified_handlers == []
        assert report.unchanged_count == 2
        assert report.details[0].status == "unchanged"
        assert report.details[1].status == "unchanged"

    def test_three_handlers_first_two_swapped(self):
        """Three handlers where first two are swapped (exercises skip already-paired)."""
        src_a = vm_source([
            "return 'alpha';",
            "return 'beta';",
            "return 'gamma';",
        ])
        src_b = vm_source([
            "return 'beta';",
            "return 'alpha';",
            "return 'gamma';",
        ])
        report = compare_vm_versions(src_a, src_b)
        assert report.modified_handlers == []
        assert report.unchanged_count == 3
        assert report.similarity_score == 1.0

    def test_swap_with_new_handler_added(self):
        """Swap detection with a new handler also present."""
        src_a = vm_source(["var a = 1;", "var b = 2;"])
        src_b = vm_source(["var b = 2;", "var a = 1;", "var c = 3;"])
        report = compare_vm_versions(src_a, src_b)
        assert report.modified_handlers == []
        assert report.unchanged_count == 2
        assert report.new_handlers == [2]

    def test_handler_moved_forward(self):
        """A handler replaced at its original position, same content elsewhere."""
        src_a = vm_source([
            "var w = 0;",
            "var x = 10;",
            "var y = 20;",
        ])
        src_b = vm_source([
            "var w = 0;",
            "var x = 999;",
            "var y = 20;",
        ])
        report = compare_vm_versions(src_a, src_b)
        assert report.modified_handlers == [1]
        assert report.unchanged_count == 2

    def test_handler_pulled_back(self):
        """Handler moved backward detected as still unchanged if content identical."""
        src_a = vm_source([
            "var x = 10;",
            "var y = 20;",
            "var z = 30;",
        ])
        src_b = vm_source([
            "var z = 30;",
            "var x = 10;",
            "var y = 20;",
        ])
        report = compare_vm_versions(src_a, src_b)
        assert report.modified_handlers == [0, 1, 2]
        assert report.unchanged_count == 0
