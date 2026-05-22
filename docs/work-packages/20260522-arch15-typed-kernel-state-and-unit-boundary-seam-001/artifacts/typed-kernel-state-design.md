# Typed Kernel State Design

Static: source inspection of implemented ARCH15 seam refactor.
Ran: workspace gates executed and passing.
Status: complete.

## Design Summary

ARCH15 replaces stringly kernel seam state/writeback maps with typed boundary
symbol/value surfaces.

- Kernel seam symbol type:
  - `BoundarySymbol` in
    `crates/openwepp-kernel-contract/src/lib.rs`
  - wraps symbol labels (`String`) while preserving canonical legacy WEPP
    symbol continuity.
- Kernel seam value type:
  - `BoundaryValue` in
    `crates/openwepp-kernel-contract/src/lib.rs`
  - supports scalar and unit-safe variants.

## Seam Types Introduced

- `BoundarySymbol`
  - typed map key for state/flux surfaces and writeback fields.
- `BoundaryValue`
  - `Scalar(f64)`
  - `RunoffDepthMillimeters`
  - `FlowRateCubicMetersPerSecond`
  - `StorageVolumeCubicMeters`
  - `ProcessRateMillimetersPerHour`
  - `SurfaceAreaSquareMeters`

## Contract Surface Changes

- `WritebackField`
  - `symbol: BoundarySymbol`
  - `value: BoundaryValue`
- `HillslopeKernelRequest`
  - `state_surface: BTreeMap<BoundarySymbol, BoundaryValue>`
  - `flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>`
- `WatershedKernelRequest`
  - same typed surfaces as hillslope.
- `apply_kernel_writeback(...)`
  - now applies to typed maps and returns typed symbol lists.

## Invariant and Guard Continuity

Writeback accept/reject/apply semantics were preserved:

- finite checks still enforced (`INV-WRITEBACK-001`)
- domain/range checks still enforced (`INV-WRITEBACK-002..004`)
- deterministic symbol-order apply remains enforced by sorting
- orchestrator-owned mutable commit authority remains unchanged

The closure subject string now includes unit labels to improve diagnostics,
without changing boundary-class decision behavior.
