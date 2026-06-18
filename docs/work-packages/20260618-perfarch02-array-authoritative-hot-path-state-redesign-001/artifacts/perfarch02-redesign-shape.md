# PERFARCH02 Redesign Shape

Evidence class: Static design, backed by the artifact-local prototype in
`artifacts/perfarch02-floor-prototype/`.

## Verdict

The next performance architecture should make the dense indexed representation
the authoritative hot-path state, not a read mirror beside the current logical
`BTreeMap<BoundarySymbol, BoundaryValue>` surface. The logical surface remains
the external contract and publication boundary. It must not remain the daily
kernel-loop store.

This is a representation/interface change, not a science-contract change. The
physics output obligation stays bit-identical on migrated flows.

## Proposed Hot-Path Shape

The authoritative runtime object should be a run-scoped dense state/flux pair:

- `SymbolRegistry`: frozen, sorted, run-scoped symbol universe from ADR-0022.
- `ArrayHotState`: dense slots keyed by `SymbolId`, split by state/flux class.
- `ArrayWritebackPayload`: writeback fields carry `SymbolId`, scalar value, and
  typed finite/range constraints.
- `LogicalSurfaceView`: materializes `HillslopeWritebackSurface` only at input,
  test, debug, failure, or output/publication boundaries.

The hot loop should pass array-backed borrowed state into kernels. Kernel phases
should either update slots directly or return id-backed writeback batches that
are applied to the array. No success-path update should allocate or compare
`BoundarySymbol` strings.

## Three Problems

### Export Seam

PERFIDX03 failed because the indexed representation was treated as authority
inside the lane but had to reconstruct a full `BTreeMap` at the kernel boundary.
That export was still in the daily path.

PERFARCH02 resolves this by moving the logical export boundary outward:

- seed dense state once from logical input surfaces;
- execute daily kernel phases against the dense store;
- materialize logical maps only when a legacy API, test assertion, HBP/parquet
  writer, or diagnostics consumer explicitly needs the logical surface.

The prototype measures a one-time export separately as `export_once`; it is not
part of the per-OFE-day candidate timing.

### Dual-Write

PERFIDX05 proved that writing the logical map and indexed mirror together is a
losing design. The dual-write cost exceeds the id lookup saving.

PERFARCH02 removes the mirror. The array is the only mutable hot-path authority.
Debug/shadow identity checks are allowed only behind validation flags or test
harnesses. They must not be part of normal timing gates.

### Typed Guards Without String Cost

Current writeback evaluation validates finite/domain semantics on
`WritebackField` values that still carry `BoundarySymbol`. The candidate keeps
the same finite and range semantics but stores the pre-resolved `SymbolId` and
numeric bounds in the hot payload.

The success path checks:

- finite scalar value;
- lower/upper domain constraints;
- known slot id;
- correct state/flux class.

The failure path lazily resolves `SymbolId` back to `BoundarySymbol` for status
subjects. The prototype validates that invalid payloads remain fail-closed and
that lazy failure subjects are resolved without making string formatting part of
the success path.

## Non-Goals

This package does not migrate production code. It does not change parquet/HBP
output schemas, science contracts, legacy comparator posture, irrigation
activation, or conservation requirements. It only establishes the shape and
floor evidence for a downstream migration.

## Design Decision

Use this shape for the next package, but enter production migration through a
single integrated WB11 pilot. The standalone floor prototype is strong enough
to reject the read-mirror architecture and proceed to an integrated pilot; it
is not strong enough to claim the full H2637 ratio by itself.
