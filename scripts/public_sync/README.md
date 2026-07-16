# Public dual-repo sync (dry-run first)

Private → public **path filter** tooling. Default mode is **dry-run only** (no public push).

## Files

| Path | Role |
|---|---|
| `build_keep_paths.py` | Merge keep manifests + `git ls-files` → `generated/keep-paths.txt` |
| `leak_scan.py` | Pattern LEAK scan on a tree |
| `dry_run.ps1` / `dry_run.sh` | Clone → `git filter-repo` → LEAK report |
| `generated/` | Local outputs (gitignored) |

Keep manifests (public SoT under this tree):

- `manifests/keep-top.txt`
- `manifests/keep-scripts.txt`
- `manifests/keep-tools.txt`

Private analysis copies may still exist under `docs/roadmap/...`; `build_keep_paths.py`
prefers `manifests/`.

## Local dry-run (Windows)

```powershell
# from repo root (git-filter-repo pulled ephemerally via uv --with)
pwsh -File scripts/public_sync/dry_run.ps1
```

Report under `%TEMP%\opencode\public-dry-run-*\report\SUMMARY.md`.

## CI

Workflow: `.github/workflows/public-sync-dry-run.yml`

- Triggers: `workflow_dispatch`, and path-filtered push/PR on keep lists / this dir
- Installs `git-filter-repo`, filters a **local clone**, LEAK scan `--strict`
- Uploads artifact `public-sync-dry-run-report`
- **Never** creates or pushes a public remote

## Explicit drops

- `scripts/sample_chrome_surface.py` (local Chrome path sampler)
- tools private plane stubs (see tools-keep comments)

## After dry-run is green

1. Review filtered file list + leak report  
2. Explicitly authorize: create public `ming_iv8_rs` (or agreed name) + first filtered push  
3. Optional: message rewrite / squash policy (separate decision)
