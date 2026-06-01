# HPHYS0230 Contract Implementation Evidence

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Contract Updates (Ran)

Updated canonical authority:
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`

Applied amendments:
1. `contract_version` advanced to `15`; `last_reviewed` set to `2026-06-01`.
2. WB18 conductivity-shape authority changed from constant exponent to
   dynamic per-layer derivation:
   - `Bi = -2.655 / log10(FC/UL)`.
3. Added explicit `FC/UL` domain obligation (`0 < FC/UL < 1`) and recorded the
   HPHYS0230 addendum prohibiting constant exponent substitution.
4. Constant table updated to `WB18_PERC_BI_COEFFICIENT = 2.655`.

## Production Implementation Mapping (Ran)

Updated runtime implementation:
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`

Mapping:
1. Replaced `WB18_PERC_SHAPE_EXPONENT` with
   `WB18_PERC_BI_COEFFICIENT`.
2. Derived per-layer `bi` from `fc/ul` in the WB18 loop and applied
   `stz.powf(bi)` for `fx` under the active branch.
3. Retained typed domain hard-fail posture (`HKERNEL-WB11-PERC-E-003`) for
   non-finite/out-of-domain ratio states.

## Measure Mapping

- `MEASURE-HP230-001`: satisfied.
- `MEASURE-HP230-003`: satisfied.
