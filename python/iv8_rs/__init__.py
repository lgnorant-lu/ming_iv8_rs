"""iv8-rs: High-fidelity browser runtime for Python, powered by V8."""

import json
import os
import threading
from pathlib import Path
from typing import Any, Dict, Optional

# ICU data for V8 Intl (must be set before first JSContext / V8::Initialize).
# Chromium/d8/ref-iv8 pattern: icudtl.dat side-by-side with the native extension.
_ICUDTL = Path(__file__).resolve().parent / "icudtl.dat"
if _ICUDTL.is_file() and "IV8_ICUDTL_PATH" not in os.environ:
    os.environ["IV8_ICUDTL_PATH"] = str(_ICUDTL)

from iv8_rs._iv8 import (
    Debugger,
    JSCompileError,
    JSError,
    JSMemoryError,
    JSPanic,
    JSTimeoutError,
    __version__,
    enable_logging,
    instrument_source,
    plan_multi_entry,
    prepare_entry,
    run_with_entry,
    trace_diff,
)
from iv8_rs._iv8 import JSContext as _JSContextRust
from iv8_rs.analysis import diff_analysis
from iv8_rs.cfg import CFG
from iv8_rs.corpus import (
    CorpusManifestItem,
    CorpusRunOptions,
    build_corpus_report,
    default_executor,
    load_manifest,
    run_corpus_manifest,
)
from iv8_rs.corpus import (
    main as corpus_main,
)
from iv8_rs.deobf_reports import (
    DeobfRegistryReport,
    RegistryEntry,
    SelectionReport,
    ValidationCheck,
    ValidationReport,
    registry_report_from_dict,
    registry_report_to_dict,
    validation_report_from_dict,
    validation_report_to_dict,
)
from iv8_rs.diagnostics import (
    DIAGNOSTIC_CATALOG,
    TRACE_PREFIX_REGISTRY,
    DiagnosticRecord,
    EvidenceGateResult,
    EvidenceRecord,
    FallbackAttempt,
    TraceEvent,
    build_evidence_diagnostics,
    build_trace_diagnostics,
    build_trace_events,
    classify_trace_prefix,
    confidence_from_evidence,
    evaluate_evidence_gate,
    evidence_satisfies,
)
from iv8_rs.entry import EntryPlan, EntryResult, SelectedStrategy
from iv8_rs.environment import (
    EnvironmentPatch,
    EnvironmentPlaneReport,
    build_environment_patch,
    run_environment_plane,
)
from iv8_rs.environment_policy import (
    EnvironmentPatchCandidate,
    PatchPolicyDecision,
    PatchPolicyOptions,
    block_mutation,
    decide_patch_policy,
    runtime_safe_candidate,
)
from iv8_rs.environment_pressure import (
    ENVIRONMENT_PRESSURE_SCHEMA_VERSION,
    EXECUTION_MODES,
    FAILURE_KINDS,
    INPUT_KINDS,
    PRESSURE_KINDS,
    PROMOTION_LEVELS,
    EnvironmentPressureBatch,
    EnvironmentPressureReport,
    PressureManifestItem,
    PressureSample,
    PressureSignal,
    PromotionDecision,
    build_pressure_report,
    classify_failure_kind,
    classify_input_kind,
    default_execution_mode,
    environment_pressure_batch_to_toolchain_diagnostics,
    pressure_batch_diagnostics,
    pressure_from_failure,
    pressure_report_from_dict,
    pressure_report_to_dict,
    promotion_for_pressure,
    run_environment_pressure_manifest,
    run_environment_pressure_samples,
)
from iv8_rs.environment_toolchain import (
    CoverageDelta,
    CoverageSnapshot,
    EnvironmentToolchainReport,
    ProfileSuggestion,
    ToolchainPatchEntry,
    toolchain_report_from_dict,
    toolchain_report_to_dict,
)
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain
from iv8_rs.experimental_report import (
    EXPERIMENTAL_SCHEMA_VERSIONS,
    ExperimentalDiagnosticRecord,
    ExperimentalEvidenceRecord,
    ExperimentalReport,
    experimental_report_from_dict,
    experimental_report_roundtrip,
    experimental_report_to_dict,
)
from iv8_rs.ir_reports import (
    ConfidenceSummary,
    IRNode,
    IRNodeReport,
    ir_node_report_from_dict,
    ir_node_report_to_dict,
)
from iv8_rs.isolation import exec_vm_handler
from iv8_rs.patterns import (
    ConstantMatch,
    CryptoDetection,
    PatternMatch,
    SequenceMatch,
    detect_all,
    detect_constants,
    detect_hotspots,
    detect_loops,
    detect_patterns,
    detect_sequences,
)
from iv8_rs.probe import probe_environment
from iv8_rs.string_array_reports import (
    ReplacementSite,
    RotationIIFE,
    StringArrayCandidate,
    StringArrayReport,
    StringDecoder,
    string_array_report_from_dict,
    string_array_report_to_dict,
)
from iv8_rs.taint import TaintEngine, TaintReport
from iv8_rs.trace import (
    CompressedTrace,
    StructuredTrace,
    compress_trace,
    parse_trace,
    parse_trace_stream,
)
from iv8_rs.vm_diff import DiffReport, HandlerDiff, compare_vm_versions
from iv8_rs.vm_reports import (
    BytecodeCandidate,
    HandlerEntry,
    HandlerTableSummary,
    OpcodeHint,
    StateModel,
    TraceSummary,
    VMAnalysisReport,
    VMHandlerTable,
    vm_analysis_report_from_dict,
    vm_analysis_report_to_dict,
    vm_handler_table_from_dict,
    vm_handler_table_to_dict,
)

# --- Stack size configuration (K-010) ---
# V8 FunctionTemplate creation (1287 interfaces, 9223 members after mixin
# merge) requires deep C++ stack recursion. Ensure Python threads have
# sufficient stack before any JSContext creation. 128MB is virtual memory
# with lazy physical commit — actual RSS is far smaller.
_PYTHON_MIN_STACK = 128 * 1024 * 1024
try:
    _current_stack = threading.stack_size()
    if _current_stack == 0 or _current_stack < _PYTHON_MIN_STACK:
        threading.stack_size(_PYTHON_MIN_STACK)
except (ValueError, OSError):
    pass

# --- Profile System ---

_PROFILES_DIR = Path(__file__).parent / "profiles"


def load_profile(path: str) -> dict[str, Any]:
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
        with open(profile_path, encoding="utf-8") as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        raise ValueError(f"Invalid JSON in profile {profile_path}: {e}")

    # Filter out _meta.* fields
    return {k: v for k, v in data.items() if not k.startswith("_meta.")}


def _merge_profile_env(profile: str | None, environment: dict | None) -> dict | None:
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
#
# V8 isolates are thread-bound (rusty-v8 #643, #1467) and cannot be
# transferred between threads after creation. JSContext must be created
# AND used on the same thread. Stack size is configured at module import
# time via threading.stack_size(128MB) — see K-010 above.

_RustJSContext = _JSContextRust


def JSContext(*args, profile=None, **kwargs):
    """Create a JSContext, optionally loading a browser fingerprint profile.

    The `profile` parameter loads a JSON file and merges it with the
    environment dict. Priority: environment > profile > defaults.

    Stack size is configured automatically at module import time
    (threading.stack_size(128MB)). No manual stack configuration needed.

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
    "parse_trace_stream",
    "compress_trace",
    "StructuredTrace",
    "CompressedTrace",
    "DIAGNOSTIC_CATALOG",
    "TRACE_PREFIX_REGISTRY",
    "DiagnosticRecord",
    "EvidenceRecord",
    "EvidenceGateResult",
    "FallbackAttempt",
    "TraceEvent",
    "build_evidence_diagnostics",
    "build_trace_diagnostics",
    "build_trace_events",
    "classify_trace_prefix",
    "confidence_from_evidence",
    "evaluate_evidence_gate",
    "evidence_satisfies",
    "CFG",
    "probe_environment",
    "EnvironmentPatch",
    "EnvironmentPlaneReport",
    "build_environment_patch",
    "run_environment_plane",
    "EnvironmentPatchCandidate",
    "PatchPolicyDecision",
    "PatchPolicyOptions",
    "block_mutation",
    "decide_patch_policy",
    "runtime_safe_candidate",
    "CorpusManifestItem",
    "CorpusRunOptions",
    "build_corpus_report",
    "default_executor",
    "load_manifest",
    "run_corpus_manifest",
    "load_profile",
    "TaintEngine",
    "TaintReport",
    "detect_constants",
    "detect_sequences",
    "detect_patterns",
    "detect_all",
    "detect_loops",
    "detect_hotspots",
    "ConstantMatch",
    "SequenceMatch",
    "PatternMatch",
    "CryptoDetection",
    "compare_vm_versions",
    "DiffReport",
    "HandlerDiff",
    "exec_vm_handler",
    "prepare_entry",
    "plan_multi_entry",
    "run_with_entry",
    "ExperimentalEvidenceRecord",
    "ExperimentalDiagnosticRecord",
    "ExperimentalReport",
    "experimental_report_from_dict",
    "experimental_report_to_dict",
    "experimental_report_roundtrip",
    "EXPERIMENTAL_SCHEMA_VERSIONS",
    "CoverageSnapshot",
    "CoverageDelta",
    "ToolchainPatchEntry",
    "ProfileSuggestion",
    "EnvironmentToolchainReport",
    "toolchain_report_from_dict",
    "toolchain_report_to_dict",
    "ENVIRONMENT_PRESSURE_SCHEMA_VERSION",
    "INPUT_KINDS",
    "EXECUTION_MODES",
    "FAILURE_KINDS",
    "PRESSURE_KINDS",
    "PROMOTION_LEVELS",
    "PressureSignal",
    "PressureSample",
    "PressureManifestItem",
    "PromotionDecision",
    "EnvironmentPressureReport",
    "EnvironmentPressureBatch",
    "build_pressure_report",
    "classify_input_kind",
    "default_execution_mode",
    "classify_failure_kind",
    "pressure_from_failure",
    "pressure_batch_diagnostics",
    "environment_pressure_batch_to_toolchain_diagnostics",
    "promotion_for_pressure",
    "pressure_report_from_dict",
    "pressure_report_to_dict",
    "run_environment_pressure_samples",
    "run_environment_pressure_manifest",
    "run_environment_toolchain",
    "RegistryEntry",
    "SelectionReport",
    "DeobfRegistryReport",
    "registry_report_from_dict",
    "registry_report_to_dict",
    "ValidationCheck",
    "ValidationReport",
    "validation_report_from_dict",
    "validation_report_to_dict",
    "StringArrayCandidate",
    "RotationIIFE",
    "StringDecoder",
    "ReplacementSite",
    "StringArrayReport",
    "string_array_report_from_dict",
    "string_array_report_to_dict",
    "HandlerTableSummary",
    "TraceSummary",
    "StateModel",
    "OpcodeHint",
    "VMAnalysisReport",
    "vm_analysis_report_from_dict",
    "vm_analysis_report_to_dict",
    "HandlerEntry",
    "BytecodeCandidate",
    "VMHandlerTable",
    "vm_handler_table_from_dict",
    "vm_handler_table_to_dict",
    "IRNode",
    "ConfidenceSummary",
    "IRNodeReport",
    "ir_node_report_from_dict",
    "ir_node_report_to_dict",
    "EntryPlan",
    "EntryResult",
    "SelectedStrategy",
    "JSError",
    "JSCompileError",
    "JSTimeoutError",
    "JSMemoryError",
    "JSPanic",
]
