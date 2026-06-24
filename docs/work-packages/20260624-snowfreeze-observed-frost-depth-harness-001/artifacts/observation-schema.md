# Observation Schema

Evidence class: Static/Ran.

Status: implemented in
`tests/fixtures/snowfreeze_observed/observations/manifest.json` and
`tools/snowfreeze_observed/observed_harness.py`.

Normalized CSV columns:

| Column | Type | Units | Required | Meaning |
| --- | --- | --- | --- | --- |
| `site_id` | string | unitless | yes | Fixture directory name. |
| `source_id` | string | unitless | yes | Stable source/dataset identifier. |
| `date` | ISO date | day | yes | Observation date in local site calendar. |
| `water_year` | integer | water year | yes | Hydrologic water year (`Oct-Dec => year + 1`). |
| `method` | enum | unitless | yes | `frost_tube` or `soil_temperature_zero_c_isotherm`. |
| `authority_role` | enum | unitless | yes | `magnitude`, `magnitude_limited_overlap`, or `timing_upper_bound`. |
| `observed_frost_depth_m` | number/null | m | method-dependent | Frost-tube bottom/front depth. |
| `observed_isotherm_depth_m` | number/null | m | method-dependent | Derived `0 degC` isotherm depth. |
| `observed_snow_depth_m` | number/null | m | no | Paired snow-depth control surface when present. |
| `censoring` | enum | unitless | yes | `none`, `right_censored_sensor_depth`, or source-specific marker. |
| `quality_flag` | string | unitless | yes | Source quality/count metadata. |
| `source_record_id` | string | unitless | yes | Pointer back to raw source row. |

Validation rules implemented:

- Depth units are meters in normalized files.
- Frost-tube rows require `observed_frost_depth_m`.
- Soil-temperature rows require `observed_isotherm_depth_m` and must leave
  `observed_frost_depth_m` empty.
- Soil-temperature isotherm rows are timing/upper-bound authority only.
- Right-censored sensor-depth rows are retained in the corpus but excluded from
  magnitude/upper-bound residual metrics.
- `observed_snow_depth_m` absence prevents a defect verdict unless an
  independently approved snow-depth control is added.
- WAT `Snow-Water` remains SWE and is not accepted as snow depth.
- CSV row counts must match `manifest.json`.
- CSV byte counts and SHA-256 checksums must match `manifest.json`.
- Every source listed in `manifest.json` must have a provenance JSON record.
- Provenance `normalized_files` byte counts and SHA-256 checksums must match the
  checked-in CSV files.
