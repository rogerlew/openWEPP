# Calibration Readiness Matrix

Status: `NOT_CALIBRATION_READY`; implementation authority may still be released

Evidence mode: `Static + Ran` (synthetic reference calculator only)

This matrix is an assessment of future empirical obligations, not a declaration
of calibration-readiness implementation intent. Its `BLOCKED` rows prohibit a
calibration claim but do not block the separately scoped science implementation
authority under ADR-0042.

| Obligation | Disposition | Evidence/rationale |
|---|---|---|
| typed/enumerable parameter surface | `PASS` | parameter/configuration manifest and `INV-VEGETATION-071` |
| observation operator with units/scale | `BLOCKED` | contract symbols exist, but no observation dataset/operator pairing is admitted |
| deterministic candidate execution | `BLOCKED` | package oracle is deterministic; the production implementation does not yet exist |
| objective reconstruction | `BLOCKED` | conservation ledgers are test oracles, not an admitted empirical calibration objective |
| sensitivity analysis | `BLOCKED` | runtime implementation and observations absent |
| identifiability/confounding analysis | `BLOCKED` | no prospectively assigned calibration observations |
| boundary/saturation/failure reporting | `PASS` | typed guards, limiting vectors and rollback |
| equifinality/uncertainty retention | `BLOCKED` | empirical campaign dependency |
| synthetic recovery | `BLOCKED` | future implementation dependency; oracle is not parameter recovery |
| additional-data inventory | `PASS` | leaf gas exchange/N, sap flow, soil uptake, LAI/litterfall, biomass/pool and flux observations required at matched scale |

The blocked empirical rows set `calibration_evidence_status =
NOT_CALIBRATION_READY` and `identifiability_status = NOT_ASSESSED`. Under
ADR-0042 they restrict claims but do not hold science implementation authority.
