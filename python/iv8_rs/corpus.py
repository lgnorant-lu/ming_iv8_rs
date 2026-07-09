"""Minimal manifest-first corpus runner contract.

The runner reports observations and eligibility decisions. It may execute
eligible samples through the Entry Plane. It does not mutate manifests,
profiles, baselines, or samples.
"""

from __future__ import annotations

import sys
from collections.abc import Callable, Iterable, Sequence
from dataclasses import asdict, dataclass, field
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

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
    expected_evidence: list[str] = field(default_factory=list)
    automation_status: str = "not_started"
    validation_status: str = "not_validated"
    notes: str = ""
    entry_expr: str | None = None
    profile: str | None = None
    environment: dict[str, Any] | None = None
    tags: list[str] = field(default_factory=list)
    fixtures: list[str] = field(default_factory=list)
    policy_overrides: dict[str, Any] = field(default_factory=dict)

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
    def from_dict(cls, data: dict[str, Any]) -> CorpusManifestItem:
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
            fixtures=list(data.get("fixtures", [])),
            policy_overrides=dict(data.get("policy_overrides", {})),
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(slots=True)
class CorpusRunOptions:
    sample_filter: set[str] | None = None
    include_external: bool = False
    include_missing: bool = False
    dry_run: bool = False
    strict: bool = False
    policy: str = "runtime_safe"
    runner_version: str = "0.7.0-dev"


def load_manifest(path: str | Path) -> list[CorpusManifestItem]:
    """Load the current Markdown manifest table into typed records."""
    manifest_path = Path(path)
    text = manifest_path.read_text(encoding="utf-8")
    records: list[CorpusManifestItem] = []
    in_table = False
    headers: list[str] = []

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
            target_goal=row.get("target_goal", ""),
            expected_evidence=_parse_list_cell(row.get("expected_evidence", "")),
            automation_status=row["automation_status"],
            validation_status=row.get("validation_status", "not_validated"),
            notes=row.get("notes", ""),
        ))

    if not records:
        raise ValueError(f"no corpus manifest records found: {manifest_path}")
    return records


def build_corpus_report(
    items: Sequence[CorpusManifestItem | dict[str, Any]],
    *,
    manifest_path: str,
    options: CorpusRunOptions | None = None,
    executor: Callable[[CorpusManifestItem], dict[str, Any]] | None = None,
) -> dict[str, Any]:
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
        "created_at": datetime.now(UTC).isoformat().replace("+00:00", "Z"),
        "manifest_path": manifest_path,
        "policy": {"level": opts.policy},
        "summary": summary,
        "samples": sample_reports,
        "diagnostics": runner_diagnostics,
        "artifacts": [],
    }


def _classify_result(result_state: str, expected_evidence: list[str], observed_evidence: list[dict[str, Any]]) -> str:
    """Classify result per corpus-runner-contract.md section 12.

    PASS requires:
    - result_state is collected/completed/finalized
    - all expected evidence kinds are present in observed_evidence
    """
    if result_state in {"partial", "degraded"}:
        return "WARN"
    if result_state not in {"collected", "completed", "finalized"}:
        return "FAIL"
    observed_kinds = {e.get("kind") for e in observed_evidence if isinstance(e, dict)}
    for expected in expected_evidence:
        if expected not in observed_kinds:
            return "WARN"
    return "PASS"



def default_executor(item: CorpusManifestItem) -> dict[str, Any]:
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
            "module_graph": None,
            "environment_report": None,
            "diagnostics": [{
                "code": "CORPUS_SAMPLE_PATH_MISSING",
                "severity": "error",
                "stage": "corpus.execute",
                "message": f"sample source not found: {item.source_path}",
                "sample_id": item.sample_id,
            }],
        }
    source = source_path.read_text(encoding="utf-8")
    plan = iv8_rs.prepare_entry(source, persona=item.persona)
    plan_id = plan.get("plan_id")
    result = iv8_rs.run_with_entry(plan, source)

    executed_strategies = result.get("executed_strategies", [])
    trace_meta = result.get("trace_meta")
    module_graph = result.get("module_graph")
    environment_report = result.get("environment_report")
    result_state = result.get("final_state", "unknown")

    # Use structured diagnostic_records and observed_evidence from EntryResult
    diagnostic_records = result.get("diagnostic_records", [])
    observed_evidence = result.get("observed_evidence", [])
    fallback_attempts = result.get("diagnostics", {}).get("fallback_attempts", [])

    # Compute missing_evidence as expected minus observed
    observed_kinds = {e.get("kind") for e in observed_evidence if isinstance(e, dict)}
    expected_kinds = set(item.expected_evidence)
    missing_evidence = list(expected_kinds - observed_kinds)

    # Determine outcome based on result_state AND evidence gate
    result_class = _classify_result(result_state, item.expected_evidence, observed_evidence)

    return {
        "plan_id": plan_id,
        "result_class": result_class,
        "selected_strategy": result.get("selected_strategy"),
        "executed_strategies": executed_strategies,
        "observed_evidence": observed_evidence,
        "missing_evidence": missing_evidence,
        "fallback_attempts": fallback_attempts,
        "trace_meta": trace_meta,
        "module_graph": module_graph,
        "environment_report": environment_report,
        "diagnostics": diagnostic_records,
    }


def run_corpus_manifest(
    manifest_path: str | Path,
    *,
    options: CorpusRunOptions | None = None,
) -> dict[str, Any]:
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
    executor: Callable[[CorpusManifestItem], dict[str, Any]] | None = None,
) -> dict[str, Any]:
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
    artifacts = _build_sample_artifacts(item.sample_id, execution)
    if execution:
        diagnostics.extend(execution.get("diagnostics", []))

    sample = {
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
        "artifacts": artifacts,
        "notes": item.notes,
    }

    # Embed module_graph fragment
    if execution and execution.get("module_graph"):
        sample["module_graph"] = execution["module_graph"]

    # Embed environment_report fragment
    if execution and execution.get("environment_report"):
        sample["environment_report"] = execution["environment_report"]

    # Embed source_ast_report fragment
    if execution:
        observed_kinds = {e.get("kind") for e in execution.get("observed_evidence", []) if isinstance(e, dict)}
        has_ast = any(e.get("source") == "source_ast" for e in execution.get("observed_evidence", []) if isinstance(e, dict))
        if has_ast or "source_ast_transform_applied" in observed_kinds or "source_ast_runtime_validated" in observed_kinds:
            selected_jps = []
            if "source_ast_runtime_validated" in observed_kinds:
                selected_jps.append("dispatch_expression")
            if "eval_source_captured" in observed_kinds:
                selected_jps.append("eval_source_point")
            if "function_constructor_source_captured" in observed_kinds:
                selected_jps.append("function_ctor_source_point")
            if not selected_jps and "source_ast_transform_applied" in observed_kinds:
                selected_jps.append("dispatch_expression")

            runtime_validated = "source_ast_runtime_validated" in observed_kinds or "eval_source_captured" in observed_kinds or "function_constructor_source_captured" in observed_kinds
            source_ast_ev = [e.get("kind") for e in execution.get("observed_evidence", []) if isinstance(e, dict) and e.get("source") == "source_ast"]
            source_ast_diags = [d.get("code") for d in execution.get("diagnostics", []) if isinstance(d, dict) and d.get("stage", "").startswith("source_ast")]

            sample["source_ast_report"] = {
                "source_id": "input.js",
                "selected_join_points": selected_jps,
                "runtime_validated": runtime_validated,
                "evidence": source_ast_ev,
                "diagnostic_codes": source_ast_diags
            }

    return sample


def _decide_eligibility(
    item: CorpusManifestItem,
    opts: CorpusRunOptions,
    *,
    executor: Callable[[CorpusManifestItem], dict[str, Any]] | None = None,
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


def _sample_diagnostics(item: CorpusManifestItem, eligibility: str, reason: str) -> list[dict[str, Any]]:
    if eligibility == "run":
        # Running samples get diagnostics from execution results, not eligibility
        return []
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


def _build_summary(samples: Iterable[dict[str, Any]], *, total: int) -> dict[str, int]:
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
    items: list[CorpusManifestItem],
    sample_reports: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    diag: list[dict[str, Any]] = []
    for report in sample_reports:
        for d in report.get("diagnostics", []):
            if d.get("stage") in {"corpus.execute"} and d.get("severity") == "error":
                diag.append(d)
    return diag


def _build_sample_artifacts(
    sample_id: str,
    execution: dict[str, Any] | None = None,
) -> list[dict[str, Any]]:
    artifacts: list[dict[str, Any]] = []
    if execution and execution.get("trace_meta"):
        artifacts.append({
            "kind": "trace_meta",
            "sample_id": sample_id,
            "path": None,
            "size_bytes": None,
            "retention": "temporary",
        })
    if execution and execution.get("errors"):
        artifacts.append({
            "kind": "error_log",
            "sample_id": sample_id,
            "path": None,
            "size_bytes": None,
            "retention": "temporary",
        })
    return artifacts


def _clean_markdown_cell(value: str) -> str:
    value = value.strip()
    if value.startswith("`") and value.endswith("`") and len(value) >= 2:
        return value[1:-1]
    return value


def _parse_list_cell(value: str) -> list[str]:
    """Parse a Markdown cell containing a list-like value.
    
    Handles formats: `a, b, c` or `a` or empty.
    """
    raw = _clean_markdown_cell(value)
    if not raw or raw == "-":
        return []
    return [item.strip() for item in raw.split(",") if item.strip()]


# ───
# CLI entry point with exit codes
# ───

EXIT_CODE_OK = 0
EXIT_CODE_FAIL = 1
EXIT_CODE_CONFIG_ERROR = 2
EXIT_CODE_WRITE_FAILURE = 3
EXIT_CODE_STRICT_WARN = 4


def _resolve_exit_code(summary: dict[str, int], *, strict: bool) -> int:
    """Resolve exit code per corpus-runner-contract.md section 17.

    | Code | Meaning |
    |------|---------|
    | 0 | No FAIL or ERROR; WARN allowed unless strict |
    | 1 | One or more FAIL |
    | 2 | Not used from here (config errors are callerside) |
    | 3 | Not used from here (write failures are callerside) |
    | 4 | WARN present under strict mode |
    """
    if summary.get("fail", 0) > 0 or summary.get("error", 0) > 0:
        return EXIT_CODE_FAIL
    if strict and summary.get("warn", 0) > 0:
        return EXIT_CODE_STRICT_WARN
    return EXIT_CODE_OK


def main(argv: list[str] | None = None) -> int:
    """Run corpus CLI.
    
    Returns exit code 0-4 per corpus-runner-contract.md.
    """
    if argv is None:
        argv = sys.argv[1:]

    manifest_path: str | None = None
    output_path: str | None = None
    sample_filter: set[str] | None = None
    include_external = False
    dry_run = False
    strict = False

    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg == "--manifest" and i + 1 < len(argv):
            manifest_path = argv[i + 1]
            i += 2
        elif arg == "--out" and i + 1 < len(argv):
            output_path = argv[i + 1]
            i += 2
        elif arg == "--sample" and i + 1 < len(argv):
            if sample_filter is None:
                sample_filter = set()
            sample_filter.add(argv[i + 1])
            i += 2
        elif arg == "--include-external":
            include_external = True
            i += 1
        elif arg == "--dry-run":
            dry_run = True
            i += 1
        elif arg == "--strict":
            strict = True
            i += 1
        else:
            print(f"error: unknown argument: {arg}", file=sys.stderr)
            return EXIT_CODE_CONFIG_ERROR

    if not manifest_path:
        print("error: --manifest is required", file=sys.stderr)
        return EXIT_CODE_CONFIG_ERROR

    try:
        items = load_manifest(manifest_path)
    except (FileNotFoundError, ValueError) as exc:
        print(f"error: manifest load failed: {exc}", file=sys.stderr)
        return EXIT_CODE_CONFIG_ERROR

    opts = CorpusRunOptions(
        sample_filter=sample_filter,
        include_external=include_external,
        dry_run=dry_run,
        strict=strict,
    )
    exec_fn = None if dry_run else default_executor
    report = build_corpus_report(
        items,
        manifest_path=manifest_path,
        options=opts,
        executor=exec_fn,
    )
    summary = report.get("summary", {})


    if output_path:
        import json
        try:
            out = Path(output_path)
            out.parent.mkdir(parents=True, exist_ok=True)
            out.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
        except OSError as exc:
            print(f"error: report write failed: {exc}", file=sys.stderr)
            return EXIT_CODE_WRITE_FAILURE

    return _resolve_exit_code(summary, strict=strict)


if __name__ == "__main__":
    sys.exit(main())

