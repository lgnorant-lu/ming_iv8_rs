"""Module Isolation tests (M29)."""
import pytest
import iv8_rs
from iv8_rs.isolation import exec_vm_handler


@pytest.fixture
def vm_ctx():
    """JSContext with a simple VM initialized."""
    ctx = iv8_rs.JSContext()
    ctx.eval("""
        var A = [
            function(){ g.push(42); },                    // 0: push 42
            function(){ g.push(g.pop() + g.pop()); },    // 1: add top two
            function(){ g.push(g.pop() * 2); },          // 2: double top
            function(){ return g.pop(); },               // 3: pop and return
            function(){ throw new Error('boom'); },      // 4: throws
            function(){ g.push(screen.width); },         // 5: reads env
        ];
        var g = [];
        var U = 0;
    """)
    yield ctx
    ctx.close()


class TestExecVmHandler:
    def test_push_handler(self, vm_ctx):
        result = exec_vm_handler(vm_ctx, "A[0]", stack_var="g", stack_input=[])
        assert result["error"] is None
        assert result["stack_pushed"] == [42]
        assert result["stack_after"] == [42]

    def test_add_handler(self, vm_ctx):
        result = exec_vm_handler(vm_ctx, "A[1]", stack_var="g", stack_input=[10, 20])
        assert result["error"] is None
        assert result["stack_after"] == [30]  # 10+20
        assert result["stack_popped"] == 2
        assert result["stack_pushed"] == [30]

    def test_double_handler(self, vm_ctx):
        result = exec_vm_handler(vm_ctx, "A[2]", stack_var="g", stack_input=[7])
        assert result["error"] is None
        assert result["stack_after"] == [14]

    def test_return_value(self, vm_ctx):
        result = exec_vm_handler(vm_ctx, "A[3]", stack_var="g", stack_input=[99])
        assert result["return_value"] == 99
        assert result["stack_after"] == []

    def test_handler_throws(self, vm_ctx):
        result = exec_vm_handler(vm_ctx, "A[4]", stack_var="g", stack_input=[])
        assert result["error"] is not None
        assert "boom" in result["error"]

    def test_stack_preserved_on_error(self, vm_ctx):
        result = exec_vm_handler(vm_ctx, "A[4]", stack_var="g", stack_input=[1, 2, 3])
        # Stack should be unchanged after error (handler didn't modify it)
        assert result["stack_before"] == [1, 2, 3]

    def test_pc_setting(self, vm_ctx):
        result = exec_vm_handler(vm_ctx, "A[0]", stack_var="g",
                                 stack_input=[], pc_var="U", pc_value=42)
        assert result["error"] is None
        # Verify PC was set
        assert vm_ctx.eval("U") == 42

    def test_mock_env(self, vm_ctx):
        vm_ctx.eval("var screen = {width: 1920};")
        result = exec_vm_handler(vm_ctx, "A[5]", stack_var="g",
                                 stack_input=[],
                                 mock_env={"screen.width": 2560})
        assert result["error"] is None
        assert result["stack_pushed"] == [2560]
        # Verify env restored
        assert vm_ctx.eval("screen.width") == 1920

    def test_empty_stack_input(self, vm_ctx):
        result = exec_vm_handler(vm_ctx, "A[0]", stack_var="g", stack_input=[])
        assert result["stack_before"] == []
        assert result["stack_after"] == [42]

    def test_no_stack_input_preserves_existing(self, vm_ctx):
        vm_ctx.eval("g = [100, 200]")
        result = exec_vm_handler(vm_ctx, "A[0]", stack_var="g")
        # stack_input=None means don't reset stack
        assert result["stack_before"] == [100, 200]
        assert result["stack_after"] == [100, 200, 42]
