# Calibration Readiness Matrix

Evidence class: `Static`

- `science_implementation_status`: `AUTHORITY_MISSING`
- `calibration_evidence_status`: `NOT_CALIBRATION_READY`
- `identifiability_status`: `NOT_ASSESSED`

No calibration or independent-validation claim is in scope. The Stevens Canyon
investigation is `DIAGNOSTIC_ONLY`. Site-specific stratum values and compatible
state are caller configuration; accepting them confers no suitability claim.

| Readiness obligation | Result | Evidence path and rationale |
|---|---|---|
| typed/enumerable parameter surface | `BLOCKED` | `SC-VEGETATION-001` `GAP-VEGETATION-011/012`: versioned schema form is admitted, but the complete consumed-field manifest is successor work. |
| observation operator with units and scale | `NOT_APPLICABLE` | `package.md` Excluded Scope; this package assigns no measured dataset to calibration or validation. |
| deterministic candidate execution | `NOT_APPLICABLE` | `package.md` Excluded Scope and `pre-implementation-contract-gate.md`; no production constitutive candidate is implemented here. |
| objective reconstruction | `NOT_APPLICABLE` | `package.md` Excluded Scope; no calibration objective is defined or executed. |
| sensitivity analysis | `NOT_APPLICABLE` | `SC-VEGETATION-001` Calibration and Identifiability; no admitted executable parameterized family exists. |
| identifiability/confounding analysis | `NOT_APPLICABLE` | `SC-VEGETATION-001` Calibration and Identifiability; no empirical estimation question is authorized. |
| boundary, saturation, and failure reporting | `BLOCKED` | `SC-VEGETATION-001` `GAP-VEGETATION-004/014..019/021/023`: the A0 boundary is explicit, but complete constitutive domains and guards remain successor work. |
| equifinality/uncertainty retention | `NOT_APPLICABLE` | `package.md` Excluded Scope; caller inputs are not fitted and no uncertainty model is claimed. |
| synthetic recovery | `NOT_APPLICABLE` | `package.md` Excluded Scope; no implementation or calibration machinery exists to recover parameters. |
| additional-data inventory | `PASS` | `primary-source-ledger.md`, `stevens-canyon-invariant-map.md`, and `SC-VEGETATION-001` gap register identify process-authority needs without asking literature to choose every site value. |

The two `BLOCKED` rows describe the deliberately held implementation successor,
not an unmet exit criterion of this bounded authority-reframe package. They
force the global `NOT_CALIBRATION_READY` posture and prohibit releasing the
successor, but they do not require this package to implement excluded process
physics. Demonstration fixtures remain `ASSUMED_FOR_EXECUTION` and prove
semantics and behavior only.
