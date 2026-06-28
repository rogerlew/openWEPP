# SNOWDENSITY-10.3.13 Residual Tail And Policy-B Diagnostic

Evidence mode: Static/Ran.

- Disposition: `HOLD-ACTIVATION-EVIDENCE-MISSING`
- Activation policy: `POLICY-B`
- Activation ready: `False`
- Activation blocker: `POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING`
- Frost-attribution blocker: `SNOW-CONTROL-RESIDUALS-REMAIN`
- Complete transition rows: `1414`
- Source bundle paired rows: `1415`
- Default failures: `1147`
- Bundle failures: `498`
- Default -> bundle delta: `649`
- Under-persistence induced by bundle density arm: `177`
- Lead under-persistence hypothesis: `BULK_COMPACTION_MECHANISM_COST_SUPPORTED`

## Transition Summary

- Holding -> bundle transitions: `{'OVER_FAIL->OVER_FAIL': 254, 'OVER_FAIL->PASS': 421, 'OVER_FAIL->UNDER_FAIL': 27, 'PASS->OVER_FAIL': 10, 'PASS->PASS': 493, 'PASS->UNDER_FAIL': 150, 'UNDER_FAIL->PASS': 2, 'UNDER_FAIL->UNDER_FAIL': 57}`
- Under-persistence transitions: `{'UNDER_INDUCED_FROM_HOLDING_OVER': 27, 'UNDER_INDUCED_FROM_HOLDING_PASS': 150, 'UNDER_PERSISTED_FROM_HOLDING': 57}`
- March/April cap classes: `{'CAP_LIMITED_DEPLETION_REQUIRED': 33, 'COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP': 20, 'PATCHY_MELTOUT_OR_DEPLETION_REQUIRED': 16, 'UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT': 128}`

## Policy-B Evidence Matrix

| Scope | Status | Evidence |
|---|---|---|
| direct bundle trace proof | `PASS` | 10.3.12 trace rows count both selected bundle members. |
| gate-eligible paired-snow improvement versus current default | `PASS` | default 1147 -> bundle 498; delta 649 |
| paired surface no-worse guard versus holding-only | `PASS` | worse surface count 0 |
| full workspace regression/identity with bundle as default | `MISSING` | No default-activation branch was exercised in this diagnostic. |
| non-snow climate no-regression | `MISSING` | No global default bundle run over non-snow climates was produced. |
| erosion and water-balance no-regression | `MISSING` | No global default bundle comparison over erosion/WB outputs was produced. |
| watershed routing no-regression | `MISSING` | No watershed default bundle comparison was produced. |
| composite melt-density conservation under bundle | `MISSING` | Component conservation exists, but composite global activation evidence is absent. |

## Surface Results

| Surface | Cover | Rows | Bundle under | Induced under | Bundle over |
|---|---|---:|---:|---:|---:|
| `sleepers_south_field` | open_field | 383 | 27 | 21 | 123 |
| `sleepers_w9_hardwood` | hardwood | 193 | 27 | 22 | 30 |
| `harvard_hardwood` | hardwood | 448 | 106 | 73 | 47 |
| `harvard_open` | open | 390 | 74 | 61 | 64 |

## Boundary Disposition

- Default activation changed: `false`.
- Production physics changed: `false`.
- Density cap changed: `false`.
- `550 kg m^-3` cap re-anchor status: `FOLLOW_UP_ONLY_NOT_EVALUATED_HERE`.
- Frost attribution authorized: `false`.
