# PERFARRAY02 Determinism Evidence

Evidence: Static + Ran.

Static:

- The pilot uses the existing `SymbolRegistry` and `HotSymbolTables`; no registry
  order mutation is introduced in the hot path.
- `ArrayHotState` is keyed by `SymbolId`; state/flux export is sorted by registry id
  before logical materialization.
- The WB11 runoff kernel arithmetic order is unchanged. The accessor changes only the
  storage representation used for scalar reads.
- The scheduler still runs the same phase graph and same per-OFE sequence. Only the
  `RunoffReconciliation` request/apply authority is switched under the flag.
- No `SC-*` contract was changed.

Ran:

- OFE5 default vs pilot HBP/loss/plot/wat checksums matched.
- OFE5 pass parquet schema and rows matched.
- H2637 default vs pilot HBP/loss/plot/wat checksums matched.
- H2637 pass parquet schema and rows matched.
- `cargo test --workspace` passed.

Conclusion: within-config deterministic output identity passed for the scoped pilot.
