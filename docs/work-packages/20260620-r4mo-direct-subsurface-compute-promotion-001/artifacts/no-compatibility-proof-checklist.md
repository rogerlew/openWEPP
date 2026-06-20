# No-Compatibility Proof Checklist

Status: complete.

Evidence class: Static/Ran.

Required checks:

- Static: direct runtime files contain no `SymbolRegistry`, `BoundarySymbol`,
  `BoundaryValue`, `HillslopeKernelRequest`, `KernelWritebackPayload`,
  `HillslopeWritebackSurface`, dense refresh, dirty flush, or
  `execute_with_kernel` references.
- Static: scheduler diff is empty.
- Ran: default-disabled runner fixture reports zero direct-runtime counters.
- Ran: opt-in runner fixture reports positive direct span counters through
  R4M/O and one compatibility-edge handoff while compatibility publication
  remains authoritative.

## Evidence

Static forbidden-token scan:

```text
rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs
```

Result: PASS. No matches.

Scheduler no-diff:

```text
git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs
```

Result: PASS. No diff.

Default-disabled runner fixture:

```text
cargo test -p openwepp-runner r2a_default_fixture_run_constructs_no_direct_runtime_skeleton -- --nocapture
```

Result: PASS through the `cargo test -p openwepp-runner r2a_ -- --nocapture`
filter; default-disabled direct-runtime counters remain zero.

Opt-in runner fixture:

```text
cargo test -p openwepp-runner r2a_explicit_direct_skeleton_selection_runs_before_compatibility_outputs -- --nocapture
```

Result: PASS through the `r2a_` filter; opt-in counters include R4M/R4O direct
span entries and one production compatibility handoff.

Verdict: PASS.
