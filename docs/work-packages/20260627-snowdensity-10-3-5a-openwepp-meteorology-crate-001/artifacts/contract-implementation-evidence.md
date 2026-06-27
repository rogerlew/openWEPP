# Contract Implementation Evidence

Status: complete
Evidence mode: Static

`SC-SNOWFREEZE-001` was amended before `crates/openwepp-meteorology` was
implemented.

## Amendment

- Static: contract version `90 -> 91`; `last_reviewed: 2026-06-27`.
- Static: science-contract registry row for `SC-SNOWFREEZE-001` updated to
  `last_reviewed 2026-06-27`.
- Static: added `REF-SNOWFREEZE-HARDER-POMEROY-2013` for the candidate
  hydrometeor-temperature method.
- Static: added candidate variables
  `hydrometeor_temperature`, `relative_humidity`, `dew_point_temperature`,
  `air_vapor_density`, `hydrometeor_saturation_vapor_density`,
  `harder_pomeroy_rain_fraction`, and `harder_pomeroy_snow_fraction`.
- Static: added `INV-SNOWFREEZE-064` and `OBL-SNOWFREEZE-P-039`.
- Static: added candidate API alias rows for
  `openwepp_meteorology::phase::*` and `openwepp_meteorology::psychrometrics::*`.
- Static: added invalid states blocking production `RST`/`stmtim`,
  parser/runfile/user selectors, output schemas, fixtures, compatibility
  runtime, and default activation.
- Static: added
  `SNOWDENSITY-10.3.5a Harder-Pomeroy Meteorology Crate Addendum`.

## Candidate Boundary

- Static: method is candidate-only and production-free in this package.
- Static: accepted units are Celsius temperatures, unit-interval humidity,
  `kg m^-3` vapor density, `kPa` vapor pressure, `J kg^-1` latent heat,
  `m^2 s^-1` diffusivity, `W m^-1 K^-1` conductivity, and unit-interval
  rain/snow fractions.
- Static: rollback/default isolation is explicit: production WEPP `RST`,
  `stmtim`, daily/hourly phase partition behavior, schemas, selectors, defaults,
  fixtures, compatibility runtime, melt, density, canopy, albedo, radiation, and
  frost physics remain unchanged.
- Static: no site-specific calibration or observed-phase/Jennings validation is
  authorized until a follow-on 10.3.5b amendment.
