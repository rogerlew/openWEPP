# Compatibility Edge Inventory

Status: complete.

## Starting Production Direct Hot Edge

Static:

- Starting producer call site:
  `execute_hillslope_direct_production_days` in
  `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- Starting builder:
  `DirectPublicationDayInputBuilder::new_with_seed_surfaces_and_erosion_guard`.
- Consumer:
  `DirectFrameExecutor::run_publication_capture_with_interleaved_day_inputs`.
- Starting compatibility-shaped structures in the hot loop:
  - `HillslopeWritebackSurface` seed/context surfaces;
  - `BoundarySymbol`/`BoundaryValue` maps;
  - `merge_runtime_surfaces`;
  - symbol lookups through `require_runtime_surface_scalar` and
    `runtime_surface_symbol_value`;
  - explicit compatibility-edge counter call.

## Compatibility Edges Allowed Outside R7F Production Hot Loop

- Explicit compatibility runtime.
- Explicit direct publication shadow/cutover paths inherited from R6.
- Diagnostic/replay utilities that are not entered by production direct mode.
- Static setup-time parsing of existing legacy seed authority until a later
  package migrates every static process-control input into typed authority.

## No-Hide Rule

A new production direct helper is not acceptable if it still constructs the
same `HillslopeWritebackSurface`/symbol-map day/OFE input in the hot loop.

## Final Production Direct Hot Loop

Static:

- Producer call site:
  `execute_hillslope_direct_production_days` now constructs
  `DirectProductionDayInputBuilder`.
- Setup-time authority extraction:
  `DirectProductionDayInputBuilder::new` parses lane authority from seeded
  day-zero surfaces before entering the direct executor loop. This remains
  allowed setup authority, not a hot-loop compatibility edge.
- Hot-loop callback:
  `DirectProductionDayInputBuilder::build` consumes:
  - `HillslopeClimateRuntimeRequest::direct_day_forcing`;
  - `ClimateRunSpanSummary` calendar day metadata;
  - committed `DirectRunFrame` and `DirectLaneFrame` state;
  - pre-parsed lane authority structs.
- Negative source-scan proof:
  `r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads` verifies the
  hot-loop `build` body does not contain `HillslopeWritebackSurface`,
  `BoundarySymbol`, `BoundaryValue`, `merge_runtime_surfaces`,
  `require_runtime_surface_scalar(`, `runtime_surface_symbol_value(`,
  `DirectPublicationDayInputBuilder`, or
  `record_direct_runtime_compatibility_edge_invocation`.

Runtime:

- Explicit production direct fixture manifest:
  `/direct_runtime_counters/compatibility_edge_invocations = 0`.
- Default-activated production direct fixture manifest:
  `/direct_runtime_counters/compatibility_edge_invocations = 0`.
