# Consumer Path

Status: complete.

## Required Proof

R7F can close only if production direct output consumers still read the direct
executor's `DirectRunPublicationFrame`:

- Producer source: typed production direct day-input path into
  `DirectFrameExecutor`.
- In-memory state: `DirectRunFrame`, `DirectLaneFrame`,
  `DirectPublicationDayInput`, and `DirectRunPublicationFrame`.
- Runner handoff: `HillslopeClimateExecution.retained_direct_publication`.
- Downstream consumers:
  - HBP writer;
  - WAT writer;
  - PASS writer;
  - loss JSON writer;
  - manifest writer.
- Negative proof: production direct must not read compatibility `wb13_rows` as
  authority for those outputs.

## Evidence

Static:

- `execute_hillslope_direct_production_days` stores
  `DirectPublicationExecution` in
  `HillslopeClimateExecution.retained_direct_publication`.
- `build_direct_publication_artifacts` selects the retained direct publication
  frame for `HillslopeRuntimeSelection::DirectProductionExecutor`.
- `build_hillslope_publication_provenance` reads
  `artifacts.execution.publication_frame` for production direct.
- `write_hillslope_run_outputs` routes production direct through
  `write_hillslope_direct_publication_outputs`, which writes HBP, WAT, PASS,
  loss, and manifest artifacts from the direct publication artifacts.
- `execute_hillslope_direct_production_days` returns empty compatibility
  `wb13_rows` and `pass_rows` for production direct; output authority is the
  retained direct publication frame.

Ran:

- `cargo test -p openwepp-runner r6 -- --nocapture` passed, including direct
  cutover/publication consumer tests.
- `cargo test -p openwepp-runner r7 -- --nocapture` passed, including
  production direct manifest/counter tests.
