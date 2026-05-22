# Seam Impact and Compatibility Notes

Static: contract and orchestrator seam review complete.
Ran: compile/test gates confirm integration validity.
Status: complete.

## Seam Changes

1. Kernel requests now borrow orchestrator-owned state/flux surfaces.
- `HillslopeKernelRequest<'a>` fields now use references to typed maps.
- `WatershedKernelRequest<'a>` fields now use references for typed maps and contributor hillslope slice.

2. Kernel traits were updated to accept lifetime-parameterized request types.
- `HillslopeKernel::run_hillslope_phase(&HillslopeKernelRequest<'_>)`
- `WatershedKernel::run_watershed_node(&WatershedKernelRequest<'_>)`

3. Typed seam boundaries from ARCH15 are preserved.
- `BoundarySymbol`/`BoundaryValue` remain the state/flux map key/value contract.
- No `BTreeMap<String, f64>` kernel seam maps were reintroduced.

## Behavior / Invariant Compatibility

- Deterministic scheduler ordering is unchanged.
- Writeback accept/reject/apply semantics are unchanged.
- Status phase/classification routing is unchanged.
- Failure-class and closure handling remain explicit and typed.

## Compatibility Implications

- In-workspace implementers were migrated in this package.
- Out-of-tree kernel implementations must update trait signatures to include
  `HillslopeKernelRequest<'_>` / `WatershedKernelRequest<'_>`.
- Request lifetime is scoped to call execution, preventing ownership transfer of
  scheduler writeback surfaces across call boundaries.
