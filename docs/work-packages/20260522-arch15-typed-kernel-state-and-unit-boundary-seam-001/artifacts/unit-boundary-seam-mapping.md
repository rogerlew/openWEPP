# Unit Boundary Seam Mapping

Static: code-level mapping from kernel seam value variants to
`openwepp-unit-boundary` types.
Ran: `cargo test --workspace` includes kernel contract + orchestrator +
integration coverage.
Status: complete.

## Seam Mapping Table

| BoundaryValue variant | Unit-boundary type | Canonical unit | Domain guard source |
|---|---|---|---|
| `RunoffDepthMillimeters` | `RunoffDepthMillimeters` | `mm` | `openwepp-unit-boundary` constructors |
| `FlowRateCubicMetersPerSecond` | `FlowRateCubicMetersPerSecond` | `m3/s` | `openwepp-unit-boundary` constructors |
| `StorageVolumeCubicMeters` | `StorageVolumeCubicMeters` | `m3` | `openwepp-unit-boundary` constructors |
| `ProcessRateMillimetersPerHour` | `ProcessRateMillimetersPerHour` | `mm/hr` | `openwepp-unit-boundary` constructors |
| `SurfaceAreaSquareMeters` | `SurfaceAreaSquareMeters` | `m2` | `openwepp-unit-boundary` constructors |
| `Scalar` | n/a (`f64`) | `scalar` | writeback finite/range closure checks |

## Wiring Evidence

- Kernel seam crate now depends on `openwepp-unit-boundary`:
  - `crates/openwepp-kernel-contract/Cargo.toml`
- Typed variants are first-class in kernel request/writeback surfaces:
  - `crates/openwepp-kernel-contract/src/lib.rs`
- Hillslope and watershed writeback surfaces now use typed maps:
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`

## Validation Evidence

- `openwepp-kernel-contract` unit test `accepts_unit_boundary_typed_values` passes.
- Integration test `kernel_writeback_contract` passes with typed key/value
  assertions on state/flux surfaces.
