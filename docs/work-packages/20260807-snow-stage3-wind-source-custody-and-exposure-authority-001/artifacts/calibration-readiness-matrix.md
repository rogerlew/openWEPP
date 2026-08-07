# Calibration Readiness Matrix

Status: `complete / not calibration work`.

Evidence mode: `Static`.

Intent: `implementation authority reconciliation`, not calibration.

| Field | Status | Rationale |
| --- | --- | --- |
| `science_implementation_status` | `NOT_IMPLEMENTED` | no wind correction/canopy operator is authorized or selected |
| `calibration_evidence_status` | `NOT_APPLICABLE` | no parameter estimation or attenuation is admitted |
| `identifiability_status` | `NOT_APPLICABLE` | exposure cannot be inferred from values/residuals |

Schema readiness obligations for parameters, observation operators, candidate
execution, objective reconstruction, sensitivity, identifiability, and
synthetic recovery are `NOT_APPLICABLE`: the package introduces no calibratable
science. Source/exposure custody is `BLOCKED` scientifically but this is the
package's truthful authority outcome, not a missing calibration-readiness gate.
