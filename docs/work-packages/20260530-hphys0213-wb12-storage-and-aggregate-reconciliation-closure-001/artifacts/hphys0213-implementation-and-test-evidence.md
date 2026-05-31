# HPHYS0213 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production changes
1. WB19 lateral-transfer realized-withdrawal publication
   - `run_lateral_transfer` now publishes realized `q` from
     `wb19_withdraw_top_down`, recomputes `wb11_drainable_storage` from mutated
     WB18 layers, and updates `wb11_soil_water`.
   - File:
     `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
2. WB19 drainage realized-withdrawal publication
   - `run_drainage` now publishes realized `Qdd` from
     `wb19_withdraw_tile_to_surface`, recomputes `wb11_drainable_storage`,
     updates `wb11_soil_water`, and preserves `Qd = q + Qdd`.
   - File:
     `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
3. WB12 storage failure context enrichment
   - Added WB12 diagnostic term serialization (`wb12_terms=...`) in
     scheduler-kernel failure context for storage-reconciliation failures.
   - File: `crates/openwepp-runner/src/hillslope/mod.rs`

## Integration-surface stabilization
- Updated WB12 observed-storage expectations and WB11 coupling assertions in
  integration fixtures to match realized WB19 withdrawals and new aggregate
  continuity behavior.
- Updated WB19 integration fixture to seed `wb11_soil_water` so drainage/lateral
  guard branches are evaluated as intended.

## Validation execution
- Required workspace gates: pass (see `artifacts/gate-results.md`).
- Contract-derived targeted tests: pass.
- 39-hillslope rerun executed:
  `/tmp/hphys0213_20260530T233248Z/parity/`.

## Rerun outcome summary
- Hillslope execution: `39/39` (`hillslope_batch_status.tsv` all `rc=0`).
- Semantic execution: `39/39` (`semantic_status.tsv` all `rc=0`).
- H5 closure: no `HKERNEL-WB12-STORAGE-E-003` in `h5.stderr.log`.
- Monitored-family summary:
  - `ProfileFCStore`: `27/39` fail hillslopes.
  - `Dp`: `39/39` fail hillslopes.
  - `latqcc`: `39/39` fail hillslopes.
  - `Total-Soil`: `39/39` fail hillslopes.
  - `SoilWaterTotal`: `39/39` fail hillslopes.
