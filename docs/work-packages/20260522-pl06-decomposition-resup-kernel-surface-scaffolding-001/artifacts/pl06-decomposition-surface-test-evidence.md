# PL06 Decomposition Surface Test Evidence

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Added decomposition interface shape assertions in kernel-contract unit tests.
- Added decomposition context and decomposition boundary guard coverage in hillslope scheduler tests.
- Updated integration tests for expanded 13-phase scheduler and decomposition adapter semantics.

Ran:
- `cargo test --workspace` passed.

## Added/Updated Test Coverage

1. `openwepp-kernel-contract` unit tests:
- `phase_class_decomposition_predicate_matches_contract`
- `request_with_decomposition_context_preserves_typed_phase_metadata`

2. `openwepp-hillslope-orchestrator` unit tests:
- `annual_growth_phase_emits_typed_growth_context` (now validates decomposition context too)
- `perennial_growth_phase_emits_typed_growth_context` (now validates decomposition context too)
- `decomposition_boundary_missing_required_symbol_returns_typed_failure`
- `decomposition_boundary_invalid_ordering_flag_returns_typed_failure`
- Updated deterministic order/phase-failure/writeback assertions for 13-phase graph.

3. Integration updates:
- `tests/integration/hillslope_consumer_boundary_integration.rs`
  - decomposition adapter handling
  - expanded phase-name mapping
  - updated runoff-failure index/count expectations
- `tests/integration/kernel_writeback_contract.rs`
  - updated 13-phase writeback totals

## Ran Command

```bash
cargo test --workspace
```

Result: `PASS`

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:927`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:965`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:2153`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:2203`
- `/home/workdir/openWEPP/tests/integration/hillslope_consumer_boundary_integration.rs:47`
- `/home/workdir/openWEPP/tests/integration/kernel_writeback_contract.rs:76`
