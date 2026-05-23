# WB12 Reconciliation Kernel Algorithm Guard Map

Status: `completed`
Evidence mode: `Static`

## Production Kernel
- Type: `Wb11HydrologyKernel`
- File: `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- WB12 phase classes:
  - `hydrology_runoff_reconciliation`
  - `hydrology_storage_reconciliation`

## Runoff Reconciliation Algorithm
- Inputs:
  - `wb12_rainfall_input`
  - `wb12_runon_input`
  - `wb12_infiltration`
  - `wb12_depression_storage_delta`
  - `wb12_runoff_observed`
  - `wb12_runoff_closure_tolerance`
- Outputs:
  - flux `Q`
  - flux `wb12_runoff_closure_delta`
  - state `wb12_runoff_reconciled`
- Deterministic rules:
  - `Q = rainfall + runon - infiltration - depression_storage_delta`
  - `closure_delta = Q - runoff_observed`
  - hard-fail when `abs(closure_delta) > tolerance`

## Storage Reconciliation Algorithm
- Inputs:
  - `wb12_storage_initial`
  - `wb12_storage_observed`
  - `wb12_storage_closure_tolerance`
  - `wb12_precip_input`
  - prior fluxes `Q`, `ET`, `D`, `Qd`
- Outputs:
  - state `wb12_storage_reconciled`
  - flux `wb12_storage_closure_delta`
- Deterministic rules:
  - `storage_reconciled = storage_initial + precip_input - Q - ET - D - Qd`
  - `closure_delta = storage_reconciled - storage_observed`
  - hard-fail when `abs(closure_delta) > tolerance`

## Typed Guard Classes
- Missing required symbol: `*-E-001` (`BoundaryClass::MissingRequiredInput`)
- Non-finite required symbol: `*-E-002` (`BoundaryClass::NonFinite`)
- Domain/closure violation: `*-E-003` (`BoundaryClass::DomainViolation`)

## Guard Code Families
- Runoff reconciliation:
  - `HKERNEL-WB12-RUNOFF-E-001`
  - `HKERNEL-WB12-RUNOFF-E-002`
  - `HKERNEL-WB12-RUNOFF-E-003`
- Storage reconciliation:
  - `HKERNEL-WB12-STORAGE-E-001`
  - `HKERNEL-WB12-STORAGE-E-002`
  - `HKERNEL-WB12-STORAGE-E-003`
