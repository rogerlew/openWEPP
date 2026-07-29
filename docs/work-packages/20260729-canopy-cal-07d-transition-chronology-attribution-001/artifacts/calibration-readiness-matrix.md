# CAL-07D Calibration Readiness Matrix

Evidence class: `Static + Ran`

| Field | Status | Basis |
| --- | --- | --- |
| `science_implementation_status` | `IMPLEMENTED` | CP-GSI01 equations, indicators, guards, and 21-day state are canonical and implemented. |
| `calibration_evidence_status` | `NOT_APPLICABLE` | CAL-07D assigns all measured evidence `DIAGNOSTIC_ONLY` and performs no calibration. |
| `identifiability_status` | `NOT_ASSESSED` | Constraint attribution is not parameter identification. |

| Readiness obligation | Status | Evidence or rationale |
| --- | --- | --- |
| Typed/enumerable parameters | `PASS` | Frozen CAL-04B 37-member ensemble. |
| Unit/scale-defined observation operator | `PASS` | `source-level-audit.csv` and `model-level-sensitivity.csv`; scale analogy remains assumption-bounded. |
| Deterministic execution | `PASS` | `base-member-daily.csv`, `daily-scenario-ensemble.csv`, and independent validator PASS. |
| Objective reconstruction | `NOT_APPLICABLE` | No calibration objective is optimized. |
| Sensitivity and boundary reporting | `PASS` | `scenario-event-screen.csv`, `decision-screen.csv`, and four figure/sidecar pairs. |
| Synthetic recovery | `NOT_APPLICABLE` | No calibration or recovery claim. |
| Additional-observation inventory | `PASS` | `additional-evidence-needed.csv` and `solution-route-evidence.md`. |
