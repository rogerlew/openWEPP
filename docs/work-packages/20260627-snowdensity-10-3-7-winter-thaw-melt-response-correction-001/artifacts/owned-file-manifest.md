# Owned File Manifest

Evidence mode: Static.

## Package Files

- `docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/package.md`
- `docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/prompts/active/execute.md`
- `docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/artifacts/**`

## Contract And Planning

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`

## Rust Implementation

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs`
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`

## Tests And Tooling

- `Cargo.toml`
- `tests/integration/snowdensity10_3_7_winter_thaw_melt_response_correction.rs`
- `tests/integration/snowdensity05e_melt_adjudication.rs`
- `tests/integration/snowdensity06b_coe_bound_density_replay.rs`
- `tests/integration/snowdensity10_3_1a_per_day_cancov.rs`
- `tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py`
- `tools/snowfreeze_observed/winter_thaw_melt_response.py`
- `tools/snowfreeze_observed/winter_thaw_melt_response_correction.py`
- `tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py`

## Mechanical Contract-Version Guard Updates

- `tests/integration/snowdensity02_contract_adr_guard.rs`
- `tests/integration/snowdensity05a_melt_contract_guard.rs`
- `tests/integration/snowdensity05b_shortwave_source_contract.rs`
- `tests/integration/snowdensity05c_albedo_state_core.rs`
- `tests/integration/snowdensity05d_opt_in_coe_melt.rs`
- `tests/integration/snowdensity05f_melt_closure_handoff.rs`
- `tests/integration/snowdensity05g_harness_fidelity_rerun.rs`
- `tests/integration/snowdensity06_density_compaction.rs`
- `tests/integration/snowdensity06b_coe_bound_density_replay.rs`
- `tests/integration/snowdensity07_runtime_opt_in.rs`
- `tests/integration/snowdensity08_gate_rerun.rs`
- `tests/integration/snowdensity09_coupled_wat_rerun.rs`
- `tests/integration/snowdensity10_3_1a_per_day_cancov.rs`
- `tests/integration/snowdensity10_3_5a_meteorology_crate_contract.rs`
- `tests/integration/snowdensity10_3_5b_hourly_partition_jennings_contract.rs`

## Generated Evidence Outside Git

- `target/snowdensity10_3_7_winter_thaw_melt_response_correction/**`
