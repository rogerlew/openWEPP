# Worker Handoff

Evidence mode: Static + Ran.

## Current State

SNOWDENSITY-10.3.15 is complete with disposition
`COMPLETE-DEFAULT-ACTIVATED-UNDER-ACTIVE-CAP`.

The direct-production no-env default is now:

- `snow_melt_model = coe_liquid_holding_capacity_v1`
- `snow_density_model = physics_bulk_density_compaction_v1`
- active density cap `522 kg m^-3`

Explicit rollback/test selectors remain:

- `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=legacy_coe`
- `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=legacy_wepp`

## Evidence

- No-env activation trace rows: `112,502` melt and `112,502` density rows.
- Rollback trace rows: `13,880` melt and `13,880` density rows on the
  representative rollback surface.
- Paired snow-depth failures remain `498/1415`.
- Frost attribution remains blocked by `SNOW-CONTROL-RESIDUALS-REMAIN`.
- Full closure gates passed: fmt, clippy, workspace tests, deny, anti-evasion.

## Do Next

Recommended next work should target residual snow-control failures after
activation, not another compaction-rate acceleration:

- open-surface ablation / wind-sublimation / exposed-surface melt realization
  for cap-limited mass rows; or
- contract-first dynamic density-cap re-anchor only if external authority and a
  full rerun are in scope.

Do not resume frost attribution until snow-control residuals are good enough to
isolate frost.
