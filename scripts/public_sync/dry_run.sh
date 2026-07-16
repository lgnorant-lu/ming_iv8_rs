#!/usr/bin/env bash
# Public dual-repo dry-run (Linux/macOS). See dry_run.ps1 for semantics.
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$REPO_ROOT"
WORK_ROOT="${1:-${TMPDIR:-/tmp}/opencode-public-dry-run-$(date +%Y%m%d-%H%M%S)}"
CLONE_DIR="$WORK_ROOT/filtered-repo"
REPORT_DIR="$WORK_ROOT/report"
KEEP_LIST="$REPO_ROOT/scripts/public_sync/generated/keep-paths.txt"
mkdir -p "$WORK_ROOT" "$REPORT_DIR"

echo "== build keep paths =="
uv run python scripts/public_sync/build_keep_paths.py --print-stats -o "$KEEP_LIST"

echo "== ensure git-filter-repo =="
uv run python -c "import git_filter_repo" 2>/dev/null || uv pip install git-filter-repo

echo "== clone =="
rm -rf "$CLONE_DIR"
git clone --no-local "$REPO_ROOT" "$CLONE_DIR"
cd "$CLONE_DIR"
git remote remove origin 2>/dev/null || true

echo "== filter-repo =="
# Use monorepo uv project so git-filter-repo is resolvable (not the clone's empty venv)
uv run --project "$REPO_ROOT" python -m git_filter_repo --force --paths-from-file "$KEEP_LIST"
git ls-files > "$REPORT_DIR/filtered-ls-files.txt"
git log --oneline -20 > "$REPORT_DIR/filtered-log-head.txt"

echo "== LEAK scan =="
set +e
uv run python "$REPO_ROOT/scripts/public_sync/leak_scan.py" \
  --root "$CLONE_DIR" \
  --report "$REPORT_DIR/leak-scan.md"
LEAK=$?
set -e

{
  echo "# Public dry-run summary"
  echo "- work_root: $WORK_ROOT"
  echo "- keep_count: $(wc -l < "$KEEP_LIST")"
  echo "- filtered_files: $(wc -l < "$REPORT_DIR/filtered-ls-files.txt")"
  echo "- leak_exit_code: $LEAK"
  echo "- push: NEVER"
} | tee "$REPORT_DIR/SUMMARY.md"

exit "$LEAK"
