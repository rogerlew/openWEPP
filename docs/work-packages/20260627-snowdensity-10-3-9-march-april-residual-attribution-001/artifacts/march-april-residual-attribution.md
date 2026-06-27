# March/April Residual Attribution

- Schema: `snowdensity10-3-9-march-april-residual-attribution-v1`
- Candidate baseline: `coe_liquid_holding_capacity_v1`
- Disposition: `MARCH_APRIL-RESIDUALS-ATTRIBUTED`
- Recommended next process: `SPRING-PACK-DEPLETION-AND-COMPACTION-ADJUDICATION`
- Remaining blocker: `SNOW-CONTROL-NOT-CLEARED`

## Summary

| Metric | Value |
| --- | ---: |
| Total paired rows | 1415 |
| Total failed rows | 761 |
| March/April paired rows | 463 |
| March/April failed rows | 282 |
| March/April share of all failures | 0.370565 |
| March/April fail fraction | 0.609071 |

## Mechanism Counts

| Mechanism | March/April failed rows |
| --- | ---: |
| `DENSITY_OR_COMPACTION_DEFICIT` | 86 |
| `DEPTH_ONLY_OVERPERSISTENCE_UNRESOLVED` | 127 |
| `PATCHY_MELTOUT_OR_SNOW_COVER_DEPLETION` | 26 |
| `UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT` | 43 |

## Cover Counts

| Cover | March/April failed rows |
| --- | ---: |
| `hardwood` | 109 |
| `open` | 61 |
| `open_field` | 112 |

## Surface Residuals

| Surface | Cover | Scope | All fail | March/April fail | March/April share | Dominant attribution |
| --- | --- | --- | ---: | ---: | ---: | --- |
| hjandrews_conifer | conifer | observation_blocked | 0 | 0 |  | `None` |
| sleepers_south_field | open_field | paired_observation | 254 | 112 | 0.440945 | `DEPTH_ONLY_OVERPERSISTENCE_UNRESOLVED` |
| sleepers_w9_hardwood | hardwood | paired_observation | 103 | 41 | 0.398058 | `DEPTH_ONLY_OVERPERSISTENCE_UNRESOLVED` |
| harvard_hardwood | hardwood | paired_observation | 206 | 68 | 0.330097 | `DENSITY_OR_COMPACTION_DEFICIT` |
| harvard_open | open | paired_observation | 198 | 61 | 0.308081 | `DENSITY_OR_COMPACTION_DEFICIT` |
| hubbardbrook_deciduous | deciduous | observation_blocked | 0 | 0 |  | `None` |
| hubbardbrook_mixed | mixed | observation_blocked | 0 | 0 |  | `None` |

## Boundary Disposition

- Diagnostic-only; no production physics or default behavior changed.
- Observation-blocked surfaces remain non-verdict surfaces.
- Harvard SWE/mass attribution is correspondence-caveated unless source SWE, depth, and density agree.
