# Owned File Manifest

Status: complete

Evidence mode: static

Static:

Primary implementation files:

- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/01_phase_routing.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`

Primary tests and fixture updates:

- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/wb12_reconciliation_kernel_contract.rs`
- `tests/integration/hillslope_consumer_boundary_integration.rs`
- `tests/integration/int10_plant_water_coupling_validation_contract.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- `tests/integration/erod13_wave1_core_kernel_contract.rs`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `tests/integration/irrig10_irrigation_runtime_kernel_contract.rs`
- `tests/integration/kernel_writeback_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/wb15_canopy_interception_kernel_contract.rs`
- `tests/integration/wb16_peak_runoff_kernel_contract.rs`
- `tests/integration/wb20_forward_water_balance_solver_lane_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`

Contracts and package docs:

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/**`
