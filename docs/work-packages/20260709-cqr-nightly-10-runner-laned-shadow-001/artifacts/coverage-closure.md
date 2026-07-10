# Coverage Closure

Evidence label: Static.

Status: `SCAFFOLDED`

ADR-0021 closure will be completed if characterization tests are added or
materially changed.

Initial tier:

- `science-sensitive diagnostic/runtime`
- Rationale: `laned_shadow.rs` carries Lane D runtime diagnostic shadow
  behavior and cites `SC-OFEROUTE-001#INV-OFEROUTE-012`.

Initial threshold state from saved nightly LCOV:

- Lines: `251/452` (`55.53097345132743%`) before package work.
- Regions: not yet measured in this artifact.
- Functions: `23/39`.

Initial obligation map to complete during Phase B if tests change:

| Obligation or invariant | Behavior surface | Current known tests | Status |
|---|---|---|---|
| `SC-OFEROUTE-001#INV-OFEROUTE-010` | conditional/default activation and fallback isolation | `tests/integration/laned_shadow_h2637.rs` selector tests | Pending Phase B confirmation |
| `SC-OFEROUTE-001#INV-OFEROUTE-012` | Lane D shadow/active routing closure seam and subsurface/source shape coupling | `tests/integration/laned_shadow_h2637.rs`; module tests | Pending Phase B confirmation |
| Lane D shadow dynamic friction operand sourcing | live rainfall, routed melt timing, LAI, and canopy height operands | source guard and module tests | Pending Phase B confirmation |
| Protected output identity | shadow diagnostics do not alter HBP/parquet protected outputs | ignored H2637 native-shadow fixture test | Pending Phase B confirmation |

This scaffold does not claim coverage closure.
