# Publication Authority Evidence

Status: executed-held.

## Consumer Path

Static:

- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` routes
  `HillslopeRuntimeSelection::DirectProductionExecutor` through
  `execution.retained_direct_publication`, validates the retained
  `DirectRunPublicationFrame`, and then builds HBP, WAT, PASS, loss, and
  manifest artifacts from that publication frame.
- The `DirectProductionExecutor` branch does not call
  `build_direct_publication_execution_from_simulation_outputs`; that adapter is
  used for `DirectPublicationFrameCutover`, where WB13 rows are still the
  compatibility execution source.
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
  returns `wb13_rows: Vec::new()` and `pass_rows: Vec::new()` from
  `execute_hillslope_direct_production_days`, retaining the
  `DirectPublicationExecution` produced by `DirectFrameExecutor`.

Consumer-path classification: output consumers for production direct already
read `DirectRunPublicationFrame` artifacts. The remaining R7D failure is not
the file writer selection; it is the producer authority feeding the frame.

## Static Scans

Ran:

- `rg -n "DirectProductionExecutor =>|build_direct_publication_execution_from_simulation_outputs|execution\\.wb13_rows|build_hillslope_wat_rows\\(|build_hbp_output\\(|build_hillslope_pass_row\\(" crates/openwepp-runner/src/hillslope/04_direct_publication.rs crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`

Findings:

- Production direct artifact construction starts at the
  `DirectProductionExecutor` branch in `04_direct_publication.rs` and obtains
  `execution.retained_direct_publication`.
- Compatibility public-output builders still exist in
  `05_runner_execution_and_outputs.rs` for compatibility/default modes and are
  not evidence of production-direct consumer authority.
- `build_direct_production_run_frame` constructs each lane with
  `DirectLaneConstructorInputs::from_topology`, then sets only `area_m2` and
  `upstream_area_ratio`.
- `DirectPublicationDayInputBuilder::seed_surface` clones a single
  `static_runtime_surface`, merges climate, overlays current direct lane state,
  and calls `seed_wb11_runtime_surface_inputs`.
- `OfeLanePersistentStateSequence` contains per-OFE `writeback_surface`
  authority for compatibility scheduler execution, but
  `execute_hillslope_direct_production_days` only uses
  `persistent_lane_state.is_some()` for manifest/state reporting and does not
  convert the lane surfaces into typed direct constructor/day inputs.

Static classification:

- PASS: production direct output consumers do not read `execution.wb13_rows`.
- FAIL/BLOCKED: production direct day-input/producers still read compatibility
  runtime surfaces, and direct lane frames lack lane-indexed typed seed state.

## Operand Reconstruction

Blocked:

- H2637 WAT/PASS operand reconstruction cannot be accepted because producer
  operands are not lane-authoritative. Reconstructing from the produced direct
  Parquet rows would only prove internal consistency of the wrong producer
  state.
- The follow-up must first establish typed per-lane constructor authority for
  direct water, subsurface layers, ET stage/input operands, snow/frost carry,
  transfer buffers, publication geometry, and day forcing. Then independent
  reconstruction can compare HBP/WAT/PASS/loss/manifest operands against
  default compatibility and anti-alias fixtures.
