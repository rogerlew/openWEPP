# Calibration Readiness Matrix

Status: `NOT_CALIBRATION_READY / science implementation complete`

Evidence mode: `Static + Ran`

| Readiness dimension | Disposition | Evidence boundary |
|---|---|---|
| `science_implementation_status` | `IMPLEMENTED` | E01--E22 execute through the public candidate and default-off four-owner diagnostic; final GO/PASS review and exact-head gates pass |
| `calibration_evidence_status` | `NOT_CALIBRATION_READY` | no calibration dataset, objective, parameter estimation, posterior, or acceptance exercise is in scope |
| `identifiability_status` | `NOT_ASSESSED` | no structural or practical identifiability claim is made |
| empirical validation | `NOT_PERFORMED` | digest-bound fixtures and independent equation oracles are implementation evidence, not field validation |
| transferability | `NOT_CLAIMED` | demonstration values remain `ASSUMED_FOR_EXECUTION` only |
| runtime activation | `NOT_ACTIVATED` | diagnostic is default-off; production selectors and legacy PMET/GSI-final-canopy paths are unchanged |
| production consumer cutover | `NOT_PERFORMED` | no real hydrology/LSE/BGC consumer is switched to V7 |

This matrix is a required negative readiness disposition. It deliberately does
not convert implementation completion into calibration or validation readiness.
