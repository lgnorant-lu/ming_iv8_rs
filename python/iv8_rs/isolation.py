"""
Module Isolation: execute a single VM handler in controlled conditions (M29).

Allows running one handler at a time with specified stack input and
optional environment mocking, then reading the stack output.

Usage::

    from iv8_rs.isolation import exec_vm_handler

    ctx = iv8_rs.JSContext(...)
    ctx.eval(tdc_js)  # VM initialized

    result = exec_vm_handler(ctx, handler_expr="A[15]", stack_var="g",
                             stack_input=[1920, 1080])
    print(result["stack_output"])  # values pushed by handler
"""

from __future__ import annotations
from typing import Any, Dict, List, Optional
import json


def exec_vm_handler(
    ctx,
    handler_expr: str,
    stack_var: str = "g",
    stack_input: Optional[List[Any]] = None,
    pc_var: Optional[str] = None,
    pc_value: Optional[int] = None,
    mock_env: Optional[Dict[str, Any]] = None,
) -> Dict[str, Any]:
    """Execute a single VM handler in isolation and return stack changes.

    Sets up the VM stack with specified input, optionally sets the PC,
    optionally mocks environment values, calls the handler, then reads
    the resulting stack state.

    Args:
        ctx: An active JSContext (with VM already initialized via eval).
        handler_expr: JS expression that evaluates to the handler function
                      to call, e.g. "A[15]" or "C[D[42]]".
        stack_var: Name of the VM stack variable (default "g").
        stack_input: Values to set as the stack before calling the handler.
                     If None, stack is left as-is.
        pc_var: Optional PC variable name to set before calling.
        pc_value: Value to set the PC to (only used if pc_var is set).
        mock_env: Optional dict of environment overrides to inject before
                  calling (e.g. {"screen.width": 1920}). Restored after.

    Returns:
        Dict with:
        - stack_before: list of stack values before handler call
        - stack_after: list of stack values after handler call
        - stack_pushed: values added to stack (stack_after[len(stack_before):])
        - stack_popped: number of values consumed (len(before) - common prefix)
        - return_value: handler's return value (if any)
        - error: None if success, error message string if handler threw

    Example::

        result = exec_vm_handler(ctx, "A[15]", stack_var="g",
                                 stack_input=[1920, 1080, 24])
        print(result["stack_pushed"])  # what the handler produced
    """
    # Setup: set stack
    if stack_input is not None:
        stack_json = json.dumps(stack_input)
        ctx.eval(f"{stack_var} = {stack_json};")

    # Setup: set PC
    if pc_var and pc_value is not None:
        ctx.eval(f"{pc_var} = {pc_value};")

    # Setup: mock environment
    restore_code = ""
    if mock_env:
        for path, value in mock_env.items():
            parts = path.split(".")
            if len(parts) == 2:
                obj, prop = parts
                val_json = json.dumps(value)
                # Save original and override
                ctx.eval(f"var __iso_orig_{prop} = {obj}.{prop};")
                ctx.eval(f"{obj}.{prop} = {val_json};")
                restore_code += f"{obj}.{prop} = __iso_orig_{prop};"

    # Read stack before
    stack_before = ctx.eval(f"JSON.parse(JSON.stringify({stack_var}))")
    if not isinstance(stack_before, list):
        stack_before = []

    # Execute handler
    error = None
    return_value = None
    try:
        return_value = ctx.eval(f"""
            (function() {{
                try {{
                    var __r = ({handler_expr})();
                    return __r === undefined ? null : __r;
                }} catch(e) {{
                    return '__ISO_ERROR__:' + e.message;
                }}
            }})()
        """)
        if isinstance(return_value, str) and return_value.startswith("__ISO_ERROR__:"):
            error = return_value[len("__ISO_ERROR__:"):]
            return_value = None
    except Exception as e:
        error = str(e)

    # Read stack after
    stack_after = ctx.eval(f"JSON.parse(JSON.stringify({stack_var}))")
    if not isinstance(stack_after, list):
        stack_after = []

    # Restore environment
    if restore_code:
        try:
            ctx.eval(restore_code)
        except Exception:
            pass

    # Compute diff
    before_len = len(stack_before)
    after_len = len(stack_after)
    stack_pushed = stack_after[before_len:] if after_len > before_len else []
    stack_popped = max(0, before_len - after_len) if after_len < before_len else 0
    # More precise: find common prefix length
    common = 0
    for i in range(min(before_len, after_len)):
        if stack_before[i] == stack_after[i]:
            common += 1
        else:
            break
    if after_len > common:
        stack_pushed = stack_after[common:]
    stack_popped = before_len - common

    return {
        "stack_before": stack_before,
        "stack_after": stack_after,
        "stack_pushed": stack_pushed,
        "stack_popped": stack_popped,
        "return_value": return_value,
        "error": error,
    }
