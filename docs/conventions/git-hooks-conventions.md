# Git Hooks & Dual-Repo Commit Gates

> Created: 2026-07-17  
> Status: accepted  
> Scope: local hooks, dual-repo path checks, what belongs in hooks vs CI

## Purpose

Define **lightweight, always-on local gates** so every commit (including docs)
is checked against the public keep filter, without burning free Actions minutes
or re-running full test suites on every commit.

Canonical companions:

| Topic | Document |
|---|---|
| Commit message format | `CONTRIBUTING.md` §1–3 |
| Dual-repo path checklist | `CONTRIBUTING.md` §3.6 |
| Agent hard rules | `AGENTS.md` (Dual-repo commit gate) |
| Keep lists / dry-run / funnel | `scripts/public_sync/README.md`, `FUNNEL.md` |
| Hook install | `.githooks/README.md` |

## Design principles

1. **Fast on commit, heavy on push (optional).**  
   Full `git filter-repo` dry-run takes minutes (clone + filter). It must **not**
   run on every `git commit`. Classification of staged paths is O(files).

2. **Tests are not hook duties.**  
   Full `cargo test` / pytest run in development and on **public** CI / release
   gates. Hooks assume tests were already considered for the change size.

3. **Rust compile != format/lint.**  
   `cargo build` does **not** run rustfmt or clippy. Format/lint need explicit
   `cargo fmt` / `cargo clippy` or hooks/CI lint jobs.

4. **Docs commits are dual-repo commits.**  
   A new file under `docs/` is private-only unless listed in
   `scripts/public_sync/manifests/keep-*.txt`. Missing keep entry = silent DROP
   on public (e.g. historical miss of `docs/releases/`).

5. **Private repo is storage; public repo is attestation.**  
   Free Actions minutes are scarce. Default policy: **do not burn private CI**
   on every push. Canonical green builds live on public `ming_iv8_rs`
   (`Build Wheels`, and public CI if enabled). See §Private vs public CI.

## Required local workflow (every commit)

```text
1. Stage intended files
2. uv run python scripts/public_sync/check_staged_paths.py
   - Expect KEEP / KEEP-RULE for anything that must appear on public
   - DROP is OK only for intentional private paths (AGENTS.md, docs/todo, .githooks, ...)
3. If public-ish DROP: edit keep-top/scripts/tools manifests
   -> uv run python scripts/public_sync/build_keep_paths.py
   -> re-run check_staged_paths
4. cargo fmt (or rely on pre-commit fmt --check)
5. Commit with Conventional Commits (CONTRIBUTING.md)
```

## Required before push (when public surface changes)

Public-surface path prefixes (non-exhaustive):

```text
crates/  python/  tests/  .github/
README.md  README.zh-CN.md  LICENSE  CHANGELOG.md  CONTRIBUTING.md
docs/api/  docs/conventions/  docs/source/  docs/releases/
docs/GUIDE.public.md  docs/quality-harness/
scripts/public_sync/  pyproject.toml  Cargo.toml  Cargo.lock
```

If the push range touches any of the above:

```powershell
pwsh -File scripts/public_sync/dry_run.ps1 -WorkRoot "$env:TEMP\opencode\public-export-check"
# report: SUMMARY.md + leak-scan.md  — HIGH must be 0
```

Then either:

- manual funnel: push filtered tree to `ming_iv8_rs`, or  
- private `workflow_dispatch` of Public Sync Funnel (if Actions minutes available).

## Hooks (`.githooks/`)

### Enable (once per clone)

```powershell
git config core.hooksPath .githooks
```

### Matrix

| Hook | Default | Runs | Does not run |
|---|---|---|---|
| `pre-commit` | ON | `cargo fmt --check` if `.rs` staged; `check_staged_paths.py` | full dry-run; full tests; clippy |
| `pre-push` | ON if public-surface files in range | full `dry_run.ps1` | always (skips private-only pushes) |

### Escape hatches

| Variable | Effect |
|---|---|
| `SKIP_IV8_HOOKS=1` | Skip all iv8 hooks |
| `IV8_SKIP_PUBLIC_DRY_RUN=1` | Skip pre-push dry-run only |
| `IV8_FORCE_PUBLIC_DRY_RUN=1` | Always dry-run on push |
| `IV8_STRICT_PUBLIC_PATHS=1` | pre-commit fails if public-ish path is DROP |

### Why dry-run is not on every commit

| Approach | Cost | Verdict |
|---|---|---|
| dry-run every commit | minutes × commit frequency | Rejected |
| path classification every commit | seconds | Required |
| dry-run on push (public surface) | minutes × push | Default ON |
| full test in hook | minutes + flaky isolate serial | Rejected |

## Private vs public CI (free-tier policy)

| Repo | Role | Default Actions |
|---|---|---|
| `_ming_iv8_rs` (private) | **Storage + history** of full tree | **No** push-triggered CI/bench; funnel **dispatch-only** |
| `ming_iv8_rs` (public) | **Attestation + wheels + PyPI** | Build Wheels; optional CI on public tip |

Rationale:

- Private Actions minutes are paid after a small free pool; macOS is expensive.
- Recent private failures with empty steps in ~3s are **billing/quota kills**, not product regressions (public wheel matrix green).
- Attestation that matters for users is **public** green + PyPI/GitHub Release assets.

Workflow rules (implementation):

- `ci.yml` / `bench.yml`: run only when `github.repository == 'lgnorant-lu/ming_iv8_rs'` **or** `workflow_dispatch` (so private does not auto-burn minutes).
- `public-sync-funnel.yml`: **workflow_dispatch only** (manual when minutes exist); local dry-run + force-push remains primary free path.
- `build-wheels.yml`: public only (already the release path).

Agents and humans still run **local** tests before release; absence of private CI is not absence of quality gates.

## Agent checklist (must)

Before `git commit`:

- [ ] `check_staged_paths` reviewed (KEEP for intended public files)
- [ ] keep manifests updated if new public path
- [ ] Conventional Commits subject/body (CONTRIBUTING)
- [ ] No emoji in commit or code

Before `git push` to private origin (if public-surface changed):

- [ ] dry-run PASS (or pre-push succeeded)
- [ ] LEAK HIGH = 0

After push (public surface):

- [ ] Funnel public (dispatch if minutes, else manual filter force-push)
- [ ] Do not treat private CI red as code failure when steps empty / 3s fail

## Related files

| Path | Role |
|---|---|
| `.githooks/pre-commit` | fmt + path classification |
| `.githooks/pre-push` | conditional dry-run |
| `scripts/public_sync/check_staged_paths.py` | staged KEEP/DROP |
| `scripts/public_sync/build_keep_paths.py` | rebuild keep list |
| `scripts/public_sync/dry_run.ps1` | full filter + LEAK |
| `scripts/public_sync/manifests/keep-*.txt` | whitelist source |
