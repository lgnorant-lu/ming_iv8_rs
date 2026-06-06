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
from iv8_rs.environment import EnvironmentPatch, EnvironmentPlaneReport

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

def build_environment_patch(
    probe_report: Dict[str, Any],
    *,
    policy: str = "runtime_safe",
    defaults: Optional[Dict[str, Any]] = None,
) -> EnvironmentPatch:
    ...

def run_environment_plane(
    js_source: str,
    *,
    profile: Optional[str] = "default",
    environment: Optional[Dict[str, Any]] = None,
    random_seed: Optional[int] = 42,
    time_freeze: Optional[float] = None,
    time_mode: str = "logical",
    entry_expr: Optional[str] = None,
    patch_defaults: Optional[Dict[str, Any]] = None,
    policy: str = "runtime_safe",
) -> EnvironmentPlaneReport:
    ...

__all__ = [
    "__version__",
    "JSContext",
    "Debugger",
    "enable_logging",
    "instrument_source",
    "trace_diff",
    "diff_analysis",
    "EnvironmentPatch",
    "EnvironmentPlaneReport",
    "build_environment_patch",
    "run_environment_plane",
    "load_profile",
    "JSError",
    "JSCompileError",
    "JSTimeoutError",
    "JSMemoryError",
    "JSPanic",
]
