# WB10 Phase Routing Guard Map

Status: `complete`
Evidence mode: `Static`

## Scheduler Phase-Class Routing Table

| Scheduler phase | Required phase class | Dispatch class | Guard posture |
|---|---|---|---|
| `normalization` | `hydrology` | `Generic` | hard-fail on mismatch |
| `storage_bounds` | `hydrology` | `Generic` | hard-fail on mismatch |
| `evapotranspiration` | `hydrology_evapotranspiration` | `Evapotranspiration` | hard-fail on mismatch |
| `percolation_deep_seepage` | `hydrology_percolation_deep_seepage` | `PercolationDeepSeepage` | hard-fail on mismatch |
| `lateral_transfer` | `hydrology_lateral_transfer` | `LateralTransfer` | hard-fail on mismatch |
| `drainage` | `hydrology_drainage` | `Drainage` | hard-fail on mismatch |
| `runoff_reconciliation` | `hydrology_runoff_reconciliation` | `RunoffReconciliation` | hard-fail on mismatch |
| `storage_reconciliation` | `hydrology_storage_reconciliation` | `StorageReconciliation` | hard-fail on mismatch |
| `closure_diagnostics` | `hydrology` | `Generic` | hard-fail on mismatch |

## Typed Failure Map

| Failure condition | Runtime behavior | Status code | Boundary class |
|---|---|---|---|
| Unsupported scheduler phase-class pair | Reject phase and halt scheduler execution for the phase | `HS-HYDRO-E-001` | `DomainViolation` |
| Missing required consumer symbols after routing validation | Reject phase and halt scheduler execution for the phase | `HS-CONSUMER-E-001` | `MissingRequiredInput` |

## Contract Invariant Linkage

- `SC-WATBAL-001`: `INV-WATBAL-009`, `INV-WATBAL-010`
- `SC-EVAP-001`: `INV-EVAP-011`, `INV-EVAP-012`
- `SC-PERC-001`: `INV-PERC-010`, `INV-PERC-011`
- `SC-SUBHYD-001`: `INV-SUBHYD-012`, `INV-SUBHYD-013`, `INV-SUBHYD-014`
