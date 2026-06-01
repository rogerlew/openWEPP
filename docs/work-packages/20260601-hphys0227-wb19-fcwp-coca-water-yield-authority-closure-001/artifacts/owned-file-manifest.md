# HPHYS0227 Owned File Manifest

Status: completed  
Evidence mode: Static

## Package and Queue

- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/package.md`
- `docs/work-packages/20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/prompts/active/hphys0227_kickoff_agent_prompt.md`
- `docs/work-packages/20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/artifacts/*`

## Contracts and Index

- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`

## External-authority Suite Surfaces

- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_watyld_fcwp_consistency_001.md`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/wb19_fcwp_coca_watyld_cases.json`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.sha256`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.provenance.yaml`

## Test and Registration Surfaces

- `tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `Cargo.toml`

## Production and Compatibility Stabilization Surfaces

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/wb15_canopy_interception_kernel_contract.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- `tests/integration/erod13_wave1_core_kernel_contract.rs`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `tests/integration/hphys0219_wb19_coca_threshold_contract.rs`
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
- `tests/integration/hphys0224_wb19_withdrawal_soilwater_cap_contract.rs`
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`
- `tests/integration/hphys0226_wb19_lateral_saturated_thickness_response_contract.rs`
- `tests/integration/irrig10_irrigation_runtime_kernel_contract.rs`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/wb12_reconciliation_kernel_contract.rs`
- `tests/integration/wb16_peak_runoff_kernel_contract.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `tests/integration/wb20_forward_water_balance_solver_lane_contract.rs`
