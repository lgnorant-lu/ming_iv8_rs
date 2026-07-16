# Public dual-repo dry-run (Windows PowerShell).
# Clones the current repo to a temp workdir, applies path keep via git-filter-repo,
# runs LEAK scan, writes a report. NEVER pushes to any remote.
#
# Usage (from repo root):
#   pwsh -File scripts/public_sync/dry_run.ps1
#   pwsh -File scripts/public_sync/dry_run.ps1 -WorkRoot "C:\Users\Lenovo\AppData\Local\Temp\opencode\public-dry-run"
#
# Requires: git, python/uv, git-filter-repo (pip install git-filter-repo)

param(
    [string]$WorkRoot = "",
    [switch]$StrictLeak
)

$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
Set-Location $RepoRoot

if (-not $WorkRoot) {
    $WorkRoot = Join-Path $env:TEMP "opencode\public-dry-run-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
}
$WorkRoot = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($WorkRoot)
$CloneDir = Join-Path $WorkRoot "filtered-repo"
$ReportDir = Join-Path $WorkRoot "report"
$KeepList = Join-Path $RepoRoot "scripts\public_sync\generated\keep-paths.txt"

New-Item -ItemType Directory -Force -Path $WorkRoot, $ReportDir | Out-Null

Write-Host "== build keep paths =="
& uv run python scripts/public_sync/build_keep_paths.py --print-stats -o $KeepList
if ($LASTEXITCODE -ne 0) { throw "build_keep_paths failed" }

Write-Host "== git-filter-repo via uv --with (ephemeral) =="

Write-Host "== clone (local, no remotes push) =="
if (Test-Path $CloneDir) {
    Remove-Item -Recurse -Force $CloneDir
}
# Local clone preserves objects; filter-repo rewrites clone only
& git clone --no-local "$RepoRoot" "$CloneDir"
if ($LASTEXITCODE -ne 0) { throw "git clone failed" }

Write-Host "== filter-repo paths-from-file (destructive on clone only) =="
Push-Location $CloneDir
try {
    # Remove origin so nobody can push by accident from clone
    & git remote remove origin 2>$null

    $keepAbs = (Resolve-Path $KeepList).Path
    & uv run --with git-filter-repo python -m git_filter_repo --force --paths-from-file $keepAbs
    if ($LASTEXITCODE -ne 0) { throw "git-filter-repo failed" }

    $fileCount = (git ls-files | Measure-Object).Count
    Write-Host "filtered file count: $fileCount"
    git ls-files | Out-File -Encoding utf8 (Join-Path $ReportDir "filtered-ls-files.txt")
    git log --oneline -20 | Out-File -Encoding utf8 (Join-Path $ReportDir "filtered-log-head.txt")
}
finally {
    Pop-Location
}

Write-Host "== LEAK scan on filtered tree (keep paths only) =="
$leakArgs = @(
    "run", "python", "scripts/public_sync/leak_scan.py",
    "--root", $CloneDir,
    "--paths-file", $KeepList,
    "--report", (Join-Path $ReportDir "leak-scan.md")
)
if ($StrictLeak) { $leakArgs += "--strict" }
& uv @leakArgs
$leakCode = $LASTEXITCODE

$summary = @"
# Public dry-run summary

- source_repo: $RepoRoot
- work_root: $WorkRoot
- clone: $CloneDir
- keep_list: $KeepList
- keep_count: $((Get-Content $KeepList | Measure-Object).Count)
- filtered_files: $((Get-Content (Join-Path $ReportDir "filtered-ls-files.txt") | Measure-Object).Count)
- leak_exit_code: $leakCode
- push: NEVER (dry-run only)

## Next

1. Inspect $ReportDir\leak-scan.md
2. Fix keep lists or scrub sources if high hits
3. Authorize public remote create + push separately

"@
$summaryPath = Join-Path $ReportDir "SUMMARY.md"
$summary | Out-File -Encoding utf8 $summaryPath
Write-Host $summary
Write-Host "report: $summaryPath"

if ($leakCode -ne 0) {
    Write-Host "DRY_RUN_RESULT: LEAK_FAIL ($leakCode)"
    exit $leakCode
}
Write-Host "DRY_RUN_RESULT: PASS"
exit 0
