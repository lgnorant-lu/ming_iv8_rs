# _archive/

Archived historical code. These files have been removed from the active
codebase but are retained for historical reference.

## Directory Structure

- `shims/` -- Deprecated JS shim files
- `dom/` -- Deprecated DOM-related files
- `kernel/` -- Deprecated kernel init chain (reserved, filled in v0.8.24)
- `stubs/` -- Deprecated constructor stubs (reserved, filled in v0.8.24)

## Archive Records

| File | Archived Version | Archive Date | Reason |
|------|-----------------|--------------|--------|
| shims/dom_prototypes.rs | v0.8.23 | 2026-06-12 | Dead code, replaced by FunctionTemplate system |
| shims/element_prototypes.rs | v0.8.23 | 2026-06-12 | Dead code, replaced by FunctionTemplate system |
| dom/navigation.rs | v0.8.23 | 2026-06-12 | Comment-only placeholder file, implementation lives in dom/template.rs |
