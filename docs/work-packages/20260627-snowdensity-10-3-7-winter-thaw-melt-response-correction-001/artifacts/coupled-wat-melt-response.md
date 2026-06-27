# SNOWDENSITY-10.3.7 Coupled WAT Melt Response Gate

Evidence mode: Static/Ran.

- Disposition: `WINTER-THAW-COUPLED-WAT-IMPROVES`
- Blocker: `SNOW-CONTROL-NOT-CLEARED`
- Coupled no-worse gate passed: `True`
- Candidate snow-control passed: `False`
- Default fail count: `1147`
- Candidate fail count: `978`
- Fail delta default-minus-candidate: `169`
- Trace proof: `{'default_trace_selected_count': 112502, 'candidate_trace_selected_count': 112502, 'trace_counts_by_path': {'legacy_coe': {'target/snowdensity10_3_7_coupled_wat_melt_response/traces/hjandrews_conifer_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/sleepers_south_field_legacy_coe.jsonl': {'legacy_coe': 13880}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/sleepers_w9_hardwood_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/harvard_hardwood_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/harvard_open_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/hubbardbrook_deciduous_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/hubbardbrook_mixed_legacy_coe.jsonl': {'legacy_coe': 16437}}, 'coe_winter_thaw_state_loss_v1': {'target/snowdensity10_3_7_coupled_wat_melt_response/traces/hjandrews_conifer_coe_winter_thaw_state_loss_v1.jsonl': {'coe_winter_thaw_state_loss_v1': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/sleepers_south_field_coe_winter_thaw_state_loss_v1.jsonl': {'coe_winter_thaw_state_loss_v1': 13880}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/sleepers_w9_hardwood_coe_winter_thaw_state_loss_v1.jsonl': {'coe_winter_thaw_state_loss_v1': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/harvard_hardwood_coe_winter_thaw_state_loss_v1.jsonl': {'coe_winter_thaw_state_loss_v1': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/harvard_open_coe_winter_thaw_state_loss_v1.jsonl': {'coe_winter_thaw_state_loss_v1': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/hubbardbrook_deciduous_coe_winter_thaw_state_loss_v1.jsonl': {'coe_winter_thaw_state_loss_v1': 16437}, 'target/snowdensity10_3_7_coupled_wat_melt_response/traces/hubbardbrook_mixed_coe_winter_thaw_state_loss_v1.jsonl': {'coe_winter_thaw_state_loss_v1': 16437}}}}`

## Surface Results

| Surface | Scope | Impact | Default fails | Candidate fails | Mean abs reduction m | Depth day-sum delta m days |
|---|---|---|---:|---:|---:|---:|
| `hjandrews_conifer` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | -220.813741 |
| `sleepers_south_field` | paired_observation | IMPROVED | 322 | 282 | 0.145080 | -724.111491 |
| `sleepers_w9_hardwood` | paired_observation | IMPROVED | 143 | 122 | 0.118497 | -1017.649225 |
| `harvard_hardwood` | paired_observation | IMPROVED | 357 | 313 | 0.194903 | -823.203309 |
| `harvard_open` | paired_observation | IMPROVED | 325 | 261 | 0.196427 | -943.173725 |
| `hubbardbrook_deciduous` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | -707.691452 |
| `hubbardbrook_mixed` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | -608.923811 |

## Boundary Disposition

- Default activation changed: `false`.
- Parser/runfile/user CLI selector added: `false`.
- Fixture inputs changed: `false`.
- Public output schema changed: `false`.
- Observation-blocked surfaces are diagnostic-only and carry no defect verdict.
