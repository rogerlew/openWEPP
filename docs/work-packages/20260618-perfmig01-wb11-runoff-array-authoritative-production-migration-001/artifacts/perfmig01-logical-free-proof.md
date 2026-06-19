# PERFMIG01 Logical-Free Proof

Evidence: Static + Ran.

## Migrated Success Path

Static:

- `run_runoff_reconciliation` calls
  `build_warm_rain_runoff_indexed_writeback` before the legacy
  `state_updates` vector is constructed.
- When the builder returns `Some`, the phase returns
  `KernelRunResponse::with_indexed_writeback(status, indexed_writeback)`.
- `KernelRunResponse::with_indexed_writeback` constructs an empty logical
  `KernelWritebackPayload` and stores the id-backed payload separately.
- The migrated builder creates `Vec::with_capacity(543)` state updates and
  `Vec::with_capacity(8)` flux updates with `IndexedWritebackField`.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator perfmig01_wb11_warm_rain_indexed_writeback_is_bit_identical -- --nocapture
```

Observed: the indexed response had `0` logical state updates and `0` logical
flux updates while the id-backed payload had `543` state and `8` flux updates.

## No Full Seam Export

The indexed scheduler branch does not call `export_btreemap_surfaces` or
`from_logical_payload` for the migrated payload. It calls
`apply_indexed_kernel_writeback`, which sorts only the updated ids and
materializes only those symbols for downstream compatibility.

Static boundary:

- `apply_indexed_kernel_writeback` updates `IndexedWritebackSurface` first;
- then it resolves each updated `SymbolId` to a `BoundarySymbol`;
- then it inserts only that updated symbol/value into the logical state or flux
  map.

This is a transitional compatibility boundary, not the migrated kernel authority
surface.

## No Normal-Path Dual Write Inside The Migrated Kernel

The migrated WB11 branch builds a single id-backed payload. It does not also
build the legacy logical writeback payload. The scheduler performs compatibility
materialization after the kernel response because downstream phases still read
logical maps in this rung.

## Remaining Logical Branches

The following branches are explicitly not migrated in PERFMIG01:

- active snow coupling;
- active frost coupling;
- active irrigation event;
- MOFE hourly carry arrays.

Those branches remain logical by design and are the boundary for the next rung,
not evidence of a hidden fallback in the migrated warm-rain path.
