# Calibration-Readiness Matrix

Evidence mode: **Ran + Inference**.

## Orthogonal Status

| Field | Status | Basis |
|---|---|---|
| `science_implementation_status` | `IMPLEMENTED` | Existing authoritative phase and CoE snow physics remain implemented; EB-04W adds contract-authorized, real-consumer diagnostic observability without changing that physics. |
| `calibration_evidence_status` | `NOT_CALIBRATION_READY` | The frozen observations are `DIAGNOSTIC_ONLY`; no calibratable candidate, objective, or independent validation set is authorized, and realized input-pathway/pre-peak-loss ownership remains mixed. |
| `identifiability_status` | `NONIDENTIFIABLE` | The ledger partitions realized modeled input, storage loss, sublimation, and empirical melt terms. It cannot uniquely separate external input causes, endogenous liquid retention, and phase-conditioned pre-peak modeled loss. |

These axes are independent. Implemented science does not imply calibrated or
identified parameters. `NOT_CALIBRATION_READY` is the disposition of a
deliberately diagnostic package, not an implementation failure.

## Readiness Obligations

| Obligation | Status | Evidence path | Applicability rationale |
|---|---|---|---|
| typed/enumerable parameter surface | `NOT_APPLICABLE` | `package.md` Included/Excluded Scope; `SC-SNOWFREEZE-001#INV-SNOWFREEZE-088` | EB-04W admits no coefficient or candidate parameter surface; it observes fixed authoritative phase/CoE terms. |
| observation operator with units and scale | `PASS` | `population-freeze.json`; `accumulation-mechanics-summary.csv` | Five water-year operators are frozen in days over paired SNOTEL depth/SWE observations and executed with the canonical rubric. |
| deterministic candidate execution | `PASS` | `execution-receipt.json`; `accumulation-mechanics-results.json` | Exact release binary and sanitized selectors execute deterministic B/L/S/LS diagnostic cells; these are process contrasts, not fitted candidates. |
| objective reconstruction | `PASS` | `accumulation-mechanics-results.json`; `scientific-synthesis.md` | Exact frozen operator values and pre-observed-peak mass ledgers are independently reconstructed for every cell. |
| sensitivity analysis | `NOT_APPLICABLE` | `package.md` Excluded Scope; `scientific-disposition.md` | No numeric parameter or admissible calibration candidate exists in this package; B/L/S/LS contrasts are reported diagnostically and are not coefficient sensitivity. |
| identifiability/confounding analysis | `PASS` | `scientific-synthesis.md`; `scientific-disposition.md` | The algebraic input/storage boundary is decomposed while causal ownership among realized input, endogenous liquid retention, external forcing, and pre-peak modeled loss is explicitly retained as unresolved. |
| boundary, saturation, and failure reporting | `PASS` | `gate-results.md`; `accumulation-mechanics-results.json` | Phase/depth/SWE/component/cap/mass closures, inactive redistribution, no-pack applicability, and execution failures are explicit and fail closed. |
| equifinality/uncertainty retention | `PASS` | `scientific-disposition.md`; figure sidecars | The result does not allocate a residual to phase, undercatch, representativeness, redistribution, liquid retention, a CoE term, or pre-peak loss timing where ownership is unresolved. |
| synthetic recovery | `NOT_APPLICABLE` | `package.md` Excluded Scope; `SC-SNOWFREEZE-001#INV-SNOWFREEZE-088` | There is no parameterized inverse problem or fitted candidate for which synthetic recovery would be structurally meaningful. |
| additional-data inventory | `PASS` | `scientific-disposition.md`; `worker-handoff.md` | Finer phase-conditioned input, retention, and loss timing is the next model diagnostic; independent precipitation/gauge-catch, phase, redistribution, and process observations would strengthen identification but are not current-scope closure gates. |

No `BLOCKED` row is a required current-scope gate because empirical
calibration is excluded. The diagnostic implementation can close while
calibration remains not ready. Final posture:
`DIAGNOSTIC_COMPLETE / CALIBRATION_HOLD / NO_PROMOTION`.
