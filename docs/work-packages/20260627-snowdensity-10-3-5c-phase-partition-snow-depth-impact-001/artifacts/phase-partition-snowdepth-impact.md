# SNOWDENSITY-10.3.5c Phase Partition Snow-Depth Impact

Evidence mode: Static/Ran.

- Disposition: `PHASE-PARTITION-NEUTRAL-OR-WORSE`
- Blocker: `PHASE-PARTITION-DID-NOT-REDUCE-PAIRED-SNOW-DEPTH-FAILURES`
- Paired surfaces: `4`
- Observation-blocked surfaces: `3`
- WAT-changed surfaces: `7`
- Default fail count: `1147`
- Opt-in fail count: `1273`
- Fail delta default-minus-opt-in: `-126`
- Next route: target 10.3.4 rank-2 winter-thaw melt response before longwave or rain-heat

## Surface Results

| Surface | Scope | Impact | Default fails | Opt-in fails | Mean abs reduction (m) | WAT changed days |
|---|---:|---:|---:|---:|---:|---:|
| `hjandrews_conifer` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | 3472 |
| `sleepers_south_field` | paired_observation | WORSE | 322 | 349 | -0.151001 | 6300 |
| `sleepers_w9_hardwood` | paired_observation | WORSE | 143 | 153 | -0.112010 | 7702 |
| `harvard_hardwood` | paired_observation | WORSE | 357 | 402 | -0.214631 | 5906 |
| `harvard_open` | paired_observation | WORSE | 325 | 369 | -0.212269 | 6322 |
| `hubbardbrook_deciduous` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | 6600 |
| `hubbardbrook_mixed` | observation_blocked | OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY | 0 | 0 | n/a | 6625 |

## Boundary Disposition

- Default activation changed: `false`.
- Parser/runfile/user CLI selector added: `false`.
- Fixture inputs changed: `false`.
- Public output schema changed: `false`.
- Production physics outside the existing opt-in selector changed: `false`.
- Observation-blocked surfaces are diagnostic-only and carry no defect verdict.
