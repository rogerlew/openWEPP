# Owned file manifest

Status: `PROSPECTIVE — CONTRACT EDIT AUTHORIZED`

Evidence mode: `Static`

## Version-16 exact surface-enthalpy extension

The package is prospectively widened to close the retained `p61` sub-ULP
surface-credit HOLD after the v16 contract-derived expected red. In addition to
the existing owner groups, the bounded write set is:

- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`;
- `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`;
- `docs/specifications/science-contracts/index.md`;
- `tests/integration/land_surface_energy_balance_authority_contract.rs`;
- `tests/integration/surface_liquid_hydrology_custody_authority_contract.rs`;
- `crates/openwepp-land-surface-energy/src/exact_dyadic_enthalpy.rs`;
- new
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_exact_enthalpy.rs`
  and included focused tests;
- new
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v4_projection.rs`
  and included focused tests;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` for module/export
  wiring only;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs`
  only at `credit_retained_receipt_group` and the complete-owner join;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_multitile_adoption.rs`;
- package-owned modules below
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/` only as
  required for exact-total real-consumer adoption;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs`
  only at the covered LSE support-receipt beginning-state selector and focused
  legacy/native-V3/mismatch/rollback/no-publication tests; no other covered
  owner-finalization behavior is authorized;
- new
  `crates/openwepp-persisted-restart-v1/src/frozen_litter_v4_exact_enthalpy.rs`
  and included focused tests;
- `crates/openwepp-persisted-restart-v1/src/lib.rs` for wiring only;
- new
  `crates/openwepp-persisted-restart-v1/src/snow_stage3_v11_v4_exact_enthalpy.rs`,
  plus `crates/openwepp-persisted-restart-v1/src/snow_stage3_v11.rs` and
  `crates/openwepp-persisted-restart-v1/src/hydrology_restart.rs`, only for an
  additive real Stage-3 checkpoint/reload supplement that nests unchanged V3
  bytes and atomically preserves/reconstructs every authoritative V4 exact
  resident; existing V1/V2/V3 schemas and unrelated hydrology behavior are
  frozen;
- `crates/openwepp-persisted-restart-v1/src/projection.rs` and
  `crates/openwepp-persisted-restart-v1/src/transaction.rs`, limited to the
  real checkpoint/reload selection, retention, and restoration of
  `DirectHydrologyExactEnthalpyRestartV2` for live V4 residents; V1 inputs keep
  their unchanged V1 schema and bytes;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs`
  and its existing runtime-accessor seam, limited to read-only detection of any
  retained V4 resident for fail-closed restart selection;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/frozen_litter_v4_adoption.rs`,
  limited to atomic validated installation of a V2-restored hydrology frame;
- `crates/openwepp-hillslope-orchestrator/src/canonical_owner_bytes.rs` and
  `crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs`,
  limited to selecting the already-authoritative augmented V4 LSE owner bytes
  for initial and covered-finalized staged complete-owner publication; r96b
  failed at 0..1800 s before solver because the staged bytes remained V3;
- one new `#[cfg(feature = "restart-authority-evidence")]` helper module under
  `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/` plus
  module wiring only, limited to authentic accepted V3/V4 owner/projection,
  receipt, and seal constituents for the persisted accepted-credit restart
  test; the default/production build and physics are unchanged;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_input_projection.rs`,
  limited to widening the existing crate-private test constructor gate to
  `cfg(any(test, feature = "restart-authority-evidence"))` for that helper;
- `tests/integration/erosion_single_ofe_p61_sediment.rs` and
  `tests/integration/dff_ws1_native_forest_cli.rs` only for successor identity
  binding and independent exact-total assertions;
- `tools/release/authority-policy/impact-map.json` only if a new production
  path lacks an existing exact-path binding.

No production edit is authorized until both v16 authority tests pass and the
new v16 source-obligation tests fail only for missing successor symbols.

The covered-selector seam was added prospectively after the retained 64 MiB
`p61` run cleared the former exact-high mirror refusal and then failed after
`193.22 s` at `176400000000000..178200000000000 ns` with `VEG-E-123`. The
preceding `174600000000000..176400000000000 ns` receipt matched both staged LSE
and soil digests; static inspection then identified the covered path's legacy
inner LSE receipt source. This addition provides adoption parity with the
existing snow-free staged-byte selector and does not alter physics, tolerance,
temporal resolution, closure, custody, rollback, or publication authority.

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

- `crates/openwepp-hillslope-orchestrator/src/lib.rs` (successor export wiring
  only; no legacy export removal);
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` (successor
  module/export wiring only; no legacy export removal);
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner_tests.rs`;
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
- new `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_projection.rs`
  and included focused tests, consuming but never mutating the parent-owned
  `SoilThermalOwnerEnvelopeV2`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_forest.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/multi_tile_runtime.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_derived_ingress.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_input_projection.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_projection.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_rollback.rs`;
- new V3 projection/execution/rollback modules below
  `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint.rs`
  (additive V3 projection/physical endpoint only; legacy entry points unchanged);
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_input_projection.rs`
  (additive checked V3 solver-ready projection only; legacy projection unchanged);
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_multitile_adoption.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs`
  (crate-private successor wiring only);
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_serialization.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_v10_accessors.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs`;
- new frozen-litter modules below
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/`.

## Restart/consumer/boundary evidence owner

- `crates/openwepp-vegetation/src/v8_state.rs` (explicit canonical u128 serde
  plus its existing inline focused tests only; no vegetation physics/state
  changes);
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

## Terminal V3 real-consumer adoption owner

The terminal adoption increment may edit only the following clean successor
seams. It must not edit active V33, open-snow, terminal, carrier, or
owner-finalization sources.

- `crates/openwepp-land-surface-energy/src/transaction.rs` (successor include
  hook only);
- `crates/openwepp-land-surface-energy/src/transaction_v3.rs`;
- new `crates/openwepp-land-surface-energy/src/transaction_v3_covered.rs`
  included as a child of `transaction.rs` when private sealed-carrier access is
  required;
- `crates/openwepp-land-surface-energy/src/solver_litter_phase.rs`;
- `crates/openwepp-land-surface-energy/src/lib.rs` (successor exports only);
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_publication_retention.rs`;
- new
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/frozen_litter_v3_publication_retention.rs`
  plus its module hook;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_projection.rs`
  (read-only canonical nested-byte getters only);
- new frozen-litter V3 production/test modules below
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/`;
- `crates/openwepp-persisted-restart-v1/src/lib.rs` (successor wiring only);
- new V3 checkpoint/scientific-owner/projection/host/test modules below
  `crates/openwepp-persisted-restart-v1/src/`;
- `crates/openwepp-runner/src/hillslope/snow_stage3_v11_production_seed.rs`;
- new frozen-litter V3 seed/test modules below
  `crates/openwepp-runner/src/hillslope/`;
- `crates/openwepp-runner/tests/common/stage3_owner_seed.rs`;
- one new successor seed below `crates/openwepp-runner/tests/fixtures/`;
- `tests/integration/common/mod.rs`;
- `tests/integration/direct_hydrology_persisted_restart_implementation_contract.rs`;
- `tests/integration/direct_hydrology_restart_authority_contract.rs`;
- `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs`;
- new focused modules below
  `tests/integration/land_surface_energy_real_hydrology_shadow_contract/`;
- `tests/integration/land_surface_energy_strict_v8_public_contract.rs`;
- `tests/integration/dff_ws1_native_forest_cli.rs`;
- `tests/integration/erosion_single_ofe_p61_sediment.rs`.
