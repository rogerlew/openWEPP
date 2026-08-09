# Calibration-Readiness Matrix

Status: `NOT_CALIBRATION_READY`

Evidence mode: `Static`

`science_implementation_status=AUTHORITY_MISSING`,
`calibration_evidence_status=NOT_CALIBRATION_READY`, and
`identifiability_status=NOT_ASSESSED`.

| Obligation | Status | Evidence/rationale |
| --- | --- | --- |
| typed/enumerable parameter surface | `PARTIAL PASS` | contract version 3 admits schema form and classifications; complete field declarations and selected values remain missing |
| observation operator with units and scale | `PARTIAL` | initial-state identity/metadata obligations are typed, but no complete pool/flux operator or selected stand/date is admitted |
| deterministic candidate execution | `NOT_APPLICABLE` | no implementation in this authority-only package |
| objective reconstruction | `NOT_APPLICABLE` | no empirical calibration objective authorized |
| sensitivity analysis | `BLOCKED` | no executable constitutive surface or selected values |
| identifiability/confounding analysis | `BLOCKED` | no complete selected observation/parameter pairing |
| boundary, saturation, and failure reporting | `PASS` | gate artifacts retain sentinels, invalid optics, defaults, and exact blockers |
| equifinality/uncertainty retention | `PASS` | no unique value, prior, or transferability claim is made |
| synthetic recovery | `NOT_APPLICABLE` | no implementation or calibration machinery exists |
| additional-data inventory | `PASS` | Gate 1 names field-level provenance and dated initial-state needs |

The package did not use missing data to defer remaining science work: Gates 2
and 3 separately adjudicate symbolic equation/ownership routes. The residual
rows require package `HOLD` under ADR-0042 because both science-authority and
selected state/value obligations remain open.
