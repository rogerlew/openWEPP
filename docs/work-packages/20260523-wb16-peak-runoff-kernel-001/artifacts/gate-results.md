# WB16 Gate Results

Status: `completed`
Evidence mode: `Static + Ran`

## Pre-Implementation Contract Gate
- `cargo test --test wb16_peak_runoff_kernel_contract`: **expected fail** before production WB16 implementation (`0 passed; 4 failed`).

## Targeted WB16/Dependency Test Gates
- `cargo test --test wb16_peak_runoff_kernel_contract`: pass (`4 passed`).
- `cargo test --test wb11_hydrology_kernel_contract --test wb12_reconciliation_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract --test wb15_canopy_interception_kernel_contract --test irrig10_irrigation_runtime_kernel_contract --test clim05_snow_runtime_kernel_contract --test clim06_frost_frozen_soil_kernel_contract`: pass.
- `cargo test -p openwepp-hillslope-orchestrator`: pass.

## Post-Implementation Repository Gates
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`) with non-fatal `license-not-encountered` allowlist warnings.
