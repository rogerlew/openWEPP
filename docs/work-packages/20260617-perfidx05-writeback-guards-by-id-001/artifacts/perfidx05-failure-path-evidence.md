# PERFIDX05 Failure Path Evidence

Ran:
- `cargo test -p openwepp-kernel-contract`
- `cargo test -p openwepp-hillslope-orchestrator`
- `cargo test --workspace`

New focused coverage:
- `apply_by_id_rejects_unknown_symbol_before_mutation`: id-backed writeback rejects an
  unknown logical symbol before mutating surfaces.
- `apply_by_id_keeps_logical_applied_symbols_in_sorted_order`: applied-symbol vectors
  remain in logical string order while application is id ordered.
- `perfidx05_indexed_writeback_rejects_unknown_symbol_with_logical_name`: scheduler
  preserves the logical unknown symbol name in the indexed writeback failure path.
- `indexed_consumer_boundary_reports_same_missing_symbol_for_seeded_family`: indexed
  consumer-boundary validation reports the same missing logical symbol as the logical path.
- `perfidx05_indexed_mofe_sequence_carries_transfer_arrays_by_id`: indexed transfer
  success path carries hourly arrays through pre-resolved ids.

Existing negative coverage exercised in the focused/workspace gates:
- Missing: consumer boundary, decomposition/growth boundary, PL payload, and writeback
  required-symbol tests.
- Non-finite: writeback, growth/decomposition ordering flags, PL dispatch, and hydrology
  guard tests.
- Out-of-range: decomposition payload, PL dispatch, hydrology guard, and transfer overflow
  tests.
- Unknown-symbol: new id-backed writeback registry tests.

Residual:
- The remaining `07_decomposition_equations.rs` prefix overflow guard was not migrated, so
  its failure-path parity remains covered by existing logical tests rather than new id-range
  tests.
