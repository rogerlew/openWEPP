# Current Implementation Grounding

Evidence class: Static.

## Existing Density Lane

- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs:10`
  defines the active density model enum. Current variants are legacy WEPP,
  `physics_bulk_density_compaction_v1`,
  `physics_bulk_spring_densification_v1`, and
  `physics_bulk_shallow_guard_v1`.
- `09_snow_density.rs:32` defines one scalar
  `SnowDensityCompactionConstants` struct. The current adjustable surface is
  fresh-snow density, dry/wet compaction limits and multipliers, PTM/POC rates,
  and the shallow-pack guard threshold.
- `09_snow_density.rs:102` defines scalar runtime inputs:
  prior SWE/depth/density, boundary SWE/depth/density, daily snow input, liquid
  for compaction, mean air temperature, and runtime density cap.
- `09_snow_density.rs:194` through `09_snow_density.rs:244` updates one
  `CoeBoundDensityState`, adds fresh snow, applies daily compaction, then
  force-normalizes mass back to CoE boundary SWE. The runtime output is one
  SWE/depth/density tuple plus closure residuals.

Implication: Paradigm 1 can be expressed as an additional opt-in selector and a
class-aware constants path around the existing scalar state. It can preserve the
current mass-normalization and density-cap behavior if designed carefully.

## Winter Column State

- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs:13` defines
  `DirectWinterColumnState` with separate `snow` and `frost` sub-states.
- `winter_column.rs:28` defines `DirectSnowLaneState` as one runtime SWE,
  depth, density, settle-day count, boundary depth/density, retained liquid, and
  optional albedo state.
- `winter_column.rs:162` starts `DirectFrostLaneState`, which already has a more
  detailed frost lane. Snow does not currently carry a layer vector, grain
  state, layer temperatures, layer liquid, or layer age.

Implication: Paradigm 2 is a state-shape and consumer-surface change, not a
coefficient change. It would require a snow-layer representation, projection
from layer state to scalar HBP/WAT/PASS outputs, compatibility with frost
insulation reads, persistence/trace design, and new performance evidence under
ADR-0025.

## Assessment Boundary

This package makes no code changes. The implementation facts above are used only
to score candidate paradigms and frame a later package if a candidate is chosen.

