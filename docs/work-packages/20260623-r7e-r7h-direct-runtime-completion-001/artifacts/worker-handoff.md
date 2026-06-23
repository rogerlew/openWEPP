# Worker Handoff

Status: executed-held.

## First Action

Close defect
`HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`.
Start by replacing the production direct interleaved
`DirectPublicationDayInputBuilder` path with typed direct day-input/state
projection that does not construct or clone `HillslopeWritebackSurface`,
`BoundarySymbol`, `BoundaryValue`, symbol registries, indexed surfaces, dense
refreshes, dirty flushes, or compatibility wrappers inside the production
direct day/OFE loop.

The first concrete proof target is:

- `cargo test -p openwepp-runner r7 -- --nocapture` must pass with
  `r7c_direct_production_executor_reports_interleaved_day_input_compatibility_edges`
  either removed/replaced by a no-compatibility test or updated to assert
  `compatibility_edge_invocations = 0` only after the compatibility edge is
  actually removed.
- Production direct manifests must report
  `/direct_runtime_counters/compatibility_edge_invocations = 0` because no
  compatibility edge was invoked, not because the counter missed the edge.
- Source scans must prove production direct no longer uses
  `HillslopeWritebackSurface`/symbol maps in direct day-input construction.

R7E is already closed in this package. Do not rework the runtime-selection
policy unless the typed day-input replacement needs a manifest field addition.

## Continuation Rule

After each blocker is fixed, rerun the focused and broader failed gates, update
the blocker ledger, and continue to the next R7E-H blocker. Do not stop with a
diagnostic-only handoff unless a legitimate `HOLD-R7-<SPECIFIC-BOUNDARY>` is
proven and reviewed.
