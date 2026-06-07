# Disposition

Status: complete

Evidence mode: Static + Ran.

## Decision

`FQ3-DC-ET-CORN-ENGAGEMENT-001` is closed for the observed Corn annual
plant/canopy engagement defect.

The in-envelope root cause was loss/misprojection of annual PL activation
control: pre-plant filtering deleted the PL activation sentinel, and the
scheduler used day-of-month instead of Julian day for annual planting
activation. After that correction, a WB15 consumer guard was amended to match
pinned-baseline interception semantics: finite non-negative live biomass state
is accepted, while only the interception equation biomass input is capped at
`8000 kg ha^-1`.

## Acceptance Criteria

- Corn `Ep` engagement: satisfied (`36/36` Corn prefixes nonzero).
- Corn canopy `Interception` engagement: satisfied (`36/36` Corn prefixes
  nonzero).
- `Er`: original package wording overclaimed this as a defect; upstream FQ-3
  classified `Er=0` as expected-config-zero with legacy `Er=0`. No unresolved
  `Er` defect remains in this package.
- Perennial non-regression: satisfied on p1 (`Ep=5475.201811235968`,
  `Interception=643.3614332068395`). The prior p1 `Ep` reference was
  approximately `5511 mm`; the `-36 mm` (`-0.65%`) delta is accepted as a
  non-blocking corrected-calendar perturbation from publishing Julian scheduler
  `day` rather than day-of-month across management paths.
- Conservation closure: satisfied (Corn population annual max abs residual
  `3.1604940886609256e-11 mm`).
- Contract-derived tests: satisfied.
- No comparator tuning or protected-boundary compensation: satisfied.
- Dual review and dual verification: satisfied.

## Finding Disposition

- Accepted Review A finding 1: `Er` overclaim corrected in validation and
  disposition.
- Accepted Review A finding 2: Julian-day scheduler symbol implemented and
  verified.
- Accepted Review B finding 1: RUNOFFPART touch classified as WB15 consumer
  contract mirroring, not runoff tuning.
- Accepted Review B finding 2: high-biomass interception equation input cap
  implemented and verified.
- Accepted Claude review finding F4: perennial p1 `Ep` delta recorded and
  classified as non-blocking corrected-calendar perturbation.
- Follow-up Claude review finding F5: runoff-magnitude characterization must use
  the post-interception Corn `Q` baseline (`p8 Q=320.73667698020574`) rather
  than the runoff-DC-alone value (`p8 Q≈513`) if opened later.

No undispositioned findings remain.
