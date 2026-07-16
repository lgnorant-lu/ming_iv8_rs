# Versioning (D-151 dual-track)

## Two tracks

| Track | Artifact | Moves when |
|---|---|---|
| **Milestone tag** | annotated `v0.8.x` | Each authorized Lightweight/Normal closeout |
| **Package** | `Cargo.toml` / `pyproject.toml` / locks | Only when package-track release is **explicitly authorized** |

**Package number need not equal milestone tag number.**  
Example: package **0.8.12** with tags `v0.8.100` … `v0.8.102`.

## Current (at doc write)

| Item | Value |
|---|---|
| Package | **0.8.12** |
| Latest milestone continuum | **v0.8.102** (see CHANGELOG) |

## Import

```python
import iv8_rs
iv8_rs.__version__  # package version string from native module
```

## See also

- `docs/roadmap/v0.8/shared/v0.8-release-and-tag-governance-closeout.md` (private governance detail)
- Root `CHANGELOG.md`
