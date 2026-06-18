# PERFIDX05 Determinism Evidence

Static:
- ADR-0022 assigns `SymbolId` in sorted logical-symbol order. PERFIDX05 uses that invariant
  for id-ordered writeback application.
- Applied state/flux symbol vectors are still collected in sorted logical order from the
  id-sorted field lists.
- No phase ordering, OFE lane ordering, or floating-point reduction ordering was changed.
- Transfer array validation still sums the same fixed 24-hour arrays in ascending hour
  order.

Ran:
- `apply_by_id_keeps_logical_applied_symbols_in_sorted_order`
- Full seven-case identity anchor, with `pass.parquet` row-equality and byte identity for
  the other required outputs.
- `cargo test --workspace`

Conclusion:
- Determinism of public outputs and logical applied-symbol ordering held for the final
  PERFIDX05 binary.
