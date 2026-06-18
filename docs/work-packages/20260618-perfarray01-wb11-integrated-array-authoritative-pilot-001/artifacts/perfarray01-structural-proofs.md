# PERFARRAY01 Structural Proofs

Evidence class: Static.

## Required Proof 1 - No Per-Day Full `BTreeMap` Export

Result: FAIL for Stage B as scoped.

The current kernel request type requires logical maps:

- `core_types.rs:2453` stores logical state as
  `&BTreeMap<BoundarySymbol, BoundaryValue>`;
- `core_types.rs:2454` stores logical flux as
  `&BTreeMap<BoundarySymbol, BoundaryValue>`;
- `core_types.rs:2504` constructs an indexed request from those logical maps
  plus an optional indexed read mirror.

The current scheduler constructs every kernel request from
`writeback_surface.state_surface` and `writeback_surface.flux_surface`
(`scheduler.rs:1606`). Therefore an array-authoritative pilot that reuses the
current request must materialize logical maps before calling WB11. That is the
PERFIDX03 export seam.

## Required Proof 2 - No Normal-Path Logical + Array Dual-Write

Result: FAIL for Stage B as scoped.

The current indexed scheduler applies logical writeback first and then mirrors
to the indexed surface:

- `scheduler.rs:1676` calls `apply_kernel_writeback` on logical maps;
- `scheduler.rs:1714` calls `indexed_writeback_surface.apply_writeback_payload`
  to synchronize the indexed mirror.

Adding `ArrayHotState` beside this path would be another mirror unless the
scheduler mutable authority is flipped to the array. That flip is Stage C-scale
authority work and is explicitly out of scope for PERFARRAY01.

## Perf Evidence

Not run. Producing `perf` evidence for a path known statically to violate both
structural constraints would be misleading. The package acceptance requires
proof that the valid pilot path has no export seam and no dual-write; such a
valid path does not exist yet.

## Conclusion

The two structural proofs are make-or-break gates in `package.md`. Stage B
cannot proceed honestly from the current request/scheduler architecture.
