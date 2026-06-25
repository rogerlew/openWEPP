# Non-SNOTEL Snow/Frost Rubric Baseline

Evidence mode: Ran.

- Schema: `snowfreeze-non-snotel-rubric-baseline-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-048 INV-SNOWFREEZE-050 TOL-SNOWFREEZE-011`
- Runtime: `direct-production-executor`
- Site count: `5`
- Rubric counts: `{'fail': 19, 'marginal': 8, 'pass': 5, 'strong': 20, 'unavailable': 63}`
- Forcing-robust rubric counts: `{'fail': 9, 'marginal': 7, 'pass': 4, 'strong': 20, 'unavailable': 45}`
- Snow-control status counts: `{'MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW': 2, 'SNOW_CONTROL_FAILED': 3}`
- OpenWEPP defective cells: `0`
- Next route: `snow-depth structural remediation before frost physics attribution`

## Site Profile Summary

| Site | Method | Snow control | Robust counts | Key blockers | Frost residual rows | Isotherm rows |
| --- | --- | --- | --- | --- | ---: | ---: |
| site1_sleepers_south_field_vt | frost_tube | SNOW_CONTROL_FAILED | fail:2, marginal:1, pass:2, strong:5, unavailable:7 | cross_cutting_snow_depth_bias_sign, cross_cutting_snow_control_gate | 392 | 0 |
| site2_sleepers_w9_hardwood_vt | frost_tube | SNOW_CONTROL_FAILED | fail:1, marginal:2, pass:1, strong:6, unavailable:7 | cross_cutting_snow_control_gate | 200 | 0 |
| site4_ggd498_morris_mn | frost_tube | SNOW_CONTROL_FAILED | fail:2, marginal:2, pass:1, strong:5, unavailable:7 | long_term_snow_cover_duration, cross_cutting_snow_control_gate | 83 | 0 |
| site3_scan_mandan_nd | soil_temperature_zero_c_isotherm | MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW | fail:3, marginal:1, strong:1, unavailable:12 | frost_isotherm_upper_bound, frost_thaw_timing, frost_frozen_duration | 0 | 10583 |
| site5_reynolds_creek_us_rls_id | soil_temperature_zero_c_isotherm | MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW | fail:1, marginal:1, strong:3, unavailable:12 | frost_frozen_duration | 0 | 4356 |

## Disposition

- This is a baseline profile, not a remediation.
- Snow-control failures or unavailable paired snow observations remain blockers before frost attribution.
- SWE, density, event, and conservation cells unavailable for this non-SNOTEL corpus are explicit unavailable cells.
- `OPENWEPP-DEFECTIVE` remains `0` because ADR-0017 requires independent correctness authority, not observation disagreement alone.
