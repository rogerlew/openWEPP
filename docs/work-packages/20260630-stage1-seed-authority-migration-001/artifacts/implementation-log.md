# Implementation Log

Evidence mode: Static/Ran.

## Initial Inventory

Ran:

```text
rg -n "require_runtime_surface_scalar|runtime_surface_symbol_value|direct_publication_optional|direct_publication_required|DirectProduction.*from_seed|from_seed\\(|direct_publication_layer_states|direct_publication_profile_inputs|direct_publication_percolation_inputs|direct_publication_subsurface_inputs" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs | wc -l
```

Result: `208`.

Ran:

```text
rg -n "HillslopeWritebackSurface" crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers | wc -l
```

Result: `59`.

## Stage 1B

Static:

- Removed `seed_authority` from
  `direct_production_runoff_publication_geometry`.
- Seeded direct runoff publication `efflen_m` from typed
  `Wb13RunoffPublicationGeometry::ofe_length_m`.
- Left the compatibility replay seam untouched.

Ran:

```text
cargo fmt --check
cargo check -p openwepp-runner
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: all passed.

Ran:

```text
rg -n "require_runtime_surface_scalar|runtime_surface_symbol_value|direct_publication_optional|direct_publication_required|DirectProduction.*from_seed|from_seed\\(|direct_publication_layer_states|direct_publication_profile_inputs|direct_publication_percolation_inputs|direct_publication_subsurface_inputs" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs | wc -l
```

Result after Stage 1B: `207`.

Ran:

```text
rg -n "HillslopeWritebackSurface" crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers | wc -l
```

Result after Stage 1B: `58`.

## Stage 1C Boundary

Static:

- The next seed-authority surface is
  `seed_direct_production_lane_constructor_inputs`.
- It seeds `DirectLaneConstructorInputs` from a day-zero
  `HillslopeWritebackSurface`:
  - `water.soil_water_m`
  - `subsurface_layers`
  - `evapotranspiration_stage_state`
  - `plant_growth_state`
  - `plant_water_stress`
  - `winter_column.snow`
- The day-zero surface is not only a static parser projection. It merges the
  first climate day and then calls `seed_wb11_runtime_surface_inputs`, which
  derives the mutable WB11/WB18/WB19 storage seed state and lane substep
  controls.
- The parsed input-contract data is available earlier in setup, but
  `HillslopeClimateExecutionState` does not carry a typed per-lane seed
  authority into direct execution.

Decision:

- Stop before Stage 1C rather than duplicate runtime-input projection formulas
  ad hoc or hide seed reads behind a wrapper.
- Required follow-on: add a typed per-lane seed-authority carrier derived from
  parsed soil/management/snow/frost/Pmet/slope inputs plus a typed equivalent of
  the WB11 day-zero seed projection, thread it into direct execution, then
  migrate day-zero constructor seeding to that carrier.
