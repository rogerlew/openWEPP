# simimpl19-closure-criteria-evaluation-matrix

Status: complete-with-residual
Evidence mode: static+ran
Date: 2026-05-25

## Static
| Criterion | Result | Notes |
|---|---|---|
| Day-1 `RM` closure (`0.0` under cold all-snow) | pass | contract test passes |
| Day-1 `Snow-Water` runtime SWE closure (`4.4`) | pass | fixture row verified |
| Static `ssd` publication leak removal | pass | runtime SWE-derived publication observed |
| Storage tuple mutation signal | pass | mutation test passes |
| Baseline-authoritative `Total-Soil = watcon` path | partial | provisional fallback still active when `wb11_soil_water` absent |
| Baseline-fidelity `Ep/Es/Er` process implementation | fail | still placeholder formulas in runner publication path |

## Ran
- Verified by targeted contract tests and fixture manifest/row inspection.
