# Winter Rain-On-Snow Melt Partition Localization

Status: complete
Evidence mode: Static + Ran

## Baseline Authority

Static:
- `/workdir/wepp-forest_260430_baseline/src/snowd.for` retains rain in low-density snow until the density/holding-capacity threshold, then leaves residual `hrrain` as liquid available to release.
- `/workdir/wepp-forest_260430_baseline/src/winter.for` adds positive residual `hrrain(hour)` to `hrmlt(hour,iplane)`, `totmel`, and `wmelt` after daily signed melt redistribution.
- `/workdir/wepp-forest_260430_baseline/src/wshirs.for` and `/workdir/wepp-forest_260430_baseline/src/grna.for` consume positive `wmelt` as runoff/infiltration event forcing (`smrate = wmelt / dur`).
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` includes `wmelt` in both infiltration input and runoff accounting.

## Defect

Static:
- Before HPHYS0288, openWEPP retained rain in snowpack but did not publish residual rain-on-snow release as routed `snow.hourly.melt_m_####`/`wmelt` forcing.
- The missing lineage made residual rain-on-snow remain on the direct-rain side of the partition rather than entering the baseline snowmelt forcing seam.

## Contract Vector

Ran:
- `cargo test --test hphys0288_winter_rain_snowmelt_partition_contract -- --nocapture` failed before production edits with: `released rain-on-snow must be routed through final hrmlt/wmelt; observed 0`.
- The vector uses 0.100 m snow depth, 340 kg/m3 density, 0.034 m SWE, and 0.003 m rain-on-snow.
- Expected baseline partition: 0.001 m retained to reach 350 kg/m3; 0.002 m released into routed melt forcing; signed snow storage `S = -0.001 m`.

## Implemented Correction

Static:
- `compute_active_snow_coupling` now records hourly `rain_released_m` when `hrrain` exceeds retained-rain capacity on active snow.
- Residual released rain is added to hourly routed melt after daily signed-melt redistribution, matching the `snowd.for -> winter.for` order.
- Signed snow storage excludes released rain so snowpack state only reflects physical melt, accumulation, and retained rain.
- WB12/WB14 liquid input reconciliation subtracts accumulation, retained rain, and released rain from direct rainfall while adding released rain to snow/runoff forcing.
- The HPHYS0245 trace schema now emits `snow_hourly_rain_released_sum_m` and closes snow storage against physical melt by subtracting released rain from routed melt.
