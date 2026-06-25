# H02: Environment Fingerprint Consistency

> Created: 2026-06-25
> Status: candidate (spec defined, implementation basic)
> Harness Charter: HARNESS-CHARTER.md

## Domain

Environment fingerprint cross-field consistency — verifies that profile-driven
environment values are internally consistent (e.g. GPU renderer matches WebGL
vendor, screen dimensions match window dimensions, media preferences don't
contradict pointer type).

## Motivation

Detection scripts (CreepJS, BotD, FingerprintJS) check cross-field consistency:
- If `navigator.platform === 'Win32'` but `navigator.userAgent` contains
  `Macintosh`, that's a lie.
- If `webgl.UNMASKED_VENDOR_WEBGL` is `Google Inc. (NVIDIA)` but
  `webgl.UNMASKED_RENDERER_WEBGL` contains `Intel`, that's a lie.
- If `media.pointer === 'fine'` but `media.any-pointer === 'coarse'`,
  that's a contradiction.

## Evaluation Categories

### A. Data Correctness (mandatory)

Each environment field is checked against expected Chrome desktop values.

| Check | Description |
|---|---|
| A01 | navigator.platform matches UA OS |
| A02 | navigator.vendor matches UA browser |
| A03 | webgl vendor/renderer consistency |
| A04 | screen dimensions >= window dimensions |
| A05 | media.pointer/any-pointer consistency |
| A06 | media.hover/any-hover consistency |
| A07 | permissions states are valid (granted/denied/prompt) |
| A08 | audio baseLatency > 0 and < 1.0 |
| A09 | fonts.families is non-empty array |
| A10 | display.color-gamut in [srgb, p3, rec2020] |

### B. Edge Cases

| Check | Description |
|---|---|
| B01 | Empty profile defaults don't crash |
| B02 | Extra permissions map accepted |
| B03 | Media prefs with unusual values (e.g. prefers-color-scheme: dark) |

### C. False Positive Resistance (mandatory)

| Check | Description |
|---|---|
| C01 | Default profile passes all consistency checks |
| C02 | Profile with single field override still passes |
| C03 | Profile with contradictory fields is flagged |

## Thresholds

| Category | Threshold | Current Baseline |
|---|---|---|
| A | 100% pass | 10/10 |
| B | 100% pass | 3/3 |
| C | 100% pass | 3/3 |

## Implementation

Single command: `python scripts/evaluate_env_consistency.py`
Exit code: 0 = pass, 1 = fail

## Known Limitations

- Only checks default Chrome 147 Win10 profile
- Does not verify runtime JS behavior (only profile config consistency)
- Cross-layer checks (HTTP headers vs JS values) require v0.9+ HTTP layer
