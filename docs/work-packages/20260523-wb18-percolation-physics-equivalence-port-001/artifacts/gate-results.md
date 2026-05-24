# Gate Results

Status: `completed`
Evidence mode: `Static + Ran`

## Pre-Implementation Contract Gate
- `cargo test --test wb18_percolation_physics_kernel_contract`:
  - expected fail before production WB18 implementation (`0 passed; 4 failed`)
  - recorded in `wb18-preimplementation-contract-gate.md`

## Targeted WB18/Dependency Gates
- `cargo test --test wb18_percolation_physics_kernel_contract`: pass (`4 passed`)
- `cargo test --test wb11_hydrology_kernel_contract --test wb12_reconciliation_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract --test wb15_canopy_interception_kernel_contract --test wb16_peak_runoff_kernel_contract --test wb17_et_physics_kernel_contract --test irrig10_irrigation_runtime_kernel_contract --test clim05_snow_runtime_kernel_contract --test clim06_frost_frozen_soil_kernel_contract`: pass
- `cargo test --test parser_runtime_seam_integration --test arch22_typed_state_surface_contract`: pass

## Post-Implementation Repository Gates
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`)
  with non-fatal `license-not-encountered` warnings.
