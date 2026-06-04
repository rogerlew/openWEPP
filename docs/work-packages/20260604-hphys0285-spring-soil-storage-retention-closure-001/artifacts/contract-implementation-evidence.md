# Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Contract Amendments

Static:
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - Bumped `contract_version` to `26`.
  - Added `INV-PERC-017`, requiring WB18 to apply positive local same-pass `fin/xfin` infiltration to layer storage before percolation and aggregate `watcon` recomputation.
  - Explicitly prohibits active-snow state from gating non-snow local direct-rain/irrigation infiltration storage ingress; MOFE carry/runon storage ingress is deferred to follow-up scope.
  - Records hourly cadence: apply `xfin = fin / ui_LFtstpF` per lane substep rather than one daily pulse.
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - Bumped `contract_version` to `29`.
  - Added `REF-RUNOFFPART-LEGACY-FIN-INFIL` and `INV-RUNOFFPART-016`, tying WB12/WB14 local infiltration publication to WB18 storage-ingress availability for direct rain, routed melt, and irrigation.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Bumped `contract_version` to `104`.
  - Added `REF-WATBAL-LEGACY-HOURLY-FIN` and `INV-WATBAL-060`, making `Total-Soil`/`SoilWaterTotal` closure consume post-ingress WB18 storage for local same-pass infiltration.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - Bumped `contract_version` to `21`.
  - Clarified `INV-SNOWFREEZE-019` for pack-exhaustion after corrected negative-melt state-loss: within-tolerance exhausted SWE canonicalizes to zero instead of publishing negative snow storage or blocking downstream waterbalance.
  - Added bounded fail-closed behavior for material carried state-loss overdraw above `0.005 m` water equivalent per day, resolving `CLAUDE-0285-001`.

## Baseline Authority

Static:
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:342-345` defines local `fin = rain - interception + wmelt + irrigation`.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:471-479` computes hourly `xfin = fin / ui_LFtstpF`.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:494-516` routes same-pass `xfin` into layer `st(i)` before percolation.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:520-545` routes remaining percolation after same-pass layer ingress.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1018-1025` recomputes `soilw(i)`/`watcon` from mutated layer storage.
