# PARADIGM-2 Stage 2 Frost-Primary Rubric

Schema: `paradigm2-stage2-snow-frost-insulation-profile-v1`
Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-079 INV-SNOWFREEZE-050 ADR-0029`
Evidence: `Static + Ran (completed outputs reused)`

## Summary

- Gate passed: `False`
- Reason: available=22; fail_delta=0; score_delta=0; improved_cells=0; worsened_cells=0
- Bulk robust fails/score: `3` / `49`
- Candidate robust fails/score: `3` / `49`
- Primary improved/worsened cells: `0` / `0`

## Aggregate

| Model | Primary robust counts | Primary score | Limited report-only counts |
|---|---|---:|---|
| stage1_layered_density_bulk_snow_frost_handoff | {'fail': 3, 'marginal': 3, 'pass': 2, 'strong': 14} | 49 | {'fail': 4, 'marginal': 1, 'pass': 1, 'unavailable': 4} |
| stage2_layered_resistance_v1 | {'fail': 3, 'marginal': 3, 'pass': 2, 'strong': 14} | 49 | {'fail': 3, 'marginal': 3, 'unavailable': 4} |

## Primary Cell Deltas

| Site | Cell | Bulk | Candidate | Delta |
|---|---|---:|---:|---:|
| site1_sleepers_south_field_vt | frost_frozen_duration | strong | strong | 0 |
| site1_sleepers_south_field_vt | frost_isotherm_upper_bound | unavailable | unavailable | 0 |
| site1_sleepers_south_field_vt | frost_measurement_correspondence | strong | strong | 0 |
| site1_sleepers_south_field_vt | frost_onset_timing | strong | strong | 0 |
| site1_sleepers_south_field_vt | frost_thaw_timing | strong | strong | 0 |
| site2_sleepers_w9_hardwood_vt | frost_frozen_duration | strong | strong | 0 |
| site2_sleepers_w9_hardwood_vt | frost_isotherm_upper_bound | unavailable | unavailable | 0 |
| site2_sleepers_w9_hardwood_vt | frost_measurement_correspondence | strong | strong | 0 |
| site2_sleepers_w9_hardwood_vt | frost_onset_timing | strong | strong | 0 |
| site2_sleepers_w9_hardwood_vt | frost_thaw_timing | pass | pass | 0 |
| site4_ggd498_morris_mn | frost_frozen_duration | strong | strong | 0 |
| site4_ggd498_morris_mn | frost_isotherm_upper_bound | unavailable | unavailable | 0 |
| site4_ggd498_morris_mn | frost_measurement_correspondence | strong | strong | 0 |
| site4_ggd498_morris_mn | frost_onset_timing | strong | strong | 0 |
| site4_ggd498_morris_mn | frost_thaw_timing | strong | strong | 0 |
| site3_scan_mandan_nd | frost_frozen_duration | fail | fail | 0 |
| site3_scan_mandan_nd | frost_isotherm_upper_bound | fail | fail | 0 |
| site3_scan_mandan_nd | frost_measurement_correspondence | strong | strong | 0 |
| site3_scan_mandan_nd | frost_onset_timing | marginal | marginal | 0 |
| site3_scan_mandan_nd | frost_thaw_timing | fail | fail | 0 |
| site5_reynolds_creek_us_rls_id | frost_frozen_duration | marginal | marginal | 0 |
| site5_reynolds_creek_us_rls_id | frost_isotherm_upper_bound | pass | pass | 0 |
| site5_reynolds_creek_us_rls_id | frost_measurement_correspondence | strong | strong | 0 |
| site5_reynolds_creek_us_rls_id | frost_onset_timing | strong | strong | 0 |
| site5_reynolds_creek_us_rls_id | frost_thaw_timing | marginal | marginal | 0 |

## Limited Frost-Depth Cells

| Site | Cell | Bulk | Candidate | Delta |
|---|---|---:|---:|---:|
| site1_sleepers_south_field_vt | frost_depth_timeseries | fail | marginal | 1 |
| site1_sleepers_south_field_vt | frost_max_depth_bias | marginal | marginal | 0 |
| site2_sleepers_w9_hardwood_vt | frost_depth_timeseries | fail | fail | 0 |
| site2_sleepers_w9_hardwood_vt | frost_max_depth_bias | fail | fail | 0 |
| site4_ggd498_morris_mn | frost_depth_timeseries | fail | fail | 0 |
| site4_ggd498_morris_mn | frost_max_depth_bias | pass | marginal | -1 |
| site3_scan_mandan_nd | frost_depth_timeseries | unavailable | unavailable | 0 |
| site3_scan_mandan_nd | frost_max_depth_bias | unavailable | unavailable | 0 |
| site5_reynolds_creek_us_rls_id | frost_depth_timeseries | unavailable | unavailable | 0 |
| site5_reynolds_creek_us_rls_id | frost_max_depth_bias | unavailable | unavailable | 0 |
