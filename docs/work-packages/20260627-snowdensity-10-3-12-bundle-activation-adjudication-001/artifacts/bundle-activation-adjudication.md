# SNOWDENSITY-10.3.12 Bundle Activation Adjudication

Evidence mode: Static/Ran.

- Disposition: `HOLD-OPT-IN-BUNDLE`
- Blocker: `POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING`
- Activation policy: `POLICY-B`
- Activation ready: `False`
- Frost attribution unblocked: `False`
- Default failures: `1147`
- Holding-capacity-only failures: `761`
- Bundle failures: `498`
- Spring-densification failures: `502`
- Paired rows: `1415`
- Trace melt count: `112502`
- Trace density count: `112502`

## Activation Policy B

- Zero paired snow-depth failures required for activation: `False`
- Gate-eligible snow surfaces strictly better than current default: `True`
- Full-surface no-regression evidence present: `False`
- Full-surface no-regression scope: `['workspace regression/identity suite', 'non-snow climate fixtures', 'erosion and water-balance surfaces', 'watershed routing surfaces']`
- Paired snow-control zero failures: `False`
- Frost attribution blocker: `SNOW-CONTROL-RESIDUALS-REMAIN`

## Remaining Failure Profile

- Counts by residual sign: `{'MODELED_OVER_OBSERVED': 264, 'MODELED_UNDER_OBSERVED': 234}`
- Counts by month: `{'1': 138, '11': 5, '12': 46, '2': 112, '3': 170, '4': 27}`
- March/April cap classes: `{'CAP_LIMITED_DEPLETION_REQUIRED': 33, 'COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP': 20, 'PATCHY_MELTOUT_OR_DEPLETION_REQUIRED': 16, 'UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT': 128}`

## Surface Results

| Surface | Scope | Cover | Holding fails | Bundle fails | Spring fails | Bundle vs holding |
|---|---|---|---:|---:|---:|---:|
| `hjandrews_conifer` | observation_blocked | conifer | 0 | 0 | 0 | None |
| `sleepers_south_field` | paired_observation | open_field | 254 | 150 | 149 | 104 |
| `sleepers_w9_hardwood` | paired_observation | hardwood | 103 | 57 | 57 | 46 |
| `harvard_hardwood` | paired_observation | hardwood | 206 | 153 | 156 | 53 |
| `harvard_open` | paired_observation | open | 198 | 138 | 140 | 60 |
| `hubbardbrook_deciduous` | observation_blocked | deciduous | 0 | 0 | 0 | None |
| `hubbardbrook_mixed` | observation_blocked | mixed | 0 | 0 | 0 | None |

## Boundary Disposition

- Default activation changed: `false`.
- Parser/runfile/user CLI selector added: `false`.
- Fixture inputs changed: `false`.
- Public output schema changed: `false`.
- New process physics added: `false`.
- Frost attribution remains blocked while snow-control residuals remain; this is separate from Policy-B default activation.
