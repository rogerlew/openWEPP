# Calibration Readiness Matrix

Status: `BLOCKED / NOT CALIBRATION READY`

Evidence class: `Ran + Static`

| Obligation | Status | Evidence path | Rationale |
|---|---|---|---|
| typed/enumerable parameter surface | `PASS` | `candidate-configurations.csv`; `gsi-domain-grid.csv` | All 9,261 frozen vectors rebuild deterministically. |
| observation operator with units and scale | `PASS` | `calibration-forcing-authority-resolution.md`; `executor-schema.md` | Plot/year/calendar, interval, unit, and equal-year objective semantics are explicit and prospectively reviewed. |
| deterministic candidate execution | `BLOCKED` | `execution-incident-004.md` | Frozen interior `GSI-5557` cannot traverse the real production consumer because positive LAI is paired with non-positive canopy height. |
| objective reconstruction | `BLOCKED` | `execution-incident-004.md` | Population execution was correctly prohibited after native proof failed. |
| sensitivity analysis | `BLOCKED` | `execution-incident-004.md` | No population or later-stage result was generated. |
| identifiability/confounding analysis | `BLOCKED` | `identifiability-and-equifinality.md` | Result-bearing analysis did not run. |
| boundary, saturation, and failure reporting | `PASS` | `saturation-evidence.csv`; `execution-incident-004.md` | Prospective saturation inventory and the observed production failure are retained without filtering. |
| equifinality/uncertainty retention | `BLOCKED` | `execution-incident-004.md` | No accepted ensemble exists to characterize or retain. |
| synthetic recovery | `BLOCKED` | `execution-incident-004.md` | The frozen DAG correctly stopped before synthetic recovery after the real path failed. |
| additional-data inventory | `PASS` | `additional-data-inventory.csv` | Prospective measurements needed for stronger separation are explicit; they do not cure the implementation blocker. |

Required terminal values are `PASS`, `BLOCKED`, or `NOT_APPLICABLE`.
