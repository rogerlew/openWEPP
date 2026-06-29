# Required Reading

Evidence mode: Static.

## Read

- `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
  surface residue / litter cover section: identified the promoted backlog item
  and its residue-specific gates.
- `docs/planning/snow-frost-fidelity-strategy.md` section 11 step 3:
  identified branch C and the mass-to-depth-to-frost missing-coupling root
  cause.
- `docs/work-packages/20260629-frost-step3-residue-parameterization-001/`:
  confirmed the prior entry-gate result that `Dec_*` delivered a flat
  `residue_depth_m` to frost.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  identified the frost consumer invariant and legacy `resdep` lineage.
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`:
  identified the residue/decomposition producer contract home.
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`:
  identified the plant-to-residue transfer authority surface if Phase 0 shows a
  missing input limb.

## Static Code Map

- `runtime_inputs/01_management.rs` seeds `frost.runtime_residue_depth_m` from
  legacy initial residue depth once.
- `direct_runtime/decomposition.rs` computes `surface_residue_kg_m2` and
  `root_residue_kg_m2`, but the current direct-publication builder does not yet
  feed those results into the frost residue-depth thermal input.
- `frost_entry.rs` and `frost.rs` consume `residue_depth_m` in the active frost
  thermal resistance path.

