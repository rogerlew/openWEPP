Static: experimental implementation posture; no empirical calibration claim.

Scope: A observation harness, isolated F invocation-local feed-forward carrier,
and isolated R same-sweep component-temperature dependency replay. These change
measurement or execution scheduling, not process equations, parameter values,
domains, finite-difference arithmetic, stopping criteria, or observation models.
Authority: SC-SNOWENERGY-001 INV-088/C-056 and its prospective experiment
subsection; SC-LANDSURFACEENERGY-001 component-temperature replay and prospective
stencil-aware experiment subsections. Source availability does not establish
admission: gate-results.md and final-disposition.md own actual acceptance.

| Reporting field | Disposition | Meaning |
|---|---|---|
| science_implementation_status | IMPLEMENTED | Existing canonical science is implemented; A/F/R source implementations exist. Experimental F/R validation and retention remain separately governed; this is not production promotion. |
| calibration_evidence_status | NOT_APPLICABLE | No process parameter estimation or new parameterized science is authorized. |
| identifiability_status | NOT_APPLICABLE | Neither mechanism estimates physical parameters or identifies a new physical model. |

The following rows disposition every readiness obligation in
docs/specifications/science-contract-spec.md:137–157. NOT_APPLICABLE concerns
calibration obligations only; it does not waive execution, error/rollback,
output parity, uncertainty, or boundary tests required by this experiment.

| Obligation | Disposition | Evidence path and rationale |
|---|---|---|
| typed/enumerable parameter surface | NOT_APPLICABLE | package.md, protected-boundaries clause: no parameter introduced, fitted, or changed. Typed experimental request/evaluator interfaces are implementation interfaces, not calibration parameters. |
| observation operator with units and scale | NOT_APPLICABLE | artifacts/experiment-protocol.md: wall/CPU/RSS observations assess engineering mechanisms, not a physical observation operator. Science-output-parity.md owns the separately required physical-output units map. |
| deterministic candidate execution | NOT_APPLICABLE | Calibration candidate execution is absent. artifacts/source-and-build-manifest.json and artifacts/gate-results.md separately bind mandatory deterministic experimental execution. |
| objective reconstruction | NOT_APPLICABLE | No parameter-fitting objective. artifacts/experiment-protocol.md specifies independently reconstructed timing/memory treatment comparisons. |
| sensitivity analysis | NOT_APPLICABLE | No calibratable physical parameter perturbation. F/R treatment contrasts are engineering experiments, not physical parameter sensitivity. |
| identifiability/confounding analysis | NOT_APPLICABLE | No physical parameter inference. Matched posture/isolation controls in artifacts/experiment-protocol.md address experimental confounding separately. |
| boundary, saturation, and failure reporting | NOT_APPLICABLE | No calibration boundary ensemble. artifacts/gate-results.md must still record protected physical boundary, stale/foreign/reuse, rollback and failure tests required for admission. |
| equifinality/uncertainty retention | NOT_APPLICABLE | No fitted parameter ensemble/equifinality claim. Timing uncertainty and memory lifetimes remain mandatory in artifacts/treatment-results.md. |
| synthetic recovery | NOT_APPLICABLE | There is no inverse problem or planted parameter to recover; creating one would exceed scope. |
| additional-data inventory | NOT_APPLICABLE | No empirical calibration data are needed for behavior-preserving scheduling. Lack of observations cannot justify surrogate physics or a calibration HOLD. |

All runtime timing/memory samples and generated fixtures are DIAGNOSTIC_ONLY
for empirical science. Executed contract/output parity is software correctness
evidence, not empirical calibration, independent field validation, or model
transferability. The prospective 5% engineering decision threshold and sampling
counts are experimental design choices, not fitted physical constants.
