"""iv8-rs: High-fidelity browser runtime for Python, powered by V8."""

import json
import os
from pathlib import Path
from typing import Dict, Any, Optional

from iv8_rs._iv8 import __version__, JSContext as _JSContextRust, Debugger, enable_logging
from iv8_rs._iv8 import instrument_source, trace_diff, prepare_entry, run_with_entry
from iv8_rs._iv8 import JSError, JSCompileError, JSTimeoutError, JSMemoryError, JSPanic
from iv8_rs.analysis import diff_analysis
from iv8_rs.trace import parse_trace, StructuredTrace, parse_trace_stream, compress_trace, CompressedTrace
from iv8_rs.diagnostics import (
    DIAGNOSTIC_CATALOG,
    TRACE_PREFIX_REGISTRY,
    DiagnosticRecord,
    EvidenceRecord,
    EvidenceGateResult,
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
from iv8_rs.probe import probe_environment
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
from iv8_rs.corpus import (
    CorpusManifestItem,
    CorpusRunOptions,
    build_corpus_report,
    default_executor,
    load_manifest,
    main as corpus_main,
    run_corpus_manifest,
)
from iv8_rs.cfg import CFG
from iv8_rs.taint import TaintEngine, TaintReport
from iv8_rs.patterns import (
    detect_constants, detect_sequences, detect_patterns, detect_all,
    detect_loops, detect_hotspots,
    ConstantMatch, SequenceMatch, PatternMatch, CryptoDetection,
)
from iv8_rs.vm_diff import compare_vm_versions, DiffReport, HandlerDiff
from iv8_rs.isolation import exec_vm_handler
from iv8_rs.entry import EntryPlan, EntryResult, SelectedStrategy
from iv8_rs.experimental_report import (
    EXPERIMENTAL_SCHEMA_VERSIONS,
    ExperimentalDiagnosticRecord,
    ExperimentalEvidenceRecord,
    ExperimentalReport,
    experimental_report_from_dict,
    experimental_report_roundtrip,
    experimental_report_to_dict,
)
from iv8_rs.environment_toolchain import (
    CoverageSnapshot,
    CoverageDelta,
    ToolchainPatchEntry,
    ProfileSuggestion,
    EnvironmentToolchainReport,
    toolchain_report_from_dict,
    toolchain_report_to_dict,
)
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain
from iv8_rs.deobf_reports import (
    RegistryEntry,
    SelectionReport,
    DeobfRegistryReport,
    registry_report_from_dict,
    registry_report_to_dict,
    ValidationCheck,
    ValidationReport,
    validation_report_from_dict,
    validation_report_to_dict,
)
from iv8_rs.string_array_reports import (
    StringArrayCandidate,
    RotationIIFE,
    StringDecoder,
    ReplacementSite,
    StringArrayReport,
    string_array_report_from_dict,
    string_array_report_to_dict,
)
from iv8_rs.vm_reports import (
    HandlerTableSummary,
    TraceSummary,
    StateModel,
    OpcodeHint,
    VMAnalysisReport,
    vm_analysis_report_from_dict,
    vm_analysis_report_to_dict,
    HandlerEntry,
    BytecodeCandidate,
    VMHandlerTable,
    vm_handler_table_from_dict,
    vm_handler_table_to_dict,
)
from iv8_rs.ir_reports import (
    IRNode,
    ConfidenceSummary,
    IRNodeReport,
    ir_node_report_from_dict,
    ir_node_report_to_dict,
)

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
