# No-Compatibility Proof Checklist

Static: planned. Ran evidence must be recorded before closure.

## Forbidden Direct Runtime Tokens

The package must scan `direct_runtime.rs` and all files under
`direct_runtime/*.rs` for:

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

## Scheduler/API Boundary

Static:

- R5C is not authorized to edit scheduler compatibility dispatch or public
  runner API selection.
- Closure must record a scheduler/API diff review for:
  - `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  - `crates/openwepp-runner/src/api.rs`

## Test Binding

Static: the existing source-token unit test must include the new
`direct_runtime/decomposition.rs` file.

## Ran Evidence

Ran:

```text
rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/*.rs
```

Result: PASS, no matches.

Ran:

```text
git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/api.rs
```

Result: PASS, empty diff.

Ran: `r2a_direct_runtime_source_excludes_compatibility_storage_tokens` includes
`direct_runtime/decomposition.rs` and passed under the direct-runtime test
filter.
