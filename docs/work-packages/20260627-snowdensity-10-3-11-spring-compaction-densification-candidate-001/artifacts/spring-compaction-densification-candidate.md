# SNOWDENSITY-10.3.11 Spring Compaction/Densification Candidate

Evidence mode: Static/Ran.

- Disposition: `SPRING-DENSIFICATION-NON-PROMOTION`
- Blocker: `COUPLED-SNOW-CONTROL-OR-UNDER-PERSISTENCE-WORSENED`
- Coupled no-worse gate passed: `False`
- Prior 10.3.8 fail count: `761`
- Density-baseline fail count: `498`
- Candidate fail count: `502`
- Density-minus-candidate fail delta: `-4`
- Candidate under-persistence count: `128`
- Trace proof: `{'density_baseline_trace_selected_count': 112502, 'candidate_trace_selected_count': 112502, 'trace_counts_by_path': {'physics_bulk_density_compaction_v1': {'target/snowdensity10_3_11_spring_compaction_densification/traces/hjandrews_conifer_physics_bulk_density_compaction_v1.jsonl': {'physics_bulk_density_compaction_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/sleepers_south_field_physics_bulk_density_compaction_v1.jsonl': {'physics_bulk_density_compaction_v1': 13880}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/sleepers_w9_hardwood_physics_bulk_density_compaction_v1.jsonl': {'physics_bulk_density_compaction_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/harvard_hardwood_physics_bulk_density_compaction_v1.jsonl': {'physics_bulk_density_compaction_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/harvard_open_physics_bulk_density_compaction_v1.jsonl': {'physics_bulk_density_compaction_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/hubbardbrook_deciduous_physics_bulk_density_compaction_v1.jsonl': {'physics_bulk_density_compaction_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/hubbardbrook_mixed_physics_bulk_density_compaction_v1.jsonl': {'physics_bulk_density_compaction_v1': 16437}}, 'physics_bulk_spring_densification_v1': {'target/snowdensity10_3_11_spring_compaction_densification/traces/hjandrews_conifer_physics_bulk_spring_densification_v1.jsonl': {'physics_bulk_spring_densification_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/sleepers_south_field_physics_bulk_spring_densification_v1.jsonl': {'physics_bulk_spring_densification_v1': 13880}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/sleepers_w9_hardwood_physics_bulk_spring_densification_v1.jsonl': {'physics_bulk_spring_densification_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/harvard_hardwood_physics_bulk_spring_densification_v1.jsonl': {'physics_bulk_spring_densification_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/harvard_open_physics_bulk_spring_densification_v1.jsonl': {'physics_bulk_spring_densification_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/hubbardbrook_deciduous_physics_bulk_spring_densification_v1.jsonl': {'physics_bulk_spring_densification_v1': 16437}, 'target/snowdensity10_3_11_spring_compaction_densification/traces/hubbardbrook_mixed_physics_bulk_spring_densification_v1.jsonl': {'physics_bulk_spring_densification_v1': 16437}}}}`

## Surface Results

| Surface | Scope | Impact | Density fails | Candidate fails | Density compaction-only | Candidate compaction-only | Under-persistence candidate |
|---|---|---|---:|---:|---:|---:|---:|
| `hjandrews_conifer` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | 0 | 0 | 0 |
| `sleepers_south_field` | paired_observation | IMPROVED | 150 | 149 | 15 | 15 | 19 |
| `sleepers_w9_hardwood` | paired_observation | WORSE | 57 | 57 | 2 | 2 | 10 |
| `harvard_hardwood` | paired_observation | WORSE | 153 | 156 | 0 | 0 | 64 |
| `harvard_open` | paired_observation | WORSE | 138 | 140 | 3 | 3 | 35 |
| `hubbardbrook_deciduous` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | 0 | 0 | 0 |
| `hubbardbrook_mixed` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | 0 | 0 | 0 |

## Boundary Disposition

- Default activation changed: `false`.
- Parser/runfile/user CLI selector added: `false`.
- Fixture inputs changed: `false`.
- Public output schema changed: `false`.
- Density cap changed: `false`.
- Runtime calculation consumes observed depth/density: `false`.
- Observation-blocked surfaces are diagnostic-only and carry no defect verdict.
