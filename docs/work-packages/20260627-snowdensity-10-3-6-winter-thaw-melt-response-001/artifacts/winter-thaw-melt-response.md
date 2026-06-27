# SNOWDENSITY-10.3.6 Winter-Thaw Melt Response

Evidence mode: Static/Ran.

- Schema: `snowdensity10-3-6-winter-thaw-melt-response-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-048 INV-SNOWFREEZE-050`
- Runtime coupling: `diagnostic snowbench replay only; legacy_coe; no production activation`
- Rank source: SNOWDENSITY-10.3.4 rank-2 winter_thaw_melt_response
- No physics change: `True`
- No tuning: `True`
- Default activation changed: `False`
- Public output schema changed: `False`
- Disposition: `WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE`
- Next route: scaffold contract-first opt-in winter-thaw melt-response correction; preserve rain-heat and longwave as separate later levers

## Cohort Summary

| Metric | Value |
|---|---:|
| `paired_surface_count` | 4 |
| `observation_blocked_surface_count` | 3 |
| `event_window_count` | 1345 |
| `observed_ablation_interval_count` | 238 |
| `thaw_observed_ablation_interval_count` | 219 |
| `under_ablation_interval_count` | 132 |
| `under_ablation_fraction` | 0.60274 |
| `total_observed_depth_loss_m` | 37.1179 |
| `total_modeled_depth_loss_m` | 15.8682 |
| `total_depth_loss_deficit_m` | 24.1051 |
| `total_positive_temp_snowpack_hours` | 19166 |
| `total_raw_melt_m` | 8.68532 |
| `total_routed_melt_m` | 5.89518 |
| `total_snowpack_swe_loss_m` | 4.62814 |
| `warm_rain_heat_melt_equiv_m` | 0.189965 |

## Surface Event Windows

| Surface | Scope | Pairs | Windows | Thaw ablation windows | Under-ablation windows | Under-ablation fraction | Positive-temp snowpack h | Depth-loss deficit m | Warm-rain heat equiv m |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `hjandrews_conifer` | `OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY` | 0 | 0 | 0 | 0 | n/a | 33661 | 0 | 0 |
| `sleepers_south_field` | `PAIRED-OBSERVATION-EVENT-WINDOW` | 384 | 350 | 108 | 61 | 0.564815 | 11594 | 11.6052 | 0.104062 |
| `sleepers_w9_hardwood` | `PAIRED-OBSERVATION-EVENT-WINDOW` | 193 | 167 | 51 | 35 | 0.686275 | 6629 | 8.892 | 0.0611142 |
| `harvard_hardwood` | `PAIRED-OBSERVATION-EVENT-WINDOW` | 448 | 443 | 29 | 15 | 0.517241 | 500 | 1.62407 | 0.00850395 |
| `harvard_open` | `PAIRED-OBSERVATION-EVENT-WINDOW` | 390 | 385 | 31 | 21 | 0.677419 | 443 | 1.98383 | 0.0162849 |
| `hubbardbrook_deciduous` | `OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY` | 0 | 0 | 0 | 0 | n/a | 34125 | 0 | 0 |
| `hubbardbrook_mixed` | `OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY` | 0 | 0 | 0 | 0 | n/a | 35405 | 0 | 0 |

## Largest Under-Ablation Intervals

| Surface | Start | End | Days | Observed loss m | Modeled loss m | Deficit m | Positive-temp h | Raw melt m | SWE loss m |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| `sleepers_south_field` | 1996-04-09 | 1996-04-19 | 10 | 0.1278 | -0.542311 | 0.670111 | 157 | 0.0628806 | 0 |
| `sleepers_south_field` | 1996-01-18 | 1996-02-06 | 19 | 0.2772 | -0.284193 | 0.561393 | 54 | 0.027956 | 0.00671023 |
| `sleepers_south_field` | 2007-03-09 | 2007-03-26 | 17 | 0.366 | -0.124868 | 0.490868 | 163 | 0.0445837 | 0 |
| `sleepers_south_field` | 1986-03-10 | 1986-03-20 | 10 | 0.2198 | -0.167795 | 0.387595 | 118 | 0.0257545 | 0 |
| `sleepers_south_field` | 2013-12-18 | 2014-01-09 | 22 | 0.118 | -0.238807 | 0.356807 | 78 | 0.0131354 | 0 |
| `sleepers_south_field` | 2017-02-17 | 2017-02-28 | 11 | 0.506 | 0.161629 | 0.344371 | 134 | 0.0499994 | 0 |
| `sleepers_south_field` | 2016-02-02 | 2016-02-17 | 15 | 0.068 | -0.269497 | 0.337497 | 44 | 0.00494352 | 0 |
| `sleepers_south_field` | 2002-02-19 | 2002-03-12 | 21 | 0.3354 | 0.00405448 | 0.331346 | 153 | 0.012127 | 0 |
| `sleepers_w9_hardwood` | 1996-01-18 | 1996-02-06 | 19 | 0.31 | -0.312989 | 0.622989 | 51 | 0.0252499 | 0.00642014 |
| `sleepers_w9_hardwood` | 2005-12-20 | 2006-01-03 | 14 | 0.196667 | -0.360866 | 0.557533 | 45 | 0.00253402 | 0 |
| `sleepers_w9_hardwood` | 2017-02-17 | 2017-03-07 | 18 | 0.626 | 0.141583 | 0.484417 | 163 | 0.0567684 | 0 |
| `sleepers_w9_hardwood` | 2007-03-09 | 2007-03-27 | 18 | 0.354 | -0.0865447 | 0.440545 | 186 | 0.0550778 | 0 |
| `sleepers_w9_hardwood` | 2015-03-06 | 2015-04-08 | 33 | 0.214 | -0.202897 | 0.416897 | 150 | 0.00159741 | 0 |
| `sleepers_w9_hardwood` | 2019-03-19 | 2019-04-01 | 13 | 0.31 | -0.0946177 | 0.404618 | 109 | 0.0309853 | 0 |
| `sleepers_w9_hardwood` | 2007-12-31 | 2008-01-31 | 31 | 0.139 | -0.253723 | 0.392723 | 120 | 0.0491404 | 0 |
| `sleepers_w9_hardwood` | 2010-03-02 | 2010-03-24 | 22 | 0.486 | 0.106291 | 0.379709 | 304 | 0.103549 | 0.0668993 |
| `harvard_hardwood` | 2010-01-24 | 2010-01-25 | 1 | 0.11 | -0.183545 | 0.293545 | 15 | -0.00257088 | 0 |
| `harvard_hardwood` | 2014-01-05 | 2014-01-06 | 1 | 0.17 | -0.0639527 | 0.233953 | 11 | 0 | 0 |
| `harvard_hardwood` | 2008-12-24 | 2008-12-25 | 1 | 0.24 | 0.00975383 | 0.230246 | 12 | -0.00569144 | 0 |
| `harvard_hardwood` | 2009-12-11 | 2009-12-15 | 4 | 0.08 | -0.0416904 | 0.12169 | 36 | 0.00686437 | 0 |
| `harvard_hardwood` | 2014-02-20 | 2014-02-21 | 1 | 0.06 | -0.0298426 | 0.0898426 | 10 | 0 | 0 |
| `harvard_hardwood` | 2013-12-20 | 2013-12-21 | 1 | 0.13 | 0.0425142 | 0.0874858 | 18 | 0.0105607 | 0 |
| `harvard_hardwood` | 2013-12-22 | 2013-12-23 | 1 | 0.1 | 0.019717 | 0.080283 | 24 | 0.00675533 | 0.00392391 |
| `harvard_hardwood` | 2010-02-27 | 2010-02-28 | 1 | 0.07 | 0 | 0.07 | 6 | 0 | 0 |
| `harvard_open` | 2010-01-24 | 2010-01-25 | 1 | 0.1 | -0.169352 | 0.269352 | 15 | 0.00499024 | 0 |
| `harvard_open` | 2013-12-21 | 2013-12-22 | 1 | 0.26 | 0.0777243 | 0.182276 | 24 | 0.0234704 | 0 |
| `harvard_open` | 2009-02-21 | 2009-02-22 | 1 | 0.07 | -0.0968851 | 0.166885 | 7 | 0 | 0 |
| `harvard_open` | 2014-02-20 | 2014-02-21 | 1 | 0.11 | -0.0285171 | 0.138517 | 10 | 0 | 0 |
| `harvard_open` | 2009-02-27 | 2009-02-28 | 1 | 0.13 | 0 | 0.13 | 12 | 0 | 0 |
| `harvard_open` | 2013-12-19 | 2013-12-20 | 1 | 0.12 | 0.0256536 | 0.0943464 | 14 | 0.00275964 | 0 |
| `harvard_open` | 2008-12-24 | 2008-12-25 | 1 | 0.11 | 0.0156576 | 0.0943424 | 13 | -0.00192127 | 0 |
| `harvard_open` | 2013-03-14 | 2013-03-15 | 1 | 0.08 | -0.00472727 | 0.0847273 | 7 | 0 | 0 |

## Observation-Blocked Surfaces

| Surface | Positive-temp snowpack h | Reason |
|---|---:|---|
| `hjandrews_conifer` | 33661 | Fixture exists, but EDI MS007 / SNOTEL paired snow observations are not installed. |
| `hubbardbrook_deciduous` | 34125 | Fixture exists, but Hubbard Brook paired snow observations are not installed. |
| `hubbardbrook_mixed` | 35405 | Fixture exists, but Hubbard Brook paired snow observations are not installed. |

Conclusion: this package adjudicates winter-thaw melt response using observed snow-depth loss windows. Warm-rain heat and sub-canopy longwave are reported as context only and remain separate candidate levers.
