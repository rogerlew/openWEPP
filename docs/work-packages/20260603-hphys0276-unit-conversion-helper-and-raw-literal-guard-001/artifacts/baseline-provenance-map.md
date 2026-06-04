# Baseline Provenance Map

Status: completed/HOLD
Evidence mode: Static

Static: HPHYS0276 did not change physics formulas. It replaced selected raw
unit conversion literals with named helpers while preserving baseline
directions and constants.

## Pinned Baseline

- Baseline: `/workdir/wepp-forest_260430_baseline`
- Commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Provenance Rows

| openWEPP helper | Baseline source | Preserved direction |
| --- | --- | --- |
| `langleys_per_day_to_megajoules_per_square_meter_per_day` | `src/sunmap.for:99`, `src/winter.for:185` | `Ly * 0.04184 -> MJ m^-2` |
| `megajoules_per_square_meter_per_day_to_uniform_hourly` | SIMIMPL28 no-sunrise fallback lineage | `daily MJ m^-2 / 24 -> hourly MJ m^-2` |
| `meters_per_second_to_legacy_miles_per_hour` | `src/melt.for:214` | `(vwind * 3600) / 1609 -> mph` |
| `meters_to_legacy_inches` | `src/melt.for:243` | `hrrain_m * 39.37 -> inch` |
| `legacy_inches_to_meters` | `src/melt.for:273-275` | `melt_in * 0.0254 -> m` |
| `water_equivalent_meters_to_snow_depth_meters` | `src/melt.for:295`, `src/snowd.for:198` | `swe_m * 1000 / density -> snow depth m` |
| `snow_depth_meters_to_water_equivalent_meters` | `src/melt.for:298`, `src/snowd.for:230,241,277` | `snow depth m * density / 1000 -> water m` |
| `water_depth_meters_to_snow_density_increment` | `src/snowd.for:262` | `water_m * 1000 / snodep -> density increment` |
| `meters_per_second_to_centimeters_per_hour` | `src/drain.for:151` | `m s^-1 * 3600 * 100 -> cm h^-1` |
| `meters_to_centimeters` / `centimeters_to_meters` | `src/drain.for:160,165,167,191,202` | drainage geometry and return conversion |
| `kilograms_per_cubic_meter_to_grams_per_cubic_centimeter` | `src/frostn.for:486-489` | `density kg m^-3 / 1000 -> g cm^-3` |

Ran: not-run; provenance is static source inspection.
