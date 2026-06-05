# Post-Winter Rain Publication Localization

Status: complete
Evidence mode: Static + Ran

## Baseline Authority

Static:

- `/workdir/wepp-forest_260430_baseline/src/contin.for:847-880` calls `winter(rain(iplane), snoflg)`, clears `rain(iplane) = 0.0`, then restores `rain(iplane) = warain` only when `tmin >= 0.0` and `warain > 0.0`.
- `/workdir/wepp-forest_260430_baseline/src/winter.for:456-464` adds residual hourly `hrrain` to `hrmlt` before publishing `wmelt(iplane)=totmel`.
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for:84-106` and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1082-1142` publish `RM = rain + wmelt + irrigation`.

## openWEPP Defect

Static:

- HPHYS0289 made routed `wmelt` explicit as `snow.routed_melt_m` but left post-winter rain inferred inside WB13 from raw `prcp`, runtime SWE, and routed melt activity.
- That inference could not distinguish baseline winter-cleared rain from warm-rain/no-snow restoration and allowed stale or reconstructed publication behavior.

## Implemented Lineage

Static:

- WB12/WB14 now publish `snow.post_winter_rain_m` from the same post-snow liquid partition that feeds direct-rain runoff/infiltration forcing.
- WB13 now consumes explicit `snow.post_winter_rain_m + snow.routed_melt_m + Irr` and fails closed on missing, negative, or non-finite post-winter rain.
- WB13 requires `snow.post_winter_rain_m` from the same-day flux surface; state-only defaults or stale state values cannot satisfy publication.
- The unit registry declares `snow.post_winter_rain_m` as a typed required non-negative finite depth under `SC-SNOWFREEZE-001#INV-SNOWFREEZE-023`.

## Runtime Observation

Ran:

- Full H1..H39 root: `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix`
- H1/H7/H39 trace root: `/tmp/hphys0290_target_traces_current_20260605T011834Z_postfix`
- H39 2014-146 now records `snow_post_winter_rain_m = 0.002620000 m`, `snow_routed_melt_m = 0.0 m`, and `WB13 RM = 2.620000 mm`.

Interpretation: H39 2014-146 is not a WB13 inference defect after HPHYS0290; it is the baseline warm-rain restoration branch. Remaining semantic residuals should be pursued upstream in snowpack timing/state and liquid/storage partitioning, not by changing WB13 `RM` publication math.
