# Calibration-Readiness Matrix

Status: `PASS`

Evidence mode: **Ran + Inference**.

## Orthogonal Status

| Field | Status | Basis |
|---|---|---|
| `science_implementation_status` | `IMPLEMENTED` | Existing phase and CoE snow physics ran unchanged; only external forcing copies changed. |
| `calibration_evidence_status` | `EMPIRICALLY_CALIBRATED` | Mica Creek, Niwot, and Paradise have interior site-specific candidates selected from real calibration records under a prospectively frozen objective. Snowbird remains boundary-censored. |
| `identifiability_status` | `PARTIALLY_IDENTIFIABLE` | The precipitation response is identified on these fixtures, but correction factors remain confounded with forcing representativeness, phase, retention, and pre-peak loss. |

`EMPIRICALLY_CALIBRATED` applies only to the three named fixture/record pairs.
It does not imply independent validation, transferability, a regional default,
or unique causal ownership. Snowbird is not classified as a final calibration.

## Readiness Obligations

| Obligation | Status | Evidence | Rationale |
|---|---|---|---|
| enumerable parameter surface | `PASS` | freeze | one dimensionless forcing multiplier over an exact grid |
| unit/scale-defined operator | `PASS` | operand lineage | water-year SWE ratios and day offsets are explicit |
| deterministic candidate execution | `PASS` | receipt and retained-anchor evidence | 20 new runs and 24 anchors are identity-bound |
| objective reconstruction | `PASS` | results JSON and summary CSV | magnitude-first ranking and chronology requirement are explicit |
| local sensitivity | `PASS` | response curves | 11-point response exists per lane |
| identifiability/confounding | `PASS` | scientific disposition | forcing response is separated from unique causal ownership |
| boundary/failure/saturation | `PASS` | results and gates | Snowbird's `2.0` boundary and zero runtime failures are explicit |
| covariance/equifinality | `PASS` | scientific disposition | covariance is not estimable on a one-coefficient surface; cross-process equifinality remains explicit |
| synthetic recovery | `NOT_APPLICABLE` | transformer self-check | algebraic external forcing scaling has no hidden inverse parameter to recover |
| additional-data inventory | `PASS` | worker handoff | independent forcing/snow observations are named for later validation, not required for this calibration closure |

Final posture:
`THREE_SITE_CALIBRATION_COMPLETE / SNOWBIRD_BOUNDARY / NO_VALIDATION_OR_PROMOTION`.
