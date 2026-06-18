# PERFIDX05 Writeback And Guard Inventory

Static:
- Writeback apply: `openwepp-kernel-contract::writeback` now exposes
  `apply_kernel_writeback_by_id` and `apply_kernel_writeback_by_id_with_indexed_mirror`.
  The scheduler uses the combined path when an indexed mirror and registry are available.
- Indexed mirror apply: `IndexedWritebackSurface::apply_writeback_payload` resolves and
  applies writeback fields by `SymbolId` order.
- Consumer boundary: `validate_hillslope_consumer_boundary_indexed` checks required
  runtime-family symbols through hot id tables while preserving logical symbol names in
  `HillslopeConsumerBoundaryError`.
- Transfer validation/mutation: MOFE transfer insertion, removal, readback, and validation
  use hot symbol ids where available and logical fallback otherwise.
- Hot tables: `build_hillslope_hot_symbol_tables` includes consumer-boundary sentinel
  scalars and MOFE transfer roots used by the migrated paths.
- Tests: added id-order writeback, unknown-symbol, indexed consumer-boundary, and indexed
  MOFE transfer coverage.

Static residuals:
- The scoped PL dispatch files (`00_pl_slot_resolution.rs`, `05_pl_phase_dispatch.rs`)
  already use exact `IndexedPlSymbolTables` lookups for production indexed execution.
- A remaining decomposition overflow guard in
  `hydrology/07_decomposition_equations.rs::ensure_no_overflow_indexed_symbols_for_decomposition`
  still scans logical symbols by prefix. Migrating it safely requires threading indexed PL
  symbol context through decomposition-equation control builders; this is recorded as the
  PERFIDX05 residual blocker.
- No irrigation pre-resolution, wiring, or activation was added.

Ran:
- `rg -n "starts_with|strip_prefix|prefix|range|require_integral_pl_dispatch_symbol_ref_in_range" ...`
  confirmed the scoped PL dispatch files do not contain broad production prefix scans;
  the residual prefix scan is in `07_decomposition_equations.rs`.
