# Private → public funnel

## Model

```
_ming_iv8_rs (private, origin)  --filter keep-->  ming_iv8_rs (public)
     storage + history                         attestation + wheels + PyPI
     local dry-run / manual funnel             Build Wheels = green source of truth
```

- **Path filter only** (same as dry-run). Commit messages are rewritten SHAs but text not denylisted.
- **Never** push unfiltered private history to public.
- **Free-tier:** private does **not** auto-run CI/bench/funnel on every push
  (see `docs/conventions/git-hooks-conventions.md` §Private vs public CI).
  Funnel is `workflow_dispatch` or local dry-run + force-push.

## CI

| Workflow | Repo | Role |
|---|---|---|
| `public-sync-dry-run.yml` | both | Filter + LEAK, no push |
| `public-sync-funnel.yml` | **private only** (`_ming_iv8_rs`) | Filter + LEAK + force-push public |

## Secret (required for auto push)

On **private** repo `_ming_iv8_rs`:

1. Create a fine-grained PAT (or classic `repo` scope):
   - Resource: `lgnorant-lu/ming_iv8_rs` (public)
   - Permission: **Contents: Read and write**
2. Secret name: **`PUBLIC_SYNC_TOKEN`**
3. Value: the PAT

Without the secret, funnel jobs fail at the push step (filter/LEAK still run and upload artifacts).

## Per-commit / per-push gates (agents + humans)

Full filter-repo on **every commit** is too slow. Split:

| When | Tool | Cost |
|---|---|---|
| Before **commit** | `uv run python scripts/public_sync/check_staged_paths.py` | seconds |
| Before **push** (public-surface paths) | `dry_run.ps1` (or `.githooks/pre-push`) | minutes |
| Private CI funnel | `public-sync-funnel.yml` | needs Actions minutes + `PUBLIC_SYNC_TOKEN` |

If a new path must appear on the public repo: edit `manifests/keep-top.txt` (or keep-scripts / keep-tools), run `build_keep_paths.py`, re-check staged paths, then commit.

Enable hooks once per clone: `git config core.hooksPath .githooks` (see `.githooks/README.md`).

## Manual one-shot (no secret)

```powershell
pwsh -File scripts/public_sync/dry_run.ps1 -WorkRoot $env:TEMP\public-export
cd $env:TEMP\public-export\filtered-repo
git remote add origin https://github.com/lgnorant-lu/ming_iv8_rs.git
git push --force origin main
```

## PyPI first publish

Pending Trusted Publisher (already configured):

- Project: `ming_iv8_rs`
- Repo: `ming_iv8_rs` (public)
- Workflow: `build-wheels.yml`
- Environment: `pypi`

```text
gh workflow run "Build Wheels" --repo lgnorant-lu/ming_iv8_rs -f publish=false
# after green:
gh workflow run "Build Wheels" --repo lgnorant-lu/ming_iv8_rs -f publish=true
```
