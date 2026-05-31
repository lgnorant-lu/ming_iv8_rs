"""iv8-rs: High-fidelity browser runtime for Python, powered by V8."""

from typing import Any, Dict, List, Optional

from iv8_rs._iv8 import (
    __version__,
    Debugger,
    enable_logging,
    instrument_source,
    trace_diff,
    JSError,
    JSCompileError,
    JSTimeoutError,
    JSMemoryError,
    JSPanic,
)
from iv8_rs._iv8 import JSContext as _RustJSContext

# --- v0.4: Profile System ---

def load_profile(path: str) -> Dict[str, Any]:
    """Load a browser fingerprint profile from a JSON file.

    Args:
        path: Path to profile JSON, or "default" for built-in Chrome 147 preset.

    Returns:
        Dict suitable for passing to JSContext(environment=...).
    """
    ...

def JSContext(
    *,
    environment: Optional[Dict[str, Any]] = None,
    config: Optional[Dict[str, Any]] = None,
    time_mode: str = "logical",
    js_api: str = "__iv8__",
    strict_compat: bool = True,
    random_seed: Optional[int] = None,
    crypto_seed: Optional[int] = None,
    time_freeze: Optional[float] = None,
    profile: Optional[str] = None,
) -> _RustJSContext:
    """Create a JSContext, optionally loading a browser fingerprint profile.

    The `profile` parameter loads a JSON file and merges it with the
    environment dict. Priority: environment > profile > defaults.

    Args:
        profile: Path to profile JSON, or "default" for built-in preset.
        All other args are passed to the Rust JSContext constructor.
    """
    ...

# --- v0.4: Diff Analysis ---

def diff_analysis(
    js_source: str,
    eval_expr: str,
    base_env: Dict[str, Any],
    test_variables: Dict[str, List[Any]],
    random_seed: Optional[int] = 42,
    time_freeze: Optional[float] = None,
    time_mode: str = "logical",
    max_workers: int = 4,
    progress_callback: Any = None,
) -> Dict[str, Dict[str, Any]]:
    """Analyze which environment variables affect the JS output.

    Returns dict mapping variable names to impact reports.
    """
    ...

__all__ = [
    "__version__",
    "JSContext",
    "Debugger",
    "enable_logging",
    "instrument_source",
    "trace_diff",
    "diff_analysis",
    "load_profile",
    "JSError",
    "JSCompileError",
    "JSTimeoutError",
    "JSMemoryError",
    "JSPanic",
]
