# Snow Carry/Depletion Source Lineage

Status: complete

Evidence mode: static

- Baseline snow-state entry/carry: `/workdir/wepp-forest_260430_baseline/src/snowd.for:50-53`
  initializes hourly `snodep`/`snodpt`/`densgy`/`densgt` from carried
  `snodpy`/`densg`.
- Baseline melt depletion: `/workdir/wepp-forest_260430_baseline/src/snowd.for:215-230`
  records pre-melt `snodpt`, subtracts `smelt`, and clamps all-melted pack.
- Baseline carry publication: `/workdir/wepp-forest_260430_baseline/src/snowd.for:303-312`
  caps density, zeroes density for zero depth, and writes `snodpt`, `snodpy`,
  and `densg` for the next hour/day.
- Fixed negative-melt comparator: branch `wepp_260430_negmeltfix_comparator`,
  tag `wepp_260430_negmeltfix_comparator_47ac4c32faee`, commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`, `src/winter.for:434-453`,
  with patch provenance in
  `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-comparator-source-delta.patch`.
  This compares net daily melt, scales positive routed melt by
  `1 + ngtvML/pstvML`, and applies the companion carried-depth adjustment.
- openWEPP hourly snow state publication:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3879-4105`
  records hourly before/after depth/density and branch-active surfaces.
- openWEPP runtime carry publication:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4111-4177`
  computes daily runtime SWE/depth after redistributed state loss.
- openWEPP signed-melt redistribution:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4231-4277`
  preserves corrected routed melt and carried state-loss lineage.
