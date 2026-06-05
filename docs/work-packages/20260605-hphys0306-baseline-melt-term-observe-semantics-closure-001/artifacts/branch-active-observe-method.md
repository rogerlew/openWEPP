# HPHYS0306 Branch-Active Observe Method

Static:

- Baseline active keys are fixed-comparator `melt.for` observe keys where `amelt` was emitted.
- openWEPP active keys are `snow_hourly_melt_branch_active == true` keys from the final `post_wb13` daily trace snapshot.
- Inactive fixed-baseline hours are not zero-imputed.
- Numeric forcing, snow-state, and melt-term comparisons are interpreted only after active-mask comparison and selected-snapshot conflict checks.
- Any active-mask mismatch routes to `branch-active-mask-hold` before numeric term correction.
- Any selected-snapshot branch-active conflict routes to `branch-active-parser-conflict-hold`.
- Numeric first-source classification is chronological; same-hour multi-symbol divergences remain HOLD.
