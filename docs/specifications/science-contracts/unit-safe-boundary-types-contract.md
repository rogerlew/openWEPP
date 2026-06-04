# Unit-Safe Boundary Types Contract

Status: Active (ARCH09, HPHYS0280 amended)
Evidence: Static + Ran  
Ran evidence:
- `cargo test --manifest-path crates/openwepp-unit-boundary/Cargo.toml`
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract`
- `cargo test --test hphys0276_raw_unit_conversion_guard_contract`
- `tools/release/check_raw_unit_conversions.sh`

## 1. Contract Scope

This contract defines typed numerical boundary-value obligations for:
- runoff depth
- flow rate
- storage volume
- process rate
- conversion area (supporting conversion boundary)
- runtime water depth in meters
- elapsed seconds and hour-of-day markers
- linear rates in `m s^-1`
- daily and hourly solar radiation
- signed Celsius temperatures
- directional degrees
- density
- unit-interval fractions
- named directional unit conversions for first-wave high-risk runtime/kernel
  seams
- raw conversion literal guard enforcement for first-wave high-risk production
  paths

It is implemented by:
- `/home/workdir/openWEPP/crates/openwepp-unit-boundary/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`

## 2. Normative Type Obligations

| type | canonical dimension | accepted domain | rejection mode |
| --- | --- | --- | --- |
| `RunoffDepthMillimeters` | length (`L`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `FlowRateCubicMetersPerSecond` | volumetric flow (`L^3/T`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `StorageVolumeCubicMeters` | volume (`L^3`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `ProcessRateMillimetersPerHour` | rate (`L/T`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `SurfaceAreaSquareMeters` | area (`L^2`) | finite and `> 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `WaterDepthMeters` | length (`L`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `ElapsedTimeSeconds` | time (`T`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `HourOfDay` | time (`T`) | finite and `[0, 24]` | `BoundaryError::{NonFinite|BelowMinimum|AboveMaximum}` |
| `LinearRateMetersPerSecond` | rate (`L/T`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `SolarRadiationLangleysPerDay` | daily radiation | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `SolarRadiationMegajoulesPerSquareMeterPerDay` | daily radiation | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `SolarRadiationMegajoulesPerSquareMeterPerHour` | hourly radiation | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `TemperatureCelsius` | temperature | finite signed value | `BoundaryError::NonFinite` |
| `DirectionDegrees` | direction angle | finite and `[0, 360]` | `BoundaryError::{NonFinite|BelowMinimum|AboveMaximum}` |
| `DensityKilogramsPerCubicMeter` | density (`M/L^3`) | finite and `>= 0` | `BoundaryError::{NonFinite|BelowMinimum}` |
| `FractionUnitInterval` | fraction | finite and `[0, 1]` | `BoundaryError::{NonFinite|BelowMinimum|AboveMaximum}` |

Normative requirements:
1. Constructors MUST reject `NaN`, `+Inf`, and `-Inf`.
2. Constructors MUST reject values below the domain minimum.
3. Conversion helpers MUST validate conversion results for non-finite outcomes.
4. No constructor or conversion helper may silently clamp, default, or coerce
   invalid values.
5. HPHYS0275 migrated runtime producer seams MUST publish high-risk climate and
   SIMIMPL28 hourly symbols through typed `BoundaryValue` constructors rather
   than `BoundaryValue::scalar`.
6. HPHYS0276 first-wave production conversion seams MUST call named
   `openwepp-unit-boundary::conversions` helpers rather than spelling raw
   dimensional conversion literals directly.
7. HPHYS0280 continuation seams MUST publish climate `wind` direction,
   watershed-prefixed climate aliases for HPHYS0275 typed families, and
   selected snow runtime/trace state surfaces through typed `BoundaryValue`
   constructors rather than `BoundaryValue::scalar`.

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

### 3.5 HPHYS0276 Directional Helper Surface

The following helpers are canonical first-wave conversion authority:

| helper | source unit | target unit | formula/provenance |
| --- | --- | --- | --- |
| `meters_to_millimeters` | `m` | `mm` | `m * 1000`; WEPP water-balance output lineage (`watbal*.for`) |
| `millimeters_to_meters` | `mm` | `m` | `mm / 1000`; parser/runtime input normalization |
| `meters_to_centimeters` | `m` | `cm` | `m * 100`; WB19 drainage tile geometry |
| `centimeters_to_meters` | `cm` | `m` | `cm / 100`; WB19 drainage return conversion |
| `hours_to_seconds` | `h` | `s` | `h * 3600`; climate breakpoint timing |
| `seconds_to_hours` | `s` | `h` | `s / 3600`; SIMIMPL28 storm duration partitioning |
| `seconds_to_legacy_stmtim_hours` | `s` | legacy STMTIM `h` | `s * 0.00027778`; `/workdir/wepp-forest_260430_baseline/src/stmtim.for` line 49 |
| `meters_per_second_to_centimeters_per_hour` | `m s^-1` | `cm h^-1` | `m s^-1 * 3600 * 100`; WB19 drainage conductivity |
| `meters_per_second_to_legacy_miles_per_hour` | `m s^-1` | `mile h^-1` | `(m s^-1 * 3600) / 1609`; SIMIMPL29 legacy snowmelt wind term |
| `meters_to_legacy_inches` | `m` | `inch` | `m * 39.37`; SIMIMPL29 rain heat term |
| `legacy_inches_to_meters` | `inch` | `m` | `inch * 0.0254`; SIMIMPL29 melt water term |
| `langleys_per_day_to_megajoules_per_square_meter_per_day` | `Ly d^-1` | `MJ m^-2 d^-1` | `Ly d^-1 * 0.04184`; `/workdir/wepp-forest_260430_baseline/src/sunmap.for` line 99 |
| `megajoules_per_square_meter_per_day_to_uniform_hourly` | `MJ m^-2 d^-1` | `MJ m^-2 h^-1` | `daily / 24`; SIMIMPL28 no-sunrise hourly fallback |
| `snow_depth_meters_to_water_equivalent_meters` | `m`, `kg m^-3` | `m` | `depth_m * density_kg_m3 / 1000`; SIMIMPL29 snowpack density lineage |
| `water_equivalent_meters_to_snow_depth_meters` | `m`, `kg m^-3` | `m` | `swe_m * 1000 / density_kg_m3`; SIMIMPL29 snowpack depth lineage |
| `water_depth_meters_to_snow_density_increment` | `m`, `m` | `kg m^-3` | `water_m * 1000 / snow_depth_m`; SIMIMPL29 rain-retention density increment |
| `kilograms_per_cubic_meter_to_grams_per_cubic_centimeter` | `kg m^-3` | `g cm^-3` | `density_kg_m3 / 1000`; frost/snow conductivity density lineage |
| `celsius_delta_to_fahrenheit_delta` | `degC delta` | `degF delta` | `degC * 9 / 5`; SIMIMPL29 snowmelt temperature terms |

Every helper validates finite inputs and finite outputs. Helpers with
non-negative source domains reject negative values. Helpers with density or
depth divisors reject non-positive divisors.

## 4. Invariants

Invariant IDs:
- `INV-USB-001`: every boundary wrapper instance is finite.
- `INV-USB-002`: runoff/flow/storage/rate wrappers are non-negative.
- `INV-USB-003`: area wrapper is strictly positive.
- `INV-USB-004`: overflow/invalid conversion intermediates are surfaced as
  typed errors (never silently accepted).
- `INV-USB-005`: unit-interval and hour-of-day wrappers reject values above
  their declared maximum.
- `INV-USB-006`: migrated HPHYS0275 climate/winter-hourly runtime symbols
  carry non-scalar unit labels at publication.
- `INV-USB-007`: first-wave high-risk production conversion seams use named
  directional helpers and the raw-literal guard rejects unauthorized literals.
- `INV-USB-008`: HPHYS0280 continuation surfaces carry non-scalar unit labels:
  direction degrees, watershed-prefixed climate aliases, and selected snow
  runtime/trace depth/density/temperature/rate/fraction families.

## 5. Test Evidence Mapping

| test | invariant coverage |
| --- | --- |
| `runoff_depth_rejects_non_finite` | `INV-USB-001` |
| `runoff_depth_rejects_negative` | `INV-USB-002` |
| `flow_rate_liters_round_trip` | `INV-USB-001`, `INV-USB-002` |
| `storage_volume_to_depth_rejects_zero_area` | `INV-USB-003` |
| `runoff_conversion_rejects_overflow` | `INV-USB-004` |
| `process_rate_rejects_non_finite_conversion` | `INV-USB-001`, `INV-USB-004` |
| `fraction_unit_interval_rejects_above_one` | `INV-USB-005` |
| `hour_of_day_rejects_out_of_range` | `INV-USB-005` |
| `direction_degrees_rejects_out_of_range` | `INV-USB-008` |
| `hphys0275_daily_climate_surface_publishes_high_risk_symbols_as_typed_values` | `INV-USB-006`, `INV-USB-008` |
| `hphys0275_winter_hourly_surface_publishes_high_risk_symbols_as_typed_values` | `INV-USB-006` |
| `hphys0280_watershed_climate_surface_preserves_typed_units` | `INV-USB-008` |
| `clim05_snow_runtime_kernel_contract` HPHYS0280 assertions | `INV-USB-008` |
| `radiation_conversion_direction_uses_langley_to_mj_m2` | `INV-USB-004`, `INV-USB-007` |
| `legacy_snow_melt_conversion_helpers_preserve_direction` | `INV-USB-004`, `INV-USB-007` |
| `snow_density_depth_conversions_are_directional` | `INV-USB-004`, `INV-USB-007` |
| `hphys0276_raw_unit_conversion_guard_rejects_unauthorized_literal` | `INV-USB-007` |
| `hphys0276_raw_unit_conversion_guard_accepts_helper_based_source` | `INV-USB-007` |

## 6. Boundary Naming and Alias Posture

- [DIRECT] This crate governs typed numeric wrappers and unit conversions only.
- [INFERENCE] Canonical WEPP symbol-to-field alias mapping remains owned by
  input/science contracts and kernel/orchestrator boundary payload contracts.
- [INFERENCE] Numeric wrappers can be applied to those symbol-bound payload
  fields without renaming canonical symbols.
