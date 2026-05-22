# Unit-Safe Boundary Types Contract

Status: Draft (ARCH09)  
Evidence: Static + Ran  
Ran evidence:
- `cargo test --manifest-path crates/openwepp-unit-boundary/Cargo.toml`

## 1. Contract Scope

This contract defines typed numerical boundary-value obligations for:
- runoff depth
- flow rate
- storage volume
- process rate
- conversion area (supporting conversion boundary)

It is implemented by:
- `/home/workdir/openWEPP/crates/openwepp-unit-boundary/src/lib.rs`

## 2. Normative Type Obligations

| type | canonical dimension | accepted domain | rejection mode |
| --- | --- | --- | --- |
| `RunoffDepthMillimeters` | length (`L`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `FlowRateCubicMetersPerSecond` | volumetric flow (`L^3/T`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `StorageVolumeCubicMeters` | volume (`L^3`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `ProcessRateMillimetersPerHour` | rate (`L/T`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `SurfaceAreaSquareMeters` | area (`L^2`) | finite and `> 0` | `BoundaryError::{NonFinite|BelowMinimum}` |

Normative requirements:
1. Constructors MUST reject `NaN`, `+Inf`, and `-Inf`.
2. Constructors MUST reject values below the domain minimum.
3. Conversion helpers MUST validate conversion results for non-finite outcomes.
4. No constructor or conversion helper may silently clamp, default, or coerce
   invalid values.

## 3. Conversion Contracts

### 3.1 Runoff
- `from_meters(m)` maps `m * 1000` to `mm`.
- `as_meters()` maps `mm / 1000` to `m`.
- `to_volume(area)` maps `depth_m * area_m2` to `m^3`.

### 3.2 Flow
- `from_liters_per_second(lps)` maps `lps / 1000` to `m^3/s`.
- `as_liters_per_second()` maps `m^3/s * 1000` to `L/s`.

### 3.3 Storage
- `from_liters(liters)` maps `liters / 1000` to `m^3`.
- `as_liters()` maps `m^3 * 1000` to `L`.
- `to_depth(area)` maps `(volume_m3 / area_m2) * 1000` to `mm`.

### 3.4 Process Rate
- `from_meters_per_second(mps)` maps `mps * 1000 * 3600` to `mm/hr`.
- `as_meters_per_second()` maps `mm/hr / 1000 / 3600` to `m/s`.

## 4. Invariants

Invariant IDs:
- `INV-USB-001`: every boundary wrapper instance is finite.
- `INV-USB-002`: runoff/flow/storage/rate wrappers are non-negative.
- `INV-USB-003`: area wrapper is strictly positive.
- `INV-USB-004`: overflow/invalid conversion intermediates are surfaced as
  typed errors (never silently accepted).

## 5. Test Evidence Mapping

| test | invariant coverage |
| --- | --- |
| `runoff_depth_rejects_non_finite` | `INV-USB-001` |
| `runoff_depth_rejects_negative` | `INV-USB-002` |
| `flow_rate_liters_round_trip` | `INV-USB-001`, `INV-USB-002` |
| `storage_volume_to_depth_rejects_zero_area` | `INV-USB-003` |
| `runoff_conversion_rejects_overflow` | `INV-USB-004` |
| `process_rate_rejects_non_finite_conversion` | `INV-USB-001`, `INV-USB-004` |

## 6. Boundary Naming and Alias Posture

- [DIRECT] This crate governs typed numeric wrappers and unit conversions only.
- [INFERENCE] Canonical WEPP symbol-to-field alias mapping remains owned by
  input/science contracts and kernel/orchestrator boundary payload contracts.
- [INFERENCE] Numeric wrappers can be applied to those symbol-bound payload
  fields without renaming canonical symbols.
