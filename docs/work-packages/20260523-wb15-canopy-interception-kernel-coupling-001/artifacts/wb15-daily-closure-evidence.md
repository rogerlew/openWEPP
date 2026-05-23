# WB15 Daily Closure Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## WB15 Coupled Closure Equation
`wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - I - Q - ET - D - Qd`

## Nominal WB15 Closure Vector
Inputs/fluxes:
- `wb12_storage_initial = 12.0`
- `wb12_precip_input = 3.0`
- `S = 0.0`
- `I = 0.00157166334`
- `Q = 0.0`
- `ET = 2.0`
- `D = 1.0`
- `Qd = 1.75`

Computed:
- `wb12_storage_reconciled = 10.24842833666`

Observed runtime output:
- `wb12_storage_reconciled` asserted equal to `10.24842833666`
- storage closure delta remained within tolerance in the WB15 nominal vector.
