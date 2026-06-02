# Owned-File Manifest

Status: updated

Evidence mode: static

Static:
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  winter sidecar-discoverability authority.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`: WB19
  `meblfc`/`tdvv`/`fffx` authority.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`: H39
  closure cross-contract invariant.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`:
  winter hourly forcing trigger predicate.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`:
  hydrology active snow coupling trigger predicate.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`:
  WB19 lateral selector, capacity cap, and conductivity weighting.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`:
  SIMIMPL28 no-sidecar cold-trigger test.
- `crates/openwepp-runner/src/hillslope/mod.rs`: manifest/coupling provenance
  `winter.active` semantics.
- `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`:
  runner manifest expectation for sidecar discoverability semantics.
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`: CLIM05
  no-sidecar cold-trigger scheduler test.
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`: WB19
  bottom-contiguous and `fffx` tests plus updated expected values.
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`:
  HPHYS0221 branch expectation updated for HPHYS0247 bottom-contiguous
  saturated selection.
- `tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001/withdrawal_soilwater_cap_cases.json`:
  WB19 `fffx` fixture expected values.
- `tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001/fixtures.sha256`:
  refreshed fixture hash.
- `tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001/fixtures.provenance.yaml`:
  refreshed fixture provenance.
- `tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/layer_pool_withdrawal_cap_cases.json`:
  WB19 `fffx` fixture expected values.
- `tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/fixtures.sha256`:
  refreshed fixture hash.
- `tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/fixtures.provenance.yaml`:
  refreshed fixture provenance.
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/wb19_fcwp_coca_watyld_cases.json`:
  WB19 `fffx` fixture expected values while preserving FC/WP watyld lineage.
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.sha256`:
  refreshed fixture hash.
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.provenance.yaml`:
  refreshed fixture provenance.
- `docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/**`:
  execution evidence, disposition, and handoff artifacts.

Ran:
- Not applicable; this is a static manifest.
