# Worker Handoff

Status: complete
Evidence mode: Static + Ran

## Current State

HPHYS0288 is executed-hold. Residual rain-on-snow release now routes through the baseline snowmelt forcing seam, and the full suite shows storage/ET/lateral improvement but no `Q`/`RM`/`Snow-Water` movement.

## Final Evidence Roots

Ran:
- Full semantic root: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z`
- Target trace root: `/tmp/hphys0288_target_traces_v13_20260604T162402Z`

## Key Observations

Ran:
- H1/H7/H39 traces include real released-rain rows with `snow_hourly_rain_released_sum_m`, `wb12_infiltration_m`, `wb18_theta_sum_m`, `wb13_rm_mm`, `q_m`, `wb13_snow_water_mm`, and `wb13_total_soil_mm`.
- Material top residual days remain spring 2014 and H7 2016:
  - H1 `[1,142,2014]`: candidate `RM=36.136740 mm`, `Snow-Water=88.187978 mm`, `Total-Soil=299.276292 mm`; released rain is `0` that day.
  - H39 `[1,143,2014]`: released rain `0.0017 m`, `wb12_infiltration=0.045601941 m`, `RM=45.601941 mm`, `Snow-Water=39.922453 mm`, `Total-Soil=260.556496 mm`.
  - H7 `[1,110,2016]`: candidate `RM=29.922681 mm`, `Snow-Water=5.263813 mm`, `Total-Soil=232.222427 mm`; released rain is `0` that day.

## Recommended Next Package

Scaffold HPHYS0289 focused on WB13/RM and winter runoff/snowpack publication lineage:

- Contract-first amend `SC-WATBAL-001`, `SC-RUNOFFPART-001`, and `SC-SNOWFREEZE-001` for WB13 `RM`/`Snow-Water` publication provenance where needed.
- Use target traces from `/tmp/hphys0288_target_traces_v13_20260604T162402Z` to compare `snow.hourly.melt_m`, `snow_hourly_rain_released_sum_m`, `wb12_infiltration_m`, `q_m`, `wb13_rm_mm`, and `wb13_snow_water_mm` on H1/H7/H39 material days.
- Diagnose why `RM`, `Q`, and `Snow-Water` remain unchanged despite corrected released-rain routing.
- Do not tune `Ep` or storage as compensation; use baseline-authoritative winter/runoff/WB13 publication lineage only.
- Keep HPHYS0287 fail-closed snow-state guards and HPHYS0288 `resolve_snow_partition_terms` centralization intact.

## Suggested Closure Criteria

- Contract-derived tests proving WB13 `RM` and `Snow-Water` consume the same baseline forcing/state lineage as the hydrology kernel.
- H1/H7/H39 trace rows showing movement in at least one of `RM`, `Q`, or `Snow-Water` on the current top residual days, or a clear baseline-cited hold explaining why movement is not expected.
- Full H1..H39 semantic metrics with deltas against HPHYS0288.
- Dual review, disposition, dual verification, and executed-hold/complete final disposition.
