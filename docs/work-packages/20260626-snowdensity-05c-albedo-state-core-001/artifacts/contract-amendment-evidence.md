# Contract Amendment Evidence

Static:

- `SC-SNOWFREEZE-001` header moved to `contract_version: 78`.
- Added `snow_albedo`, `snow_albedo_model_id`,
  `snow_albedo_accumulated_positive_temperature_c_day`, and
  `snow_albedo_fresh_snow_reset_water_equiv_m` variables.
- Added `INV-SNOWFREEZE-054` for the SNOWDENSITY-05C albedo-state core.
- Added `OBL-SNOWFREEZE-P-029` for formula provenance, bounds, fresh-snow
  reset, fail-closed behavior, and no-runtime-wiring evidence.
- Added the `SNOWDENSITY-05C Albedo State Core Addendum`.

Disposition: the amendment authorizes a typed albedo state core only. It does
not authorize routed melt, default activation, radiation-source changes,
coefficient fitting, parser surfaces, or output schemas.
