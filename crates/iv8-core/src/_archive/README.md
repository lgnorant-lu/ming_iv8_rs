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
| shims/tier1_stubs.js | v0.8.27 | 2026-06-12 | 716 empty constructors; replaced by 1284 IDL FunctionTemplates from install_browser_surface_init |
| shims/tier1_stubs.rs | v0.8.27 | 2026-06-12 | Loader for tier1_stubs.js; removed with its JS payload |
| shims/browser_apis.rs | v0.8.27 | 2026-06-12 | Loader for browser_apis.js; removed with its JS payload |
| dom/browser_apis.js | v0.8.27 | 2026-06-12 | API existence stubs (navigator.bluetooth, navigator.usb, etc.); replaced by 1284 IDL templates |
| kernel/embedded_v8.rs | v0.8.27 | 2026-06-12 | Legacy 20-step init chain snapshot from v0.8.24 tag; replaced by install_browser_surface_init default path |

