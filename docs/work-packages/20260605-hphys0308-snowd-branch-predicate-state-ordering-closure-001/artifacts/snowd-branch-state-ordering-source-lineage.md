# HPHYS0308 Snowd Branch State-Ordering Source Lineage

Static:

- Fixed comparator commit: `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- Baseline source: `/workdir/wepp-forest_260430_baseline/src/snowd.for:116-193`
- openWEPP source: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3889-3949`

## Baseline Predicate

- `/workdir/wepp-forest_260430_baseline/src/snowd.for:116` selects the
  freezing/non-freezing daily mean branch using `(tmax + tmin)/2`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:180-193` enters the
  non-freezing branch, computes `snodep`, requires `snodep .gt. 0.0`, and
  executes `call melt(irtype,wrain,hour)`.

## openWEPP Predicate and State Surfaces

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3889-3936`
  routes no-snow, freezing, and non-freezing snowpack branches.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3937-3949`
  invokes `compute_simimpl29_melt_hour` and sets `melt_branch_active = 1.0`.
- HPHYS0308 evidence reads `snow_hourly_depth_before_m`,
  `snow_hourly_snowfall_depth_m`, `snow_hourly_depth_available_m`,
  `snow_hourly_depth_after_m`, and density/forcing maps at each branch-extra
  key.

## Closure Rule

- Baseline-extra keys with openWEPP zero depth surfaces are
  `snow-state-carry-depletion-hold`, not branch-predicate edit authority.
- openWEPP-extra keys without fixed-baseline `melt.for` observations are
  `baseline-branch-instrumentation-hold`.
- Any production branch-predicate edit requires direct source-line evidence
  beyond aggregate active-mask counts.
