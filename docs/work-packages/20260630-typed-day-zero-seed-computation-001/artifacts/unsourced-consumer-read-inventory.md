# Unsourced Consumer Read Inventory

Evidence mode: Static.

## Historical Gate 1 Result

Static direct-publication seed-read inventory:

```text
rg -n "require_runtime_surface_scalar|runtime_surface_symbol_value|direct_publication_optional|direct_publication_required|DirectProduction.*from_seed|from_seed\\(|direct_publication_layer_states|direct_publication_profile_inputs|direct_publication_percolation_inputs|direct_publication_subsurface_inputs" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs | wc -l
```

Result: `207`.

This was the held-state broad helper inventory. It counted retained
transition/test-only seed-surface adapters as well as production consumer reads.

## Current Production Consumer Result

Production direct no longer calls the symbol-map day-zero seed authority.

Static production call-site audit:

```text
rg -n "from_day_zero_seed_surfaces|direct_publication_day_zero_seed_surface|direct_production_lane_seed_surfaces\\(" \
  crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs \
  crates/openwepp-runner/src/hillslope/snowbench.rs
```

Result: `0` production call sites.

The five clarified Gate 1 consumer groups now read the typed carrier:

- lane constructor;
- `DirectProductionDayInputBuilder`;
- coupling metadata;
- Wave-2 flag;
- winter hourly geometry.

## Historical Unsourced Consumer Groups

The remaining reads still cover the five named consumers:

- lane constructor:
  `seed_direct_production_lane_constructor_inputs`;
- day-input builder:
  `DirectProductionDayInputBuilder::new` and `build_lane_authority`;
- coupling metadata:
  `build_direct_production_coupling_vector_provenance`;
- Wave-2 flag:
  `direct_production_erod14_wave2_enabled`;
- winter hourly geometry:
  `DirectProductionWinterHourlyGeometry::from_climate_context_surface`.

Representative unsourced value families:

- soil/layer storage and profile inputs;
- ET/PMET authority inputs;
- residue cover and plant-growth state/schedules;
- snow/frost options, runtime state, and fine-layer carry;
- erosion/Wave-1/Wave-2 authority inputs;
- lateral/drainage validation and WB16 producer inputs.

## Static Projection Gap

Static parsed-input projection remains symbol-map-fragment based:

```text
rg -n "build_hillslope_runtime_surface_from_|HillslopeWritebackSurface::default\\(|merge_runtime_surfaces|BoundarySymbol::from" crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/intake_lane_setup crates/openwepp-hillslope-orchestrator/src/runtime_inputs | wc -l
```

Result: `499`.

The next valid implementation unit is a real typed static parsed-input
projection plus a full typed per-lane seed carrier. Calling the existing
surface-fragment builders and copying their outputs would be a false
single-authority implementation and is not allowed by the package or the
array-native runtime specification.
