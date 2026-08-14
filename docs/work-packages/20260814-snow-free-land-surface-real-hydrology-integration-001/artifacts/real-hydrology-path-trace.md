# Real Hydrology Path Trace

Status: `Static source trace complete`

1. Owner: `DirectFrameExecutor::run_day_spans_hydrology` over
   `DirectDayFrame`, committed into the lane state by `commit_day`.
2. Snapshot: after day input projection and before the hydrology span.
3. Infiltration/runoff at snapshot: not yet executed.
4. Available water: layer liquid/frozen stores are persistent; canopy release,
   runon, depression and residue interception are day/interval operands. There
   is no persistent snow-free pond/litter store.
5. Legacy double-debit path: both R4N surface ET and root uptake must be omitted
   on the shadow clone, without changing production execution.
6. Clone posture: run/lane/day frames are losslessly cloneable in memory;
   complete restart serialization does not exist.
7. Area: each lane is one OFE; lane area is `fwidth * slplen`; hydrology is
   local depth and converts by lane area only at volume boundaries.
8. Routing: default execution propagates upstream surface/lateral carries to
   the next lane with an upstream/local area ratio; active routing solves lanes
   then routes a cascade. Energy lineage is currently absent.
9. Thermal receipt: no current exact `-G` consumer exists.
10. Real-owner feasibility: a same-day default-off clone is mechanically
    feasible, but production/longitudinal claims require new thermal state,
    advected-energy lineage and snapshot serialization.

Primary source anchors are
`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`,
`03_executor.rs`, `runoff.rs`, `subsurface.rs`, `evapotranspiration.rs`,
`storage.rs`, and the frost coupling modules.
