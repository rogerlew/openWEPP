# Owned File Manifest

Status: executed-hold

Evidence mode: Static

Date: 2026-06-12

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
    available-liquid handoff boundary. Increment C1b adds v60/v61,
    authorizing fine-layer capacity/overflow semantics, `watpdg`/`watbtm`
    publication surfaces, `watbtm` as WB13 `Dp` lineage, and bounded WB18/WB13
    roundoff handling. Increment C2 adds v62, authorizing top/bottom thaw,
    sandwich geometry, `fgthwd`, `nwfrzz` release, and non-amplifying
    repeated freeze/thaw conservation. Increment Db adds v63, binding
    freeze-active `frzng` to in-hour surface-resistance/`Qsrf` recomputation
    after each fine-layer front advance. Increment Dc1 adds v64, retiring the
    stable lower-front heat surrogate in favor of legacy seasonal
    `tmpbl`/`Qdry`, requiring in-hour thaw resistance feedback, and
    authorizing only bounded fine-theta lower-bound roundoff canonicalization.
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
    debit overdraws at the available-liquid boundary. Increment C1b adds
    capacity-limited fine-layer liquid/ice ownership, downward redistribution,
    `watpdg`/`watbtm` overflow accounting, and overflow-inclusive shadow
    residual accounting. Increment C2 adds top/bottom thaw arms, sandwich
    geometry, thaw-through handoff, release of thawed `nwfrzz`, and final
    scalar/layer egress reconciliation from the owned fine state. Increment Db
    adds in-hour freeze-front resistance feedback so the freeze loop
    recomputes surface resistance after each fine-layer advance before
    consuming additional freezing time. Increment Dc1 adds seasonal
    lower-front heat from the legacy monthly temperature wave, in-hour thaw
    resistance feedback, and bounded fine-theta lower-bound canonicalization.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
  - Seeded initial `frost.runtime_frwatc_*` diagnostics, including
    `frost.runtime_frwatc_frozen_water_after_m`, so WAT publication has a
    fail-closed source symbol before frost activation.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - Frost outcome/profile-depth fields, heat-flow constants, and layer frozen
    depth/`frzw` state fields. Increment A adds shadow fine-layer diagnostic
    structs and symbol roots. Increment B adds hourly `frzflg` diagnostics.
    Increment C1b adds `watpdg`/`watbtm` runtime overflow fields and symbols.
    Increment Dc1 replaces the stable lower-front heat constants with seasonal
    lower-front heat, fallback conductivity, damping-depth, and fine-theta
    boundary constants.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
  - Frost writeback bounds now use physical profile depth and persist layer
    frozen-depth/`frzw` state. Increment A writes the shadow aggregate,
    residual, `yst`/`nwfrzz`, and fine-layer diagnostic symbols. Increment B
    writes bounded hourly `frzflg` diagnostics. Increment C1b writes
    `watpdg`/`watbtm` overflow surfaces and bounds aggregate `frzw` by `ul`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_infiltration_evap.rs`
  - Increment C1b preserves same-pass infiltration lineage from WB12 instead
    of replaying WB14 while reconciling the fine frost state.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`
  - Increment C1b resolves effective frozen depth from the fine/WB19 state,
    canonicalizes bounded deep-percolation dust before storage debit, preserves
    scalar soil water on zero uptake, and rebalances bounded scalar/layer
    roundoff without hiding real percolation or overflow.
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
    `frost.runtime_frwatc_frozen_water_after_m`. Increment C1b adds WB13
    `Dp` publication from `D + frost.runtime_watbtm_m` with bounded source
    roundoff canonicalization.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/hphys_trace.rs`
  - Increment C1b trace guard diagnostics prefer `wb19_dg_####` over legacy
    `dg_####` when validating WB18 frozen-depth state.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
  - Increment C1b refreshes WB18 frozen-depth state from the fine frost state
    before downstream hydrology phases.
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
    fine-front energy, `frznw`, and `watdst` vectors. Increment C1b adds
    capacity, active-`ul`, overflow, and shadow-identity vectors. Increment C2
    adds bottom-thaw, top-thaw, sandwich geometry, `fgthwd`, and multicycle
    non-amplification vectors. Increment Db adds a within-hour freeze-front
    resistance feedback vector that fails on stale start-hour `Qsrf` spending.
    Increment Dc1 adds seasonal lower-front heat, thaw-feedback, and
    fine-theta boundary vectors.
- `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - Required WAT `frdp` metadata and dataset version `1.4`.
- `tests/integration/sim_contract_boundary_unit_registry.rs`
  - Required `hillslope_wat.frdp` canonical registry alias.
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`
  - Updated contract-version expectations for `SC-SNOWFREEZE-001` v64 and
    `SC-WATBAL-001` v152.
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`
  - Updated contract-version expectations for `SC-SNOWFREEZE-001` v64 and
    `SC-WATBAL-001` v152.
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13.rs`
  - Proves WAT `frozwt` follows
    `frost.runtime_frwatc_frozen_water_after_m`, not `runtime_ws_frz`.
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13_guard.rs`
  - Adds a fail-closed guard for missing
    `frost.runtime_frwatc_frozen_water_after_m`; Increment C1b adds WB13
    `watbtm`/`Dp` publication and bounded source-dust tests.
- `crates/openwepp-runner/src/hillslope/tests03/trace.rs`
  - Increment C1b adds a WB18 guard-term test for preferred WB19 layer
    geometry in trace diagnostics.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs`
  - Increment C1b adds WB17/WB18 scalar preservation, positive deep-loss,
    no-flux rebalance, and roundoff canonicalization regressions.

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
- `d3-increment-c1a-seam-accounting-20260611.md`
- `fdhp01_increment_c1a_seam_accounting_summary_20260611.json`
- `fdhp01_increment_c1a_seam_ledger_excerpt_20260611.csv`
- `d3-increment-c1b-capacity-overflow-20260612.md`
- `fdhp01_increment_c1b_execution_summary_20260612.json`
- `fdhp01_increment_c1b_run_status_20260612.tsv`
- `fdhp01_increment_c1b_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_c1b_depth_metrics_20260612.csv`
- `fdhp01_increment_c1b_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_c1b_starter_capacity_20260612.json`
- `d3-increment-c2-thaw-arms-20260612.md`
- `fdhp01_increment_c2_execution_summary_20260612.json`
- `fdhp01_increment_c2_run_status_20260612.tsv`
- `fdhp01_increment_c2_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_c2_depth_metrics_20260612.csv`
- `fdhp01_increment_c2_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_c2_activation_summary_20260612.csv`
- `d3-increment-da-energy-characterization-20260612.md`
- `fdhp01_increment_da_execution_summary_20260612.json`
- `fdhp01_increment_da_run_status_20260612.tsv`
- `fdhp01_increment_da_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_da_depth_metrics_20260612.csv`
- `fdhp01_increment_da_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_da_activation_summary_20260612.csv`
- `fdhp01_increment_da_c2_row_equality_20260612.json`
- `fdhp01_increment_da_p1_hourly_energy_trace_20260612.csv`
- `fdhp01_increment_da_p1_energy_summary_20260612.json`
- `d3-increment-db-freeze-resistance-20260612.md`
- `fdhp01_increment_db_execution_summary_20260612.json`
- `fdhp01_increment_db_run_status_20260612.tsv`
- `fdhp01_increment_db_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_db_depth_metrics_20260612.csv`
- `fdhp01_increment_db_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_db_activation_summary_20260612.csv`
- `d3-increment-dc-seasonal-thaw-20260612.md`
- `fdhp01_increment_dc_execution_summary_20260612.json`
- `fdhp01_increment_dc_run_status_20260612.tsv`
- `fdhp01_increment_dc_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dc_depth_metrics_20260612.csv`
- `fdhp01_increment_dc_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dc_activation_summary_20260612.csv`
- `d3-increment-dc1-accounting-repair-20260612.md`
- `fdhp01_increment_dc1_execution_summary_20260612.json`
- `fdhp01_increment_dc1_run_status_20260612.tsv`
- `fdhp01_increment_dc1_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dc1_depth_metrics_20260612.csv`
- `fdhp01_increment_dc1_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dc1_activation_summary_20260612.csv`
- `d3-increment-dd-legacy-snow-forced-20260612.md`
- `fdhp01_increment_dd_execution_summary_20260612.json`
- `fdhp01_increment_dd_run_status_20260612.tsv`
- `fdhp01_increment_dd_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dd_depth_metrics_20260612.csv`
- `fdhp01_increment_dd_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dd_activation_summary_20260612.csv`
- `fdhp01_increment_dd_legacy_winter_generation_20260612.json`
- `fdhp01_increment_dd_legacy_snow_forcing_summary_20260612.json`
