# Calibration Readiness Matrix

Status: `terminal implementation disposition`

Evidence class: `Static`

| Claim | Science implementation | Calibration evidence | Identifiability | Rationale/evidence |
| --- | --- | --- | --- | --- |
| Existing CP-GSI02 deciduous leaf-off transfer | `IMPLEMENTED` | `CALIBRATION_READY_DATA_LIMITED` | `PARTIALLY_IDENTIFIABLE` | Retained CAL-04B/CAL-05 disposition; unchanged by this package |
| Predictive evergreen needle ground deposition | `AUTHORITY_MISSING` | `NOT_CALIBRATION_READY` | `NOT_ASSESSED` | `source-admission-matrix.md`: longevity/stock omits dry-mass retention and timing |
| Predictive fine-woody ground deposition | `AUTHORITY_MISSING` | `NOT_CALIBRATION_READY` | `NOT_ASSESSED` | `authority-law-and-operand-lineage.md`: missing branch/crown/stand state and turnover-to-deposition law |
| Authenticated prescribed daily external boundary | `IMPLEMENTED` | `NOT_APPLICABLE` | `NOT_APPLICABLE` | Exogenous scenario values are authenticated inputs, not estimable process parameters |
| Authenticated exhaustive measured-daily boundary | `IMPLEMENTED` | `NOT_APPLICABLE` | `NOT_APPLICABLE` | Observed exact-daily boundary values are authenticated inputs; interval observations are explicitly non-executable |
| Native recurring litter-source sufficiency | `AUTHORITY_MISSING` | `NOT_CALIBRATION_READY` | `NOT_ASSESSED` | Predictive needle and fine-wood rows remain open even if a boundary interface is later implemented |

`NOT_APPLICABLE` is justified for the boundary rows because this package
defines no estimable parameter, observation operator, objective, sensitivity
campaign, or recovery target for values supplied exogenously. Interface
validation and mass reconstruction remain required implementation gates but
are not empirical calibration.
