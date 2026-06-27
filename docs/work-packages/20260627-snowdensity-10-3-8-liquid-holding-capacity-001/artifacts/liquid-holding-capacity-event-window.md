# Winter-Thaw Melt Response Correction

Evidence mode: Static/Ran.

- Schema: `snowdensity10-3-8-liquid-holding-capacity-event-window-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-067 OBL-SNOWFREEZE-P-042`
- Runtime coupling: `diagnostic snowbench replay of typed CoE melt path; opt-in only`
- Legacy model: `legacy_coe`
- Candidate model: `coe_liquid_holding_capacity_v1`
- Disposition: `WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES`
- Default activation changed: `False`
- Parser/runfile/user CLI selector added: `False`
- Public output schema changed: `False`
- Rain heat and sub-canopy longwave remain out of scope.

## Improvement Gate

| Metric | Legacy | Candidate | Delta |
|---|---:|---:|---:|
| `under_ablation_interval_count` | 132 | 94 | -38 |
| `aggregate_depth_loss_deficit_m` | 24.1051 | 15.5064 | -8.59869 |
| `total_modeled_depth_loss_m` | 15.8682 | 30.2687 | |
| `total_raw_melt_m` | 8.68532 | 7.79165 | |
| `total_routed_melt_m` | 5.89518 | 12.5277 | |
| `total_snowpack_swe_loss_m` | 4.62814 | 9.91669 | |
| `total_rain_retained_m` | 1.66414 | 0.0973008 | |
| `total_rain_released_m` | 1.26704 | 2.611 | |
| `total_liquid_water_released_m` | 0 | 9.91506 | |
| `max_liquid_holding_capacity_m` | 0 | 0.013503 | |
| `max_liquid_water_retained_m` | 0 | 0.0128774 | |

## Conservation Gate

- Tolerance: `1e-09` m
- Candidate conservation passed: `True`

| Model | SWE balance residual m | Routed state-loss residual m | Min storage margin m | Passed |
|---|---:|---:|---:|---:|
| `legacy_coe` | 0 | 0 | 0.00921429 | `True` |
| `coe_liquid_holding_capacity_v1` | 0 | 0 | 0 | `True` |

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

## `coe_liquid_holding_capacity_v1` Surface Summary

| Metric | Value |
|---|---:|
| `paired_surface_count` | 4 |
| `thaw_observed_ablation_interval_count` | 211 |
| `under_ablation_interval_count` | 94 |
| `under_ablation_fraction` | 0.445498 |
| `total_depth_loss_deficit_m` | 15.5064 |
| `total_raw_melt_m` | 7.79165 |
| `total_routed_melt_m` | 12.5277 |
| `total_snowpack_swe_loss_m` | 9.91669 |

