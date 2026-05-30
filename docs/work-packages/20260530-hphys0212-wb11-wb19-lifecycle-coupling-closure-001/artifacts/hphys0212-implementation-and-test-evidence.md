# HPHYS0212 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production changes
1. WB11/WB18 lifecycle separation
   - Added one-time seed marker and carry-state behavior so daily execution does
     not reinitialize mutable WB18/WB11 layer state.
   - File: `crates/openwepp-runner/src/hillslope/mod.rs:1681-1908`.
2. WB19 control-source correction
   - Removed runner hardcoded WB19 controls from seed path and added strict
     runtime-domain guards.
   - File: `crates/openwepp-runner/src/hillslope/mod.rs:1931-2000`.
   - Added runtime projections:
     - Soil anisotropy: `runtime_inputs/02_soil_slope.rs:369-381`
     - Management drain controls: `runtime_inputs/01_management.rs:505-593`
3. WB13 coupling publication correction
   - Replaced legacy hardcoded `Tile/SubRIn` behavior with `Qdd`/`SubRIn`
     lineage and guard-enforced `Qd = latqcc + Tile`.
   - File: `crates/openwepp-runner/src/hillslope/mod.rs:4026-4090`.
4. WB19 drainage guard branch correction
   - Drain geometry guards now execute only when `wb19_drain_enabled = 1`.
   - File:
     `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1101-1167`.

## Regression found and fixed during package execution
- Initial WB19 control projection was annual-branch scoped only, which missed
  perennial primary slots.
- Fixed by moving primary-slot WB19 projection to branch-agnostic path in
  `runtime_inputs/01_management.rs:505-593`.
- Added test:
  `management_runtime_projection_projects_wb19_controls_for_primary_perennial_slot`
  in `runtime_inputs/08_tests.rs:1228-1260`.

## Validation execution
- Required workspace gates: pass (see `artifacts/gate-results.md`).
- Targeted touched-crate tests: pass.
- 39-hillslope rerun executed:
  `/tmp/hphys0212_20260530T221447Z/parity/`.
