# Serial iv8-core integration gate (avoids link.exe / disk thrash from parallel test binaries).
# Usage: pwsh scripts/_cargo_test_serial.ps1 [-TargetDir D:\Caches\cargo-target]
param(
  [string]$TargetDir = "D:\Caches\cargo-target",
  [switch]$LibOnly
)
$ErrorActionPreference = "Stop"
$env:RUST_MIN_STACK = "134217728"
function Invoke-CargoTest([string[]]$Args) {
  & cargo @Args
  if ($LASTEXITCODE -ne 0) { throw "cargo failed: $Args" }
}
Write-Host "== lib =="
Invoke-CargoTest @("test","-p","iv8-core","--lib","--target-dir",$TargetDir)
if ($LibOnly) { exit 0 }
$tests = Get-ChildItem "crates/iv8-core/tests" -Filter "test_*.rs" | ForEach-Object { $_.BaseName }
$failed = @()
foreach ($t in $tests) {
  Write-Host "== $t =="
  & cargo test -p iv8-core --target-dir $TargetDir --test $t -- --test-threads=1
  if ($LASTEXITCODE -ne 0) { $failed += $t }
}
if ($failed.Count) {
  Write-Host "FAILED: $($failed -join ',')"
  exit 1
}
Write-Host "ALL PASS ($($tests.Count) integration + lib)"
