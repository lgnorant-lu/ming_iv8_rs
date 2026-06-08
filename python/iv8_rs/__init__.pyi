"""iv8-rs: High-fidelity browser runtime for Python, powered by V8."""

from os import PathLike
from typing import Any

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
    trace_diff,
)
from iv8_rs._iv8 import JSContext as _RustJSContext
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
from iv8_rs.environment import EnvironmentPatch, EnvironmentPlaneReport
from iv8_rs.environment_toolchain import (
    CoverageDelta,
    CoverageSnapshot,
    EnvironmentToolchainReport,
    ProfileSuggestion,
    ToolchainPatchEntry,
    toolchain_report_from_dict,
    toolchain_report_to_dict,
)
from iv8_rs.environment_toolchain_runtime import CandidatePack, ProbePack
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
from iv8_rs.string_array_reports import (
    ReplacementSite,
    RotationIIFE,
    StringArrayCandidate,
    StringArrayReport,
    StringDecoder,
    string_array_report_from_dict,
    string_array_report_to_dict,
)
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

# --- v0.4: Profile System ---

def load_profile(path: str) -> dict[str, Any]:
    """Load a browser fingerprint profile from a JSON file.

    Args:
        path: Path to profile JSON, or "default" for built-in Chrome 147 preset.

    Returns:
        Dict suitable for passing to JSContext(environment=...).
    """
    ...

def JSContext(  # noqa: N802 - public factory mirrors the exported JSContext API.
    *,
    environment: dict[str, Any] | None = None,
    config: dict[str, Any] | None = None,
    time_mode: str = "logical",
    js_api: str = "__iv8__",
    strict_compat: bool = True,
    random_seed: int | None = None,
    crypto_seed: int | None = None,
    time_freeze: float | None = None,
    profile: str | None = None,
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
    base_env: dict[str, Any],
    test_variables: dict[str, list[Any]],
    random_seed: int | None = 42,
    time_freeze: float | None = None,
    time_mode: str = "logical",
    max_workers: int = 4,
    progress_callback: Any = None,
) -> dict[str, dict[str, Any]]:
    """Analyze which environment variables affect the JS output.

    Returns dict mapping variable names to impact reports.
    """
    ...

def build_environment_patch(
    probe_report: dict[str, Any],
    *,
    policy: str = "runtime_safe",
    defaults: dict[str, Any] | None = None,
) -> EnvironmentPatch:
    ...

def run_environment_plane(
    js_source: str,
    *,
    profile: str | None = "default",
    environment: dict[str, Any] | None = None,
    random_seed: int | None = 42,
    time_freeze: float | None = None,
    time_mode: str = "logical",
    entry_expr: str | None = None,
    patch_defaults: dict[str, Any] | None = None,
    policy: str = "runtime_safe",
) -> EnvironmentPlaneReport:
    ...

def run_environment_toolchain(
    js_source: str,
    *,
    probe_pack: str | ProbePack | dict[str, Any] | PathLike[str] = "fingerprint.m1",
    profile: str | None = "default",
    environment: dict[str, Any] | None = None,
    candidate_pack: str | CandidatePack | dict[str, Any] | PathLike[str] | None = "chrome_generic",
    apply_runtime_safe: bool = False,
    adapt_runtime_safe: bool = False,
    local_overlay: dict[str, Any] | PathLike[str] | None = None,
    max_iterations: int = 1,
    stop_on_regression: bool = True,
    random_seed: int | None = 42,
    time_freeze: float | None = None,
    time_mode: str = "logical",
    entry_expr: str | None = None,
    dry_run_planning: bool = False,
    rollback_diagnostics: bool = False,
    substrate_coverage: bool = False,
    scaffold_gaps: bool = False,
) -> EnvironmentToolchainReport:
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
    "JSError",
    "JSCompileError",
    "JSTimeoutError",
    "JSMemoryError",
    "JSPanic",
]
