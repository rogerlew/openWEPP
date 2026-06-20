# No-Compatibility Proof Checklist

Static plan:

- Source scan direct-runtime files for forbidden compatibility tokens.
- Review scheduler/runner diff to ensure edits are limited to direct executor
  selection/reporting and do not change default compatibility behavior.
- Verify opt-in runner fixture records exactly one compatibility-edge handoff
  after direct execution.

Ran:

- Forbidden-token scan:

  `rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/*.rs`

  Result: PASS, no matches.

- Scheduler/API diff review:

  `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/api.rs`

  Result: PASS, empty diff.

- Runner opt-in counter proof:

  `cargo test -p openwepp-runner r2a_ -- --nocapture`

  Result: PASS. Default-disabled counters remained zero. Explicit opt-in
  constructed one run frame, two day frames, two day-frame commits, and exactly
  one compatibility-edge handoff for the one-OFE/two-day fixture.
