"""
Diff Analysis Framework for iv8-rs.

Automates the discovery of which environment variables affect which
output positions. Uses deterministic mode to eliminate random noise.
"""

from __future__ import annotations
from typing import Any, Dict, List, Optional, Tuple
from concurrent.futures import ThreadPoolExecutor, as_completed


def diff_analysis(
    js_source: str,
    eval_expr: str,
    base_env: Dict[str, Any],
    test_variables: Dict[str, List[Any]],
    random_seed: Optional[int] = 42,
    time_freeze: Optional[float] = None,
    time_mode: str = "logical",
    max_workers: int = 4,
    progress_callback=None,
) -> Dict[str, Dict[str, Any]]:
    """
    Analyze which environment variables affect the JS output.

    For each variable in test_variables, creates a JSContext with that
    variable changed (all others at base_env values), executes the JS,
    and compares the output to the base case.

    Uses deterministic mode (random_seed + time_freeze) to ensure that
    output differences are ONLY caused by the variable change, not by
    random/time noise.

    Args:
        js_source: JavaScript source code to execute.
        eval_expr: Expression to evaluate after js_source (the "output").
        base_env: Base environment dict (all variables at their default values).
        test_variables: Dict mapping variable names to lists of test values.
            Example: {"screen.width": [1280, 1920, 2560]}
        random_seed: Seed for Math.random determinism (default 42).
        time_freeze: Frozen timestamp in ms (default None = use logical time).
        time_mode: Time mode (default "logical").
        max_workers: Max parallel threads (default 4).
        progress_callback: Optional callable(var_name, value, result) for progress.

    Returns:
        Dict mapping each variable name to its analysis result:
        {
            "screen.width": {
                "affected": True,
                "positions": [12, 13, 14, 15],
                "values_tested": [1280, 1920, 2560],
                "results": ["output_for_1280", "output_for_1920", ...],
                "base_result": "output_for_base",
            },
            "navigator.hardwareConcurrency": {
                "affected": False,
                "positions": [],
                ...
            }
        }
    """
    from iv8_rs._iv8 import JSContext as _RustCtx

    def run_single(env: Dict[str, Any]) -> str:
        """Execute JS with given environment and return result as string."""
        ctx = _RustCtx(
            environment=env,
            random_seed=random_seed,
            time_freeze=time_freeze,
            time_mode=time_mode,
        )
        try:
            ctx.eval(js_source)
            result = ctx.eval(eval_expr)
            return str(result) if result is not None else ""
        except Exception as e:
            return f"__ERROR__:{e}"
        finally:
            ctx.close()

    # 1. Run base case
    base_result = run_single(base_env)

    # 2. Run each variable variation
    report: Dict[str, Dict[str, Any]] = {}
    tasks = []

    for var_name, values in test_variables.items():
        var_results = []
        for val in values:
            test_env = dict(base_env)
            test_env[var_name] = val
            tasks.append((var_name, val, test_env))

    # Execute (parallel if multiple tasks)
    results_map: Dict[Tuple[str, Any], str] = {}

    if max_workers > 1 and len(tasks) > 1:
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            futures = {}
            for var_name, val, test_env in tasks:
                future = executor.submit(run_single, test_env)
                futures[future] = (var_name, val)

            for future in as_completed(futures):
                var_name, val = futures[future]
                result = future.result()
                results_map[(var_name, val)] = result
                if progress_callback:
                    progress_callback(var_name, val, result)
    else:
        for var_name, val, test_env in tasks:
            result = run_single(test_env)
            results_map[(var_name, val)] = result
            if progress_callback:
                progress_callback(var_name, val, result)

    # 3. Build report
    for var_name, values in test_variables.items():
        var_results = [results_map[(var_name, val)] for val in values]
        positions = _find_diff_positions(base_result, var_results)
        report[var_name] = {
            "affected": len(positions) > 0,
            "positions": positions,
            "values_tested": values,
            "results": var_results,
            "base_result": base_result,
        }

    return report


def _find_diff_positions(base: str, variants: List[str]) -> List[int]:
    """Find character positions where any variant differs from base."""
    positions = set()
    base_bytes = base.encode("utf-8", errors="replace")
    for variant in variants:
        var_bytes = variant.encode("utf-8", errors="replace")
        min_len = min(len(base_bytes), len(var_bytes))
        for i in range(min_len):
            if base_bytes[i] != var_bytes[i]:
                positions.add(i)
        # Length difference means everything after min_len is affected
        if len(base_bytes) != len(var_bytes):
            for i in range(min_len, max(len(base_bytes), len(var_bytes))):
                positions.add(i)
    return sorted(positions)
