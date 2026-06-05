# HPHYS0301 H39 Forcing/Release Lineage Summary

Evidence mode: static + ran

Static:

- Baseline partition authority: `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95`.
- Baseline rain-retention authority: `/workdir/wepp-forest_260430_baseline/src/snowd.for:240-279`.
- Baseline daily routed-melt authority: `/workdir/wepp-forest_260430_baseline/src/winter.for:420-476`.
- OpenWEPP forcing authority: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`.

Ran:

- Parsed HPHYS0300 H39 trace and pinned-baseline observe artifacts from `/tmp/hphys0300_full_20260605T155527Z`.

## Totals

| Metric | Value (mm) |
|---|---:|
| `baseline_residual_rain_mm` | 30.769731 |
| `openwepp_raw_rain_mm` | 47.246717 |
| `openwepp_retained_rain_mm` | 16.928079 |
| `openwepp_released_plus_post_rain_mm` | 31.006924 |
| `baseline_minus_open_raw_rain_mm` | -16.476986 |
| `baseline_minus_open_released_plus_post_rain_mm` | -0.237193 |
| `baseline_snowfall_depth_mm` | 220.540730 |
| `openwepp_snowfall_depth_mm` | 217.149972 |
| `baseline_minus_open_snowfall_depth_mm` | 3.390758 |
| `baseline_raw_melt_mm` | 21.502584 |
| `openwepp_raw_melt_mm` | 37.646948 |
| `baseline_minus_open_raw_melt_mm` | -16.144364 |
| `baseline_post_wmelt_mm` | 52.272600 |
| `openwepp_routed_melt_mm` | 40.902416 |
| `baseline_minus_open_routed_melt_mm` | 11.370184 |

## Decision

- Route: `h39-rain-release-lineage-reclassified-hold`.
- Production edit authorized: `false`.
- Forcing root cause confirmed: `false`.
- HPHYS0300's `baseline_minus_open_raw_rain_mm = -16.476985` raw-rain comparison is not production forcing authority because it compares baseline residual rain-on-snow evidence against openWEPP raw rain.
- Comparing baseline residual rain to openWEPP released plus post-winter rain leaves a sub-millimeter aggregate residual, so H39 first-2013 must move to rain-retention/post-raw melt lineage closure.
- Remaining `hrmlt`/`wmelt` residuals require paired `melt.for`/`snowd.for` term/state evidence before a production snow producer edit.

## Daily Rows

| Day | Base Resid Rain | Open Raw Rain | Open Retained | Open Released+Post | Δ Raw | Δ Released+Post | Base Snow Depth | Open Snow Depth | Δ Snow Depth |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 97 | 4.519662 | 7.417500 | 2.967000 | 4.450500 | -2.897838 | 0.069162 | 24.725000 | 24.725000 | -0.000000 |
| 98 | 1.365030 | 1.365000 | 0.000000 | 1.365000 | 0.000030 | 0.000030 | 22.750500 | 22.750000 | 0.000500 |
| 99 | 0.296253 | 0.296250 | 0.000000 | 0.296250 | 0.000003 | 0.000003 | 4.937550 | 4.937500 | 0.000050 |
| 100 | 2.216850 | 4.420000 | 2.281338 | 2.138662 | -2.203150 | 0.078188 | 0.000000 | 0.000000 | 0.000000 |
| 101 | 0.780000 | 0.975000 | 0.292500 | 0.780000 | -0.195000 | 0.000000 | 8.775000 | 8.775000 | 0.000000 |
| 102 | 6.832000 | 6.832000 | 0.000000 | 6.832000 | 0.000000 | 0.000000 | 17.080000 | 17.080000 | 0.000000 |
| 103 | 3.051450 | 3.051429 | 0.000000 | 3.390476 | 0.000021 | -0.339026 | 40.686000 | 37.295238 | 3.390762 |
| 104 | 2.570600 | 2.570588 | 0.000000 | 2.570588 | 0.000012 | 0.000012 | 17.994200 | 17.994118 | 0.000082 |
| 105 | 0.148749 | 0.148750 | 0.000000 | 0.148750 | -0.000001 | -0.000001 | 10.412430 | 10.412500 | -0.000070 |
| 106 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 |
| 107 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 | 0.000000 |
| 108 | 0.862497 | 0.862500 | 0.000000 | 0.862500 | -0.000003 | -0.000003 | 14.374950 | 14.375000 | -0.000050 |
| 109 | 0.000000 | 5.376250 | 5.376250 | 0.000000 | -5.376250 | 0.000000 | 22.137500 | 22.137500 | 0.000000 |
| 110 | 0.000000 | 5.034783 | 5.034783 | 0.251739 | -5.034783 | -0.251739 | 5.034800 | 5.034783 | 0.000017 |
| 111 | 6.930000 | 7.700000 | 0.976208 | 6.723792 | -0.770000 | 0.206208 | 7.700000 | 7.700000 | 0.000000 |
| 112 | 1.196640 | 1.196667 | 0.000000 | 1.196667 | -0.000027 | -0.000027 | 23.932800 | 23.933333 | -0.000533 |
