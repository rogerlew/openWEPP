# Worker Handoff

Evidence mode: Static.

SNOWDENSITY-06 is closed as density-compaction complete with no runtime
promotion. The density-only variant improves the intended density-cell profile,
but whole-rubric promotion is worse than legacy/as-built because the offline
physics-bulk path still carries the old degree-day melt surrogate.

Next recommended package: SNOWDENSITY-06B CoE-Bound Density Replay.

First actionable item: build an offline snowbench replay that feeds
`density_compaction_v1` state updates from fixed CoE melt/liquid/SWE-loss
operands (`legacy_coe`, and optionally opt-in `coe_shortwave_albedo_v1`) instead
of the physics-bulk degree-day melt surrogate. Preserve no-site-tuning and
fixed radiation/canopy/albedo/melt coefficients.
