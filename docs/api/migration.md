# GUIDE ↔ API map & migration notes

## GUIDE section → API page

| GUIDE (approx.) | API |
|---|---|
| §1 Install | README + [overview.md](overview.md) |
| §2 Basic usage | [runtime/jscontext.md](runtime/jscontext.md) |
| §3–6 Environment / DOM / events | jscontext + profiles |
| §7 Anti-detection | overview bounds + GUIDE |
| §8–10 Debugger / CDP / Trace | [runtime/debugger.md](runtime/debugger.md), jscontext CDP |
| §11–16 VM / instrument_source | [instrumentation/](instrumentation/) |
| §17–21 Canvas / Profile / UAData | profiles + GUIDE |
| §22–31 Analysis modules | [analysis/](analysis/) |
| §32+ Entry / reports / toolchain | [entry/](entry/), [environment/](environment/), [reports/](reports/) |
| §129–131 v0.8.101–102 | instrumentation + overview ICU/DOM |

## Breaking / honesty changes worth remembering

| Topic | Note |
|---|---|
| ICU | Requires ICU 77 data file; wrong major fails Intl |
| ChaosVM | Prefer `instrument_source` for closure handlers |
| Bundler network | Offline-first; no silent ensureChunk fetch |
| D-151 | Package version ≠ milestone tag |

## Sample adapters

`docs/samples/adapters/*` are **sample-track**, not stable product API.  
Public examples (if any) will be maintained separately from private sample registry.
