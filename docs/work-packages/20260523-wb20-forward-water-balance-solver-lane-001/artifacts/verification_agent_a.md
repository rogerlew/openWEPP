# Verification Agent A

Status: `completed`
Evidence mode: `Ran`

## Commands
- `cargo test --test wb20_forward_water_balance_solver_lane_contract`
- `cargo test --test wb12_reconciliation_kernel_contract --test wb11_hydrology_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract --test wb15_canopy_interception_kernel_contract --test wb16_peak_runoff_kernel_contract --test wb17_et_physics_kernel_contract --test clim05_snow_runtime_kernel_contract --test clim06_frost_frozen_soil_kernel_contract --test irrig10_irrigation_runtime_kernel_contract`

## Result
All WB20-targeted and dependent hydrology suites passed.
