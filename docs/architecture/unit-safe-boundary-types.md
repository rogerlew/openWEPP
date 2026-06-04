# Unit-Safe Boundary Types

Status: Active (ARCH09, HPHYS0280 amended)
Evidence: Static + Ran  
Ran evidence:
- `cargo fmt --manifest-path crates/openwepp-unit-boundary/Cargo.toml --check`
- `cargo clippy --manifest-path crates/openwepp-unit-boundary/Cargo.toml --all-targets -- -D warnings`
- `cargo test --manifest-path crates/openwepp-unit-boundary/Cargo.toml`
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract`
- `cargo test --test hphys0276_raw_unit_conversion_guard_contract`
- `tools/release/check_raw_unit_conversions.sh`

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
| `WaterDepthMeters` | `m` | finite, `>= 0` | climate and snow hourly depth seams preserve meters explicitly |
| `ElapsedTimeSeconds` | `s` | finite, `>= 0` | runtime storm-duration and hyetograph time seams are elapsed seconds |
| `HourOfDay` | `h` | finite, `[0, 24]` | breakpoint storm-start markers are hours of day |
| `LinearRateMetersPerSecond` | `m s^-1` | finite, `>= 0` | climate intensity and wind-speed seams preserve SI rate labels |
| `SolarRadiationLangleysPerDay` | `Ly d^-1` | finite, `>= 0` | daily climate `rad` remains legacy Langley/day at the parser/runtime seam |
| `SolarRadiationMegajoulesPerSquareMeterPerDay` | `MJ m^-2 d^-1` | finite, `>= 0` | internal daily radiation after explicit conversion can be typed without changing lineage |
| `SolarRadiationMegajoulesPerSquareMeterPerHour` | `MJ m^-2 h^-1` | finite, `>= 0` | SIMIMPL28 hourly winter forcing must not carry Langley-scale values under MJ labels |
| `TemperatureCelsius` | `degC` | finite | thermal forcing can be signed and must not be treated as dimensionless |
| `DirectionDegrees` | `deg` | finite, `[0, 360]` | climate wind-direction seams are angles, not scalar magnitudes or wind speeds |
| `DensityKilogramsPerCubicMeter` | `kg m^-3` | finite, `>= 0` | snow/freeze density seams require explicit density units |
| `FractionUnitInterval` | `dimensionless` | finite, `[0, 1]` | fractions remain dimensionless but bounded at construction |

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
- named first-wave conversion helpers in
  `openwepp_unit_boundary::conversions`:
  - `meters_to_millimeters`
  - `millimeters_to_meters`
  - `meters_to_centimeters`
  - `centimeters_to_meters`
  - `hours_to_seconds`
  - `seconds_to_hours`
  - `seconds_to_legacy_stmtim_hours`
  - `langleys_per_day_to_megajoules_per_square_meter_per_day`
  - `megajoules_per_square_meter_per_day_to_uniform_hourly`
  - `meters_per_second_to_centimeters_per_hour`
  - `meters_per_second_to_legacy_miles_per_hour`
  - `meters_to_legacy_inches`
  - `legacy_inches_to_meters`
  - `snow_depth_meters_to_water_equivalent_meters`
  - `water_equivalent_meters_to_snow_depth_meters`
  - `water_depth_meters_to_snow_density_increment`
  - `kilograms_per_cubic_meter_to_grams_per_cubic_centimeter`
  - `celsius_delta_to_fahrenheit_delta`

HPHYS0275 adds direct runtime constructors for already-canonical seam units.
Those constructors do not rescale values; they validate domain and carry unit
identity through `BoundaryValue::unit_label()`.
HPHYS0280 adds `DirectionDegrees` for climate wind direction and continues
typed publication for declared watershed-prefixed climate aliases plus selected
snow runtime/trace depth, density, temperature, rate, radiation, and fraction
families.

All conversion paths perform finite checks on inputs and conversion results.
No silent clamping/coercion/defaulting is permitted.

HPHYS0276 adds a source-level raw literal guard at
`tools/release/check_raw_unit_conversions.py`. The guard enforces selected
high-risk production files first and keeps broader conversion inventory as
follow-up evidence rather than silently allowing unreviewed literals.

## Error Policy

The crate uses a typed `BoundaryError`:
- `NonFinite { boundary, value }`
- `BelowMinimum { boundary, value, minimum }`
- `AboveMaximum { boundary, value, maximum }`

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
