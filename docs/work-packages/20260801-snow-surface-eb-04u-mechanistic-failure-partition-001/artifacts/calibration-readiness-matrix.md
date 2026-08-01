# Calibration-Readiness Matrix

Evidence mode: `Static + Reused Ran`.

## ADR-0042 Status By Mechanism

| Mechanism | Science implementation | Calibration evidence | Identifiability | Read |
| --- | --- | --- | --- | --- |
| Existing sub-canopy longwave | `IMPLEMENTED` | `NOT_APPLICABLE` | `NONIDENTIFIABLE` in current failures | All timing failures are open controls; EB-04X needs new paired evidence. |
| Existing sublimation | `IMPLEMENTED` | `NOT_APPLICABLE` | `PARTIALLY_IDENTIFIABLE` | Open timing cells respond, but forcing and interactions remain confounded. |
| Density-process amendment | `NOT_IMPLEMENTED` | `NOT_APPLICABLE` | `PARTIALLY_IDENTIFIABLE` | Mixed bias and missing process tendencies require EB-04V selection before implementation. |
| Wind-redistribution snow process | `NOT_IMPLEMENTED` | `NOT_APPLICABLE` | `NONIDENTIFIABLE` | openWEPP process authority is unreconciled and redistribution is not separately observed. |
| Precipitation/phase forcing correction | `NOT_APPLICABLE` in openWEPP | `NOT_APPLICABLE` | `NONIDENTIFIABLE` | wepppy/forcing-provider ownership must be resolved; openWEPP may diagnose but cannot compensate for forcing undercatch or phase error. |
| Canopy-snow interception amendment | `NOT_IMPLEMENTED` | `NOT_APPLICABLE` | `NONIDENTIFIABLE` | Authority has not yet been reconciled; Harvard pairing exists, but the snow-interception ledger is absent. |

## Current EB-04U Readiness Obligations

| Obligation | Disposition | Evidence and rationale |
| --- | --- | --- |
| Typed/enumerable parameter surface | `NOT_APPLICABLE` | EB-04U selects no candidate or tunable parameter. |
| Observation operator with units and scale | `PASS` | Failure mechanics matrix and seasonal-phase protocol. |
| Deterministic candidate execution | `NOT_APPLICABLE` | Candidate/model execution is explicitly excluded. |
| Objective reconstruction | `PASS` | Operators and independent operand requirements are frozen prospectively. |
| Sensitivity analysis | `NOT_APPLICABLE` | No candidate parameter or threshold is introduced. |
| Identifiability/confounding analysis | `PASS` | Cohort matrix, observability matrix, and ownership split. |
| Boundary, saturation, and failure reporting | `PASS` | Decision protocol and successor admission prerequisites. |
| Equifinality/uncertainty retention | `PASS` | Competing explanations remain explicit; no unique-cause claim. |
| Synthetic recovery | `NOT_APPLICABLE` | No estimable parameter surface or calibration machinery is in scope. |
| Additional-data inventory | `PASS` | Evidence-role protocol and missing observability entries. |

The successor statuses are prospective intake findings, not unmet EB-04U
implementation gates. EB-04U's current-scope readiness-design obligations pass.
