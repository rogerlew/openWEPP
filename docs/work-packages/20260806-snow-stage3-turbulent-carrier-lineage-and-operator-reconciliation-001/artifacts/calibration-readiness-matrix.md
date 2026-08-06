# Calibration Readiness Matrix

Status: `result-blind frozen`.

Evidence class: `Static`.

| Contract-spec obligation | Status | Evidence / rationale |
| --- | --- | --- |
| `science_implementation_status` | `PASS: IMPLEMENTED` | Existing carrier/transfer authority remains unchanged; contract and `operand-lineage.md`. |
| `calibration_evidence_status` | `PASS: NOT_APPLICABLE` | Package prohibits fitting and calibration. |
| `identifiability_status` | `PASS: NOT_ASSESSED` | Operator mechanics, not parameters, are estimated. |
| Observation role declared prospectively | `PASS` | SNOTEL is `DIAGNOSTIC_ONLY`; `protocol-freeze.json`. |
| Target quantity and observation operator | `NOT_APPLICABLE` | No production observation operator or fitted target is introduced. |
| Parameter names, units, bounds, and provenance | `NOT_APPLICABLE` | No new parameter or bound. Existing solver options are observed, not fitted. |
| Objective/loss and weighting | `NOT_APPLICABLE` | No calibration objective. Frozen reconciliation estimators are in the protocol. |
| Candidate execution and deterministic recovery | `NOT_APPLICABLE` | No candidate search. Result reconstruction is a consumer gate, not calibration. |
| Sensitivity/identifiability analysis | `NOT_APPLICABLE` | No fitted parameter. |
| Synthetic recovery | `NOT_APPLICABLE` | No calibration machinery. |
| Empirical calibration data independence | `NOT_APPLICABLE` | Observations only select diagnostic windows. |
| Independent validation | `NOT_APPLICABLE` | The package cannot make validation or transferability claims. |
| Uncertainty/equifinality reporting | `NOT_APPLICABLE` | No inferred distribution; multi-cause disposition is separately frozen. |
| Calibration/publication claim limits | `PASS` | Package/protocol prohibit calibration, validation, promotion, and cutover. |

No row authorizes a physical-validity, calibration, transferability, or
promotion claim.
