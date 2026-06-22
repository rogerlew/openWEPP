# Constructor Inventory

Status: complete.

## Surfaces

Static:

- Existing constructor surface before R7B:
  - `DirectRunFrame::skeleton(identity)`.
  - private `DirectLaneFrame::skeleton(lane_index, lane_count)`.
  - `DirectDayFrame::seed(identity, lane_index, day_index)`.
  - private `DirectRunFrame::seed_day_frame(...)`, used by the direct skeleton
    executor and publication capture.
- New R7B constructor surface:
  - `DirectRunConstructorInputs`.
  - `DirectLaneConstructorInputs`.
  - `DirectDayConstructorInputs`.
  - `DirectRunFrame::from_constructor_inputs(...)`.
  - `DirectDayFrame::from_constructor_inputs(...)`.
- The constructor inputs carry typed direct seed data for run identity, lane
  topology, lane geometry, lane persistent state, per-day forcing, R4/R5
  direct phase input families, PMET operands, snow/frost handoffs, hydrology
  projection inputs, and frost layer carry projection metadata.
- Existing `skeleton` / `seed` helpers remain for scaffold/default tests and
  current non-production direct modes.

## Forbidden Compatibility Storage Check

Static:

- Constructor declarations contain no `HillslopeWritebackSurface`,
  `BoundarySymbol`, `BoundaryValue`, `SymbolRegistry`,
  `IndexedWritebackSurface`, `HillslopeKernelRequest`,
  `KernelWritebackPayload`, `Wb13`, or `WB13` tokens.
- Runner source contains no `DirectRunConstructorInputs`,
  `DirectLaneConstructorInputs`, `DirectDayConstructorInputs`, or
  `from_constructor_inputs` invocation, proving R7B does not add default-path
  production constructor work.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r7b -- --nocapture`
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`
- `rg -n "HillslopeWritebackSurface|BoundarySymbol|BoundaryValue|SymbolRegistry|IndexedWritebackSurface|HillslopeKernelRequest|KernelWritebackPayload|WB13|Wb13" crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `if rg -n "DirectRunConstructorInputs|DirectLaneConstructorInputs|DirectDayConstructorInputs|from_constructor_inputs" crates/openwepp-runner/src; then exit 1; fi`
