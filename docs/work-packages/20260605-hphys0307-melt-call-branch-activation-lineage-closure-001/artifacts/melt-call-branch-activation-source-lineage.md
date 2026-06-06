# HPHYS0307 Melt-Call Branch Activation Source Lineage

Static:

- Fixed comparator commit: `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- Baseline winter driver: `/workdir/wepp-forest_260430_baseline/src/winter.for`
- Baseline snow driver: `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- openWEPP branch publisher: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`

## Baseline Predicate

- `/workdir/wepp-forest_260430_baseline/src/winter.for:366-373` calls
  `call snowd(iresd(1,iplane),denh2o,iplane,driftf,driftg,snodep,densgy,densgt,smelt,hour)`
  for each winter hour and then publishes `hrmlt(hour,iplane) = wmelt(iplane)`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:70-90` handles the
  no-existing-snowpack lanes without `call melt`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:116-174` handles the
  freezing daily-mean branch and new-snow/drift accumulation without
  `call melt`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:180-193` enters the
  non-freezing daily-mean existing-snowpack branch and calls
  `call melt(irtype,wrain,hour)` when `snodep .gt. 0.0`.

## openWEPP Predicate

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3887`
  initializes `melt_branch_active = 0.0`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3925-3936`
  enters the non-freezing snowpack branch and requires `snodep > WB11_ZERO_THRESHOLD`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3937-3949`
  invokes `compute_simimpl29_melt_hour` and then publishes
  `melt_branch_active = 1.0`.

## Classification Rule

- `baseline-extra-melt-call`: fixed baseline reached `melt.for` for keys where
  openWEPP published inactive.
- `openwepp-extra-melt-call`: openWEPP published active keys with no paired
  fixed-baseline `melt.for` observation.
- `matched-branch-active-same-hour-multi-source`: active masks match but the
  first active-domain divergence has multiple sources.

No production code edit is authorized by classification alone; source-line proof
must identify an openWEPP branch-predicate defect first.
