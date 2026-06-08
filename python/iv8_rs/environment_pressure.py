"""Environment pressure observation report surface.

Typed dataclasses for the environment-pressure.v0.1 report schema. This module
is intentionally report-only: it does not execute samples, apply candidates, or
mutate profiles, manifests, baselines, corpus state, or native substrate.
"""

from __future__ import annotations

import json
import re
from dataclasses import dataclass, field
from typing import Any

from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

__all__ = [
    "ENVIRONMENT_PRESSURE_SCHEMA_VERSION",
    "INPUT_KINDS",
    "EXECUTION_MODES",
    "FAILURE_KINDS",
    "PRESSURE_KINDS",
    "PROMOTION_LEVELS",
    "PressureSignal",
    "PromotionDecision",
    "EnvironmentPressureReport",
    "EnvironmentPressureBatch",
    "PressureManifestItem",
    "PressureSample",
    "build_pressure_report",
    "classify_failure_kind",
    "classify_input_kind",
    "default_execution_mode",
    "pressure_from_failure",
    "promotion_for_pressure",
    "pressure_report_from_dict",
    "pressure_report_to_dict",
    "pressure_batch_diagnostics",
    "environment_pressure_batch_to_toolchain_diagnostics",
    "run_environment_pressure_samples",
    "run_environment_pressure_manifest",
]

ENVIRONMENT_PRESSURE_SCHEMA_VERSION = "environment-pressure.v0.1"

INPUT_KINDS = frozenset({
    "direct_js",
    "html_document",
    "script_tag_page",
    "json_payload",
    "page_snapshot",
    "network_trace",
    "empty_or_invalid",
    "unknown",
})

EXECUTION_MODES = frozenset({
    "bare_v8",
    "browser_like_global",
    "page_dom_fixture",
    "script_tag_bootstrap",
    "prelude_bootstrap",
    "network_stubbed",
})

FAILURE_KINDS = frozenset({
    "success",
    "parse_error",
    "input_format_error",
    "missing_global_symbol",
    "missing_constructor_surface",
    "missing_method_surface",
    "missing_descriptor_shape",
    "missing_dom_fixture",
    "missing_page_bootstrap",
    "missing_prelude_state",
    "runtime_internal_error",
    "timeout_or_loop",
    "unsupported_semantics",
    "unclassified",
})

PRESSURE_KINDS = frozenset({
    "input_normalization",
    "web_api_surface",
    "dom_surface",
    "descriptor_shape",
    "page_lifecycle",
    "prelude_contract",
    "network_surface",
    "timing_surface",
    "event_loop_semantics",
    "runtime_stability",
    "analysis_observability",
})

PROMOTION_LEVELS = frozenset({
    "observe_only",
    "fixture_only",
    "profile_candidate",
    "candidate_pack",
    "generic_substrate_candidate",
    "default_substrate",
})

_REPORT_STATUSES = frozenset({"success", "failed", "skipped"})
_NETWORK_SYMBOLS = frozenset({"fetch", "Request", "Response", "Headers", "XMLHttpRequest"})
_DOM_SYMBOLS = frozenset({"document", "window", "Element", "HTMLElement", "Node"})
_TIMING_SYMBOLS = frozenset({"performance", "setTimeout", "setInterval", "requestAnimationFrame"})
_REFERENCE_ERROR_RE = re.compile(
    r"(?:ReferenceError:\s*)?([$A-Z_a-z][$\w]*)\s+is\s+not\s+defined"
)
_CONSTRUCTOR_ERROR_RE = re.compile(r"([$A-Z_a-z][$\w.]*)\s+is\s+not\s+a\s+constructor")
_READING_ERROR_RE = re.compile(r"reading\s+['\"]([^'\"]+)['\"]")


def _require_allowed(value: str, allowed: frozenset[str], field_name: str) -> None:
    if value not in allowed:
        raise ValueError(f"invalid {field_name}: {value}")


def _extract_missing_symbol(message: str) -> str | None:
    match = _REFERENCE_ERROR_RE.search(message)
    if match:
        return match.group(1)
    match = _CONSTRUCTOR_ERROR_RE.search(message)
    if match:
        return match.group(1).split(".")[0]
    return None


def classify_input_kind(source: str | bytes | None) -> str:
    """Classify sample input shape without executing or normalizing it."""
    if source is None:
        return "empty_or_invalid"
    text = source.decode("utf-8", errors="ignore") if isinstance(source, bytes) else str(source)
    stripped = text.strip()
    if not stripped:
        return "empty_or_invalid"

    lowered = stripped[:4096].lower()
    if "<script" in lowered:
        return "script_tag_page"
    if lowered.startswith(("<!doctype html", "<html", "<body", "<head")):
        return "html_document"
    if lowered.startswith(("get ", "post ", "http/", "har ")) or '"entries"' in lowered:
        return "network_trace"
    if stripped[0] in "[{":
        try:
            json.loads(stripped)
        except json.JSONDecodeError:
            return "direct_js"
        return "json_payload"
    if "snapshot" in lowered and ("document" in lowered or "location" in lowered):
        return "page_snapshot"
    return "direct_js"


def default_execution_mode(input_kind: str) -> str:
    """Return the safest default execution mode for a classified input kind."""
    _require_allowed(input_kind, INPUT_KINDS, "input_kind")
    if input_kind == "direct_js":
        return "browser_like_global"
    if input_kind == "script_tag_page":
        return "script_tag_bootstrap"
    if input_kind in {"html_document", "page_snapshot"}:
        return "page_dom_fixture"
    if input_kind == "network_trace":
        return "network_stubbed"
    return "bare_v8"


def classify_failure_kind(
    message: str | BaseException | None,
    *,
    input_kind: str | None = None,
) -> str:
    """Classify an execution or parsing failure into the v0.8.7 taxonomy."""
    if message is None:
        return "success"
    text = str(message)
    lowered = text.lower()
    if not text.strip():
        return "unclassified"
    if "runtime initialization" in lowered or "jspanic" in lowered or "isolate" in lowered:
        return "runtime_internal_error"
    if "timeout" in lowered or "execution budget" in lowered or "infinite loop" in lowered:
        return "timeout_or_loop"
    if "syntaxerror" in lowered or "unexpected token" in lowered or "parse" in lowered:
        return "parse_error"
    if input_kind in {"json_payload", "network_trace", "empty_or_invalid", "unknown"}:
        return "input_format_error"
    if "script tag" in lowered or "document.currentScript" in text:
        return "missing_page_bootstrap"
    if "is not a constructor" in lowered:
        return "missing_constructor_surface"
    symbol = _extract_missing_symbol(text)
    if symbol is not None:
        if symbol.startswith("$_") or symbol.startswith("__"):
            return "missing_prelude_state"
        if symbol in _DOM_SYMBOLS:
            return "missing_dom_fixture"
        return "missing_global_symbol"
    if "cannot read" in lowered or "is not a function" in lowered:
        return "missing_method_surface"
    if "descriptor" in lowered or "prototype" in lowered or "native code" in lowered:
        return "missing_descriptor_shape"
    if "not implemented" in lowered or "unsupported" in lowered:
        return "unsupported_semantics"
    return "unclassified"


def pressure_from_failure(
    failure_kind: str,
    *,
    message: str | BaseException | None = None,
    input_kind: str | None = None,
) -> PressureSignal:
    """Map a classified failure to a generic pressure signal."""
    _require_allowed(failure_kind, FAILURE_KINDS, "failure_kind")
    symbol = _extract_missing_symbol(str(message)) if message is not None else None
    details: dict[str, Any] = {}
    if input_kind is not None:
        _require_allowed(input_kind, INPUT_KINDS, "input_kind")
        details["input_kind"] = input_kind

    if failure_kind in {"parse_error", "input_format_error"}:
        return PressureSignal("input_normalization", details=details)
    if failure_kind == "missing_page_bootstrap":
        return PressureSignal("page_lifecycle", symbol=symbol, details=details)
    if failure_kind == "missing_prelude_state":
        return PressureSignal("prelude_contract", symbol=symbol, details=details)
    if failure_kind == "missing_dom_fixture":
        return PressureSignal("dom_surface", symbol=symbol, details=details)
    if failure_kind == "missing_descriptor_shape":
        return PressureSignal("descriptor_shape", symbol=symbol, details=details)
    if failure_kind == "runtime_internal_error":
        return PressureSignal("runtime_stability", details=details)
    if failure_kind == "timeout_or_loop":
        return PressureSignal("event_loop_semantics", details=details)
    if symbol in _NETWORK_SYMBOLS:
        return PressureSignal(
            "network_surface",
            symbol=symbol,
            access_pattern="global_reference",
            behavior_depth="presence_only",
            details=details,
        )
    if symbol in _TIMING_SYMBOLS:
        return PressureSignal("timing_surface", symbol=symbol, details=details)
    if failure_kind in {
        "missing_global_symbol",
        "missing_constructor_surface",
        "missing_method_surface",
    }:
        return PressureSignal(
            "web_api_surface",
            symbol=symbol,
            access_pattern="global_reference" if symbol else None,
            behavior_depth="presence_only" if symbol else None,
            details=details,
        )
    return PressureSignal("analysis_observability", symbol=symbol, details=details)


def promotion_for_pressure(
    pressure: PressureSignal,
    *,
    failure_kind: str,
    sample_count: int = 1,
) -> PromotionDecision:
    """Recommend the highest safe v0.8.7 promotion level for a pressure signal."""
    _require_allowed(failure_kind, FAILURE_KINDS, "failure_kind")
    if sample_count < 1:
        raise ValueError("sample_count must be >= 1")
    if pressure.pressure_kind in {
        "input_normalization",
        "page_lifecycle",
        "prelude_contract",
        "dom_surface",
    }:
        return PromotionDecision(
            "fixture_only",
            "sample context or harness boundary; do not promote to default substrate",
        )
    if pressure.pressure_kind in {"runtime_stability", "analysis_observability"}:
        return PromotionDecision(
            "observe_only",
            "diagnostic signal only; requires separate engineering review",
        )
    if sample_count >= 3 and pressure.behavior_depth not in {None, "presence_only"}:
        return PromotionDecision(
            "generic_substrate_candidate",
            "repeated non-presence pressure can enter generic substrate review",
        )
    if pressure.pressure_kind in {"web_api_surface", "network_surface", "timing_surface"}:
        return PromotionDecision(
            "candidate_pack",
            "standard-like surface pressure can become explicit candidate, not default substrate",
        )
    return PromotionDecision("observe_only", "insufficient evidence for promotion")


def build_pressure_report(
    sample_id: str,
    source: str | bytes | None,
    *,
    message: str | BaseException | None = None,
    status: str | None = None,
    sample_count: int = 1,
) -> EnvironmentPressureReport:
    """Build a no-write pressure report from sample text and an optional failure.

    This is a pure report builder. It does not execute `source`; callers provide
    `message` only after an external execution attempt or input-normalization
    decision.
    """
    input_kind = classify_input_kind(source)
    execution_mode = default_execution_mode(input_kind)
    if message is None and input_kind in {
        "json_payload",
        "network_trace",
        "empty_or_invalid",
        "unknown",
    }:
        failure_kind = "input_format_error"
        inferred_status = "skipped"
    else:
        failure_kind = classify_failure_kind(message, input_kind=input_kind)
        inferred_status = "success" if failure_kind == "success" else "failed"
    final_status = status or inferred_status
    pressure = pressure_from_failure(failure_kind, message=message, input_kind=input_kind)
    promotion = promotion_for_pressure(
        pressure,
        failure_kind=failure_kind,
        sample_count=sample_count,
    )
    return EnvironmentPressureReport(
        schema_version=ENVIRONMENT_PRESSURE_SCHEMA_VERSION,
        sample_id=sample_id,
        input_kind=input_kind,
        execution_mode=execution_mode,
        status=final_status,
        failure_kind=failure_kind,
        pressure=pressure,
        promotion=promotion,
        evidence=[
            ExperimentalEvidenceRecord("environment_pressure_report_built", "diagnostic_only"),
            ExperimentalEvidenceRecord("environment_pressure_classified", "diagnostic_only"),
        ],
        diagnostics=[
            ExperimentalDiagnosticRecord(
                "ENV_PRESSURE_REPORT_BUILT",
                "info",
                {
                    "input_kind": input_kind,
                    "execution_mode": execution_mode,
                    "status": final_status,
                    "failure_kind": failure_kind,
                },
            ),
            ExperimentalDiagnosticRecord(
                "ENV_PRESSURE_CLASSIFIED",
                "info",
                {
                    "pressure_kind": pressure.pressure_kind,
                    "promotion_level": promotion.level,
                    "evidence_ceiling": promotion.evidence_ceiling,
                },
            ),
        ],
        writes=[],
    )


@dataclass
class PressureSignal:
    pressure_kind: str
    symbol: str | None = None
    access_pattern: str | None = None
    behavior_depth: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def __post_init__(self) -> None:
        _require_allowed(self.pressure_kind, PRESSURE_KINDS, "pressure_kind")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PressureSignal:
        known = {"pressure_kind", "symbol", "access_pattern", "behavior_depth"}
        details = dict(data.get("details", {}))
        details.update({k: v for k, v in data.items() if k not in known and k != "details"})
        return cls(
            pressure_kind=data["pressure_kind"],
            symbol=data.get("symbol"),
            access_pattern=data.get("access_pattern"),
            behavior_depth=data.get("behavior_depth"),
            details=details,
        )

    def to_dict(self) -> dict[str, Any]:
        data: dict[str, Any] = {"pressure_kind": self.pressure_kind}
        if self.symbol is not None:
            data["symbol"] = self.symbol
        if self.access_pattern is not None:
            data["access_pattern"] = self.access_pattern
        if self.behavior_depth is not None:
            data["behavior_depth"] = self.behavior_depth
        if self.details:
            data["details"] = dict(self.details)
        return data


@dataclass
class PromotionDecision:
    level: str
    reason: str
    evidence_ceiling: str = "diagnostic_only"
    review_status: str = "review_only"

    def __post_init__(self) -> None:
        _require_allowed(self.level, PROMOTION_LEVELS, "promotion level")
        if self.evidence_ceiling != "diagnostic_only":
            raise ValueError("environment pressure promotion is diagnostic_only in v0.8.7")
        if self.review_status != "review_only":
            raise ValueError("environment pressure promotion is review_only in v0.8.7")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PromotionDecision:
        return cls(
            level=data["level"],
            reason=data["reason"],
            evidence_ceiling=data.get("evidence_ceiling", "diagnostic_only"),
            review_status=data.get("review_status", "review_only"),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "level": self.level,
            "reason": self.reason,
            "evidence_ceiling": self.evidence_ceiling,
            "review_status": self.review_status,
        }


@dataclass
class EnvironmentPressureReport:
    schema_version: str
    sample_id: str
    input_kind: str
    execution_mode: str
    status: str
    failure_kind: str
    pressure: PressureSignal
    promotion: PromotionDecision
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]
    writes: list[Any] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.schema_version != ENVIRONMENT_PRESSURE_SCHEMA_VERSION:
            raise ValueError(f"invalid schema_version: {self.schema_version}")
        _require_allowed(self.input_kind, INPUT_KINDS, "input_kind")
        _require_allowed(self.execution_mode, EXECUTION_MODES, "execution_mode")
        _require_allowed(self.status, _REPORT_STATUSES, "status")
        _require_allowed(self.failure_kind, FAILURE_KINDS, "failure_kind")
        if self.writes != []:
            raise ValueError("environment pressure reports are no-write in v0.8.7")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> EnvironmentPressureReport:
        return cls(
            schema_version=data["schema_version"],
            sample_id=data["sample_id"],
            input_kind=data["input_kind"],
            execution_mode=data["execution_mode"],
            status=data["status"],
            failure_kind=data["failure_kind"],
            pressure=PressureSignal.from_dict(data["pressure"]),
            promotion=PromotionDecision.from_dict(data["promotion"]),
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
            writes=list(data.get("writes", [])),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema_version": self.schema_version,
            "sample_id": self.sample_id,
            "input_kind": self.input_kind,
            "execution_mode": self.execution_mode,
            "status": self.status,
            "failure_kind": self.failure_kind,
            "pressure": self.pressure.to_dict(),
            "promotion": self.promotion.to_dict(),
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d.to_dict() for d in self.diagnostics],
            "writes": list(self.writes),
        }


def pressure_report_from_dict(data: dict[str, Any]) -> EnvironmentPressureReport:
    return EnvironmentPressureReport.from_dict(data)


def pressure_report_to_dict(report: EnvironmentPressureReport) -> dict[str, Any]:
    return report.to_dict()


@dataclass
class PressureSample:
    sample_id: str
    source: str | bytes | None
    message: str | BaseException | None = None
    status: str | None = None
    sample_count: int = 1

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PressureSample:
        return cls(
            sample_id=data["sample_id"],
            source=data.get("source"),
            message=data.get("message"),
            status=data.get("status"),
            sample_count=data.get("sample_count", 1),
        )


@dataclass
class PressureManifestItem:
    sample_id: str
    source: str | bytes | None = None
    message: str | BaseException | None = None
    status: str | None = None
    sample_count: int = 1
    source_ref: str | None = None
    notes: dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PressureManifestItem:
        return cls(
            sample_id=data["sample_id"],
            source=data.get("source"),
            message=data.get("message"),
            status=data.get("status"),
            sample_count=data.get("sample_count", 1),
            source_ref=data.get("source_ref"),
            notes=dict(data.get("notes", {})),
        )

    def to_sample(self) -> PressureSample:
        if self.source is not None:
            return PressureSample(
                self.sample_id,
                self.source,
                message=self.message,
                status=self.status,
                sample_count=self.sample_count,
            )
        if self.source_ref is not None:
            return PressureSample(
                self.sample_id,
                None,
                message=self.message or "source_ref provided without inline source",
                status=self.status or "skipped",
                sample_count=self.sample_count,
            )
        return PressureSample(
            self.sample_id,
            None,
            message=self.message,
            status=self.status,
            sample_count=self.sample_count,
        )

    def redacted_source_ref(self) -> str | None:
        if self.source_ref is None:
            return None
        normalized = self.source_ref.replace("\\", "/").rstrip("/")
        return normalized.rsplit("/", 1)[-1] or None


@dataclass
class EnvironmentPressureBatch:
    schema_version: str
    reports: list[EnvironmentPressureReport]
    summary: dict[str, Any]
    writes: list[Any] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.schema_version != ENVIRONMENT_PRESSURE_SCHEMA_VERSION:
            raise ValueError(f"invalid schema_version: {self.schema_version}")
        if self.writes != []:
            raise ValueError("environment pressure batches are no-write in v0.8.7")

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema_version": self.schema_version,
            "reports": [report.to_dict() for report in self.reports],
            "summary": dict(self.summary),
            "writes": list(self.writes),
        }


def run_environment_pressure_samples(
    samples: list[PressureSample | dict[str, Any]],
) -> EnvironmentPressureBatch:
    """Build pressure reports for in-memory samples and summarize taxonomy coverage."""
    pressure_samples = [
        sample if isinstance(sample, PressureSample) else PressureSample.from_dict(sample)
        for sample in samples
    ]
    reports = [
        build_pressure_report(
            sample.sample_id,
            sample.source,
            message=sample.message,
            status=sample.status,
            sample_count=sample.sample_count,
        )
        for sample in pressure_samples
    ]
    return EnvironmentPressureBatch(
        schema_version=ENVIRONMENT_PRESSURE_SCHEMA_VERSION,
        reports=reports,
        summary=_pressure_batch_summary(reports),
        writes=[],
    )


def run_environment_pressure_manifest(
    manifest: list[PressureManifestItem | dict[str, Any]] | dict[str, Any],
) -> EnvironmentPressureBatch:
    """Build a pressure batch from an in-memory manifest.

    `source_ref` is redacted into summary metadata only. This function never
    reads referenced files or directories.
    """
    raw_items = manifest.get("samples", []) if isinstance(manifest, dict) else manifest
    items = [
        item if isinstance(item, PressureManifestItem) else PressureManifestItem.from_dict(item)
        for item in raw_items
    ]
    batch = run_environment_pressure_samples([item.to_sample() for item in items])
    summary = dict(batch.summary)
    summary["manifest"] = {
        "items": len(items),
        "inline_source_count": sum(1 for item in items if item.source is not None),
        "source_ref_only_count": sum(
            1 for item in items if item.source is None and item.source_ref is not None
        ),
        "redacted_source_refs": [
            ref for ref in (item.redacted_source_ref() for item in items) if ref is not None
        ],
        "review_status": "review_only",
        "evidence_ceiling": "diagnostic_only",
    }
    return EnvironmentPressureBatch(
        schema_version=batch.schema_version,
        reports=batch.reports,
        summary=summary,
        writes=[],
    )


def pressure_batch_diagnostics(
    batch: EnvironmentPressureBatch,
) -> list[ExperimentalDiagnosticRecord]:
    """Project a pressure batch into diagnostic-only summary records."""
    summary = dict(batch.summary)
    counts = {
        "by_input_kind": summary.get("by_input_kind", {}),
        "by_failure_kind": summary.get("by_failure_kind", {}),
        "by_pressure_kind": summary.get("by_pressure_kind", {}),
        "by_promotion_level": summary.get("by_promotion_level", {}),
    }
    records = [
        ExperimentalDiagnosticRecord(
            "ENV_PRESSURE_BATCH_SUMMARY",
            "info",
            {
                "schema_version": batch.schema_version,
                "total": summary.get("total", 0),
                "classified_count": summary.get("classified_count", 0),
                "unclassified_count": summary.get("unclassified_count", 0),
                "review_status": "review_only",
                "evidence_ceiling": "diagnostic_only",
                "writes": [],
            },
        ),
        ExperimentalDiagnosticRecord(
            "ENV_PRESSURE_BATCH_CLASSIFICATION_COUNTS",
            "info",
            counts,
        ),
    ]
    if "manifest" in summary:
        records.append(ExperimentalDiagnosticRecord(
            "ENV_PRESSURE_BATCH_MANIFEST_SUMMARY",
            "info",
            dict(summary["manifest"]),
        ))
    return records


def environment_pressure_batch_to_toolchain_diagnostics(
    batch: EnvironmentPressureBatch,
) -> list[ExperimentalDiagnosticRecord]:
    """Return pressure batch diagnostics suitable for toolchain report append."""
    return pressure_batch_diagnostics(batch)


def _count_by(values: list[str], allowed: frozenset[str] | None = None) -> dict[str, int]:
    counts: dict[str, int] = {}
    if allowed is not None:
        counts.update(dict.fromkeys(sorted(allowed), 0))
    for value in values:
        counts[value] = counts.get(value, 0) + 1
    return counts


def _pressure_batch_summary(reports: list[EnvironmentPressureReport]) -> dict[str, Any]:
    failure_kinds = [report.failure_kind for report in reports]
    pressure_kinds = [report.pressure.pressure_kind for report in reports]
    promotion_levels = [report.promotion.level for report in reports]
    unclassified_count = failure_kinds.count("unclassified")
    return {
        "total": len(reports),
        "classified_count": len(reports) - unclassified_count,
        "unclassified_count": unclassified_count,
        "by_input_kind": _count_by([report.input_kind for report in reports], INPUT_KINDS),
        "by_failure_kind": _count_by(failure_kinds, FAILURE_KINDS),
        "by_pressure_kind": _count_by(pressure_kinds, PRESSURE_KINDS),
        "by_promotion_level": _count_by(promotion_levels, PROMOTION_LEVELS),
        "writes": [],
    }
