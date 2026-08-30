# Owned file manifest

Status: `PROSPECTIVE — CONTRACT EDIT AUTHORIZED`

Evidence mode: `Static`

Unlisted paths block edits. New modules may be renamed only by a prospective
manifest amendment before creation. Each implementation worker receives an
exclusive group below and must preserve concurrent edits in all other groups.

## Authority/package owner

- `docs/work-packages/20260830-frozen-forest-litter-phase-authority-001/**`;
- `references/vendorable/surfex-v8/README.md`;
- `references/vendorable/surfex-v8/isba_meb.F90.source.html`;
- `references/vendorable/surfex-v8/isba_fluxes_meb.F90.source.html`;
- `references/vendorable/surfex-v8/ini_csts.F90.source.html`;
- `references/vendorable/surfex-v8/CeCILL-C_V1-en.html`.

## Contract/test owner, before production edits

- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`;
- `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`;
- `docs/specifications/science-contracts/index.md`;
- `tests/integration/land_surface_energy_balance_authority_contract.rs`;
- `tests/integration/surface_liquid_hydrology_custody_authority_contract.rs`;
- `tools/release/authority-policy/impact-map.json`.

## LSE V3 production/test owner, after retained pre-red

- `crates/openwepp-land-surface-energy/src/lib.rs`;
- `crates/openwepp-land-surface-energy/src/config.rs`;
- `crates/openwepp-land-surface-energy/src/physics.rs`;
- `crates/openwepp-land-surface-energy/src/solver.rs`;
- `crates/openwepp-land-surface-energy/src/solver_covered_evaluation.rs`;
- `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs`;
- `crates/openwepp-land-surface-energy/src/transaction.rs` (include hook only);
- `crates/openwepp-land-surface-energy/src/water.rs`;
- `crates/openwepp-land-surface-energy/src/error.rs`;
- new `crates/openwepp-land-surface-energy/src/v3_state.rs`;
- new `crates/openwepp-land-surface-energy/src/litter_phase.rs`;
- new `crates/openwepp-land-surface-energy/src/litter_phase_closure.rs`;
- new `crates/openwepp-land-surface-energy/src/litter_phase_output.rs`;
- new `crates/openwepp-land-surface-energy/src/solver_litter_phase.rs`;
- new `crates/openwepp-land-surface-energy/src/transaction_v3.rs`;
- focused new V3 test modules included from those production modules;
- new `crates/openwepp-land-surface-energy/artifacts/openwepp_snow_free_lse_v3_definition.json`;
- new `crates/openwepp-land-surface-energy/artifacts/openwepp_snow_free_lse_v3_phase_vectors.json`.

## Surface-owner/orchestration production owner, after retained pre-red

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_attachment.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_coordinator.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_preflight.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ending_validation.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_enthalpy_closure.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_wb14.rs`;
- new modules below
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/`
  for V2 state, identity, migration, restart, closure, and focused tests;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_forest.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_input_projection.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_projection.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_rollback.rs`;
- new V3 projection/execution/rollback modules below
  `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_serialization.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_v10_accessors.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs`;
- new frozen-litter modules below
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/`.

## Restart/consumer/boundary evidence owner

- `crates/openwepp-runner/src/hillslope/snow_stage3_v11_production_seed.rs`;
- `crates/openwepp-runner/tests/common/stage3_owner_seed.rs`;
- new successor seed fixture below `crates/openwepp-runner/tests/fixtures/`;
- `docs/sim-contract-boundary-units.md`;
- `tests/integration/sim_contract_boundary_unit_registry.rs`;
- `tests/integration/direct_hydrology_persisted_restart_implementation_contract.rs`;
- `tests/integration/direct_hydrology_restart_authority_contract.rs`;
- `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs`;
- new focused modules below
  `tests/integration/land_surface_energy_real_hydrology_shadow_contract/`;
- `tests/integration/land_surface_energy_strict_v8_public_contract.rs`;
- `tests/integration/dff_ws1_native_forest_cli.rs`;
- `tests/integration/erosion_single_ofe_p61_sediment.rs`.
