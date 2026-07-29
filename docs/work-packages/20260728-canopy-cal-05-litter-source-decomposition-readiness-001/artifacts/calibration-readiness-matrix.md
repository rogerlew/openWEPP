# Calibration Readiness Matrix

Evidence class: `Ran + Static`

| Requirement | Native leaf source | Synthetic surface input | Surface decay |
| --- | --- | --- | --- |
| Typed/enumerable surface | Pass | Pass | Pass |
| Real production consumption | Pass, CAL-03 | Not applicable; direct-runtime assumption | Pass, runner projection plus R5C |
| Unit/scale operator | Dry-mass daily transfer defined | `kg m^-2 d^-1` defined | Daily modeled-pool stock defined |
| Deterministic execution | Prior CAL-03 pass | Pass | Pass |
| Independent reconstruction | Prior CAL-03 pass | Pass | Pass |
| Sensitivity | Not reassessed over CAL-04B ensemble | Pass | Pass |
| Synthetic recovery | Not applicable | Pass jointly with rate | Pass jointly with source |
| Boundary/failure | Prior typed guards | Pass | Pass |
| Local sensitivity | Not reassessed over CAL-04B ensemble | Nonzero at all four rate slices | Nonzero at all four source slices |
| Covariance/correlation | Not assessed | Positive source-rate ridge covariance | Ridge correlation retained; no independent endpoint identification |
| Saturation | Not assessed | No source-response saturation in frozen axis | Temperature modifier interior; water factors saturated but nonlimiting |
| Equifinality | Not assessed | Nonidentifiable from terminal stock | Nonidentifiable from terminal stock |
| Suitable empirical calibration data | Missing matched dry-mass composition | Not an empirical source | Missing modeled-pool-equivalent stock series |

The named direct-runtime source/rate operator passes
`CALIBRATION_READY_DATA_LIMITED` under the operator's 2026-07-28 governance
adjudication. The local-sensitivity stencil remains explicitly retrospective,
but it uses only the already frozen exhaustive grid and does not select a
parameter or change a result. This is readiness evidence, not empirical
calibration or validation. The source-composition system and native leaf-only
ensemble do not inherit any machinery result.
