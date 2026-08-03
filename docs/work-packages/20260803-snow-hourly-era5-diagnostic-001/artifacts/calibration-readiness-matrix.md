# Calibration Readiness Matrix

| Field | Status | Evidence and rationale |
|---|---|---|
| Science implementation | `NOT_IMPLEMENTED` | No production implementation was in scope or changed. |
| Calibration evidence | `NOT_CALIBRATION_READY` | Reanalysis source data and elevation reconciliation validate, but the empirical comparison has not run. |
| Identifiability | `NOT_ASSESSED` | No result-bearing data exist. |

| Obligation | Disposition | Evidence and rationale |
|---|---|---|
| Observation operator and units | `PASS` | `package.md` and `comparison-protocol.md` freeze variables, units, and transforms. |
| Observation roles | `PASS` | Precipitation and SNOTEL roles are protected; ERA5 is diagnostic-only here. |
| Deterministic candidate execution | `PASS` | Exact hourly/elevation inputs, units, domains, identities, and operators are validated; comparison execution remains a later phase. |
| Sensitivity/identifiability | `BLOCKED` | It cannot be inferred before the result-bearing comparison. |
| Synthetic recovery | `NOT_APPLICABLE` | Synthetic forcing cannot establish ERA5 attribution. |
| Independent validation | `NOT_APPLICABLE` | This is a diagnostic package and makes no validation claim. |

The blocked current-scope rows force terminal `HOLD`.
