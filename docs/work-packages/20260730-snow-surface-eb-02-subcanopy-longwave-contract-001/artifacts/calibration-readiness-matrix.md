# Calibration Readiness Matrix

Status: `complete / calibration not applicable`.

Evidence class: Static.

## Orthogonal status

| Field | Value | Rationale |
|---|---|---|
| `science_implementation_status` | `NOT_IMPLEMENTED` | Canonical equation/interface authority exists, but EB-02 intentionally adds no executable runtime. |
| `calibration_evidence_status` | `NOT_APPLICABLE` | All constants are fixed by authority and all varying operands are external forcing/state. |
| `identifiability_status` | `NOT_APPLICABLE` | No parameter is estimated and no observation operator is proposed for fitting. |

## Parameter disposition

| Quantity | Role | Tunable in EB-02? | Treatment |
|---|---|---:|---|
| effective canopy cover `C` | existing state input | no | governed by `SC-PLANT-001` |
| diffuse exponent `1.6` | literature equation constant | no | fixed from FSM2 |
| Dilley coefficients | atmospheric equation constants | no | fixed from corrected published equation |
| clearness limits `0.15`, `0.80` | daily cloud mapping | no | fixed from Flerchinger Table 9 |
| cloud weight `0.84` | Unsworth-Monteith correction | no | fixed |
| canopy/snow emissivity | effective exchange convention | no | exactly one |
| canopy temperature | provider state/approximation | no | EB-03 boundary; no fitted bias |
| snow-surface temperature | provider state | no | EB-03 boundary |
| `R_a,min` | numeric daylight guard | no | implementation prerequisite, not calibration |

EB-02 introduces no user coefficient and performs no parameter fitting.
Observed sky view or longwave may later validate model adequacy, but absence of
those observations is not a runtime blocker.

## Science-contract-spec readiness obligations

| Obligation | Disposition | Evidence path | Rationale |
|---|---|---|---|
| typed/enumerable parameter surface | `NOT_APPLICABLE` | `SC-SNOWENERGY-001.md#constants-and-parameters` | The admitted constants are fixed and existing states are external; there is no calibratable parameter surface. |
| observation operator with units and scale | `NOT_APPLICABLE` | `SC-SNOWENERGY-001.md#calibration-and-identifiability` | EB-02 performs no empirical calibration or observation comparison. |
| deterministic candidate execution | `NOT_APPLICABLE` | `package.md#excluded-scope` | Production runtime is deliberately excluded and held for EB-03. |
| objective reconstruction | `NOT_APPLICABLE` | `SC-SNOWENERGY-001.md#calibration-and-identifiability` | No calibration objective exists. |
| sensitivity analysis | `NOT_APPLICABLE` | `SC-SNOWENERGY-001.md#calibration-and-identifiability` | No fitted parameter or calibration ensemble exists. |
| identifiability/confounding analysis | `PASS` | `SC-SNOWENERGY-001.md#calibration-and-identifiability` | The contract identifies the prohibited extinction/cover confounding and fixes the equation constant. |
| boundary, saturation, and failure reporting | `PASS` | `SC-SNOWENERGY-001.md#branch-and-guard-table` | Input, derived-emissivity, canopy, polar-night, and provider failures are explicit. |
| equifinality/uncertainty retention | `PASS` | `SC-SNOWENERGY-001.md#gap-register` | Canopy-temperature, heterogeneous-stand, atmospheric-envelope, and polar-night limitations remain visible. |
| synthetic recovery | `NOT_APPLICABLE` | `package.md#excluded-scope` | No parameter-estimation machinery exists to recover. |
| additional-data inventory | `PASS` | `SC-SNOWENERGY-001.md#gap-register` | Optional validation observations are identified without becoming runtime prerequisites. |

There is no current-scope `BLOCKED` readiness row.
