# Unit-Safe Boundary Types

Status: Draft (ARCH09)  
Evidence: Static + Ran  
Ran evidence:
- `cargo fmt --manifest-path crates/openwepp-unit-boundary/Cargo.toml --check`
- `cargo clippy --manifest-path crates/openwepp-unit-boundary/Cargo.toml --all-targets -- -D warnings`
- `cargo test --manifest-path crates/openwepp-unit-boundary/Cargo.toml`

## Purpose

Define typed numerical boundary wrappers for high-risk hydrologic flux/state
surfaces so kernel and orchestrator interfaces reject non-finite and
domain-invalid values before mutation/aggregation paths consume them.

Implementation path:
- `/home/workdir/openWEPP/crates/openwepp-unit-boundary/src/lib.rs`

## Boundary Type Surface

| type | physical unit | domain guard | rationale |
| --- | --- | --- | --- |
| `RunoffDepthMillimeters` | `mm` | finite, `>= 0` | runoff depth cannot be negative; NaN/Inf are contract failures |
| `FlowRateCubicMetersPerSecond` | `m^3/s` | finite, `>= 0` | negative flow at this boundary class is modeled as a distinct directional surface, not a signed scalar |
| `StorageVolumeCubicMeters` | `m^3` | finite, `>= 0` | storage volume is bounded below by zero |
| `ProcessRateMillimetersPerHour` | `mm/hr` | finite, `>= 0` | rate surfaces at this boundary represent magnitudes |
| `SurfaceAreaSquareMeters` | `m^2` | finite, `> 0` | area-based conversions must reject zero/negative area to avoid divide-by-zero and sign inversion |

## Conversion Surface

The crate provides explicit constructors/helpers for unit conversions:

- runoff:
  - `RunoffDepthMillimeters::from_meters`
  - `RunoffDepthMillimeters::as_meters`
  - `RunoffDepthMillimeters::to_volume(area)`
- flow:
  - `FlowRateCubicMetersPerSecond::from_liters_per_second`
  - `FlowRateCubicMetersPerSecond::as_liters_per_second`
- storage:
  - `StorageVolumeCubicMeters::from_liters`
  - `StorageVolumeCubicMeters::as_liters`
  - `StorageVolumeCubicMeters::to_depth(area)`
- rate:
  - `ProcessRateMillimetersPerHour::from_meters_per_second`
  - `ProcessRateMillimetersPerHour::as_meters_per_second`

All conversion paths perform finite checks on inputs and conversion results.
No silent clamping/coercion/defaulting is permitted.

## Error Policy

The crate uses a typed `BoundaryError`:
- `NonFinite { boundary, value }`
- `BelowMinimum { boundary, value, minimum }`

This aligns with architecture policy from ARCH02/ARCH07:
- kernel/orchestrator boundaries are explicit
- numerical failures are surfaced as typed failures
- no implicit fallback paths are introduced

## ARCH03/ARCH07 Compatibility

- [DIRECT] ARCH03 status semantics require explicit finite/domain signaling.
- [DIRECT] ARCH07 writeback policy requires explicit reject behavior for invalid
  boundary payloads.
- [INFERENCE] `openwepp-unit-boundary` provides reusable, deterministic boundary
  constructors that reduce repeated scalar guard logic across kernel-entry and
  writeback surfaces without changing ownership semantics.
