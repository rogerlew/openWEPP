# WB20 Forward Solver Lane Input Manifest

Status: `completed`
Evidence mode: `Static`

## Lane Selector
- `wb20_forward_solver_lane_enabled`
  - `1` -> forward-solver lane (observed targets excluded from acceptance)
  - `0` or absent -> compatibility lane (observed-target diagnostics active)

## Forward Lane Required Runoff Inputs
- `wb12_rainfall_input`
- `wb12_runon_input`
- `wb12_infiltration`
- `wb12_depression_storage_delta`
- `wb12_runoff_closure_tolerance`

## Forward Lane Required Storage Inputs
- `wb12_storage_initial`
- `wb12_storage_closure_tolerance`
- `wb12_precip_input`
- `S`, `Q`, `ET`, `D`, `Qd`

## Forward Lane Acceptance-Excluded Observed Targets
- `wb12_runoff_observed`
- `wb12_storage_observed`

## Compatibility Lane Conditional Inputs
- `wb12_runoff_observed` (required for compatibility-lane runoff closure delta)
- `wb12_storage_observed` (required for compatibility-lane storage closure delta)
