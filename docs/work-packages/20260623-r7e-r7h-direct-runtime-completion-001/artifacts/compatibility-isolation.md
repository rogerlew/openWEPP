# Compatibility Isolation

Status: executed-held.

## Static

- Static: production direct selection in
  `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
  still bypasses:
  - `execute_hillslope_climate_days`;
  - `execute_scheduler_kernel_lifecycle`;
  - `execute_persistent_scheduler_kernel_lifecycle`;
  - symbol-registry audit setup;
  - indexed-shadow audit setup.
- Static: `r7c_direct_production_source_excludes_compatibility_entrypoints`
  still source-scans the production direct function body for compatibility
  scheduler/kernel request entrypoints.
- Static: the remaining blocker is not scheduler entry. It is the interleaved
  production direct day-input builder in
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`.
- Static: that builder retains:
  - `seed_surfaces: Vec<HillslopeWritebackSurface>`;
  - `DirectPublicationClimateContextState` with `rolling_surface:
    HillslopeWritebackSurface`;
  - per-day/lane `seed_surface` cloning and
    `merge_runtime_surfaces(...)`;
  - symbol reads through direct publication helper functions.
- Static: this package now records that remaining builder as a direct runtime
  compatibility edge by calling
  `record_direct_runtime_compatibility_edge_invocation()` during production
  direct day-input builds.

## Ran

- Ran: `cargo test -p openwepp-runner r7 -- --nocapture` passed. The updated
  `r7c_direct_production_executor_reports_interleaved_day_input_compatibility_edges`
  test verifies direct manifests now report compatibility-edge invocations
  equal to direct publication row count for the fixture.

## Forbidden Production Direct Hot-Path Authority

- Compatibility scheduler.
- `HillslopeWritebackSurface`.
- `KernelWritebackPayload`.
- WB13 rows as direct authority.
- Symbol registry lookups.
- Dense refresh.
- Dirty flush.
- Compatibility wrappers around any of the above.

## Disposition

R7F remains open at
`HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`.
The old no-compatibility proof is no longer accepted for current code because
it did not count the direct day-input builder. The next package must replace
that builder with typed direct projection or otherwise remove all
`HillslopeWritebackSurface`/symbol-map construction from the production direct
day/OFE loop.
