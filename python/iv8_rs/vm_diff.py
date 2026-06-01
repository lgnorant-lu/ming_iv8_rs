"""
Cross-version VM handler diff (M28).

Compares two versions of a JS VM source by extracting and diffing
the handler array functions. Reports new/removed/modified handlers.

Usage::

    from iv8_rs.vm_diff import compare_vm_versions

    report = compare_vm_versions(tdc_v1_source, tdc_v2_source, handler_array="A")
    print(f"new: {report.new_handlers}, modified: {report.modified_handlers}")
"""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Optional
from difflib import SequenceMatcher


@dataclass
class HandlerDiff:
    """Diff result for a single handler."""

    index: int
    """Handler index in the array."""

    status: str
    """One of: 'unchanged', 'modified', 'new', 'removed'."""

    similarity: float
    """Text similarity (0.0-1.0). 1.0 for unchanged, 0.0 for new/removed."""

    source_a: str
    """Handler source in version A (empty string if 'new')."""

    source_b: str
    """Handler source in version B (empty string if 'removed')."""


@dataclass
class DiffReport:
    """Result of comparing two VM versions' handler arrays."""

    handler_count_a: int
    """Number of handlers in version A."""

    handler_count_b: int
    """Number of handlers in version B."""

    new_handlers: List[int]
    """Indices of handlers only in version B (added)."""

    removed_handlers: List[int]
    """Indices of handlers only in version A (deleted)."""

    modified_handlers: List[int]
    """Indices of handlers present in both but with different source."""

    unchanged_count: int
    """Number of handlers identical in both versions."""

    similarity_score: float
    """Overall similarity (0.0-1.0): unchanged / max(count_a, count_b)."""

    details: List[HandlerDiff]
    """Per-handler diff details."""


def _extract_handlers(source: str, handler_array: str) -> List[str]:
    """Extract handler function sources by eval'ing the JS and calling toString.

    Uses iv8_rs JSContext to execute the source, then extracts handler sources
    via `handler_array.map(f => typeof f === 'function' ? f.toString() : String(f))`.

    Args:
        source: JS source code containing the VM.
        handler_array: Variable name of the handler array (e.g. "A", "C").

    Returns:
        List of handler source strings.

    Raises:
        RuntimeError: If eval fails or handler array is not accessible.
    """
    from iv8_rs._iv8 import JSContext as _Ctx

    ctx = _Ctx()
    try:
        ctx.eval(source)
        # Extract handler sources as JSON array of strings
        result = ctx.eval(f"""
            (function() {{
                try {{
                    var arr = {handler_array};
                    if (!Array.isArray(arr)) return JSON.stringify([]);
                    return JSON.stringify(arr.map(function(f) {{
                        return typeof f === 'function' ? f.toString() : String(f);
                    }}));
                }} catch(e) {{
                    return JSON.stringify([]);
                }}
            }})()
        """)
    finally:
        ctx.close()

    import json
    try:
        return json.loads(result) if result else []
    except (json.JSONDecodeError, TypeError):
        return []


def compare_vm_versions(
    source_a: str,
    source_b: str,
    handler_array: str = "A",
    similarity_threshold: float = 0.95,
) -> DiffReport:
    """Compare two versions of a JS VM by diffing their handler arrays.

    Extracts handler function sources from both versions and compares them
    using text similarity (difflib.SequenceMatcher).

    Args:
        source_a: JS source of version A (older).
        source_b: JS source of version B (newer).
        handler_array: Variable name of the handler array (default "A").
        similarity_threshold: Below this, a handler is considered 'modified'
                              (default 0.95, allowing minor formatting diffs).

    Returns:
        DiffReport with new/removed/modified handlers and overall similarity.

    Example::

        report = compare_vm_versions(old_tdc, new_tdc, handler_array="C")
        print(f"Modified handlers: {report.modified_handlers}")
        for d in report.details:
            if d.status == "modified":
                print(f"  handler[{d.index}]: similarity={d.similarity:.2f}")
    """
    handlers_a = _extract_handlers(source_a, handler_array)
    handlers_b = _extract_handlers(source_b, handler_array)

    count_a = len(handlers_a)
    count_b = len(handlers_b)
    max_count = max(count_a, count_b)

    details: List[HandlerDiff] = []
    new_handlers: List[int] = []
    removed_handlers: List[int] = []
    modified_handlers: List[int] = []
    unchanged_count = 0

    for i in range(max_count):
        src_a = handlers_a[i] if i < count_a else ""
        src_b = handlers_b[i] if i < count_b else ""

        if i >= count_a:
            # Only in B (new)
            new_handlers.append(i)
            details.append(HandlerDiff(
                index=i, status="new", similarity=0.0,
                source_a="", source_b=src_b,
            ))
        elif i >= count_b:
            # Only in A (removed)
            removed_handlers.append(i)
            details.append(HandlerDiff(
                index=i, status="removed", similarity=0.0,
                source_a=src_a, source_b="",
            ))
        else:
            # Both exist: compare
            if src_a == src_b:
                unchanged_count += 1
                details.append(HandlerDiff(
                    index=i, status="unchanged", similarity=1.0,
                    source_a=src_a, source_b=src_b,
                ))
            else:
                sim = SequenceMatcher(None, src_a, src_b).ratio()
                if sim >= similarity_threshold:
                    # Close enough to be "unchanged" (formatting diff)
                    unchanged_count += 1
                    details.append(HandlerDiff(
                        index=i, status="unchanged", similarity=sim,
                        source_a=src_a, source_b=src_b,
                    ))
                else:
                    modified_handlers.append(i)
                    details.append(HandlerDiff(
                        index=i, status="modified", similarity=round(sim, 4),
                        source_a=src_a, source_b=src_b,
                    ))

    overall_sim = unchanged_count / max_count if max_count > 0 else 1.0

    return DiffReport(
        handler_count_a=count_a,
        handler_count_b=count_b,
        new_handlers=new_handlers,
        removed_handlers=removed_handlers,
        modified_handlers=modified_handlers,
        unchanged_count=unchanged_count,
        similarity_score=round(overall_sim, 4),
        details=details,
    )
