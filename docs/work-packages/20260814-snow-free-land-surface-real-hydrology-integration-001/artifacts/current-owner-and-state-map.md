# Current Owner And State Map

Status: `Static trace complete; authority decisions pending Child 1`

V7 vegetation owns canopy radiation, interception, canopy vapor, plant
hydraulics, persistent C/N and material proposals. `SC-WATBAL-001` assigns
candidate layer-liquid mutation to hydrology.

The executable production water owner is the composition rooted at
`DirectFrameExecutor::run_day_spans_hydrology` in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`.
It mutates a lane/day `DirectDayFrame`; `DirectLaneFrame` is the persistent OFE
state inside `DirectRunFrame`. `seed_day_frame` clones lane state and
`commit_day` writes the accepted day back.

The authoritative layer masses are `DirectLaneFrame.subsurface_layers`, whose
`DirectSubsurfaceLayerState` carries liquid depth, frozen depth and frozen water.
`DirectWaterState.soil_water_m` is an aggregate reconstruction, not an
independent layer store. The current surface/depression and residue-interception
amounts are interval records, not a persistent snow-free surface liquid state.

No current owner accepts an external exact-opposite `-G`. Frost is the nearest
thermal owner, but it derives `surtmp`, computes its own surface flux and mutates
frost state. No generic snow-free soil thermal owner exists.
