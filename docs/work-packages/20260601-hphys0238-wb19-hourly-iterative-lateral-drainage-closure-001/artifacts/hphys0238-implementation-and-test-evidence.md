# HPHYS0238 Implementation and Test Evidence

Status: completed  
Evidence mode: mixed (`Static` + `Ran`)

## Production Implementation (Static)

1. `crates/openwepp-hillslope-orchestrator/src/constants.rs`
   - Added `WB19_SYMBOL_LATERAL_DRAIN_LANE_SUBSTEPS`.

2. `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
   - Added `wb19_lateral_drain_lane_substeps(...) -> Result<usize, ...>`:
     - required positive finite value,
     - required integral value (typed hard-fail on fractional domain).

3. `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
   - `run_lateral_transfer` migrated to lane-substep iteration:
     - per-substep recomputation of WB19 saturated state drivers,
     - per-substep potential scaling by `(86_400 / lane_substeps)`,
     - accumulated daily realized `q`,
     - retained WB19/legacy publication closure (`wb19_watyld`, `wb19_fcdep`,
       `wb19_unsdep`, `wb11_drainable_storage`, `wb11_soil_water`).
   - `run_drainage` migrated to lane-substep iteration:
     - per-substep recomputation of drainage potential/state,
     - per-substep scaling by `(24 / lane_substeps)`,
     - cumulative daily drainage-capacity enforcement,
     - accumulated daily realized `Qdd`, with `Qd = q + Qdd`.
   - Restored explicit `available_pool = layer_pool` assignment to satisfy
     existing HPHYS0225 source-guard contract.

4. `crates/openwepp-runner/src/hillslope/mod.rs`
   - Runner WB11 seed now publishes `wb19_lateral_drain_lane_substeps`
     (`1` daily, `24` hourly) alongside WB18 lane symbol.

## Validation Execution (Ran)

1. `cargo fmt --check`  
   - initial run: fail (formatting only)  
   - rerun after `cargo fmt`: pass

2. `cargo clippy --workspace --all-targets -- -D warnings`  
   - initial run: fail (`clippy::similar_names` in WB19 test variable names)  
   - rerun after rename: pass

3. `cargo test --workspace`  
   - intermediate run: fail due:
     - HPHYS0225 source-guard expectation (`available_pool` assignment absent),
     - WB19 new test assertions assuming lane divergence.  
   - fixes applied:
     - restored explicit `available_pool` assignment,
     - revised WB19 lane tests to stable invariance + conservation + invalid
       lane hard-fail assertions.
   - final rerun: pass (workspace green).
