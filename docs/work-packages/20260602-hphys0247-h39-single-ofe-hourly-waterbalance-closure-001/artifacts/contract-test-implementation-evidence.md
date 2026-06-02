# Contract-Test Implementation Evidence

Status: updated

Evidence mode: static + ran

Static:
- Added `runtime_inputs::tests::climate_runtime_surface_with_context_uses_cold_trigger_without_snow_sidecar` in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`.
- Added `clim05_contract_conformance_cold_trigger_runs_snow_without_sidecar_gate`
  in `tests/integration/clim05_snow_runtime_kernel_contract.rs`.
- Added `wb19_contract_conformance_requires_bottom_contiguous_lateral_saturation`
  in `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`.
- Added `wb19_contract_conformance_applies_fffx_saturation_fraction_to_lateral_conductivity`
  in `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`.
- Added post-review
  `wb19_contract_conformance_applies_legacy_solwpv_second_fffx_multiplier` in
  `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`.

Ran:
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::climate_runtime_surface_with_context -- --nocapture`
  passed `4` tests.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture`
  passed `6` tests.
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract -- --nocapture`
  passed `11` tests after correcting the full-saturation expected value and
  adding the post-review `solwpv < 2006` multiplier assertion.
