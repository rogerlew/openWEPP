# Owned File Manifest

Status: executed-hold

Evidence mode: Static

Date: 2026-06-11

## Contract And Package Evidence

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - FDHP01 heat-flow depth addendum, `frdp` alias authority, and
    `GAP-SNOWFREEZE-002` v57 D3 hold amendment prohibiting post-hoc scalar
    depth projection into layer stores. Increment A adds v58, pinning
    `frwatc(1)` to hour-1 daily ingress and authorizing shadow fine-state
    aliases plus the internal handoff residual. Increment B adds v59, promoting
    fine-state-derived depth and freeze-arm mutation authority, adding
    `frost.hourly.frzflg_####`, retiring scalar target-depth projection, and
    authorizing threshold-bounded exchange-debit limiting at the
    available-liquid handoff boundary.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - WAT additive-extension versioning clarification for required `frdp`,
    pinned `Total-Soil + frozwt` storage authority, v151 `frozwt`
    publication source binding, and v152 layered `Σ soilf(i)` store binding.
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
  - WAT dataset version `1.4` and `frdp` extension posture.
- `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/**`
  - Package evidence, gates, review/verification records, and disposition.

## Production Rust

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  - Replaced freeze-index proxy with hourly signed heat-flow frost-depth
    progression, separate lower-front `Quf`, fail-closed frozen-water
    exchange, and per-layer frozen-depth/`frzw` storage. Increment A adds
    behavior-preserving shadow `fgfrst`/`slfsd`/`slsic`/`slsw`/`sltime`/
    `yst`/`nwfrzz` handoff state. Increment B derives runtime depth from the
    fine-state scan, mutates freeze-active `slfsd`/`slsic`/`slsw`/`nwfrzz`
    through `frzng`/`frznw` lineage, and limits only threshold-sized exchange
    debit overdraws at the available-liquid boundary.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
  - Seeded initial `frost.runtime_frwatc_*` diagnostics, including
    `frost.runtime_frwatc_frozen_water_after_m`, so WAT publication has a
    fail-closed source symbol before frost activation.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - Frost outcome/profile-depth fields, heat-flow constants, and layer frozen
    depth/`frzw` state fields. Increment A adds shadow fine-layer diagnostic
    structs and symbol roots. Increment B adds hourly `frzflg` diagnostics.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
  - Frost writeback bounds now use physical profile depth and persist layer
    frozen-depth/`frzw` state. Increment A writes the shadow aggregate,
    residual, `yst`/`nwfrzz`, and fine-layer diagnostic symbols. Increment B
    writes bounded hourly `frzflg` diagnostics.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
  - Added fine-layer frost diagnostic symbol formatting.
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - Removed retired model-depth cap constant.
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
  - Added `frdp` column, schema metadata, dataset version `1.4`, and array
    plumbing. Increment A adds deterministic WAT parquet physical bytes while
    preserving file-level Arrow field metadata.
- `crates/openwepp-hillslope-output/Cargo.toml`
  - Declares direct `arrow-ipc`, `base64`, and `flatbuffers` dependencies used
    by the deterministic WAT `ARROW:schema` footer encoder.
- `Cargo.lock`
  - Records the direct `openwepp-hillslope-output` dependency edges already
    present transitively through parquet.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  - Added owned row `frdp_mm` field and retired cap import.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
  - Required runtime `frost.runtime_frdp_m`, bounded it by profile depth, and
    converted it to WAT `frdp`; WAT `frozwt` now requires
    `frost.runtime_frwatc_frozen_water_after_m`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs`
  - Replaced retired cap provenance bound with `solthk` profile bound.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
  - Seeded runner test runtime `frost.runtime_frdp_m` and
    `frost.runtime_frwatc_frozen_water_after_m`.
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
    `Qsrf`/`Quf` and frozen-water overdraw tests. D2 added
    `frost.runtime_frwatc_*` seam diagnostics and freeze/thaw reconciliation
    assertions. The layered continuation added scalar-store rejection and
    layer `frzw` update tests. Increment A adds shadow fine-state round-trip,
    seam identity, and non-driving-output tests. Increment B adds dispatch,
    fine-front energy, `frznw`, and `watdst` vectors.
- `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - Required WAT `frdp` metadata and dataset version `1.4`.
- `tests/integration/sim_contract_boundary_unit_registry.rs`
  - Required `hillslope_wat.frdp` canonical registry alias.
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`
  - Updated contract-version expectations for `SC-SNOWFREEZE-001` v59 and
    `SC-WATBAL-001` v152.
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`
  - Updated contract-version expectations for `SC-SNOWFREEZE-001` v59 and
    `SC-WATBAL-001` v152.
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13.rs`
  - Proves WAT `frozwt` follows
    `frost.runtime_frwatc_frozen_water_after_m`, not `runtime_ws_frz`.
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13_guard.rs`
  - Adds a fail-closed guard for missing
    `frost.runtime_frwatc_frozen_water_after_m`.

## Generated Cohort Evidence

- `fdhp01_closure_summary_20260611.json`
- `fdhp01_run_status_20260611.tsv`
- `fdhp01_activation_summary_20260611.csv`
- `fdhp01_annual_closure_residuals_20260611.csv`
- `fdhp01_depth_metrics_20260611.csv`
- `fdhp01_frozwt_frdp_ratio_20260611.csv`
- `fdhp01_execution_summary_20260611.json`
- `fdhp01_layered_closure_summary_20260611.json`
- `fdhp01_layered_run_status_20260611.tsv`
- `fdhp01_layered_activation_summary_20260611.csv`
- `fdhp01_layered_annual_closure_residuals_20260611.csv`
- `fdhp01_layered_depth_metrics_20260611.csv`
- `fdhp01_layered_frozwt_frdp_ratio_20260611.csv`
- `fdhp01_layered_execution_summary_20260611.json`
- `fdhp01_d3_attempt_summary_20260611.json`
- `fdhp01_d3_attempt_run_status_20260611.tsv`
- `fdhp01_d3_attempt_activation_summary_20260611.csv`
- `fdhp01_d3_attempt_depth_metrics_20260611.csv`
- `fdhp01_d3_attempt_frozwt_frdp_ratio_20260611.csv`
- `fdhp01_d3_attempt_execution_summary_20260611.json`
- `fdhp01_increment_a_pre_current_comparison_20260611.json`
- `fdhp01_increment_a_current_pair_comparison_20260611.json`
- `fdhp01_increment_a_gates_latest_20260611.json`
- `fdhp01_increment_b_execution_summary_20260611.json`
- `fdhp01_increment_b_run_status_20260611.tsv`
- `fdhp01_increment_b_annual_closure_residuals_20260611.csv`
- `fdhp01_increment_b_depth_metrics_20260611.csv`
- `fdhp01_increment_b_frozwt_frdp_ratio_20260611.csv`
- `d3-increment-c-thaw-arms-20260611.md`
- `fdhp01_increment_c_execution_summary_20260611.json`
- `fdhp01_increment_c_run_status_20260611.tsv`
- `fdhp01_increment_c_annual_closure_residuals_20260611.csv`
- `fdhp01_increment_c_depth_metrics_20260611.csv`
- `fdhp01_increment_c_frozwt_frdp_ratio_20260611.csv`
- `fdhp01_increment_c_activation_summary_20260611.csv`
- `d3-increment-c1-capacity-redistribution-20260611.md`
- `fdhp01_increment_c1_execution_summary_20260611.json`
- `fdhp01_increment_c1_run_summary_20260611.json`
- `fdhp01_increment_c1_run_status_20260611.tsv`
- `fdhp01_increment_c1_annual_closure_residuals_20260611.csv`
- `fdhp01_increment_c1_depth_metrics_20260611.csv`
- `fdhp01_increment_c1_frozwt_frdp_ratio_20260611.csv`
- `fdhp01_increment_c1_activation_summary_20260611.csv`
- `fdhp01_increment_c1_p43_aggregate_cap_smoke_20260611.json`
