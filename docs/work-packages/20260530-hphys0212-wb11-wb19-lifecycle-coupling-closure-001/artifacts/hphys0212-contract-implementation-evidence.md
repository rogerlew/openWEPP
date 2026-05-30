# HPHYS0212 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Contract authority intake
Reviewed canonical authority for this package scope:
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - WB19 symbol families and WB13 coupling addendum (`Dp`, `latqcc`,
    `SubRIn`, `Tile`; `D -> Dp`, `q -> latqcc`) and closure invariants.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - WB19 lateral/drain symbol requirements and `Qd = q + Qdd` coupling.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - WB18 mutable layer-storage lineage (`wb18_perc_theta/fc/ul/ssc_####`).
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - `wb11_soil_water` aggregate publication lineage.

No HPHYS0212-specific contract text amendment was required; implementation
closed against existing canonical authority.

## Implementation mapping to contract obligations
1. WB11 lifecycle coupling (`st(i)`/`wb18_perc_theta_####` carry state)
   - Implemented one-time seed marker and mutable-state carry behavior:
     `crates/openwepp-runner/src/hillslope/mod.rs:1681-1908`.
2. WB19 runtime-authoritative symbol sourcing
   - Removed hardcoded WB19 controls from WB11 seed path and added required
     runtime symbol/domain guards:
     `crates/openwepp-runner/src/hillslope/mod.rs:1931-2000`.
   - Projected `wb19_lateral_anisotropy_ratio` from soil runtime input:
     `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:369-381`.
   - Projected `drset`/WB19 drain controls from management primary slot with
     typed domain guards:
     `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs:505-593`.
3. WB13 `Qd` coupling visibility
   - WB13 publication now consumes `Qdd`, optional `SubRIn`, and enforces
     `Qd = latqcc + Tile` guard:
     `crates/openwepp-runner/src/hillslope/mod.rs:4026-4063`.
4. WB19 drainage branch guard posture
   - Drain geometry symbols are required only when drain branch is enabled:
     `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1101-1167`.
