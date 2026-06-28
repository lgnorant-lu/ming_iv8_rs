# IV8 Conventions Index

> Created: 2026-06-16
> Status: active
> Scope: All project-wide convention and protocol documents

## Purpose

This directory is the single index of all IV8 convention documents. Convention
documents define the rules and standards for how work is done — not what work
is done (that's scope/roadmap) and not how quality is measured (that's harness).

Individual convention documents may live in different locations (to avoid
breaking existing cross-references), but they are all indexed here.

## Convention Documents

| # | Document | Location | Scope |
|---|---|---|---|
| 1 | Execution Protocol | `docs/roadmap/v0.8/shared/v0.8-continuous-execution-protocol.md` | Version lifecycle, version modes (standard planning set vs patch aggregation), planning set requirements, commit discipline, review flow, closeout process, cross-version invariants |
| 2 | Naming Conventions | `docs/conventions/naming-conventions.md` | Python module/class/function names, test file names, schema version strings, documentation file names |
| 3 | Docs Conventions | `docs/conventions/docs-conventions.md` | docs/ directory structure, file naming, active-vs-legacy policy, cross-reference format |
| 4 | Commit Conventions | `CONTRIBUTING.md` | Subject format (English, ≤72 chars, scoped), body structure (实施内容/本提交不授权/已执行审阅), scope prefixes |
| 5 | Testing Conventions | `docs/conventions/testing-conventions.md` | Test layers, naming, assertion patterns, harness design, coverage targets, file organization |
| 6 | Python Testing Conventions | `docs/conventions/python-testing-conventions.md` | Python test naming, fixtures, pytest patterns, contract tests, hypothesis, import safety |
| 7 | Harness Charter | `docs/quality-harness/HARNESS-CHARTER.md` | Formal Quality Harness (H<NN>) creation criteria, principles, lifecycle, registration |
| 8 | Tests Directory Conventions | `docs/conventions/tests-directory-conventions.md` | tests/ directory tree, subdirectory thresholds, helper placement, fixture organization, archive policy |
| 9 | TODO Index | `docs/todo/README.md` | Cross-module TODO tracking: Bundler / Environment / Native / Infrastructure / Tools / Deobfuscation |
| 10 | Logging Conventions | `docs/conventions/logging-conventions.md` | Module hierarchy, log level semantics, structured fields, span usage, runtime control |

## Convention Hierarchy

```text
Execution Protocol (governs HOW versions are run)
  ├── Naming Conventions (governs WHAT things are called)
  ├── Commit Conventions (governs HOW changes are recorded)
  ├── Testing Conventions (governs HOW tests are written and organized)
  └── Harness Charter (governs WHEN and HOW formal harnesses are created)
```

When conventions conflict, the narrower document controls only within its
explicit scope. The Execution Protocol is the most general and overrides
where no narrower rule exists.

## Review Checklist

- [x] All active convention documents are indexed
- [x] Each entry has document name, file path, and scope description
- [x] Hierarchy clarifies override precedence
