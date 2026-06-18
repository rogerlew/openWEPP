# PERFIDX05 Prefix / Range Proofs

Static:
- No new `[first_id, last_id]` prefix range conversion was introduced in PERFIDX05.
- The existing PL indexed dispatch path uses exact root + slot + crop lookups through
  `IndexedPlSymbolTables`, not lexicographic id ranges. That avoids the interloper problem
  described in the package.
- `HotSymbolTables::state_series` was added only as a root lookup cache for already
  resolved series. It does not broaden membership; individual indexed symbols are still
  accessed by one-based index from the resolved series.

Residual blocker:
- `ensure_no_overflow_indexed_symbols_for_decomposition` still uses a logical
  `strip_prefix` scan to detect unexpected indexed decomposition payload symbols. Because
  it is an overflow guard, replacing it with an id range would require a proof that the
  production registry range contains exactly the intended `pl_decomp_slot_*_crop_*_<root>_<NNNN>`
  members for each root and no interlopers. That proof was not completed in this package.
