# Calibration Readiness Matrix

Status: not applicable / no calibration authorized

Evidence mode: Static

- `science_implementation_status`: `AUTHORITY_MISSING` for any replacement or
  state-resolved melt owner; current empirical CoE remains implemented.
- `calibration_evidence_status`: `NOT_APPLICABLE`.
- `identifiability_status`: `NOT_ASSESSED`.

| Required readiness obligation | Disposition | Evidence and rationale |
| --- | --- | --- |
| typed/enumerable parameter surface | `NOT_APPLICABLE` | `package.md` excludes selection or fitting of a candidate |
| observation operator with units and scale | `NOT_APPLICABLE` | 21L observations remain `DIAGNOSTIC_ONLY`; no candidate is selected |
| deterministic candidate execution | `NOT_APPLICABLE` | only current arithmetic is reconstructed; no counterfactual is authorized |
| objective reconstruction | `NOT_APPLICABLE` | no calibration objective exists in this authority audit |
| sensitivity analysis | `NOT_APPLICABLE` | parameter sweeps and result-aware thresholds are excluded |
| identifiability/confounding analysis | `NOT_APPLICABLE` | 21L chronology confounding is retained; no parameter claim is made |
| boundary, saturation, and failure reporting | `NOT_APPLICABLE` | candidate-bound requirement; current audit failures are in `implementation-test-evidence.md` |
| equifinality/uncertainty retention | `NOT_APPLICABLE` | no candidate ensemble or fitted result exists |
| synthetic recovery | `NOT_APPLICABLE` | no calibration machinery is in scope |
| additional-data inventory | `NOT_APPLICABLE` | the blocker is first scientific-authority adjudication, not a data collection claim |

## Existing CoE Surfaces

| Surface | Evidence available | Calibration disposition |
| --- | --- | --- |
| `A` radiation coefficient/albedo lineage | handbook, pinned source, exact reconstruction | do not fit; authority reconciliation first |
| `B` temperature/clear-sky split | pinned chronology and exact subcomponent sums | do not fit; physical-flux identity unresolved |
| `C_open` | exact reconstruction; net negative at every site | do not fit or zero; association is not correctness authority |
| `C_canopy` | exact reconstruction; sole net-positive `C` subcomponent | highest authority priority, not a tuning target |
| `D` rain heat | exact reconstruction; small positive contribution | no transferable correction established |
| daily midpoint gate | exact audited caller reconstruction | adjudicate role and transferability before changing |

Observations and 21L tables are diagnostic only. They do not identify a unique
parameter set, separate chronology from physics, or authorize calibration.
