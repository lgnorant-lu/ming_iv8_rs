"""Minimal manifest-first corpus runner contract.

The runner reports observations and eligibility decisions. It may execute
eligible samples through the Entry Plane. It does not mutate manifests,
profiles, baselines, or samples.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable, Dict, Iterable, List, Optional, Sequence, Set

import iv8_rs


PATH_STATUSES = {"present", "missing", "external", "unknown"}
AUTOMATION_STATUSES = {"ready", "manual_only", "blocked", "not_started"}
VALIDATION_STATUSES = {"pass", "warn", "fail", "not_validated"}


@dataclass(slots=True)
class CorpusManifestItem:
    sample_id: str
    source_path: str
    path_status: str
    sample_kind: str
    runtime_family: str
    persona: str
    target_goal: str = ""
    expected_evidence: List[str] = field(default_factory=list)
    automation_status: str = "not_started"
    validation_status: str = "not_validated"
    notes: str = ""
    entry_expr: Optional[str] = None
    profile: Optional[str] = None
    environment: Optional[Dict[str, Any]] = None
    tags: List[str] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.path_status not in PATH_STATUSES:
            raise ValueError(f"invalid path_status for {self.sample_id}: {self.path_status}")
        if self.automation_status not in AUTOMATION_STATUSES:
            raise ValueError(
                f"invalid automation_status for {self.sample_id}: {self.automation_status}"
            )
        if self.validation_status not in VALIDATION_STATUSES:
            raise ValueError(
                f"invalid validation_status for {self.sample_id}: {self.validation_status}"
            )

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "CorpusManifestItem":
        return cls(
            sample_id=str(data["sample_id"]),
            source_path=str(data["source_path"]),
            path_status=str(data["path_status"]),
            sample_kind=str(data["sample_kind"]),
            runtime_family=str(data["runtime_family"]),
            persona=str(data["persona"]),
            target_goal=str(data.get("target_goal", "")),
            expected_evidence=list(data.get("expected_evidence", [])),
            automation_status=str(data.get("automation_status", "not_started")),
            validation_status=str(data.get("validation_status", "not_validated")),
            notes=str(data.get("notes", "")),
            entry_expr=data.get("entry_expr"),
            profile=data.get("profile"),
            environment=data.get("environment"),
            tags=list(data.get("tags", [])),
        )

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass(slots=True)
class CorpusRunOptions:
    sample_filter: Optional[Set[str]] = None
    include_external: bool = False
    include_missing: bool = False
    dry_run: bool = False
    strict: bool = False
    policy: str = "runtime_safe"
    runner_version: str = "0.6.2-draft"


def load_manifest(path: str | Path) -> List[CorpusManifestItem]:
    """Load the current Markdown manifest table into typed records."""
    manifest_path = Path(path)
    text = manifest_path.read_text(encoding="utf-8")
    records: List[CorpusManifestItem] = []
    in_table = False
    headers: List[str] = []

    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line.startswith("|"):
            if in_table and records:
                break
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if not cells:
            continue
        if cells[0] == "sample_id":
            headers = cells
            in_table = True
            continue
        if in_table and all(set(cell) <= {"-", ":", " "} for cell in cells):
            continue
        if not in_table or not headers or len(cells) < len(headers):
            continue

        row = {header: _clean_markdown_cell(value) for header, value in zip(headers, cells)}
        records.append(CorpusManifestItem(
            sample_id=row["sample_id"],
            source_path=row["source_path"],
            path_status=row["path_status"],
            sample_kind=row["sample_kind"],
            runtime_family=row["runtime_family"],
            persona=row["persona"],
            automation_status=row["automation_status"],
            validation_status=row["validation_status"],
        ))

    if not records:
        raise ValueError(f"no corpus manifest records found: {manifest_path}")
    return records


def build_corpus_report(
    items: Sequence[CorpusManifestItem | Dict[str, Any]],
    *,
    manifest_path: str,
    options: Optional[CorpusRunOptions] = None,
    executor: Optional[Callable[[CorpusManifestItem], Dict[str, Any]]] = None,
) -> Dict[str, Any]:
    """Build a draft corpus report from manifest records.

    When `executor` is provided and a sample is eligible, the executor is
    called with the manifest item and must return a dict with execution
    metadata (plan, result, evidence, diagnostics).

    When `executor` is None, the runner only resolves eligibility without
    executing samples (default: skip with `executor_not_implemented`).
    """
    opts = options or CorpusRunOptions()
    normalized = [item if isinstance(item, CorpusManifestItem) else CorpusManifestItem.from_dict(item)
                  for item in items]
    sample_reports = [_build_sample_report(item, opts, executor=executor) for item in normalized]
    summary = _build_summary(sample_reports, total=len(normalized))
    runner_diagnostics = _runner_level_diagnostics(normalized, sample_reports)

    return {
        "schema_version": "corpus-report.v0.1",
        "runner_version": opts.runner_version,
        "created_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "manifest_path": manifest_path,
        "policy": {"level": opts.policy},
        "summary": summary,
        "samples": sample_reports,
        "diagnostics": runner_diagnostics,
        "artifacts": [],
    }


def default_executor(item: CorpusManifestItem) -> Dict[str, Any]:
    """Default executor using Entry Plane via prepare_entry + run_with_entry."""
    source_path = Path(item.source_path)
    if not source_path.exists():
        return {
            "plan_id": None,
            "result_class": "ERROR",
            "selected_strategy": None,
            "executed_strategies": [],
            "observed_evidence": [],
            "missing_evidence": list(item.expected_evidence),
            "fallback_attempts": [],
            "trace_meta": None,
            "diagnostics": [{
                "code": "CORPUS_SAMPLE_PATH_MISSING",
                "severity": "error",
                "stage": "corpus.execute",
                "message": f"sample source not found: {item.source_path}",
                "sample_id": item.sample_id,
            }],
            "errors": [f"source not found: {item.source_path}"],
        }
    source = source_path.read_text(encoding="utf-8")
    plan = iv8_rs.prepare_entry(source, persona=item.persona)
    plan_id = plan.get("plan_id")
    result = iv8_rs.run_with_entry(plan, source)

    executed_strategies = result.get("executed_strategies", [])
    trace_meta = result.get("trace_meta")
    result_state = result.get("final_state", "unknown")

    observed_evidence = []
    fallback_attempts = result.get("diagnostics", {}).get("fallback_attempts", [])
    missing_evidence = list(item.expected_evidence)

    # Determine outcome
    if result_state in {"collected", "completed"}:
        result_class = "PASS"
    elif result_state in {"partial", "degraded"}:
        result_class = "WARN"
    else:
        result_class = "FAIL"

    return {
        "plan_id": plan_id,
        "result_class": result_class,
        "selected_strategy": result.get("selected_strategy"),
        "executed_strategies": executed_strategies,
        "observed_evidence": observed_evidence,
        "missing_evidence": missing_evidence,
        "fallback_attempts": fallback_attempts,
        "trace_meta": trace_meta,
        "diagnostics": result.get("errors", []),
        "errors": result.get("errors", []),
    }


def run_corpus_manifest(
    manifest_path: str | Path,
    *,
    options: Optional[CorpusRunOptions] = None,
) -> Dict[str, Any]:
    """Load a manifest and emit a report without mutating the manifest."""
    before = Path(manifest_path).read_text(encoding="utf-8")
    items = load_manifest(manifest_path)
    report = build_corpus_report(items, manifest_path=str(manifest_path), options=options)
    after = Path(manifest_path).read_text(encoding="utf-8")
    if before != after:
        raise RuntimeError("corpus runner mutated manifest")
    return report


def _build_sample_report(
    item: CorpusManifestItem,
    opts: CorpusRunOptions,
    *,
    executor: Optional[Callable[[CorpusManifestItem], Dict[str, Any]]] = None,
) -> Dict[str, Any]:
    if opts.sample_filter is not None and item.sample_id not in opts.sample_filter:
        eligibility, reason = "skipped", "not_selected"
    else:
        eligibility, reason = _decide_eligibility(item, opts, executor=executor)

    result = "SKIP"
    if eligibility == "error":
        result = "ERROR"

    execution = None
    if eligibility == "run" and executor is not None:
        try:
            execution = executor(item)
        except Exception as exc:
            execution = {
                "plan_id": None,
                "result_class": "ERROR",
                "selected_strategy": None,
                "executed_strategies": [],
                "observed_evidence": [],
                "missing_evidence": list(item.expected_evidence),
                "fallback_attempts": [],
                "trace_meta": None,
                "diagnostics": [{
                    "code": "CORPUS_SAMPLE_RUNNER_ERROR",
                    "severity": "error",
                    "stage": "corpus.execute",
                    "message": f"execution failed: {exc}",
                    "sample_id": item.sample_id,
                }],
                "errors": [str(exc)],
            }
        result = execution["result_class"]

    diagnostics = _sample_diagnostics(item, eligibility, reason)
    if execution:
        diagnostics.extend(execution.get("diagnostics", []))

    return {
        "sample_id": item.sample_id,
        "source_path": item.source_path,
        "path_status": item.path_status,
        "sample_kind": item.sample_kind,
        "runtime_family": item.runtime_family,
        "persona": item.persona,
        "target_goal": item.target_goal,
        "eligibility": eligibility,
        "skip_reason": reason if not execution else None,
        "result": result,
        "selected_strategy": execution.get("selected_strategy") if execution else None,
        "executed_strategies": execution.get("executed_strategies", []) if execution else [],
        "expected_evidence": item.expected_evidence,
        "observed_evidence": execution.get("observed_evidence", []) if execution else [],
        "missing_evidence": execution.get("missing_evidence", item.expected_evidence) if execution else item.expected_evidence,
        "fallback_attempts": execution.get("fallback_attempts", []) if execution else [],
        "diagnostics": diagnostics,
        "trace_meta": execution.get("trace_meta") if execution else None,
        "artifacts": [],
        "notes": item.notes,
    }


def _decide_eligibility(
    item: CorpusManifestItem,
    opts: CorpusRunOptions,
    *,
    executor: Optional[Callable[[CorpusManifestItem], Dict[str, Any]]] = None,
) -> tuple[str, str]:
    if item.path_status == "missing":
        return "skipped", "dry_run" if opts.include_missing and opts.dry_run else "path_missing"
    if item.path_status == "external":
        return "skipped", "external_unresolved" if opts.include_external else "external_not_enabled"
    if item.path_status == "unknown":
        return "skipped", "unknown_path_status"
    if item.automation_status == "manual_only":
        return "skipped", "manual_only"
    if item.automation_status == "blocked":
        return "skipped", "automation_blocked"
    if item.automation_status == "not_started":
        return "skipped", "not_started"
    if opts.dry_run:
        return "dry_run", "dry_run"
    if executor is not None:
        return "run", "executor_available"
    return "skipped", "executor_not_implemented"


def _sample_diagnostics(item: CorpusManifestItem, eligibility: str, reason: str) -> List[Dict[str, Any]]:
    if reason == "path_missing":
        code = "CORPUS_SAMPLE_PATH_MISSING"
        severity = "warning"
    elif reason in {"external_not_enabled", "external_unresolved"}:
        code = "CORPUS_EXTERNAL_UNRESOLVED"
        severity = "warning"
    elif reason in {"dry_run", "not_selected", "manual_only", "automation_blocked", "not_started"}:
        code = "CORPUS_SAMPLE_SKIPPED"
        severity = "info"
    elif reason == "unknown_path_status":
        code = "CORPUS_SAMPLE_SKIPPED"
        severity = "warning"
    else:
        code = "CORPUS_SAMPLE_SKIPPED"
        severity = "info"
    return [{
        "code": code,
        "severity": severity,
        "stage": "corpus.eligibility",
        "message": f"sample {item.sample_id} {eligibility}: {reason}",
        "sample_id": item.sample_id,
        "details": {"skip_reason": reason},
    }]


def _build_summary(samples: Iterable[Dict[str, Any]], *, total: int) -> Dict[str, int]:
    sample_list = list(samples)
    return {
        "total": total,
        "selected": sum(1 for sample in sample_list if sample["skip_reason"] != "not_selected"),
        "run": sum(1 for sample in sample_list if sample["eligibility"] == "run"),
        "skipped": sum(1 for sample in sample_list if sample["result"] == "SKIP"),
        "pass": sum(1 for sample in sample_list if sample["result"] == "PASS"),
        "warn": sum(1 for sample in sample_list if sample["result"] == "WARN"),
        "fail": sum(1 for sample in sample_list if sample["result"] == "FAIL"),
        "error": sum(1 for sample in sample_list if sample["result"] == "ERROR"),
    }


def _runner_level_diagnostics(
    items: List[CorpusManifestItem],
    sample_reports: List[Dict[str, Any]],
) -> List[Dict[str, Any]]:
    diag: List[Dict[str, Any]] = []
    for report in sample_reports:
        for d in report.get("diagnostics", []):
            if d.get("stage") in {"corpus.execute"} and d.get("severity") == "error":
                diag.append(d)
    return diag


def _clean_markdown_cell(value: str) -> str:
    value = value.strip()
    if value.startswith("`") and value.endswith("`") and len(value) >= 2:
        return value[1:-1]
    return value
