# Spring Pack-Depletion and Compaction Adjudication

- Schema: `snowdensity10-3-10-spring-pack-depletion-compaction-adjudication-v1`
- Candidate baseline: `coe_liquid_holding_capacity_v1`
- Density cap: `522.0 kg m^-3`
- Disposition: `SPRING-COMPACTION-FIRST`
- Recommended next process: `SPRING-COMPACTION-DENSIFICATION-CANDIDATE`
- Remaining blocker: `SNOW-CONTROL-NOT-CLEARED`

## Summary

| Metric | Value |
| --- | ---: |
| March/April paired rows | 463 |
| March/April failed rows | 282 |
| Compaction-only feasible failures | 190 |
| Depletion-required failures | 49 |
| Depletion-required fraction | 0.173759 |
| Row-sum SWE depletion required at cap (m) | 1.23005 |

## Failure Classes

| Class | Failed rows |
| --- | ---: |
| `CAP_LIMITED_DEPLETION_REQUIRED` | 33 |
| `COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP` | 190 |
| `PATCHY_MELTOUT_OR_DEPLETION_REQUIRED` | 16 |
| `UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT` | 43 |

## Surface Results

| Surface | Cover | Scope | Failures | Compaction-only | Depletion-required | Dominant class |
| --- | --- | --- | ---: | ---: | ---: | --- |
| hjandrews_conifer | conifer | observation_blocked | 0 | 0 | 0 | `None` |
| sleepers_south_field | open_field | paired_observation | 112 | 86 | 21 | `COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP` |
| sleepers_w9_hardwood | hardwood | paired_observation | 41 | 35 | 4 | `COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP` |
| harvard_hardwood | hardwood | paired_observation | 68 | 41 | 1 | `COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP` |
| harvard_open | open | paired_observation | 61 | 28 | 23 | `COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP` |
| hubbardbrook_deciduous | deciduous | observation_blocked | 0 | 0 | 0 | `None` |
| hubbardbrook_mixed | mixed | observation_blocked | 0 | 0 | 0 | `None` |

## Boundary Disposition

- Diagnostic-only; no production physics or default behavior changed.
- The `522 kg m^-3` cap is existing `SC-SNOWFREEZE-001` authority, not a fitted threshold.
- Observation-blocked surfaces remain non-verdict surfaces.
- Row-summed required SWE depletion is diagnostic row evidence, not a water-balance ledger.
