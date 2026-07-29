# CAL-07B Terminal Verification - Agent A

Evidence class: `Static`

## Checklist

- PASS - Applicable package guidance was identified for the two Agent A output
  artifacts.
- PASS - Package scope is explicitly `DIAGNOSTIC_ONLY` and excludes production
  Rust, science-contract, fixture, test, CAL-07 input, clipping, normalization,
  and operator-replacement edits.
- PASS - The three frozen dates are unchanged: 2022-07-22, 2022-09-15, and
  2025-09-09.
- PASS - Source custody and terminal gate evidence are recorded in
  `artifacts/gate-evidence.md`; this verification did not rerun those gates.
- PASS - `artifacts/daily-decomposition.csv` records 24 hours for every case.
- PASS - `artifacts/attribution.csv` records valid hourly inventory and
  compatible daily/hourly metadata for every case.
- PASS - All 72 hourly-product VPD rows are represented as nonnegative by the
  summary evidence: `hourly_negative_count` is `0` for all cases and
  `any_hourly_product_vpd_negative` is `false`.
- PASS - Reconstructed contract-daily VPD and frozen CAL-07 contract-daily VPD
  are negative for all three cases, and daily signs agree.
- PASS - Daily operand reconstruction is within the frozen serialized
  resolution tolerance for all three cases.
- PASS - The additive decomposition supports the stated driver: the
  temperature-extrema summary term is negative and dominant, dew-point
  nonlinearity is positive and smaller, and closure residuals are within the
  `1e-9 Pa` reconstruction gate.
- PASS - Figure sidecars bind the figures to retained table hashes, describe
  limitations, and do not claim instantaneous atmospheric truth or production
  correction authority.
- PASS - The final disposition, roadmap, and work-package catalog retain
  CAL-07 on hold and state that CAL-07B does not resume CAL-07, authorize
  clipping, or replace OBL-PLANT-P-013.

## Final verification

GO for CAL-07B diagnostic completion.

CAL-07 and roadmap Order 7 remain HOLD.
