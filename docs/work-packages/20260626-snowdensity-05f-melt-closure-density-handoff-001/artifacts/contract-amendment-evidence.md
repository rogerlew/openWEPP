# Contract Amendment Evidence

Evidence class: Static.

## SC-SNOWFREEZE-001 v80

SNOWDENSITY-05F amended `SC-SNOWFREEZE-001` with:

- `INV-SNOWFREEZE-056` for melt closure / density handoff.
- `OBL-SNOWFREEZE-P-031` for density and activation producers consuming the
  opt-in melt boundary.
- Invalid-state guards for default activation, parser/runfile/CLI/output
  exposure, density-side melt retuning, and same-day future snowfall albedo
  continuity loss.
- Boundary disposition for `INV-SNOWFREEZE-056`.
- `SNOWDENSITY-05F Melt Closure / Density Handoff Addendum`.
- Revision-history row v80.

## Decision

`coe_shortwave_albedo_v1` is accepted only as an opt-in density-facing melt
interface. `legacy_coe` remains the production default and rollback path.
SNOWDENSITY-06 may consume the opt-in boundary, but it must not retune melt,
albedo, coefficients, or shared radiation to improve density signatures.

## Activation Evidence Baseline

05E's improvement versus diagnostic legacy is necessary context but insufficient
for default activation. Any default-candidate claim must report both:

- 05E diagnostic replay:
  `robust_fail_count 13 -> 10`, `robust_ordinal_score 61 -> 84`.
- H as-built context:
  `robust_fail_count=9`, `robust_ordinal_score=84`.

## Cold-Start Albedo

Same-day future snowfall is a required opt-in continuity case. A producer must
apply fresh-snow reset, carry a valid previous opt-in state, or fail closed; it
may not clear the opt-in albedo state solely because earlier same-day hours were
snow-free.
