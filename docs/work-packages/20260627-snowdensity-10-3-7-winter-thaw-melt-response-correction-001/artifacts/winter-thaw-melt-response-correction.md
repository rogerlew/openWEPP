# SNOWDENSITY-10.3.7 Winter-Thaw Melt Response Correction

Evidence mode: Static/Ran.

- Schema: `snowdensity10-3-7-winter-thaw-melt-response-correction-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-002 INV-SNOWFREEZE-066 OBL-SNOWFREEZE-P-041`
- Runtime coupling: `diagnostic snowbench replay of typed CoE melt path; opt-in only`
- Legacy model: `legacy_coe`
- Candidate model: `coe_winter_thaw_state_loss_v1`
- Disposition: `WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES`
- Default activation changed: `False`
- Parser/runfile/user CLI selector added: `False`
- Public output schema changed: `False`
- Rain heat and sub-canopy longwave remain out of scope.

## Improvement Gate

| Metric | Legacy | Candidate | Delta |
|---|---:|---:|---:|
| `under_ablation_interval_count` | 132 | 108 | -24 |
| `aggregate_depth_loss_deficit_m` | 24.1051 | 17.6289 | -6.4762 |
| `total_modeled_depth_loss_m` | 15.8682 | 26.4001 | |
| `total_raw_melt_m` | 8.68532 | 8.506 | |
| `total_routed_melt_m` | 5.89518 | 11.2347 | |
| `total_snowpack_swe_loss_m` | 4.62814 | 10.6148 | |
| `total_rain_retained_m` | 1.66414 | 2.28633 | |
| `total_rain_released_m` | 1.26704 | 0.61988 | |

## Conservation Gate

- Tolerance: `1e-09` m
- Candidate conservation passed: `True`

| Model | SWE balance residual m | Routed state-loss residual m | Min storage margin m | Passed |
|---|---:|---:|---:|---:|
| `legacy_coe` | 0 | 0 | 0.00921429 | `True` |
| `coe_winter_thaw_state_loss_v1` | 0 | 0 | 0 | `True` |

## `legacy_coe` Surface Summary

| Metric | Value |
|---|---:|
| `paired_surface_count` | 4 |
| `thaw_observed_ablation_interval_count` | 219 |
| `under_ablation_interval_count` | 132 |
| `under_ablation_fraction` | 0.60274 |
| `total_depth_loss_deficit_m` | 24.1051 |
| `total_raw_melt_m` | 8.68532 |
| `total_routed_melt_m` | 5.89518 |
| `total_snowpack_swe_loss_m` | 4.62814 |

## `coe_winter_thaw_state_loss_v1` Surface Summary

| Metric | Value |
|---|---:|
| `paired_surface_count` | 4 |
| `thaw_observed_ablation_interval_count` | 218 |
| `under_ablation_interval_count` | 108 |
| `under_ablation_fraction` | 0.495413 |
| `total_depth_loss_deficit_m` | 17.6289 |
| `total_raw_melt_m` | 8.506 |
| `total_routed_melt_m` | 11.2347 |
| `total_snowpack_swe_loss_m` | 10.6148 |

