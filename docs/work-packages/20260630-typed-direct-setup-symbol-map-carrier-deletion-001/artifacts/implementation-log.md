# Implementation Log

Evidence mode: Static/Ran.

## Read-In

Static:

- ADR-0030 accepts direct production as the normal hillslope path and permits
  deletion of obsolete compatibility hot-loop machinery under no-regression and
  static-proof gates.
- The partial compatibility deletion package removed skeleton/shadow/cutover
  transition modes, but intentionally retained the explicit
  `--compatibility-runtime` replay/comparator seam and deferred setup-carrier
  deletion.
- The R0/R1 no-compatibility proof plan requires a direct-executor allowlist,
  static call-graph exclusion of symbol-map APIs, and zero forbidden API runtime
  counters.
- The held typed-setup/RSS package corrected the RSS premise: setup carriers are
  an architecture problem, not the dominant RSS driver.
- The array-native spec requires compatibility to be edge-only and forbids
  `HillslopeWritebackSurface`, `KernelWritebackPayload`, `SymbolRegistry`, hot
  tables, indexed surfaces, dense refresh, or dirty flush in production direct
  mode.

## Static Carrier Inventory

Ran:

```text
rg -n "direct_production_lane_seed_surfaces|direct_production_execution_runtime_surface|from_topology_with_dynamic_day_inputs|HillslopeClimateExecutionState|StaticRuntimeSurfaceParts|build_direct_production_run_frame|seed_direct_production_lane_constructor_inputs|HillslopeWritebackSurface|SymbolRegistry|HotSymbol" crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
```

Finding:

- `HillslopeClimateExecutionState` still owns `runtime_surface:
  HillslopeWritebackSurface`, `persistent_lane_state:
  Option<OfeLanePersistentStateSequence>`, and optional `SymbolRegistry` /
  `HotSymbolTables`.
- `execute_hillslope_direct_production_days` still constructs lane seed
  surfaces from `runtime_surface` / persistent lane writeback surfaces.
- `build_direct_production_run_frame` seeds `DirectLaneConstructorInputs` from
  a day-zero seed surface.
- `DirectProductionDayInputBuilder::new` still builds per-lane typed authority
  from `HillslopeWritebackSurface` seed surfaces.

Ran:

```text
rg -n "require_runtime_surface_scalar|runtime_surface_symbol_value|direct_publication_optional|direct_publication_required|DirectProduction.*from_seed|from_seed\\(|direct_publication_layer_states|direct_publication_profile_inputs|direct_publication_percolation_inputs|direct_publication_subsurface_inputs" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs | wc -l
```

Result: `208` remaining direct-publication setup/authority reads from runtime
surface symbols.

Ran:

```text
rg -n "state_surface\\.insert|flux_surface\\.insert|BoundarySymbol::from" crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs crates/openwepp-runner/src/hillslope/intake_lane_setup/runtime_surface_helpers.rs | wc -l
```

Result: `266` runtime-input symbol insertions feeding the current setup bridge.

## Stage 1A

Implemented first safe increment: production direct setup no longer constructs
`SymbolRegistry` / `HotSymbolTables` or activates indexed/lane-dense writeback
authority. Compatibility execution still builds those structures.

This does not close Stage 1. Production direct still constructs
`HillslopeWritebackSurface` seed authorities for lane constructor and day-input
authority seeding.

Ran:

- `cargo fmt --check` - pass.
- `cargo check -p openwepp-runner` - pass.
- Clean `5b139058` baseline H2637 and current H2637 - pass.

Result:

- H2637 HBP/loss/plot/WAT/PASS byte-identical.
- H2637 `compatibility_edge_invocations=0` before and after.
- H2637 RSS improved from `110916 KiB` to `91796 KiB`.
