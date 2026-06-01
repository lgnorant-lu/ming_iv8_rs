"""iv8-rs: High-fidelity browser runtime for Python, powered by V8."""

import json
import os
from pathlib import Path
from typing import Dict, Any, Optional

from iv8_rs._iv8 import __version__, JSContext as _JSContextRust, Debugger, enable_logging
from iv8_rs._iv8 import instrument_source, trace_diff
from iv8_rs._iv8 import JSError, JSCompileError, JSTimeoutError, JSMemoryError, JSPanic
from iv8_rs.analysis import diff_analysis
from iv8_rs.trace import parse_trace, StructuredTrace
from iv8_rs.probe import probe_environment

# --- Profile System ---

_PROFILES_DIR = Path(__file__).parent / "profiles"


def load_profile(path: str) -> Dict[str, Any]:
    """
    Load a browser fingerprint profile from a JSON file.

    The profile is a flat dict of dot-path keys (same format as the
    environment parameter). Fields prefixed with '_meta.' are metadata
    and are excluded from the returned dict.

    Args:
        path: Path to the profile JSON file. Can be absolute, relative,
              or "default" to load the built-in Chrome 147 profile.

    Returns:
        Dict suitable for passing to JSContext(environment=...).

    Raises:
        FileNotFoundError: If the profile file does not exist.
        ValueError: If the file is not valid JSON.
    """
    if path == "default":
        profile_path = _PROFILES_DIR / "default_chrome147.json"
    else:
        profile_path = Path(path)

    if not profile_path.exists():
        raise FileNotFoundError(f"Profile not found: {profile_path}")

    try:
        with open(profile_path, "r", encoding="utf-8") as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        raise ValueError(f"Invalid JSON in profile {profile_path}: {e}")

    # Filter out _meta.* fields
    return {k: v for k, v in data.items() if not k.startswith("_meta.")}


def _merge_profile_env(profile: Optional[str], environment: Optional[Dict]) -> Optional[Dict]:
    """Merge profile + environment into final environment dict."""
    if profile is None and environment is None:
        return None
    merged = {}
    if profile is not None:
        merged.update(load_profile(profile))
    if environment is not None:
        merged.update(environment)
    return merged if merged else None


# --- JSContext with profile support ---
# PyO3 #[pyclass(frozen)] doesn't support Python subclassing.
# Use a factory function approach instead.

_RustJSContext = _JSContextRust


def JSContext(*args, profile=None, **kwargs):
    """Create a JSContext, optionally loading a browser fingerprint profile.

    The `profile` parameter loads a JSON file and merges it with the
    environment dict. Priority: environment > profile > defaults.

    Args:
        profile: Path to profile JSON, or "default" for built-in preset.
                 None (default) uses only iv8-defaults.json.
        All other args are passed to the Rust JSContext constructor.

    Returns:
        A JSContext instance.
    """
    if profile is not None:
        env = kwargs.get("environment")
        kwargs["environment"] = _merge_profile_env(profile, env)
    return _RustJSContext(*args, **kwargs)


# Preserve class-level methods
JSContext.get_defaults = _RustJSContext.get_defaults


__all__ = [
    "__version__",
    "JSContext",
    "Debugger",
    "enable_logging",
    "instrument_source",
    "trace_diff",
    "diff_analysis",
    "parse_trace",
    "StructuredTrace",
    "probe_environment",
    "load_profile",
    "JSError",
    "JSCompileError",
    "JSTimeoutError",
    "JSMemoryError",
    "JSPanic",
]
