# Unit Governance Gap Analysis

Status: queued
Evidence mode: static

Static:

HPHYS0272 exposed a unit-governance defect rather than a narrow radiation
formula defect. The repo already has several partial unit patterns, but they are
not governed as one enforceable system.

## Current Patterns

- Science contracts require `Variables and Units` tables and alias maps, but
  compliance is mostly human-reviewed.
- `openwepp-unit-boundary` defines unit-safe wrappers for a few hydrologic
  boundary classes.
- `BoundaryValue` supports typed variants, but most runtime surfaces still use
  raw `BoundaryValue::scalar`.
- Runtime symbol names often include suffixes such as `_m`, `_mm`, `_m_s`,
  `_kg_m3`, and `_mj_m2`, but legacy names such as `P`, `RM`, `Ep`, and `rad`
  remain unit-implicit.
- Hillslope and watershed output writers attach Parquet unit metadata locally,
  independent of runtime boundary-symbol authority.
- Conversion factors are scattered in production code and tests as raw numeric
  literals or package-local constants.

## Governance Gaps

| Gap | Risk | Follow-Up |
| --- | --- | --- |
| No canonical unit-governance standard | Agents can satisfy local package text while missing repo-wide unit rules | HPHYS0273 |
| No machine-readable boundary-symbol unit registry | Producers and consumers can disagree on units despite matching symbol names | HPHYS0274 |
| Dimensional values still cross runtime seams as raw scalars | Unit mistakes are not rejected mechanically at construction/extraction | HPHYS0275 |
| Raw dimensional conversion literals are not centrally governed | Correct constants can be applied in the wrong direction or wrong context | HPHYS0276 |
| Finite but physically impossible hourly radiation is not production-guarded | Future unit mistakes can pass non-finite checks and drive physics | HPHYS0277 |
| Output metadata is not registry-backed | Publication schemas can drift from runtime units | HPHYS0278 |
| SC contract unit compliance is not linted | Missing units/alias-unit checks remain manual review problems | HPHYS0279 |

## Required Standardization Direction

- Use canonical `SC-*` symbols and units as source authority.
- Make dimensional runtime boundary units registry-backed.
- Treat `BoundaryValue::scalar` as dimensionless by default, with explicit
  exceptions for legacy compatibility surfaces.
- Use named directional conversion helpers for every dimensional conversion.
- Fail closed on invalid or physically impossible unit states; never clip or
  silently convert to make a value plausible.
- Make output metadata consume the same unit authority as runtime surfaces.

Ran: not-run.
