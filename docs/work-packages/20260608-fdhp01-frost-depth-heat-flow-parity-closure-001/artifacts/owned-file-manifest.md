# Owned File Manifest

Status: complete

Evidence mode: Static

Date: 2026-06-11

## Contract And Package Evidence

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - FDHP01 heat-flow depth addendum, `frdp` alias authority, and
    `GAP-SNOWFREEZE-002` v55 reopening after cohort validation failed.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - WAT additive-extension versioning clarification for required `frdp`.
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
  - WAT dataset version `1.4` and `frdp` extension posture.
- `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/**`
  - Package evidence, gates, review/verification records, and disposition.

## Production Rust

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  - Replaced freeze-index proxy with hourly signed heat-flow frost-depth
    progression, separate lower-front `Quf`, and fail-closed frozen-water
    exchange.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - Frost outcome/profile-depth fields and heat-flow constants.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
  - Frost writeback bounds now use physical profile depth.
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - Removed retired model-depth cap constant.
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
  - Added `frdp` column, schema metadata, dataset version `1.4`, and array
    plumbing.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  - Added owned row `frdp_mm` field and retired cap import.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
  - Required runtime `frost.runtime_frdp_m`, bounded it by profile depth, and
    converted it to WAT `frdp`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs`
  - Replaced retired cap provenance bound with `solthk` profile bound.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
  - Seeded runner test runtime `frost.runtime_frdp_m`.
- `crates/openwepp-runner/src/hillslope/tests03/trace.rs`
  - Added `frdp_mm` to runner trace fixtures.
- `crates/openwepp-runner/src/constants.rs`
  - Removed retired runner cap constant.
- `crates/openwepp-sim-contract/src/units_mod/boundary_catalog.rs`
  - Added `frdp` alias and unit entry.
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`
  - Added output unit row for `hillslope_wat.frdp`.

## Tests

- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
  - Added FDHP01 heat-flow depth and warm-thaw contract tests; updated older
    CLIM06 assertions away from exact proxy expectations; added separate
    `Qsrf`/`Quf` and frozen-water overdraw tests.
- `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - Required WAT `frdp` metadata and dataset version `1.4`.
- `tests/integration/sim_contract_boundary_unit_registry.rs`
  - Required `hillslope_wat.frdp` canonical registry alias.
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`
  - Updated contract-version expectations for `SC-SNOWFREEZE-001` v55 and
    `SC-WATBAL-001` v149.
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`
  - Updated contract-version expectations for `SC-SNOWFREEZE-001` v55 and
    `SC-WATBAL-001` v149.

## Generated Cohort Evidence

- `fdhp01_closure_summary_20260611.json`
- `fdhp01_run_status_20260611.tsv`
- `fdhp01_activation_summary_20260611.csv`
- `fdhp01_annual_closure_residuals_20260611.csv`
- `fdhp01_depth_metrics_20260611.csv`
