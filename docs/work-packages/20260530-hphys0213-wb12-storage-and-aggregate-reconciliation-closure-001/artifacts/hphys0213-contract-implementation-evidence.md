# HPHYS0213 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Contract authority intake
Reviewed canonical authority for scope:
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - WB12 storage reconciliation continuity and closure constraints.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - WB19 `q`, `Qdd`, `Qd` coupling and physically bounded withdrawal semantics.
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - WB11 aggregate soil-water (`wb11_soil_water`) lineage continuity.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - WB18 layer-storage state authority used as WB19 withdrawal source.

No HPHYS0213-specific contract text amendment was required; implementation
closed against existing canonical authority.

## Implementation mapping to contract obligations
1. WB19 lateral withdrawal publication is now realized-withdrawal authoritative
   (not target/synthetic):
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
   - `run_lateral_transfer` now:
     - requires `wb11_soil_water`,
     - withdraws from WB18 layer state first,
     - publishes realized `q`,
     - recomputes `wb11_drainable_storage` from mutated layer state,
     - updates `wb11_soil_water` by realized withdrawal.
2. WB19 drainage publication is now realized-withdrawal authoritative:
   - same file, `run_drainage` now:
     - requires `wb11_soil_water`,
     - publishes realized `Qdd`,
     - recomputes `wb11_drainable_storage`,
     - updates `wb11_soil_water`,
     - preserves `Qd = q + Qdd` with realized `Qdd`.
3. WB12 failure diagnostics now emit explicit storage-term context for closure
   analysis:
   - `crates/openwepp-runner/src/hillslope/mod.rs`
   - `execute_scheduler_kernel_lifecycle` appends `wb12_terms=...` context on
     storage-reconciliation failure.
