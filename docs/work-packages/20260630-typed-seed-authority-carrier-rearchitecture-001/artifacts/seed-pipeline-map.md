# Seed Pipeline Map

Evidence mode: Static.

## Current Ordered Authority Path

Production direct execution still builds its setup seed authority as an ordered
symbol-map pipeline:

1. `build_static_hillslope_runtime_setup` constructs typed parsed inputs and
   publication/topology metadata, then calls `build_static_runtime_surface_parts`.
2. `build_static_runtime_surface_parts` converts typed soil, slope, management,
   PMET, snow, and frost inputs into `HillslopeWritebackSurface` fragments and
   merges them into a run-level `runtime_surface`.
3. `build_persistent_lane_state` builds lane-indexed
   `OfeLanePersistentStateSequence` surfaces for multi-OFE runs by calling
   `build_static_per_ofe_lane_runtime_surface`.
4. `direct_production_lane_seed_surfaces` chooses either the single run surface
   or the lane-indexed persistent writeback surfaces as seed authorities.
5. `direct_publication_day_zero_seed_surface` clones the lane seed authority,
   merges day-one climate from `build_day_climate_surface`, then mutates the
   clone through `seed_wb11_runtime_surface_inputs`.
6. The day-zero surface is then read by the direct production constructor,
   `DirectProductionDayInputBuilder`, coupling metadata, and Wave-2 flag logic.

## Day-Zero Projection Content

`seed_wb11_runtime_surface_inputs` is not a thin default writer. It computes and
publishes the day-zero state consumed by direct production:

- WB18/WB19 lane substep controls, with multi-OFE forcing hourly carry.
- Rainfall and hyetograph normalization from `prcp`, breakpoint flags, `ninten`
  or `nbrkpt`, `timem_*`, and `intsty_*`.
- Initial layer water stores when prior WB11 state is absent:
  `wb18_perc_theta_*`, `wb18_perc_fc_*`, `wb18_perc_ul_*`,
  `wb18_perc_ssc_*`, `wb11_soil_water`, `wb11_field_capacity`,
  `wb11_drainable_storage`, and `wb11_drainage_coefficient`.
- Fine-frost frozen-depth refresh into per-layer frozen-water state.
- Optional defaults for `wb17_residue_interception` and `Ws`.
- WB12 reconciliation inputs and lateral/drainage validation.
- ET demand seed via `compute_wb11_et_demand_seed` /
  `publish_wb11_et_demand_seed`.
- `efflen`, `m`, WB16 `ealpha`, and MOFE03/Wave-2 runtime seed inputs.

## Direct Consumers

| Consumer | Current authority | Values read |
|---|---|---|
| Lane constructor | Day-zero `HillslopeWritebackSurface` in `seed_direct_production_lane_constructor_inputs` | `wb11_soil_water`, layer states, ET stage state, plant growth state, `Ws`, initial snow lane state |
| Day-input builder | Day-zero surface in `DirectProductionDayInputBuilder::new` and `build_lane_authority` | peak runoff, percolation, subsurface compute, infiltration, ET/PMET, residue cover, growth schedules/crops, hydrology projection, erosion, snow/frost |
| Coupling metadata | Outlet day-zero execution surface in `build_direct_production_coupling_vector_provenance` | snow/frost option flags, runtime frost fallback, `ssc`, storage provenance |
| Wave-2 flag | Outlet day-zero execution surface in `direct_production_erod14_wave2_enabled` | `erod14_wave2_enabled` |
| Winter hourly geometry | Last seed surface in `DirectProductionWinterHourlyGeometry::from_climate_context_surface` | slope/aspect and winter hourly geometry scalars |

The previous Stage 1B moved direct runoff publication `efflen_m` to typed
topology geometry, reducing the package seed-read inventory from `208` to
`207`. A narrower direct-publication scalar-helper pattern now reports `206`
matches, but the held package's broader `207` inventory remains the active
progress metric because it includes helper-family seed authority use, not only
scalar reads.

## Missing Typed Projection APIs

No current production path constructs the above carrier directly from
`ParsedHillslopeRunInputs` plus day-one climate:

- Static input projection builders emit `HillslopeWritebackSurface` fragments,
  not typed seed state.
- `seed_wb11_runtime_surface_inputs` and its WB12/WB16/WB19/MOFE03 helpers are
  surface-mutating functions with private intermediate structs.
- Direct authority constructors for day-input state are still
  `from_seed(&HillslopeWritebackSurface)` or helper functions reading the same
  surface.
- The partial frost typed authority is nested inside
  `DirectProductionSnowFrostAuthority::from_seed`; it is not a complete
  parse-derived seed carrier.

## Required Carrier Shape

The durable single-authority carrier needs to be built from typed parse results
and day-one climate before direct execution:

- `DirectProductionSeedAuthority`: run-level static context, lane vector,
  outlet coupling metadata, winter hourly geometry, and Wave-2 flag.
- `DirectProductionLaneSeedAuthority`: per-lane static topology, soil/layer,
  management/growth/residue/PMET, snow/frost, erosion, and irrigation seed
  authority.
- `Wb11DayZeroProjection`: typed output of the day-one climate merge and WB11
  projection, including all derived WB11/WB18/WB19/WB12/WB16/MOFE03 seed values.
- Conversion methods from the typed carrier into `DirectLaneConstructorInputs`
  and `DirectProductionLaneDayInputAuthority`, without reading a
  `HillslopeWritebackSurface`.

## Hold Boundary

Building a "typed" carrier by first calling
`direct_publication_day_zero_seed_surface` would pass a shadow comparison only
because both sides have the same symbol-map authority. That would violate this
package's single-authority requirement. The correct next implementation unit is
to factor typed projection APIs and keep the existing surface writers as adapters
for compatibility replay and transition identity checks.
