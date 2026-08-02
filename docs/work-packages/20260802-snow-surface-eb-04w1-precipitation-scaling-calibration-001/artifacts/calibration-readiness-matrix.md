# Calibration-Readiness Matrix

Status: `PASS`

Evidence mode: **Ran + Inference**.

## Orthogonal Status

| Field | Status | Basis |
|---|---|---|
| `science_implementation_status` | `IMPLEMENTED` | Existing authoritative phase and CoE snow physics ran unchanged. Scaling is an external forcing calibration experiment, not new process physics. |
| `calibration_evidence_status` | `CALIBRATION_READY_DATA_LIMITED` | The grid, deterministic runner, operators, objective, provenance, sensitivities, and failures are enumerable, but three selections hit the upper search boundary and there is no independent-validation set. |
| `identifiability_status` | `PARTIALLY_IDENTIFIABLE` | Precipitation sensitivity is directly observed, but forcing bias remains confounded with phase, representativeness, retention, and pre-peak modeled loss. |

No lane is classified `EMPIRICALLY_CALIBRATED`: boundary selections are not
final estimates, and Niwot's nonboundary selection still retains a material
peak-magnitude deficit.

## Readiness Obligations

| Obligation | Status | Evidence path | Rationale |
|---|---|---|---|
| typed/enumerable calibration surface | `PASS` | `experiment-freeze.json`; `package.md` | One dimensionless external multiplier and eight exact values are frozen. |
| observation operator with units and scale | `PASS` | `operand-lineage.md`; EB-04W population freeze | Five inherited water-year operators report SWE ratios and day offsets. |
| deterministic candidate execution | `PASS` | `execution-receipt.json`; `baseline-replay-evidence.md` | All 32 release runs completed and `1.0` is byte-identical to EB-04W. |
| objective reconstruction | `PASS` | `precipitation-scaling-results.json`; `scientific-synthesis.md` | The frozen joint magnitude/chronology rule is reconstructed for every candidate. |
| local sensitivity analysis | `PASS` | response-curve figure and CSV | All lanes have monotonic magnitude response; chronology response is lane-dependent. |
| identifiability/confounding analysis | `PASS` | `scientific-disposition.md`; input/storage figure | Effective input and observed-date storage separate input insufficiency from retained-storage deficit without assigning unique cause. |
| boundary, saturation, and failure reporting | `PASS` | results JSON; `gate-results.md` | Three upper-boundary selections and zero runtime failures are explicit. |
| covariance/equifinality retention | `PASS` | `scientific-disposition.md` | Parameter covariance is not estimable on a one-coefficient surface; cross-process equifinality is retained by treating scaling as compensation, not proof of precipitation error or process correctness. |
| synthetic recovery | `NOT_APPLICABLE` | transformer self-test; package scope | The coefficient modifies external forcing algebraically; synthetic inversion would restate the known multiplier and add no structural test. |
| additional-data inventory | `PASS` | `scientific-disposition.md`; `worker-handoff.md` | Independent precipitation and snow observations are listed for later validation; they are not required to run the next bounded sensitivity experiment. |

Final posture:
`CALIBRATION_LEVER_CONFIRMED / FINAL_MULTIPLIERS_NOT_IDENTIFIED / NO_PROMOTION`.
