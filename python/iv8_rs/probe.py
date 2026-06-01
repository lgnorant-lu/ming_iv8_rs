"""
Environment Probe module for iv8-rs.

Runs a JS source with instrumentation and produces a structured report
of all environment interactions: what APIs were read, called, written,
which ones returned undefined/error, and VM detection info.
"""

from __future__ import annotations
from typing import Any, Dict, List, Optional
from collections import Counter


def probe_environment(
    js_source: str,
    profile: Optional[str] = "default",
    environment: Optional[Dict[str, Any]] = None,
    random_seed: Optional[int] = 42,
    time_freeze: Optional[float] = None,
    time_mode: str = "logical",
    entry_expr: Optional[str] = None,
) -> Dict[str, Any]:
    """Run JS with instrumentation and report all environment interactions.

    Automatically instruments the source (if VM pattern detected) or uses
    start_recording as fallback. Executes the JS, then parses the trace
    to produce a structured report.

    Args:
        js_source: JavaScript source code to analyze.
        profile: Profile to load (default "default"), or None.
        environment: Additional environment overrides.
        random_seed: Deterministic seed (default 42).
        time_freeze: Frozen timestamp (optional).
        time_mode: Time mode (default "logical").
        entry_expr: Optional expression to eval after source (e.g. "getData()").

    Returns:
        Dict with keys:
        - reads: dict of {target: value} for all property reads
        - calls: dict of {target: count} for all function calls
        - writes: dict of {target: value} for all property writes
        - missing: list of targets that returned undefined/null/empty
        - errors: list of error messages from console
        - vm_info: dict with VM detection info (or None)
        - trace_stats: summary statistics of the trace

    Example::

        report = iv8_rs.probe_environment(open("tdc.js").read())
        print(report["missing"])  # APIs that need to be stubbed
        print(report["reads"])    # All environment values read
    """
    from iv8_rs._iv8 import JSContext as _RustCtx
    from iv8_rs._iv8 import instrument_source
    from iv8_rs.trace import parse_trace

    # Build environment
    final_env = {}
    if profile is not None:
        from iv8_rs import load_profile
        final_env.update(load_profile(profile))
    if environment:
        final_env.update(environment)

    # Try to instrument (VM detection)
    vm_info = None
    try:
        patched, info = instrument_source(js_source)
        vm_info = info
        source_to_run = patched
    except (RuntimeError, Exception):
        # No VM pattern detected, use recording instead
        source_to_run = js_source

    # Execute
    ctx = _RustCtx(
        environment=final_env if final_env else None,
        random_seed=random_seed,
        time_freeze=time_freeze,
        time_mode=time_mode,
    )

    try:
        if vm_info is None:
            # No VM detected: use start_recording for env tracking
            ctx.start_recording(
                targets=["navigator", "screen", "document", "location",
                         "Math", "crypto", "performance", "window"],
                limit=50000,
            )

        ctx.eval(source_to_run)

        if entry_expr:
            try:
                ctx.eval(entry_expr)
            except Exception:
                pass

        # Collect trace
        if vm_info is not None:
            raw_trace = ctx.get_unified_trace()
        else:
            raw_trace = ctx.stop_recording()

        # Collect console errors
        console_msgs = ctx.get_console_messages()
        errors = [m["text"] for m in console_msgs if m.get("level") in ("error", "warn")]

    finally:
        ctx.close()

    # Parse trace
    trace = parse_trace(raw_trace) if raw_trace else None

    # Build report
    reads: Dict[str, str] = {}
    calls: Counter = Counter()
    writes: Dict[str, str] = {}
    missing: List[str] = []
    issues: List[Dict[str, str]] = []

    if trace:
        for entry in trace.reads:
            reads[entry.target] = entry.value
            if entry.value in ("undefined", "null", "", "NaN"):
                missing.append(entry.target)
                issues.append({
                    "target": entry.target,
                    "type": "NOT_EXIST",
                    "detail": f"returned {entry.value}",
                    "pc": str(entry.pc),
                })

        for entry in trace.calls:
            calls[entry.target] += 1
            # Detect call errors from value field
            if entry.value and ("Error" in entry.value or "error" in entry.value):
                issues.append({
                    "target": entry.target,
                    "type": "CALL_ERROR",
                    "detail": entry.value,
                    "pc": str(entry.pc),
                })

        for entry in trace.writes:
            writes[entry.target] = entry.value

    # Error classification from console
    for err_msg in errors:
        if "is not a function" in err_msg:
            # Extract function name
            parts = err_msg.split(" is not a function")
            target = parts[0].strip().split(".")[-1] if parts else "unknown"
            issues.append({
                "target": target,
                "type": "NOT_FUNCTION",
                "detail": err_msg,
                "pc": "-1",
            })
        elif "is not defined" in err_msg:
            parts = err_msg.split(" is not defined")
            target = parts[0].strip().split(" ")[-1] if parts else "unknown"
            issues.append({
                "target": target,
                "type": "NOT_DEFINED",
                "detail": err_msg,
                "pc": "-1",
            })
        elif "Cannot read" in err_msg:
            issues.append({
                "target": "unknown",
                "type": "NULL_ACCESS",
                "detail": err_msg,
                "pc": "-1",
            })

    # Coverage statistics
    total_reads = len(reads)
    missing_count = len(set(missing))
    ok_count = total_reads - missing_count
    coverage = {
        "total_targets": total_reads,
        "configured": ok_count,
        "missing": missing_count,
        "error_count": len(issues),
        "coverage_pct": round(ok_count / total_reads * 100, 1) if total_reads > 0 else 0.0,
    }

    return {
        "reads": reads,
        "calls": dict(calls),
        "writes": writes,
        "missing": sorted(set(missing)),
        "errors": errors,
        "issues": issues,
        "coverage": coverage,
        "vm_info": vm_info,
        "trace_stats": trace.summary() if trace else None,
    }
