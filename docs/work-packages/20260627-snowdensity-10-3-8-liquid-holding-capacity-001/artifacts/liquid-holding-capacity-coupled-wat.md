# SNOWDENSITY-10.3.7 Coupled WAT Melt Response Gate

Evidence mode: Static/Ran.

- Disposition: `WINTER-THAW-COUPLED-WAT-IMPROVES`
- Blocker: `SNOW-CONTROL-NOT-CLEARED`
- Coupled no-worse gate passed: `True`
- Candidate snow-control passed: `False`
- Default fail count: `1147`
- Candidate fail count: `761`
- Fail delta default-minus-candidate: `386`
- Trace proof: `{'default_trace_selected_count': 112502, 'candidate_trace_selected_count': 112502, 'trace_counts_by_path': {'legacy_coe': {'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/hjandrews_conifer_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/sleepers_south_field_legacy_coe.jsonl': {'legacy_coe': 13880}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/sleepers_w9_hardwood_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/harvard_hardwood_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/harvard_open_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/hubbardbrook_deciduous_legacy_coe.jsonl': {'legacy_coe': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/hubbardbrook_mixed_legacy_coe.jsonl': {'legacy_coe': 16437}}, 'coe_liquid_holding_capacity_v1': {'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/hjandrews_conifer_coe_liquid_holding_capacity_v1.jsonl': {'coe_liquid_holding_capacity_v1': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/sleepers_south_field_coe_liquid_holding_capacity_v1.jsonl': {'coe_liquid_holding_capacity_v1': 13880}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/sleepers_w9_hardwood_coe_liquid_holding_capacity_v1.jsonl': {'coe_liquid_holding_capacity_v1': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/harvard_hardwood_coe_liquid_holding_capacity_v1.jsonl': {'coe_liquid_holding_capacity_v1': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/harvard_open_coe_liquid_holding_capacity_v1.jsonl': {'coe_liquid_holding_capacity_v1': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/hubbardbrook_deciduous_coe_liquid_holding_capacity_v1.jsonl': {'coe_liquid_holding_capacity_v1': 16437}, 'target/snowdensity10_3_8_liquid_holding_capacity/coupled-wat/traces/hubbardbrook_mixed_coe_liquid_holding_capacity_v1.jsonl': {'coe_liquid_holding_capacity_v1': 16437}}}}`

## Surface Results

| Surface | Scope | Impact | Default fails | Candidate fails | Mean abs reduction m | Depth day-sum delta m days |
|---|---|---|---:|---:|---:|---:|
| `hjandrews_conifer` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | -598.800819 |
| `sleepers_south_field` | paired_observation | IMPROVED | 322 | 254 | 0.196613 | -1002.233083 |
| `sleepers_w9_hardwood` | paired_observation | IMPROVED | 143 | 103 | 0.164961 | -1448.010727 |
| `harvard_hardwood` | paired_observation | IMPROVED | 357 | 206 | 0.270175 | -1140.791139 |
| `harvard_open` | paired_observation | IMPROVED | 325 | 198 | 0.251997 | -1319.619066 |
| `hubbardbrook_deciduous` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | -1050.717331 |
| `hubbardbrook_mixed` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | -905.226238 |

## Boundary Disposition

- Default activation changed: `false`.
- Parser/runfile/user CLI selector added: `false`.
- Fixture inputs changed: `false`.
- Public output schema changed: `false`.
- Observation-blocked surfaces are diagnostic-only and carry no defect verdict.
