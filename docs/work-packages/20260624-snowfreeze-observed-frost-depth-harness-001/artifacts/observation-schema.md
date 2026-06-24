# Observation Schema

Evidence class: Static/Ran.

Status: queued.

Required normalized columns:

| Column | Type | Units | Required | Meaning |
| --- | --- | --- | --- | --- |
| `site_id` | string | unitless | yes | Fixture directory name. |
| `source_id` | string | unitless | yes | Stable source/dataset identifier. |
| `date` | ISO date | day | yes | Observation date in local site calendar. |
| `water_year` | integer | water year | yes | Hydrologic water year. |
| `method` | enum | unitless | yes | `frost_tube`, `soil_temperature_isotherm`, or other contract-approved method. |
| `authority_role` | enum | unitless | yes | `magnitude`, `timing_upper_bound`, `secondary`, or `blocked`. |
| `observed_frost_depth_m` | number/null | m | method-dependent | Frost-tube frozen/unfrozen boundary. |
| `observed_isotherm_depth_m` | number/null | m | method-dependent | Derived `0 degC` isotherm depth. |
| `observed_snow_depth_m` | number/null | m | no | Paired snow-depth control surface. |
| `censoring` | enum | unitless | yes | `none`, `left_onset`, `right_sensor_cap`, `method_cap`, or `unknown`. |
| `quality_flag` | string | unitless | yes | Source quality flag or normalized equivalent. |
| `source_record_id` | string | unitless | yes | Pointer back to raw source row. |

Validation rules:

- Depth units must be meters in normalized files.
- Frost-tube depth may be compared directly to modeled `frdp`.
- Soil-temperature isotherm depth may be used for onset/thaw timing and as a
  magnitude upper-bound only.
- `observed_snow_depth_m` absence makes frost defect attribution
  `UNRESOLVED` unless `SC-SNOWFREEZE-001` is amended with another
  snow-insulation control.
- Censored rows must be present but excluded from the metrics that censoring
  invalidates.
