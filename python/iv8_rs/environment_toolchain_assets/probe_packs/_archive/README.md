# Deprecated Probe Packs (archived v0.8.31)

fingerprint.m1.json and descriptor.m1.json are hand-written probe packs
retired in v0.8.31. They will be replaced by auto-generated probes from
the IDL-to-probe compiler in v0.8.32+.

These files remain in version control as references for the probe format
schema, but the active probe pack loading path will migrate to auto-generated
probes once the compiler is implemented.

## Schema Reference

- fingerprint.m1.json: 14 baseline surface probes (Navigator/Screen/Document/etc.)
- descriptor.m1.json: 8 descriptor probes (property descriptor shape)

## Migration Plan

v0.8.32: IDL-to-probe compiler generates equivalent probes from unified_ir.json
v0.8.33: Archived files removed from version control
