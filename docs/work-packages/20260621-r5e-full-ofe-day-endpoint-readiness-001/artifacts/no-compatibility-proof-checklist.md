# No-Compatibility Proof Checklist

Status: complete.
Evidence mode: Static + Ran.

R5E direct-runtime source scan rejects these tokens from direct-runtime files:

- `SymbolRegistry`
- `BoundarySymbol`
- `BoundaryValue`
- `Option<BoundaryValue>`
- `HillslopeWritebackSurface`
- `KernelWritebackPayload`
- `IndexedWritebackSurface`
- `HotSymbolTables`
- `HillslopeKernelRequest`
- `execute_with_kernel`
- `state_value_for_symbol`
- `flux_value_for_symbol`
- `dirty_state_ids`
- `dirty_flux_ids`

R5E extended the scan to include:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`

Focused scan evidence:

```text
cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture
```

Result: PASS, including
`r2a_direct_runtime_source_excludes_compatibility_storage_tokens`.

Runner counter evidence:

```text
cargo test -p openwepp-runner r2a_ -- --nocapture
```

Result: PASS. The default-disabled runner fixture constructs no direct runtime
skeleton. The explicit direct-skeleton runner fixture records direct counters
before compatibility outputs and exactly the declared publication validation
handoff.

Focused R5E runtime evidence:

- `DirectExecutionReport::canonical_phase_entry_count` records canonical phase
  entries separately from direct sub-operation counters.
- `r5e_direct_endpoint_records_exactly_ordered_fourteen_phase_entries` asserts
  `planned_phase_count == 14`, canonical entries equal
  `14 * lane_count * day_count`, all canonical phase statuses are executed,
  direct sub-operation counters exceed canonical entries, and compatibility
  edge invocations remain zero inside the direct executor.
